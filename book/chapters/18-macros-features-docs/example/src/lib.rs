#[macro_export]
macro_rules! assert_close {
    ($left:expr, $right:expr, $tol:expr) => {{
        let left_value: f64 = $left;
        let right_value: f64 = $right;
        let tolerance: f64 = $tol;
        assert!(
            (left_value - right_value).abs() <= tolerance,
            "left={}, right={}, tolerance={}",
            left_value,
            right_value,
            tolerance
        );
    }};
}

/// Computes arithmetic mean for a non-empty slice.
///
/// # Examples
///
/// ```
/// let mean = ch18_macros_features_docs::mean(&[1.0, 2.0, 3.0]);
/// assert_eq!(mean, Some(2.0));
/// ```
pub fn mean(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }

    Some(values.iter().sum::<f64>() / values.len() as f64)
}

pub fn backend_name() -> &'static str {
    if cfg!(feature = "simd") {
        "simd"
    } else {
        "scalar"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macro_encodes_repeated_test_pattern() {
        assert_close!(0.1 + 0.2, 0.3, 1e-12);
    }

    #[test]
    fn cfg_reports_active_backend() {
        assert!(matches!(backend_name(), "scalar" | "simd"));
    }
}
