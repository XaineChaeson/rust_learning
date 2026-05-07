use std::collections::BTreeMap;

use crate::{ExperimentConfig, ExperimentResult};

#[derive(Debug, Clone, PartialEq)]
pub struct LeasedExperiment {
    pub id: String,
    pub attempt: u32,
    pub config: ExperimentConfig,
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

#[derive(Debug, Clone, PartialEq)]
struct TaskState {
    config: ExperimentConfig,
    status: TaskStatus,
    attempts_used: u32,
    max_attempts: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchedulerError {
    InvalidTask,
    UnknownTask,
    AttemptMismatch,
    ResultConflict,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionOutcome {
    Stored,
    DuplicateIgnored,
}

#[derive(Debug, Clone, Default)]
pub struct ExperimentScheduler {
    now: u64,
    tasks: BTreeMap<String, TaskState>,
    results: BTreeMap<String, ExperimentResult>,
}

impl ExperimentScheduler {
    pub fn add_experiment(
        &mut self,
        config: ExperimentConfig,
        max_attempts: u32,
    ) -> Result<(), SchedulerError> {
        if config.id.trim().is_empty() || max_attempts == 0 {
            return Err(SchedulerError::InvalidTask);
        }

        self.tasks.insert(
            config.id.clone(),
            TaskState {
                config,
                status: TaskStatus::Pending,
                attempts_used: 0,
                max_attempts,
            },
        );
        Ok(())
    }

    pub fn advance_to(&mut self, now: u64) {
        self.now = now;
        self.expire_leases();
    }

    pub fn lease_next(&mut self, worker: &str, ttl: u64) -> Option<LeasedExperiment> {
        let id = self.tasks.iter().find_map(|(id, state)| {
            matches!(state.status, TaskStatus::Pending).then(|| id.clone())
        })?;
        let state = self.tasks.get_mut(&id).expect("id came from map");
        let attempt = state.attempts_used + 1;
        state.attempts_used = attempt;
        state.status = TaskStatus::Leased {
            worker: worker.to_string(),
            attempt,
            expires_at: self.now + ttl,
        };

        Some(LeasedExperiment {
            id,
            attempt,
            config: state.config.clone(),
        })
    }

    pub fn complete(
        &mut self,
        id: &str,
        attempt: u32,
        result: ExperimentResult,
    ) -> Result<CompletionOutcome, SchedulerError> {
        if let Some(existing) = self.results.get(id) {
            if existing == &result {
                return Ok(CompletionOutcome::DuplicateIgnored);
            }

            return Err(SchedulerError::ResultConflict);
        }

        let state = self.tasks.get_mut(id).ok_or(SchedulerError::UnknownTask)?;
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
        self.results.insert(id.to_string(), result);
        Ok(CompletionOutcome::Stored)
    }

    pub fn status(&self, id: &str) -> Option<&TaskStatus> {
        self.tasks.get(id).map(|state| &state.status)
    }

    pub fn result(&self, id: &str) -> Option<&ExperimentResult> {
        self.results.get(id)
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

            if attempt >= state.max_attempts {
                state.status = TaskStatus::Failed;
            } else {
                state.status = TaskStatus::Pending;
            }
        }
    }
}
