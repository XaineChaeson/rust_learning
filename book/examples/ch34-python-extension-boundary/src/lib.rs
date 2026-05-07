#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DType {
    F64,
    F32,
    I64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundaryMode {
    Borrow,
    Copy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoundaryError {
    EmptyInput,
    UnsupportedDType(DType),
    NonContiguousInput,
}

#[derive(Debug, Clone, Copy)]
pub struct PythonArrayView<'a> {
    pub dtype: DType,
    pub contiguous: bool,
    pub values: &'a [f64],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundaryPlan {
    pub mode: BoundaryMode,
    pub release_gil_around_kernel: bool,
    pub reason: &'static str,
}

pub fn plan_boundary(input: PythonArrayView<'_>, needs_owned_output: bool) -> BoundaryPlan {
    if input.dtype != DType::F64 {
        return BoundaryPlan {
            mode: BoundaryMode::Copy,
            release_gil_around_kernel: false,
            reason: "dtype conversion is required before calling the Rust kernel",
        };
    }

    if !input.contiguous {
        return BoundaryPlan {
            mode: BoundaryMode::Copy,
            release_gil_around_kernel: false,
            reason: "non-contiguous arrays need a contiguous staging buffer",
        };
    }

    if needs_owned_output {
        BoundaryPlan {
            mode: BoundaryMode::Copy,
            release_gil_around_kernel: true,
            reason: "the boundary owns a staging buffer but the kernel stays pure",
        }
    } else {
        BoundaryPlan {
            mode: BoundaryMode::Borrow,
            release_gil_around_kernel: true,
            reason: "f64 contiguous input can be borrowed for the kernel call",
        }
    }
}

pub fn mean_kernel(values: &[f64]) -> Result<f64, BoundaryError> {
    if values.is_empty() {
        return Err(BoundaryError::EmptyInput);
    }

    Ok(values.iter().sum::<f64>() / values.len() as f64)
}

pub fn execute_boundary(input: PythonArrayView<'_>) -> Result<f64, BoundaryError> {
    if input.dtype != DType::F64 {
        return Err(BoundaryError::UnsupportedDType(input.dtype));
    }

    if !input.contiguous {
        return Err(BoundaryError::NonContiguousInput);
    }

    mean_kernel(input.values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plans_borrow_for_contiguous_f64_input() {
        let input = PythonArrayView {
            dtype: DType::F64,
            contiguous: true,
            values: &[1.0, 2.0, 3.0],
        };

        assert_eq!(plan_boundary(input, false).mode, BoundaryMode::Borrow);
    }

    #[test]
    fn plans_copy_for_dtype_conversion() {
        let input = PythonArrayView {
            dtype: DType::F32,
            contiguous: true,
            values: &[1.0, 2.0, 3.0],
        };

        assert_eq!(plan_boundary(input, false).mode, BoundaryMode::Copy);
    }

    #[test]
    fn rejects_non_contiguous_input_before_kernel() {
        let input = PythonArrayView {
            dtype: DType::F64,
            contiguous: false,
            values: &[1.0, 2.0, 3.0],
        };

        assert_eq!(
            execute_boundary(input),
            Err(BoundaryError::NonContiguousInput)
        );
    }
}
