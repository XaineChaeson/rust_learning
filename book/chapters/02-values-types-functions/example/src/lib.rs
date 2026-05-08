#[derive(Debug, Clone, PartialEq)]
pub enum NumericError {
    EmptyInput,
    InvalidWindow,
    NonPositivePrice,
}

pub fn mean(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }

    Some(values.iter().sum::<f64>() / values.len() as f64)
}

pub fn returns(prices: &[f64]) -> Result<Vec<f64>, NumericError> {
    if prices.len() < 2 {
        return Ok(Vec::new());
    }

    if prices.iter().any(|price| *price <= 0.0) {
        return Err(NumericError::NonPositivePrice);
    }

    Ok(prices
        .windows(2)
        .map(|window| window[1] / window[0] - 1.0)
        .collect())
}

pub fn cumulative_sum(values: &[f64]) -> Vec<f64> {
    let mut total = 0.0;
    let mut output = Vec::with_capacity(values.len());

    for value in values {
        total += value;
        output.push(total);
    }

    output
}

pub fn rolling_mean(values: &[f64], window: usize) -> Result<Vec<f64>, NumericError> {
    if window == 0 {
        return Err(NumericError::InvalidWindow);
    }

    if values.is_empty() {
        return Err(NumericError::EmptyInput);
    }

    if window > values.len() {
        return Ok(Vec::new());
    }

    Ok(values
        .windows(window)
        .map(|slice| slice.iter().sum::<f64>() / window as f64)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(left: f64, right: f64) {
        assert!((left - right).abs() < 1e-12, "left={left}, right={right}");
    }

    #[test]
    fn mean_handles_empty_and_non_empty_input() {
        assert_eq!(mean(&[]), None);
        assert_close(mean(&[1.0, 2.0, 3.0]).expect("mean exists"), 2.0);
    }

    #[test]
    fn returns_computes_simple_returns() {
        let output = returns(&[100.0, 105.0, 102.9]).expect("prices are valid");

        assert_close(output[0], 0.05);
        assert_close(output[1], -0.02);
    }

    #[test]
    fn rolling_mean_computes_right_aligned_windows() {
        assert_eq!(
            rolling_mean(&[1.0, 2.0, 3.0, 4.0], 2).expect("window is valid"),
            vec![1.5, 2.5, 3.5]
        );
    }
}
