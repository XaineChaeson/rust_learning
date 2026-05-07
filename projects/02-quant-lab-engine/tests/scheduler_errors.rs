use quant_lab_engine::{
    ExperimentConfig, ExperimentResult, ExperimentScheduler, SchedulerError, TaskStatus,
};

fn config(id: &str) -> ExperimentConfig {
    ExperimentConfig {
        id: id.to_string(),
        window: 3,
        entry_z: 0.8,
        exit_z: 0.2,
        fee_bps: 1.0,
        seed: 42,
    }
}

fn result(id: &str, final_equity: f64) -> ExperimentResult {
    ExperimentResult {
        id: id.to_string(),
        window: 3,
        entry_z: 0.8,
        fee_bps: 1.0,
        seed: 42,
        final_equity,
        total_return: final_equity / 1_000.0 - 1.0,
        max_drawdown: 0.0,
    }
}

#[test]
fn scheduler_rejects_invalid_tasks() {
    let mut scheduler = ExperimentScheduler::default();

    assert_eq!(
        scheduler.add_experiment(config(""), 1),
        Err(SchedulerError::InvalidTask)
    );
    assert_eq!(
        scheduler.add_experiment(config("exp-a"), 0),
        Err(SchedulerError::InvalidTask)
    );
}

#[test]
fn scheduler_rejects_completion_for_unknown_task() {
    let mut scheduler = ExperimentScheduler::default();

    assert_eq!(
        scheduler.complete("missing", 1, result("missing", 1_000.0)),
        Err(SchedulerError::UnknownTask)
    );
}

#[test]
fn scheduler_rejects_wrong_attempt_completion() {
    let mut scheduler = ExperimentScheduler::default();
    scheduler
        .add_experiment(config("exp-a"), 2)
        .expect("valid task");
    let lease = scheduler.lease_next("worker-a", 10).expect("leased");

    assert_eq!(
        scheduler.complete(&lease.id, lease.attempt + 1, result(&lease.id, 1_000.0)),
        Err(SchedulerError::AttemptMismatch)
    );
}

#[test]
fn scheduler_rejects_completion_when_task_is_not_leased() {
    let mut scheduler = ExperimentScheduler::default();
    scheduler
        .add_experiment(config("exp-a"), 2)
        .expect("valid task");

    assert_eq!(
        scheduler.complete("exp-a", 1, result("exp-a", 1_000.0)),
        Err(SchedulerError::AttemptMismatch)
    );
}

#[test]
fn scheduler_marks_expired_last_attempt_as_failed() {
    let mut scheduler = ExperimentScheduler::default();
    scheduler
        .add_experiment(config("exp-a"), 1)
        .expect("valid task");

    let lease = scheduler.lease_next("worker-a", 10).expect("leased");
    assert_eq!(lease.attempt, 1);

    scheduler.advance_to(10);

    assert_eq!(scheduler.status("exp-a"), Some(&TaskStatus::Failed));
    assert!(scheduler.lease_next("worker-b", 10).is_none());
}
