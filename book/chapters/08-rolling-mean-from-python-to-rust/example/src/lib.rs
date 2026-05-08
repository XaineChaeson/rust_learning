#[derive(Debug, Clone, PartialEq)]
pub enum RollingError {
    EmptyInput,
    InvalidWindow,
    NonFiniteValue { index: usize, value: f64 },
}

fn validate(values: &[f64], window: usize) -> Result<(), RollingError> {
    if window == 0 {
        return Err(RollingError::InvalidWindow);
    }

    if values.is_empty() {
        return Err(RollingError::EmptyInput);
    }

    for (index, value) in values.iter().copied().enumerate() {
        if !value.is_finite() {
            return Err(RollingError::NonFiniteValue { index, value });
        }
    }

    Ok(())
}

pub fn rolling_mean_incremental(values: &[f64], window: usize) -> Result<Vec<f64>, RollingError> {
    validate(values, window)?;

    if window > values.len() {
        return Ok(Vec::new());
    }

    let mut sum = values[..window].iter().sum::<f64>();
    let mut output = Vec::with_capacity(values.len() - window + 1);
    output.push(sum / window as f64);

    for index in window..values.len() {
        sum += values[index] - values[index - window];
        output.push(sum / window as f64);
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_incremental_rolling_mean() {
        assert_eq!(
            rolling_mean_incremental(&[1.0, 2.0, 3.0, 4.0], 2).expect("valid input"),
            vec![1.5, 2.5, 3.5]
        );
    }
}
