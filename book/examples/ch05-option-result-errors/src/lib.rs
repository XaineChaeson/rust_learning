use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum QuantError {
    EmptyInput,
    InvalidWindow,
    NonFiniteValue { index: usize, value: f64 },
    NonPositivePrice { index: usize, value: f64 },
}

impl fmt::Display for QuantError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuantError::EmptyInput => write!(formatter, "input is empty"),
            QuantError::InvalidWindow => write!(formatter, "window must be greater than zero"),
            QuantError::NonFiniteValue { index, value } => {
                write!(formatter, "value at index {index} is not finite: {value}")
            }
            QuantError::NonPositivePrice { index, value } => {
                write!(formatter, "price at index {index} is not positive: {value}")
            }
        }
    }
}

impl Error for QuantError {}

pub fn validate_finite(values: &[f64]) -> Result<(), QuantError> {
    if values.is_empty() {
        return Err(QuantError::EmptyInput);
    }

    for (index, value) in values.iter().copied().enumerate() {
        if !value.is_finite() {
            return Err(QuantError::NonFiniteValue { index, value });
        }
    }

    Ok(())
}

pub fn validate_prices(prices: &[f64]) -> Result<(), QuantError> {
    validate_finite(prices)?;

    for (index, value) in prices.iter().copied().enumerate() {
        if value <= 0.0 {
            return Err(QuantError::NonPositivePrice { index, value });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_nan_without_comparing_nan_for_equality() {
        let error = validate_finite(&[1.0, f64::NAN]).expect_err("NaN is invalid");

        match error {
            QuantError::NonFiniteValue { index, value } => {
                assert_eq!(index, 1);
                assert!(value.is_nan());
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
