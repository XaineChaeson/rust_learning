#[derive(Debug, Clone, PartialEq)]
pub struct OwnedArray {
    values: Vec<f64>,
}

impl OwnedArray {
    pub fn from_python_like_input(values: &[f64]) -> Self {
        Self {
            values: values.to_vec(),
        }
    }

    pub fn as_slice(&self) -> &[f64] {
        &self.values
    }
}

pub fn borrowed_kernel(values: &[f64]) -> f64 {
    values.iter().sum()
}

pub fn owned_boundary_then_kernel(values: &[f64]) -> f64 {
    let owned = OwnedArray::from_python_like_input(values);
    borrowed_kernel(owned.as_slice())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boundary_copy_is_explicit() {
        let input = vec![1.0, 2.0, 3.0];

        assert_eq!(borrowed_kernel(&input), 6.0);
        assert_eq!(owned_boundary_then_kernel(&input), 6.0);
    }
}
