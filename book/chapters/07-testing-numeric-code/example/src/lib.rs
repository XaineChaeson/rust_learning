pub fn assert_close(left: f64, right: f64, tolerance: f64) {
    assert!(
        (left - right).abs() <= tolerance,
        "left={left}, right={right}, tolerance={tolerance}"
    );
}

pub fn assert_close_vec(left: &[f64], right: &[f64], tolerance: f64) {
    assert_eq!(
        left.len(),
        right.len(),
        "length mismatch: left={}, right={}",
        left.len(),
        right.len()
    );

    for (index, (left_value, right_value)) in left.iter().zip(right).enumerate() {
        assert!(
            (*left_value - *right_value).abs() <= tolerance,
            "index={index}, left={left_value}, right={right_value}, tolerance={tolerance}"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn approximate_comparison_accepts_rounding_error() {
        assert_close(0.1 + 0.2, 0.3, 1e-12);
        assert_close_vec(&[1.0, 2.000000000001], &[1.0, 2.0], 1e-10);
    }
}
