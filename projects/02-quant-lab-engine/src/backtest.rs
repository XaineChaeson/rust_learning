use crate::{EngineError, MarketSeries, Signal};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BacktestConfig {
    pub initial_cash: f64,
    pub fee_bps: f64,
}

impl BacktestConfig {
    pub fn validate(&self) -> Result<(), EngineError> {
        if !self.initial_cash.is_finite() || self.initial_cash <= 0.0 {
            return Err(EngineError::InvalidConfig {
                context: "initial_cash must be positive and finite",
            });
        }

        if !self.fee_bps.is_finite() || self.fee_bps < 0.0 {
            return Err(EngineError::InvalidConfig {
                context: "fee_bps must be non-negative and finite",
            });
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BacktestStep {
    pub timestamp: u64,
    pub close: f64,
    pub target_weight: f64,
    pub position_units: f64,
    pub cash: f64,
    pub equity: f64,
    pub fee_paid: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BacktestReport {
    pub steps: Vec<BacktestStep>,
    pub final_equity: f64,
    pub total_return: f64,
    pub max_drawdown: f64,
    pub turnover: f64,
    pub total_fees: f64,
}

pub fn run_backtest(
    series: &MarketSeries,
    signals: &[Signal],
    config: BacktestConfig,
) -> Result<BacktestReport, EngineError> {
    config.validate()?;

    if signals.is_empty() {
        return Err(EngineError::EmptyInput { context: "signals" });
    }

    let mut cash = config.initial_cash;
    let mut position_units = 0.0;
    let mut peak_equity = config.initial_cash;
    let mut max_drawdown = 0.0;
    let mut turnover = 0.0;
    let mut total_fees = 0.0;
    let mut steps = Vec::with_capacity(signals.len());

    for signal in signals {
        let close = series
            .close_at(signal.timestamp)
            .ok_or(EngineError::MissingTimestamp {
                timestamp: signal.timestamp,
            })?;

        let equity_before_trade = cash + position_units * close;
        let desired_notional = equity_before_trade * signal.target_weight;
        let desired_units = desired_notional / close;
        let trade_units = desired_units - position_units;
        let traded_notional = trade_units.abs() * close;
        let fee_paid = traded_notional * config.fee_bps / 10_000.0;

        cash -= trade_units * close + fee_paid;
        position_units = desired_units;

        let equity = cash + position_units * close;
        peak_equity = peak_equity.max(equity);

        if peak_equity > 0.0 {
            max_drawdown = f64::max(max_drawdown, (peak_equity - equity) / peak_equity);
        }

        if equity_before_trade.abs() > f64::EPSILON {
            turnover += traded_notional / equity_before_trade.abs();
        }

        total_fees += fee_paid;
        steps.push(BacktestStep {
            timestamp: signal.timestamp,
            close,
            target_weight: signal.target_weight,
            position_units,
            cash,
            equity,
            fee_paid,
        });
    }

    let final_equity = steps
        .last()
        .expect("signals is known to be non-empty")
        .equity;

    Ok(BacktestReport {
        steps,
        final_equity,
        total_return: final_equity / config.initial_cash - 1.0,
        max_drawdown,
        turnover,
        total_fees,
    })
}
