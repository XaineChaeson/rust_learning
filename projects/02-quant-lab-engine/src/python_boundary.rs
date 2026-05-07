use crate::EngineError;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PythonArraySpec {
    pub dtype: DType,
    pub rows: usize,
    pub cols: usize,
    pub contiguous: bool,
    pub contains_nan: bool,
}

impl PythonArraySpec {
    pub fn values_len(&self) -> usize {
        self.rows * self.cols
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundaryPlan {
    pub mode: BoundaryMode,
    pub release_gil_around_kernel: bool,
    pub reason: &'static str,
}

pub fn plan_python_boundary(
    spec: PythonArraySpec,
    needs_owned_output: bool,
) -> Result<BoundaryPlan, EngineError> {
    if spec.rows == 0 || spec.cols == 0 {
        return Err(EngineError::InvalidConfig {
            context: "python array shape must be non-empty",
        });
    }

    if spec.contains_nan {
        return Err(EngineError::InvalidMarketData {
            index: 0,
            reason: "python array contains NaN",
        });
    }

    if spec.dtype != DType::F64 {
        return Ok(BoundaryPlan {
            mode: BoundaryMode::Copy,
            release_gil_around_kernel: false,
            reason: "dtype conversion is required before the Rust kernel",
        });
    }

    if !spec.contiguous {
        return Ok(BoundaryPlan {
            mode: BoundaryMode::Copy,
            release_gil_around_kernel: false,
            reason: "non-contiguous arrays need a contiguous staging buffer",
        });
    }

    if needs_owned_output {
        Ok(BoundaryPlan {
            mode: BoundaryMode::Copy,
            release_gil_around_kernel: true,
            reason: "boundary owns an output buffer while the kernel stays pure",
        })
    } else {
        Ok(BoundaryPlan {
            mode: BoundaryMode::Borrow,
            release_gil_around_kernel: true,
            reason: "contiguous f64 input can be borrowed for the kernel call",
        })
    }
}
