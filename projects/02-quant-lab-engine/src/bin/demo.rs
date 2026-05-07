use quant_lab_engine::{
    BacktestConfig, MarketBar, MarketSeries, PipelineConfig, SignalConfig, run_pipeline,
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

fn main() {
    let report = run_pipeline(
        &sample_series(),
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
        },
    )
    .expect("demo pipeline should run");

    println!("{}", report.to_markdown());
}
