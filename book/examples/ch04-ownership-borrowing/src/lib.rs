pub fn sum(values: &[f64]) -> f64 {
    values.iter().sum()
}

pub fn demean(values: &[f64]) -> Option<Vec<f64>> {
    if values.is_empty() {
        return None;
    }

    let mean = sum(values) / values.len() as f64;

    Some(values.iter().map(|value| value - mean).collect())
}

pub fn demean_in_place(values: &mut [f64]) -> Option<()> {
    if values.is_empty() {
        return None;
    }

    let mean = sum(values) / values.len() as f64;

    for value in values {
        *value -= mean;
    }

    Some(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrowed_and_mutable_apis_are_distinct() {
        let values = vec![1.0, 2.0, 3.0];
        let output = demean(&values).expect("non-empty input");

        assert_eq!(values, vec![1.0, 2.0, 3.0]);
        assert_eq!(output, vec![-1.0, 0.0, 1.0]);

        let mut mutable_values = values;
        demean_in_place(&mut mutable_values).expect("non-empty input");
        assert_eq!(mutable_values, vec![-1.0, 0.0, 1.0]);
    }
}
