#[derive(Debug, Clone, PartialEq)]
pub enum DotError {
    LengthMismatch { left: usize, right: usize },
}

pub fn dot_safe(left: &[f64], right: &[f64]) -> Result<f64, DotError> {
    if left.len() != right.len() {
        return Err(DotError::LengthMismatch {
            left: left.len(),
            right: right.len(),
        });
    }

    Ok(left.iter().zip(right).map(|(x, y)| x * y).sum())
}

pub fn dot_with_unsafe_boundary(left: &[f64], right: &[f64]) -> Result<f64, DotError> {
    if left.len() != right.len() {
        return Err(DotError::LengthMismatch {
            left: left.len(),
            right: right.len(),
        });
    }

    // SAFETY: Length equality is checked above, and the unsafe helper only indexes 0..len.
    Ok(unsafe { dot_unchecked_same_len(left, right) })
}

unsafe fn dot_unchecked_same_len(left: &[f64], right: &[f64]) -> f64 {
    let mut total = 0.0;

    for index in 0..left.len() {
        // SAFETY: The caller guarantees both slices have equal length and index is in bounds.
        total += unsafe { left.get_unchecked(index) * right.get_unchecked(index) };
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsafe_boundary_matches_safe_baseline() {
        assert_eq!(
            dot_safe(&[1.0, 2.0], &[10.0, 20.0]),
            dot_with_unsafe_boundary(&[1.0, 2.0], &[10.0, 20.0])
        );
    }
}
