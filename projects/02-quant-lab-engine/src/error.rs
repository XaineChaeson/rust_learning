use std::error::Error;
use std::fmt;

use factor_core::ComputeError;

#[derive(Debug, Clone, PartialEq)]
pub enum EngineError {
    EmptyInput {
        context: &'static str,
    },
    InvalidConfig {
        context: &'static str,
    },
    InvalidMarketData {
        index: usize,
        reason: &'static str,
    },
    LengthMismatch {
        context: &'static str,
        expected: usize,
        actual: usize,
    },
    MissingTimestamp {
        timestamp: u64,
    },
    Factor(ComputeError),
}

impl fmt::Display for EngineError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EngineError::EmptyInput { context } => write!(formatter, "{context} is empty"),
            EngineError::InvalidConfig { context } => {
                write!(formatter, "invalid config: {context}")
            }
            EngineError::InvalidMarketData { index, reason } => {
                write!(formatter, "invalid market data at index {index}: {reason}")
            }
            EngineError::LengthMismatch {
                context,
                expected,
                actual,
            } => write!(
                formatter,
                "length mismatch for {context}: expected {expected}, actual {actual}"
            ),
            EngineError::MissingTimestamp { timestamp } => {
                write!(
                    formatter,
                    "signal timestamp {timestamp} is missing from market data"
                )
            }
            EngineError::Factor(error) => write!(formatter, "factor computation failed: {error}"),
        }
    }
}

impl Error for EngineError {}

impl From<ComputeError> for EngineError {
    fn from(error: ComputeError) -> Self {
        EngineError::Factor(error)
    }
}
