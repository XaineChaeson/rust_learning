use quant_lab_engine::{
    BacktestConfig, EngineError, MarketBar, MarketSeries, Signal, SignalConfig, SignalReason,
    generate_mean_reversion_signals, run_backtest,
};

fn assert_close(left: f64, right: f64) {
    assert!((left - right).abs() < 1e-9, "left={left}, right={right}");
}

fn series_from_closes(closes: &[f64]) -> MarketSeries {
    let bars = closes
        .iter()
        .enumerate()
        .map(|(index, close)| MarketBar::new(index as u64 + 1, *close))
        .collect();

    MarketSeries::new("TST", bars).expect("test market data is valid")
}

fn signal_config() -> SignalConfig {
    SignalConfig {
        entry_z: 1.0,
        exit_z: 0.2,
        long_weight: 1.0,
        short_weight: -1.0,
    }
}

fn backtest_config() -> BacktestConfig {
    BacktestConfig {
        initial_cash: 1_000.0,
        fee_bps: 0.0,
    }
}

#[test]
fn signals_cover_long_short_hold_exit_and_timestamp_alignment() {
    let series = series_from_closes(&[100.0, 101.0, 102.0, 103.0, 104.0, 105.0]);
    let signals =
        generate_mean_reversion_signals(&series, &[-1.2, 1.2, 0.5, 0.0], 3, signal_config())
            .expect("signals should generate");

    assert_eq!(
        signals,
        vec![
            Signal {
                timestamp: 3,
                target_weight: 1.0,
                reason: SignalReason::LongMeanReversion,
            },
            Signal {
                timestamp: 4,
                target_weight: -1.0,
                reason: SignalReason::ShortMeanReversion,
            },
            Signal {
                timestamp: 5,
                target_weight: -1.0,
                reason: SignalReason::Hold,
            },
            Signal {
                timestamp: 6,
                target_weight: 0.0,
                reason: SignalReason::ExitToFlat,
            },
        ]
    );
}

#[test]
fn signal_config_rejects_invalid_thresholds() {
    assert_eq!(
        SignalConfig {
            entry_z: 0.0,
            ..signal_config()
        }
        .validate(),
        Err(EngineError::InvalidConfig {
            context: "entry_z must be positive and finite",
        })
    );

    assert_eq!(
        SignalConfig {
            entry_z: 1.0,
            exit_z: 1.5,
            ..signal_config()
        }
        .validate(),
        Err(EngineError::InvalidConfig {
            context: "exit_z must be finite and in [0, entry_z]",
        })
    );
}

#[test]
fn signals_reject_non_finite_zscore() {
    let series = series_from_closes(&[100.0, 101.0, 102.0]);

    assert_eq!(
        generate_mean_reversion_signals(&series, &[f64::INFINITY], 3, signal_config()),
        Err(EngineError::InvalidMarketData {
            index: 0,
            reason: "zscore must be finite",
        })
    );
}

#[test]
fn backtest_rejects_empty_signals() {
    let series = series_from_closes(&[100.0, 101.0]);

    assert_eq!(
        run_backtest(&series, &[], backtest_config()),
        Err(EngineError::EmptyInput { context: "signals" })
    );
}

#[test]
fn backtest_rejects_invalid_config() {
    assert_eq!(
        BacktestConfig {
            initial_cash: 0.0,
            fee_bps: 0.0,
        }
        .validate(),
        Err(EngineError::InvalidConfig {
            context: "initial_cash must be positive and finite",
        })
    );

    assert_eq!(
        BacktestConfig {
            initial_cash: 1_000.0,
            fee_bps: -1.0,
        }
        .validate(),
        Err(EngineError::InvalidConfig {
            context: "fee_bps must be non-negative and finite",
        })
    );
}

#[test]
fn backtest_rejects_missing_timestamp() {
    let series = series_from_closes(&[100.0, 101.0]);
    let signals = [Signal {
        timestamp: 99,
        target_weight: 1.0,
        reason: SignalReason::LongMeanReversion,
    }];

    assert_eq!(
        run_backtest(&series, &signals, backtest_config()),
        Err(EngineError::MissingTimestamp { timestamp: 99 })
    );
}

#[test]
fn backtest_fee_turnover_drawdown_semantics_are_explicit() {
    let series = series_from_closes(&[100.0, 110.0]);
    let signals = [
        Signal {
            timestamp: 1,
            target_weight: 1.0,
            reason: SignalReason::LongMeanReversion,
        },
        Signal {
            timestamp: 2,
            target_weight: 0.0,
            reason: SignalReason::ExitToFlat,
        },
    ];
    let report = run_backtest(
        &series,
        &signals,
        BacktestConfig {
            initial_cash: 1_000.0,
            fee_bps: 100.0,
        },
    )
    .expect("backtest should run");

    assert_eq!(report.steps.len(), 2);
    assert_close(report.steps[0].fee_paid, 10.0);
    assert_close(report.steps[0].equity, 990.0);
    assert_close(report.final_equity, 1_079.0);
    assert_close(report.total_fees, 21.0);
    assert_close(report.total_return, 0.079);
    assert_close(report.max_drawdown, 0.01);
    assert_close(report.turnover, 1.0 + 1_100.0 / 1_090.0);
}
