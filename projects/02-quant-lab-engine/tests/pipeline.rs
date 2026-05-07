use quant_lab_engine::{
    BacktestConfig, EngineError, ExperimentGrid, MarketBar, MarketSeries, PipelineConfig,
    SignalConfig, run_grid, run_pipeline,
};

fn assert_close(left: f64, right: f64) {
    assert!((left - right).abs() < 1e-9, "left={left}, right={right}");
}

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

fn pipeline_config(fee_bps: f64) -> PipelineConfig {
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
            fee_bps,
        },
    }
}

#[test]
fn end_to_end_pipeline_produces_report() {
    let report = run_pipeline(&sample_series(), pipeline_config(2.0)).expect("pipeline runs");

    assert_eq!(report.symbol, "DEMO");
    assert_eq!(report.input_rows, 15);
    assert_eq!(report.factor_rows, 13);
    assert_eq!(report.signal_rows, 13);
    assert!(report.final_equity.is_finite());
    assert!(report.total_fees > 0.0);
    assert!(report.to_markdown().contains("Quant Lab Report"));
}

#[test]
fn fees_reduce_final_equity_for_same_strategy() {
    let no_fee = run_pipeline(&sample_series(), pipeline_config(0.0)).expect("pipeline runs");
    let with_fee = run_pipeline(&sample_series(), pipeline_config(10.0)).expect("pipeline runs");

    assert!(with_fee.final_equity < no_fee.final_equity);
    assert_eq!(no_fee.total_fees, 0.0);
    assert!(with_fee.total_fees > 0.0);
}

#[test]
fn experiment_grid_is_deterministic() {
    let grid = ExperimentGrid {
        windows: vec![3, 4],
        entry_z_values: vec![0.8, 1.0],
        fee_bps_values: vec![0.0, 2.0],
        seed: 42,
    };

    let first = run_grid(&sample_series(), &grid).expect("grid runs");
    let second = run_grid(&sample_series(), &grid).expect("grid runs");

    assert_eq!(first, second);
    assert_eq!(first.len(), 8);
    assert_eq!(first[0].id, "exp-0001-w3-z0.80-fee0.0");
}

#[test]
fn rejects_invalid_market_data() {
    let error = MarketSeries::new(
        "BAD",
        vec![MarketBar::new(1, 100.0), MarketBar::new(1, 101.0)],
    )
    .expect_err("duplicate timestamp should fail");

    assert_eq!(
        error,
        EngineError::InvalidMarketData {
            index: 1,
            reason: "timestamps must be strictly increasing"
        }
    );
}

#[test]
fn returns_are_simple_close_to_close_returns() {
    let series = MarketSeries::new(
        "RET",
        vec![
            MarketBar::new(1, 100.0),
            MarketBar::new(2, 110.0),
            MarketBar::new(3, 99.0),
        ],
    )
    .expect("valid data");
    let returns = series.returns();

    assert_close(returns[0], 0.1);
    assert_close(returns[1], -0.1);
}
