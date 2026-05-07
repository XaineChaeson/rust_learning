pub fn rolling_apply<F>(values: &[f64], window: usize, function: F) -> Vec<f64>
where
    F: FnMut(&[f64]) -> f64,
{
    if window == 0 || window > values.len() {
        return Vec::new();
    }

    values.windows(window).map(function).collect()
}

pub fn zscore_like(values: &[f64]) -> Vec<f64> {
    let mean = values.iter().sum::<f64>() / values.len() as f64;

    values
        .iter()
        .map(|value| value - mean)
        .filter(|value| value.abs() > 0.0)
        .collect()
}

pub fn cumulative_sum(values: &[f64]) -> Vec<f64> {
    values
        .iter()
        .scan(0.0, |state, value| {
            *state += value;
            Some(*state)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closure_parameter_turns_algorithm_into_reusable_kernel() {
        let output = rolling_apply(&[1.0, 2.0, 3.0, 4.0], 2, |window| {
            window.iter().sum::<f64>()
        });

        assert_eq!(output, vec![3.0, 5.0, 7.0]);
    }

    #[test]
    fn iterator_pipeline_preserves_correctness() {
        assert_eq!(cumulative_sum(&[1.0, 2.0, 3.0]), vec![1.0, 3.0, 6.0]);
    }
}
