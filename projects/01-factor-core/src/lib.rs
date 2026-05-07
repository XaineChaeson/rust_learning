use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct TimeSeries {
    pub name: String,
    pub values: Vec<f64>,
}

impl TimeSeries {
    pub fn new(name: &str, values: Vec<f64>) -> Self {
        Self {
            name: name.to_string(),
            values,
        }
    }

    pub fn values(&self) -> &[f64] {
        &self.values
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FactorSeries {
    pub name: String,
    pub values: Vec<f64>,
}

impl FactorSeries {
    pub fn new(name: &str, values: Vec<f64>) -> Self {
        Self {
            name: name.to_string(),
            values,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComputeError {
    EmptyInput,
    InvalidWindow,
    LengthMismatch { left: usize, right: usize },
    NonFiniteValue { index: usize, value: f64 },
    ZeroVariance,
}

impl fmt::Display for ComputeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComputeError::EmptyInput => write!(formatter, "input is empty"),
            ComputeError::InvalidWindow => write!(formatter, "window must be greater than zero"),
            ComputeError::LengthMismatch { left, right } => {
                write!(
                    formatter,
                    "input lengths differ: left={left}, right={right}"
                )
            }
            ComputeError::NonFiniteValue { index, value } => {
                write!(formatter, "value at index {index} is not finite: {value}")
            }
            ComputeError::ZeroVariance => write!(formatter, "window variance is zero"),
        }
    }
}

impl Error for ComputeError {}

fn validate_values(values: &[f64]) -> Result<(), ComputeError> {
    if values.is_empty() {
        return Err(ComputeError::EmptyInput);
    }

    for (index, value) in values.iter().copied().enumerate() {
        if !value.is_finite() {
            return Err(ComputeError::NonFiniteValue { index, value });
        }
    }

    Ok(())
}

fn validate_window(values: &[f64], window: usize) -> Result<(), ComputeError> {
    validate_values(values)?;

    if window == 0 {
        return Err(ComputeError::InvalidWindow);
    }

    Ok(())
}

fn validate_pair(left: &[f64], right: &[f64], window: usize) -> Result<(), ComputeError> {
    validate_window(left, window)?;
    validate_values(right)?;

    if left.len() != right.len() {
        return Err(ComputeError::LengthMismatch {
            left: left.len(),
            right: right.len(),
        });
    }

    Ok(())
}

fn mean(values: &[f64]) -> f64 {
    values.iter().sum::<f64>() / values.len() as f64
}

fn variance(values: &[f64]) -> f64 {
    let average = mean(values);

    values
        .iter()
        .map(|value| {
            let centered = value - average;
            centered * centered
        })
        .sum::<f64>()
        / values.len() as f64
}

fn covariance(left: &[f64], right: &[f64]) -> f64 {
    let left_mean = mean(left);
    let right_mean = mean(right);

    left.iter()
        .zip(right)
        .map(|(left_value, right_value)| (left_value - left_mean) * (right_value - right_mean))
        .sum::<f64>()
        / left.len() as f64
}

pub fn rolling_mean(values: &[f64], window: usize) -> Result<Vec<f64>, ComputeError> {
    validate_window(values, window)?;

    if window > values.len() {
        return Ok(Vec::new());
    }

    Ok(values.windows(window).map(mean).collect())
}

pub fn rolling_mean_incremental(values: &[f64], window: usize) -> Result<Vec<f64>, ComputeError> {
    validate_window(values, window)?;

    if window > values.len() {
        return Ok(Vec::new());
    }

    let mut output = Vec::with_capacity(values.len() - window + 1);
    let mut window_sum = values[..window].iter().sum::<f64>();
    output.push(window_sum / window as f64);

    for index in window..values.len() {
        window_sum += values[index];
        window_sum -= values[index - window];
        output.push(window_sum / window as f64);
    }

    Ok(output)
}

pub fn rolling_std(values: &[f64], window: usize) -> Result<Vec<f64>, ComputeError> {
    validate_window(values, window)?;

    if window > values.len() {
        return Ok(Vec::new());
    }

    Ok(values
        .windows(window)
        .map(|slice| variance(slice).sqrt())
        .collect())
}

pub fn rolling_zscore(values: &[f64], window: usize) -> Result<Vec<f64>, ComputeError> {
    validate_window(values, window)?;

    if window > values.len() {
        return Ok(Vec::new());
    }

    values
        .windows(window)
        .map(|slice| {
            let standard_deviation = variance(slice).sqrt();

            if standard_deviation == 0.0 {
                return Err(ComputeError::ZeroVariance);
            }

            let last = slice[slice.len() - 1];
            Ok((last - mean(slice)) / standard_deviation)
        })
        .collect()
}

pub fn rolling_min(values: &[f64], window: usize) -> Result<Vec<f64>, ComputeError> {
    validate_window(values, window)?;

    if window > values.len() {
        return Ok(Vec::new());
    }

    Ok(values
        .windows(window)
        .map(|slice| {
            slice
                .iter()
                .copied()
                .fold(f64::INFINITY, |minimum, value| minimum.min(value))
        })
        .collect())
}

pub fn rolling_max(values: &[f64], window: usize) -> Result<Vec<f64>, ComputeError> {
    validate_window(values, window)?;

    if window > values.len() {
        return Ok(Vec::new());
    }

    Ok(values
        .windows(window)
        .map(|slice| {
            slice
                .iter()
                .copied()
                .fold(f64::NEG_INFINITY, |maximum, value| maximum.max(value))
        })
        .collect())
}

pub fn rolling_corr(left: &[f64], right: &[f64], window: usize) -> Result<Vec<f64>, ComputeError> {
    validate_pair(left, right, window)?;

    if window > left.len() {
        return Ok(Vec::new());
    }

    left.windows(window)
        .zip(right.windows(window))
        .map(|(left_window, right_window)| {
            let left_std = variance(left_window).sqrt();
            let right_std = variance(right_window).sqrt();

            if left_std == 0.0 || right_std == 0.0 {
                return Err(ComputeError::ZeroVariance);
            }

            Ok(covariance(left_window, right_window) / (left_std * right_std))
        })
        .collect()
}

pub fn rolling_beta(
    asset: &[f64],
    benchmark: &[f64],
    window: usize,
) -> Result<Vec<f64>, ComputeError> {
    validate_pair(asset, benchmark, window)?;

    if window > asset.len() {
        return Ok(Vec::new());
    }

    asset
        .windows(window)
        .zip(benchmark.windows(window))
        .map(|(asset_window, benchmark_window)| {
            let benchmark_variance = variance(benchmark_window);

            if benchmark_variance == 0.0 {
                return Err(ComputeError::ZeroVariance);
            }

            Ok(covariance(asset_window, benchmark_window) / benchmark_variance)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(left: f64, right: f64) {
        assert!((left - right).abs() < 1e-12, "left={left}, right={right}");
    }

    fn assert_close_vec(left: &[f64], right: &[f64]) {
        assert_eq!(left.len(), right.len());

        for (left_value, right_value) in left.iter().zip(right) {
            assert_close(*left_value, *right_value);
        }
    }

    #[test]
    fn rolling_mean_uses_right_aligned_complete_windows() {
        assert_eq!(
            rolling_mean(&[1.0, 2.0, 3.0, 4.0], 2).expect("valid input"),
            vec![1.5, 2.5, 3.5]
        );
    }

    #[test]
    fn incremental_rolling_mean_matches_baseline() {
        let values = [2.0, 4.0, 8.0, 16.0, 32.0];

        assert_close_vec(
            &rolling_mean_incremental(&values, 3).expect("valid input"),
            &rolling_mean(&values, 3).expect("valid input"),
        );
    }

    #[test]
    fn rolling_std_uses_population_variance() {
        let output = rolling_std(&[1.0, 2.0, 3.0], 2).expect("valid input");

        assert_close_vec(&output, &[0.5, 0.5]);
    }

    #[test]
    fn rolling_zscore_uses_last_value_in_window() {
        let output = rolling_zscore(&[1.0, 2.0, 3.0], 2).expect("valid input");

        assert_close_vec(&output, &[1.0, 1.0]);
    }

    #[test]
    fn rejects_non_finite_values() {
        let error = rolling_mean(&[1.0, f64::NAN], 2).expect_err("NaN should be rejected");

        match error {
            ComputeError::NonFiniteValue { index, value } => {
                assert_eq!(index, 1);
                assert!(value.is_nan());
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
