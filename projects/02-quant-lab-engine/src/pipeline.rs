use crate::{
    BacktestConfig, EngineError, MarketSeries, PipelineReport, SignalConfig, compute_factor_set,
    generate_mean_reversion_signals, run_backtest,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PipelineConfig {
    pub window: usize,
    pub signal: SignalConfig,
    pub backtest: BacktestConfig,
}

pub fn run_pipeline(
    series: &MarketSeries,
    config: PipelineConfig,
) -> Result<PipelineReport, EngineError> {
    let factors = compute_factor_set(series, config.window)?;
    let signals = generate_mean_reversion_signals(
        series,
        &factors.rolling_zscore,
        config.window,
        config.signal,
    )?;
    let backtest = run_backtest(series, &signals, config.backtest)?;

    Ok(PipelineReport {
        symbol: series.symbol.clone(),
        input_rows: series.len(),
        factor_rows: factors.rolling_zscore.len(),
        signal_rows: signals.len(),
        final_equity: backtest.final_equity,
        total_return: backtest.total_return,
        max_drawdown: backtest.max_drawdown,
        turnover: backtest.turnover,
        total_fees: backtest.total_fees,
    })
}
