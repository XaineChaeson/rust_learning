#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Pending,
    Running,
    Succeeded,
    Failed,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub id: String,
    pub payload: usize,
    pub status: TaskStatus,
}

pub fn run_tasks(tasks: &mut [Task]) -> Vec<usize> {
    let mut results = Vec::new();

    for task in tasks {
        task.status = TaskStatus::Running;
        results.push(task.payload * task.payload);
        task.status = TaskStatus::Succeeded;
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scheduler_updates_task_statuses() {
        let mut tasks = vec![
            Task {
                id: String::from("a"),
                payload: 2,
                status: TaskStatus::Pending,
            },
            Task {
                id: String::from("b"),
                payload: 3,
                status: TaskStatus::Pending,
            },
        ];

        assert_eq!(run_tasks(&mut tasks), vec![4, 9]);
        assert!(
            tasks
                .iter()
                .all(|task| task.status == TaskStatus::Succeeded)
        );
    }
}
