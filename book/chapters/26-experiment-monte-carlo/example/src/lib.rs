#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExperimentParam {
    pub window: usize,
    pub threshold: f64,
}

pub fn parameter_grid(windows: &[usize], thresholds: &[f64]) -> Vec<ExperimentParam> {
    let mut output = Vec::new();

    for window in windows {
        for threshold in thresholds {
            output.push(ExperimentParam {
                window: *window,
                threshold: *threshold,
            });
        }
    }

    output
}

pub fn deterministic_walk(seed: u64, steps: usize) -> Vec<f64> {
    let mut state = seed;
    let mut value = 0.0;
    let mut path = Vec::with_capacity(steps);

    for _ in 0..steps {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let step = if state.is_multiple_of(2) { 1.0 } else { -1.0 };
        value += step;
        path.push(value);
    }

    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_expands_parameters() {
        assert_eq!(parameter_grid(&[5, 20], &[1.0, 2.0]).len(), 4);
    }

    #[test]
    fn walk_is_reproducible() {
        assert_eq!(deterministic_walk(42, 10), deterministic_walk(42, 10));
    }
}
