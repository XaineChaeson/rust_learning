pub mod backtest;
pub mod benchmark;
pub mod columnar;
pub mod data;
pub mod error;
pub mod experiment;
pub mod factor;
pub mod observability;
pub mod parallel;
pub mod pipeline;
pub mod python_boundary;
pub mod report;
pub mod scheduler;
pub mod signal;

pub use backtest::{BacktestConfig, BacktestReport, BacktestStep, run_backtest};
pub use benchmark::{
    BenchmarkDecision, BenchmarkObservation, BenchmarkPlan, BenchmarkReport, BenchmarkSummary,
    evaluate_benchmark, summarize_samples,
};
pub use columnar::{Column, ColumnType, ExperimentResultBatch, Field};
pub use data::{MarketBar, MarketPartition, MarketSeries, MarketUniverse};
pub use error::EngineError;
pub use experiment::{
    ExperimentConfig, ExperimentGrid, ExperimentResult, ExperimentTaskKey, run_experiment, run_grid,
};
pub use factor::{FactorSet, compute_factor_set};
pub use observability::{
    MetricRegistry, ObservedPipelineReport, SpanRecord, run_pipeline_observed,
};
pub use parallel::{partition_ranges, run_grid_parallel};
pub use pipeline::{PipelineConfig, run_pipeline};
pub use python_boundary::{
    BoundaryMode, BoundaryPlan, DType, PythonArraySpec, plan_python_boundary,
};
pub use report::PipelineReport;
pub use scheduler::{
    CompletionOutcome, ExperimentScheduler, LeasedExperiment, SchedulerError, TaskStatus,
};
pub use signal::{Signal, SignalConfig, SignalReason, generate_mean_reversion_signals};
