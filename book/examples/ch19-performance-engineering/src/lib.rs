pub fn rolling_sum_naive(values: &[f64], window: usize) -> Vec<f64> {
    if window == 0 || window > values.len() {
        return Vec::new();
    }

    values
        .windows(window)
        .map(|slice| slice.iter().sum::<f64>())
        .collect()
}

pub fn rolling_sum_incremental(values: &[f64], window: usize) -> Vec<f64> {
    if window == 0 || window > values.len() {
        return Vec::new();
    }

    let mut sum = values[..window].iter().sum::<f64>();
    let mut output = Vec::with_capacity(values.len() - window + 1);
    output.push(sum);

    for index in window..values.len() {
        sum += values[index] - values[index - window];
        output.push(sum);
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn baseline_and_incremental_match() {
        let values = (0..1_000)
            .map(|index| index as f64 % 17.0)
            .collect::<Vec<_>>();

        assert_eq!(
            rolling_sum_naive(&values, 20),
            rolling_sum_incremental(&values, 20)
        );
    }
}
