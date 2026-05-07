pub trait Factor {
    type Output;

    fn name(&self) -> &'static str;
    fn compute(&self, values: &[f64]) -> Self::Output;
}

#[derive(Debug, Clone, Copy)]
pub struct MeanFactor;

impl Factor for MeanFactor {
    type Output = Option<f64>;

    fn name(&self) -> &'static str {
        "mean"
    }

    fn compute(&self, values: &[f64]) -> Self::Output {
        if values.is_empty() {
            return None;
        }

        Some(values.iter().sum::<f64>() / values.len() as f64)
    }
}

pub fn run_factor<F>(factor: F, values: &[f64]) -> F::Output
where
    F: Factor,
{
    factor.compute(values)
}

pub fn normalize<T>(values: &[T]) -> Vec<f64>
where
    T: Copy + Into<f64>,
{
    let converted = values.iter().copied().map(Into::into).collect::<Vec<_>>();
    let total = converted.iter().sum::<f64>();

    converted.into_iter().map(|value| value / total).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trait_object_is_not_required_for_generic_dispatch() {
        assert_eq!(run_factor(MeanFactor, &[1.0, 2.0, 3.0]), Some(2.0));
    }

    #[test]
    fn generic_bounds_express_input_contract() {
        assert_eq!(normalize(&[1_u32, 3_u32]), vec![0.25, 0.75]);
    }
}
