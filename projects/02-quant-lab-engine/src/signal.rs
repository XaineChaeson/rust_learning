use crate::{EngineError, MarketSeries};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SignalConfig {
    pub entry_z: f64,
    pub exit_z: f64,
    pub long_weight: f64,
    pub short_weight: f64,
}

impl SignalConfig {
    pub fn validate(&self) -> Result<(), EngineError> {
        if !self.entry_z.is_finite() || self.entry_z <= 0.0 {
            return Err(EngineError::InvalidConfig {
                context: "entry_z must be positive and finite",
            });
        }

        if !self.exit_z.is_finite() || self.exit_z < 0.0 || self.exit_z > self.entry_z {
            return Err(EngineError::InvalidConfig {
                context: "exit_z must be finite and in [0, entry_z]",
            });
        }

        if !self.long_weight.is_finite() || !self.short_weight.is_finite() {
            return Err(EngineError::InvalidConfig {
                context: "signal weights must be finite",
            });
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SignalReason {
    LongMeanReversion,
    ShortMeanReversion,
    ExitToFlat,
    Hold,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Signal {
    pub timestamp: u64,
    pub target_weight: f64,
    pub reason: SignalReason,
}

pub fn generate_mean_reversion_signals(
    series: &MarketSeries,
    zscores: &[f64],
    window: usize,
    config: SignalConfig,
) -> Result<Vec<Signal>, EngineError> {
    config.validate()?;

    if window == 0 {
        return Err(EngineError::InvalidConfig {
            context: "window must be greater than zero",
        });
    }

    let expected = series.len().saturating_sub(window).saturating_add(1);

    if zscores.len() != expected {
        return Err(EngineError::LengthMismatch {
            context: "zscore alignment",
            expected,
            actual: zscores.len(),
        });
    }

    let mut target_weight = 0.0;
    let mut signals = Vec::with_capacity(zscores.len());

    for (offset, zscore) in zscores.iter().copied().enumerate() {
        if !zscore.is_finite() {
            return Err(EngineError::InvalidMarketData {
                index: offset,
                reason: "zscore must be finite",
            });
        }

        let reason = if zscore <= -config.entry_z {
            target_weight = config.long_weight;
            SignalReason::LongMeanReversion
        } else if zscore >= config.entry_z {
            target_weight = config.short_weight;
            SignalReason::ShortMeanReversion
        } else if zscore.abs() <= config.exit_z {
            target_weight = 0.0;
            SignalReason::ExitToFlat
        } else {
            SignalReason::Hold
        };

        signals.push(Signal {
            timestamp: series.bars[offset + window - 1].timestamp,
            target_weight,
            reason,
        });
    }

    Ok(signals)
}
