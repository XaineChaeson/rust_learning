use factor_core::{rolling_mean_incremental, rolling_zscore};

use crate::{EngineError, MarketSeries};

#[derive(Debug, Clone, PartialEq)]
pub struct FactorSet {
    pub window: usize,
    pub rolling_mean: Vec<f64>,
    pub rolling_zscore: Vec<f64>,
}

pub fn compute_factor_set(series: &MarketSeries, window: usize) -> Result<FactorSet, EngineError> {
    let closes = series.closes();

    Ok(FactorSet {
        window,
        rolling_mean: rolling_mean_incremental(&closes, window)?,
        rolling_zscore: rolling_zscore(&closes, window)?,
    })
}
