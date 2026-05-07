use factor_core::{
    ComputeError, rolling_beta, rolling_corr, rolling_max, rolling_mean, rolling_mean_incremental,
    rolling_min, rolling_std, rolling_zscore,
};

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
fn computes_basic_rolling_factors() {
    let values = [1.0, 2.0, 3.0, 4.0];

    assert_eq!(
        rolling_mean(&values, 2).expect("valid input"),
        vec![1.5, 2.5, 3.5]
    );
    assert_eq!(
        rolling_min(&values, 2).expect("valid input"),
        vec![1.0, 2.0, 3.0]
    );
    assert_eq!(
        rolling_max(&values, 2).expect("valid input"),
        vec![2.0, 3.0, 4.0]
    );
    assert_close_vec(
        &rolling_std(&values, 2).expect("valid input"),
        &[0.5, 0.5, 0.5],
    );
    assert_close_vec(
        &rolling_zscore(&values, 2).expect("valid input"),
        &[1.0, 1.0, 1.0],
    );
}

#[test]
fn optimized_rolling_mean_matches_baseline() {
    let values = [101.0, 102.0, 99.0, 103.0, 105.0, 104.0, 106.0];

    assert_close_vec(
        &rolling_mean_incremental(&values, 3).expect("valid input"),
        &rolling_mean(&values, 3).expect("valid input"),
    );
}

#[test]
fn computes_correlation_and_beta() {
    let asset = [1.0, 2.0, 3.0, 4.0];
    let benchmark = [2.0, 4.0, 6.0, 8.0];

    assert_close_vec(
        &rolling_corr(&asset, &benchmark, 3).expect("valid input"),
        &[1.0, 1.0],
    );
    assert_close_vec(
        &rolling_beta(&asset, &benchmark, 3).expect("valid input"),
        &[0.5, 0.5],
    );
}

#[test]
fn returns_empty_when_window_is_larger_than_input() {
    assert_eq!(
        rolling_mean(&[1.0, 2.0], 3).expect("valid input"),
        Vec::<f64>::new()
    );
}

#[test]
fn rejects_invalid_window() {
    assert_eq!(
        rolling_mean(&[1.0, 2.0], 0),
        Err(ComputeError::InvalidWindow)
    );
}

#[test]
fn rejects_empty_input() {
    assert_eq!(rolling_mean(&[], 1), Err(ComputeError::EmptyInput));
}

#[test]
fn rejects_non_finite_right_hand_input() {
    assert_eq!(
        rolling_corr(&[1.0, 2.0], &[1.0, f64::INFINITY], 2),
        Err(ComputeError::NonFiniteValue {
            index: 1,
            value: f64::INFINITY,
        })
    );
}

#[test]
fn rejects_length_mismatch() {
    assert_eq!(
        rolling_corr(&[1.0, 2.0], &[1.0], 1),
        Err(ComputeError::LengthMismatch { left: 2, right: 1 })
    );
}

#[test]
fn zscore_rejects_zero_variance_window() {
    assert_eq!(
        rolling_zscore(&[1.0, 1.0], 2),
        Err(ComputeError::ZeroVariance)
    );
}

#[test]
fn corr_rejects_zero_variance_window() {
    assert_eq!(
        rolling_corr(&[1.0, 2.0, 3.0], &[2.0, 2.0, 2.0], 3),
        Err(ComputeError::ZeroVariance)
    );
}

#[test]
fn beta_rejects_zero_variance_benchmark() {
    assert_eq!(
        rolling_beta(&[1.0, 2.0, 3.0], &[2.0, 2.0, 2.0], 3),
        Err(ComputeError::ZeroVariance)
    );
}
