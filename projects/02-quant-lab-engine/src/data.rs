use std::collections::BTreeSet;

use crate::EngineError;

#[derive(Debug, Clone, PartialEq)]
pub struct MarketBar {
    pub timestamp: u64,
    pub close: f64,
}

impl MarketBar {
    pub fn new(timestamp: u64, close: f64) -> Self {
        Self { timestamp, close }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MarketSeries {
    pub symbol: String,
    pub bars: Vec<MarketBar>,
}

impl MarketSeries {
    pub fn new(symbol: &str, bars: Vec<MarketBar>) -> Result<Self, EngineError> {
        let symbol = symbol.trim();

        if symbol.is_empty() {
            return Err(EngineError::InvalidConfig {
                context: "symbol must not be empty",
            });
        }

        if bars.is_empty() {
            return Err(EngineError::EmptyInput {
                context: "market series",
            });
        }

        for (index, bar) in bars.iter().enumerate() {
            if !bar.close.is_finite() {
                return Err(EngineError::InvalidMarketData {
                    index,
                    reason: "close must be finite",
                });
            }

            if bar.close <= 0.0 {
                return Err(EngineError::InvalidMarketData {
                    index,
                    reason: "close must be positive",
                });
            }

            if index > 0 && bar.timestamp <= bars[index - 1].timestamp {
                return Err(EngineError::InvalidMarketData {
                    index,
                    reason: "timestamps must be strictly increasing",
                });
            }
        }

        Ok(Self {
            symbol: symbol.to_string(),
            bars,
        })
    }

    pub fn len(&self) -> usize {
        self.bars.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bars.is_empty()
    }

    pub fn closes(&self) -> Vec<f64> {
        self.bars.iter().map(|bar| bar.close).collect()
    }

    pub fn close_at(&self, timestamp: u64) -> Option<f64> {
        self.bars
            .iter()
            .find(|bar| bar.timestamp == timestamp)
            .map(|bar| bar.close)
    }

    pub fn returns(&self) -> Vec<f64> {
        self.bars
            .windows(2)
            .map(|window| window[1].close / window[0].close - 1.0)
            .collect()
    }

    pub fn partition(&self, data_version: &str) -> Result<MarketPartition, EngineError> {
        MarketPartition::new(
            &self.symbol,
            self.bars[0].timestamp,
            self.bars[self.bars.len() - 1].timestamp,
            data_version,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketPartition {
    pub symbol: String,
    pub start_timestamp: u64,
    pub end_timestamp: u64,
    pub data_version: String,
}

impl MarketPartition {
    pub fn new(
        symbol: &str,
        start_timestamp: u64,
        end_timestamp: u64,
        data_version: &str,
    ) -> Result<Self, EngineError> {
        let symbol = symbol.trim();
        let data_version = data_version.trim();

        if symbol.is_empty() {
            return Err(EngineError::InvalidConfig {
                context: "partition symbol must not be empty",
            });
        }

        if start_timestamp > end_timestamp {
            return Err(EngineError::InvalidConfig {
                context: "partition start must be <= end",
            });
        }

        if data_version.is_empty() {
            return Err(EngineError::InvalidConfig {
                context: "data_version must not be empty",
            });
        }

        Ok(Self {
            symbol: symbol.to_string(),
            start_timestamp,
            end_timestamp,
            data_version: data_version.to_string(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MarketUniverse {
    pub series: Vec<MarketSeries>,
}

impl MarketUniverse {
    pub fn new(series: Vec<MarketSeries>) -> Result<Self, EngineError> {
        if series.is_empty() {
            return Err(EngineError::EmptyInput {
                context: "market universe",
            });
        }

        let mut symbols = BTreeSet::new();

        for item in &series {
            if !symbols.insert(item.symbol.clone()) {
                return Err(EngineError::InvalidConfig {
                    context: "market universe symbols must be unique",
                });
            }
        }

        Ok(Self { series })
    }

    pub fn symbols(&self) -> Vec<&str> {
        self.series
            .iter()
            .map(|series| series.symbol.as_str())
            .collect()
    }

    pub fn series(&self, symbol: &str) -> Option<&MarketSeries> {
        self.series.iter().find(|series| series.symbol == symbol)
    }

    pub fn partitions(&self, data_version: &str) -> Result<Vec<MarketPartition>, EngineError> {
        self.series
            .iter()
            .map(|series| series.partition(data_version))
            .collect()
    }
}
