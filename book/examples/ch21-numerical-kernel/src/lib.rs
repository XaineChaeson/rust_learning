#[derive(Debug, Clone, PartialEq)]
pub enum MatrixError {
    ShapeMismatch,
    IndexOutOfBounds,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize, data: Vec<f64>) -> Result<Self, MatrixError> {
        if rows * cols != data.len() {
            return Err(MatrixError::ShapeMismatch);
        }

        Ok(Self { rows, cols, data })
    }

    pub fn get(&self, row: usize, col: usize) -> Result<f64, MatrixError> {
        if row >= self.rows || col >= self.cols {
            return Err(MatrixError::IndexOutOfBounds);
        }

        Ok(self.data[row * self.cols + col])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indexes_row_major_matrix() {
        let matrix = Matrix::new(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).expect("valid shape");

        assert_eq!(matrix.get(1, 2), Ok(6.0));
    }
}
