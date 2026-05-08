#[derive(Debug, Clone, PartialEq)]
pub struct RowMajorMatrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

impl RowMajorMatrix {
    pub fn new(rows: usize, cols: usize, data: Vec<f64>) -> Self {
        assert_eq!(rows * cols, data.len());
        Self { rows, cols, data }
    }

    pub fn row_sum(&self, row: usize) -> f64 {
        let start = row * self.cols;
        self.data[start..start + self.cols].iter().sum()
    }

    pub fn col_sum(&self, col: usize) -> f64 {
        (0..self.rows)
            .map(|row| self.data[row * self.cols + col])
            .sum()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BarsSoa {
    pub open: Vec<f64>,
    pub high: Vec<f64>,
    pub low: Vec<f64>,
    pub close: Vec<f64>,
}

impl BarsSoa {
    pub fn typical_prices(&self) -> Vec<f64> {
        self.high
            .iter()
            .zip(&self.low)
            .zip(&self.close)
            .map(|((high, low), close)| (high + low + close) / 3.0)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_major_indexing_is_explicit() {
        let matrix = RowMajorMatrix::new(2, 3, vec![1.0, 2.0, 3.0, 10.0, 20.0, 30.0]);

        assert_eq!(matrix.row_sum(1), 60.0);
        assert_eq!(matrix.col_sum(2), 33.0);
    }

    #[test]
    fn soa_layout_groups_columns_for_analytics() {
        let bars = BarsSoa {
            open: vec![1.0],
            high: vec![3.0],
            low: vec![1.0],
            close: vec![2.0],
        };

        assert_eq!(bars.typical_prices(), vec![2.0]);
    }
}
