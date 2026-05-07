use quant_lab_engine::{BacktestConfig, CompletionOutcome};
use quant_lab_engine::{
    BenchmarkDecision, BenchmarkObservation, BenchmarkPlan, BoundaryMode, Column, DType,
    EngineError, ExperimentConfig, ExperimentGrid, ExperimentResult, ExperimentResultBatch,
    ExperimentScheduler, MarketBar, MarketSeries, PipelineConfig, PythonArraySpec, SchedulerError,
    SignalConfig, TaskStatus, evaluate_benchmark, plan_python_boundary, run_experiment, run_grid,
    run_grid_parallel, run_pipeline_observed,
};

fn sample_series() -> MarketSeries {
    let closes = [
        100.0, 102.0, 101.0, 105.0, 103.0, 99.0, 100.0, 98.0, 104.0, 107.0, 103.0, 101.0, 106.0,
        108.0, 105.0,
    ];
    let bars = closes
        .iter()
        .enumerate()
        .map(|(index, close)| MarketBar::new(index as u64 + 1, *close))
        .collect();

    MarketSeries::new("DEMO", bars).expect("sample data is valid")
}

fn grid() -> ExperimentGrid {
    ExperimentGrid {
        windows: vec![3, 4],
        entry_z_values: vec![0.8, 1.0],
        fee_bps_values: vec![0.0, 2.0],
        seed: 42,
    }
}

fn pipeline_config() -> PipelineConfig {
    PipelineConfig {
        window: 3,
        signal: SignalConfig {
            entry_z: 0.8,
            exit_z: 0.2,
            long_weight: 1.0,
            short_weight: -1.0,
        },
        backtest: BacktestConfig {
            initial_cash: 100_000.0,
            fee_bps: 2.0,
        },
    }
}

fn benchmark_plan() -> BenchmarkPlan {
    BenchmarkPlan {
        name: "rolling_mean".to_string(),
        baseline_name: "naive".to_string(),
        candidate_name: "incremental".to_string(),
        input_rows: 200_000,
        repeat: 3,
        minimum_speedup: 1.5,
        max_noise_ratio: 1.25,
    }
}

#[test]
fn benchmark_report_accepts_candidate_only_with_evidence() {
    let report = evaluate_benchmark(
        benchmark_plan(),
        BenchmarkObservation {
            baseline_nanos: vec![1_000, 1_020, 980],
            candidate_nanos: vec![500, 510, 490],
            outputs_match: true,
        },
    )
    .expect("valid benchmark evidence");

    assert_eq!(report.decision, BenchmarkDecision::CandidateWins);
    assert!(report.speedup > 1.5);
}

#[test]
fn benchmark_report_keeps_baseline_when_speedup_is_too_small() {
    let report = evaluate_benchmark(
        benchmark_plan(),
        BenchmarkObservation {
            baseline_nanos: vec![1_000, 1_020, 980],
            candidate_nanos: vec![800, 810, 790],
            outputs_match: true,
        },
    )
    .expect("valid benchmark evidence");

    assert_eq!(report.decision, BenchmarkDecision::KeepBaseline);
    assert!(report.speedup < benchmark_plan().minimum_speedup);
}

#[test]
fn benchmark_report_is_inconclusive_with_insufficient_samples() {
    let report = evaluate_benchmark(
        benchmark_plan(),
        BenchmarkObservation {
            baseline_nanos: vec![1_000, 1_020],
            candidate_nanos: vec![500, 510],
            outputs_match: true,
        },
    )
    .expect("valid benchmark evidence");

    assert_eq!(report.decision, BenchmarkDecision::Inconclusive);
}

#[test]
fn benchmark_report_is_inconclusive_with_high_noise() {
    let report = evaluate_benchmark(
        benchmark_plan(),
        BenchmarkObservation {
            baseline_nanos: vec![1_000, 1_010, 2_000],
            candidate_nanos: vec![500, 505, 510],
            outputs_match: true,
        },
    )
    .expect("valid benchmark evidence");

    assert_eq!(report.decision, BenchmarkDecision::Inconclusive);
}

#[test]
fn benchmark_report_rejects_mismatched_outputs() {
    assert_eq!(
        evaluate_benchmark(
            benchmark_plan(),
            BenchmarkObservation {
                baseline_nanos: vec![1_000, 1_020, 980],
                candidate_nanos: vec![500, 510, 490],
                outputs_match: false,
            },
        ),
        Err(EngineError::InvalidConfig {
            context: "benchmark candidate output does not match baseline"
        })
    );
}

#[test]
fn benchmark_plan_validation_rejects_invalid_plan() {
    assert_eq!(
        BenchmarkPlan {
            repeat: 0,
            ..benchmark_plan()
        }
        .validate(),
        Err(EngineError::InvalidConfig {
            context: "benchmark input_rows and repeat must be greater than zero",
        })
    );

    assert_eq!(
        BenchmarkPlan {
            minimum_speedup: 1.0,
            ..benchmark_plan()
        }
        .validate(),
        Err(EngineError::InvalidConfig {
            context: "minimum_speedup must be finite and greater than 1",
        })
    );
}

#[test]
fn benchmark_report_rejects_empty_sample_vectors() {
    assert_eq!(
        evaluate_benchmark(
            benchmark_plan(),
            BenchmarkObservation {
                baseline_nanos: Vec::new(),
                candidate_nanos: vec![500, 510, 490],
                outputs_match: true,
            },
        ),
        Err(EngineError::EmptyInput {
            context: "baseline benchmark samples",
        })
    );

    assert_eq!(
        evaluate_benchmark(
            benchmark_plan(),
            BenchmarkObservation {
                baseline_nanos: vec![1_000, 1_020, 980],
                candidate_nanos: Vec::new(),
                outputs_match: true,
            },
        ),
        Err(EngineError::EmptyInput {
            context: "candidate benchmark samples",
        })
    );
}

#[test]
fn parallel_grid_matches_sequential_order() {
    let series = sample_series();
    let sequential = run_grid(&series, &grid()).expect("sequential grid runs");
    let parallel = run_grid_parallel(&series, &grid(), 3).expect("parallel grid runs");

    assert_eq!(parallel, sequential);
    assert_eq!(parallel[0].id, "exp-0001-w3-z0.80-fee0.0");
}

#[test]
fn python_boundary_plans_borrow_only_for_clean_contiguous_f64() {
    let borrow = plan_python_boundary(
        PythonArraySpec {
            dtype: DType::F64,
            rows: 100,
            cols: 2,
            contiguous: true,
            contains_nan: false,
        },
        false,
    )
    .expect("valid boundary");

    assert_eq!(borrow.mode, BoundaryMode::Borrow);
    assert!(borrow.release_gil_around_kernel);

    let copy = plan_python_boundary(
        PythonArraySpec {
            dtype: DType::F32,
            rows: 100,
            cols: 2,
            contiguous: true,
            contains_nan: false,
        },
        false,
    )
    .expect("valid boundary");

    assert_eq!(copy.mode, BoundaryMode::Copy);
}

#[test]
fn python_boundary_rejects_empty_shape_and_nan() {
    assert_eq!(
        plan_python_boundary(
            PythonArraySpec {
                dtype: DType::F64,
                rows: 0,
                cols: 2,
                contiguous: true,
                contains_nan: false,
            },
            false,
        ),
        Err(EngineError::InvalidConfig {
            context: "python array shape must be non-empty"
        })
    );

    assert_eq!(
        plan_python_boundary(
            PythonArraySpec {
                dtype: DType::F64,
                rows: 1,
                cols: 2,
                contiguous: true,
                contains_nan: true,
            },
            false,
        ),
        Err(EngineError::InvalidMarketData {
            index: 0,
            reason: "python array contains NaN"
        })
    );
}

#[test]
fn columnar_result_batch_projects_and_filters_results() {
    let results = run_grid(&sample_series(), &grid()).expect("grid runs");
    let batch = ExperimentResultBatch::from_results(&results).expect("batch builds");
    let projected = batch
        .project(&["id", "total_return"])
        .expect("projection works");

    assert_eq!(projected.rows, results.len());
    assert_eq!(projected.schema.len(), 2);

    let filtered = batch
        .filter_total_return_gt(0.0)
        .expect("filter should work");
    let Column::F64(returns) = &filtered.columns[6] else {
        panic!("total_return should be f64");
    };

    assert!(returns.iter().all(|value| *value > 0.0));
}

#[test]
fn observed_pipeline_records_metrics_and_spans() {
    let observed =
        run_pipeline_observed(&sample_series(), pipeline_config()).expect("observed pipeline runs");

    assert_eq!(observed.metrics.counter("pipeline_runs_total"), 1);
    assert_eq!(observed.metrics.counter("pipeline_input_rows_total"), 15);
    assert_eq!(
        observed.metrics.counter("pipeline_signal_rows_total"),
        observed.report.signal_rows as u64
    );
    assert_eq!(
        observed.metrics.gauge("pipeline_final_equity"),
        Some(observed.report.final_equity)
    );
    assert_eq!(observed.spans[0].name, "pipeline_start");
    assert_eq!(observed.spans[1].name, "pipeline_finish");
}

#[test]
fn scheduler_handles_lease_expiry_completion_and_idempotency() {
    let series = sample_series();
    let mut scheduler = ExperimentScheduler::default();
    let config = ExperimentConfig {
        id: "exp-demo".to_string(),
        window: 3,
        entry_z: 0.8,
        exit_z: 0.2,
        fee_bps: 2.0,
        seed: 42,
    };

    scheduler
        .add_experiment(config, 2)
        .expect("valid experiment");
    let first = scheduler.lease_next("worker-a", 10).expect("leased");
    scheduler.advance_to(10);
    assert_eq!(scheduler.status("exp-demo"), Some(&TaskStatus::Pending));

    let second = scheduler.lease_next("worker-b", 10).expect("leased");
    assert_eq!(second.attempt, first.attempt + 1);

    let result = run_experiment(&series, &second.config).expect("experiment runs");
    assert_eq!(
        scheduler.complete(&second.id, second.attempt, result.clone()),
        Ok(CompletionOutcome::Stored)
    );
    assert_eq!(
        scheduler.complete(&second.id, second.attempt, result),
        Ok(CompletionOutcome::DuplicateIgnored)
    );
    assert!(scheduler.result("exp-demo").is_some());
}

#[test]
fn scheduler_rejects_conflicting_duplicate_result() {
    let mut scheduler = ExperimentScheduler::default();
    let config = ExperimentConfig {
        id: "exp-demo".to_string(),
        window: 3,
        entry_z: 0.8,
        exit_z: 0.2,
        fee_bps: 2.0,
        seed: 42,
    };

    scheduler
        .add_experiment(config.clone(), 2)
        .expect("valid experiment");
    let lease = scheduler.lease_next("worker-a", 10).expect("leased");
    let result = ExperimentResult {
        id: config.id,
        window: 3,
        entry_z: 0.8,
        fee_bps: 2.0,
        seed: 42,
        final_equity: 100.0,
        total_return: 0.0,
        max_drawdown: 0.0,
    };

    scheduler
        .complete(&lease.id, lease.attempt, result)
        .expect("stored");

    let conflicting = ExperimentResult {
        id: "exp-demo".to_string(),
        window: 3,
        entry_z: 0.8,
        fee_bps: 2.0,
        seed: 42,
        final_equity: 101.0,
        total_return: 0.01,
        max_drawdown: 0.0,
    };

    assert_eq!(
        scheduler.complete(&lease.id, lease.attempt, conflicting),
        Err(SchedulerError::ResultConflict)
    );
}
