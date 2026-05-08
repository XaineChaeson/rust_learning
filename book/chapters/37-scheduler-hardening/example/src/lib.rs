use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub id: String,
    pub payload: String,
    pub max_attempts: u32,
}

impl Task {
    pub fn new(id: &str, payload: &str, max_attempts: u32) -> Self {
        Self {
            id: id.to_string(),
            payload: payload.to_string(),
            max_attempts,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Leased {
        worker: String,
        attempt: u32,
        expires_at: u64,
    },
    Completed,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskState {
    pub task: Task,
    pub status: TaskStatus,
    pub attempts_used: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchedulerError {
    InvalidTask,
    UnknownTask,
    AttemptMismatch,
    ResultConflict,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompletionOutcome {
    Stored,
    DuplicateIgnored,
}

#[derive(Debug, Clone, Default)]
pub struct Scheduler {
    now: u64,
    tasks: BTreeMap<String, TaskState>,
    results: BTreeMap<String, String>,
}

impl Scheduler {
    pub fn add_task(&mut self, task: Task) -> Result<(), SchedulerError> {
        if task.id.trim().is_empty() || task.max_attempts == 0 {
            return Err(SchedulerError::InvalidTask);
        }

        self.tasks.insert(
            task.id.clone(),
            TaskState {
                task,
                status: TaskStatus::Pending,
                attempts_used: 0,
            },
        );
        Ok(())
    }

    pub fn advance_to(&mut self, now: u64) {
        self.now = now;
        self.expire_leases();
    }

    pub fn lease_next(&mut self, worker: &str, ttl: u64) -> Option<(String, u32)> {
        let task_id = self.tasks.iter().find_map(|(id, state)| {
            matches!(state.status, TaskStatus::Pending).then(|| id.clone())
        })?;
        let state = self
            .tasks
            .get_mut(&task_id)
            .expect("task id came from the map");
        let attempt = match state.status {
            TaskStatus::Pending => state.attempts_used + 1,
            TaskStatus::Leased { attempt, .. } => attempt,
            TaskStatus::Completed | TaskStatus::Failed => return None,
        };

        state.attempts_used = attempt;
        state.status = TaskStatus::Leased {
            worker: worker.to_string(),
            attempt,
            expires_at: self.now + ttl,
        };

        Some((task_id, attempt))
    }

    pub fn complete(
        &mut self,
        task_id: &str,
        attempt: u32,
        result: &str,
    ) -> Result<CompletionOutcome, SchedulerError> {
        if let Some(existing) = self.results.get(task_id) {
            if existing == result {
                return Ok(CompletionOutcome::DuplicateIgnored);
            }

            return Err(SchedulerError::ResultConflict);
        }

        let state = self
            .tasks
            .get_mut(task_id)
            .ok_or(SchedulerError::UnknownTask)?;

        let TaskStatus::Leased {
            attempt: leased_attempt,
            ..
        } = state.status
        else {
            return Err(SchedulerError::AttemptMismatch);
        };

        if leased_attempt != attempt {
            return Err(SchedulerError::AttemptMismatch);
        }

        state.status = TaskStatus::Completed;
        self.results.insert(task_id.to_string(), result.to_string());
        Ok(CompletionOutcome::Stored)
    }

    pub fn status(&self, task_id: &str) -> Option<&TaskStatus> {
        self.tasks.get(task_id).map(|state| &state.status)
    }

    fn expire_leases(&mut self) {
        for state in self.tasks.values_mut() {
            let TaskStatus::Leased {
                attempt,
                expires_at,
                ..
            } = state.status
            else {
                continue;
            };

            if expires_at > self.now {
                continue;
            }

            if attempt >= state.task.max_attempts {
                state.status = TaskStatus::Failed;
            } else {
                state.status = TaskStatus::Pending;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expired_lease_returns_task_to_pending_until_max_attempts() {
        let mut scheduler = Scheduler::default();
        scheduler
            .add_task(Task::new("task-1", "run factor", 2))
            .expect("valid task");

        assert_eq!(
            scheduler.lease_next("worker-a", 10),
            Some(("task-1".to_string(), 1))
        );
        scheduler.advance_to(10);
        assert_eq!(scheduler.status("task-1"), Some(&TaskStatus::Pending));

        assert_eq!(
            scheduler.lease_next("worker-b", 10),
            Some(("task-1".to_string(), 2))
        );
    }

    #[test]
    fn completion_is_idempotent_for_same_result() {
        let mut scheduler = Scheduler::default();
        scheduler
            .add_task(Task::new("task-1", "run factor", 3))
            .expect("valid task");
        let (_, attempt) = scheduler.lease_next("worker-a", 10).expect("leased");

        assert_eq!(
            scheduler.complete("task-1", attempt, "ok"),
            Ok(CompletionOutcome::Stored)
        );
        assert_eq!(
            scheduler.complete("task-1", attempt, "ok"),
            Ok(CompletionOutcome::DuplicateIgnored)
        );
    }

    #[test]
    fn conflicting_duplicate_result_is_rejected() {
        let mut scheduler = Scheduler::default();
        scheduler
            .add_task(Task::new("task-1", "run factor", 3))
            .expect("valid task");
        let (_, attempt) = scheduler.lease_next("worker-a", 10).expect("leased");

        scheduler
            .complete("task-1", attempt, "ok")
            .expect("first result stored");

        assert_eq!(
            scheduler.complete("task-1", attempt, "different"),
            Err(SchedulerError::ResultConflict)
        );
    }
}
