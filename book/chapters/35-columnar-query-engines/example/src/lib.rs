#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnType {
    F64,
    U64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub name: String,
    pub dtype: ColumnType,
}

impl Field {
    pub fn new(name: &str, dtype: ColumnType) -> Self {
        Self {
            name: name.to_string(),
            dtype,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Column {
    F64(Vec<f64>),
    U64(Vec<u64>),
}

impl Column {
    fn len(&self) -> usize {
        match self {
            Column::F64(values) => values.len(),
            Column::U64(values) => values.len(),
        }
    }

    fn dtype(&self) -> ColumnType {
        match self {
            Column::F64(_) => ColumnType::F64,
            Column::U64(_) => ColumnType::U64,
        }
    }

    fn filter(&self, mask: &[bool]) -> Column {
        match self {
            Column::F64(values) => Column::F64(
                values
                    .iter()
                    .zip(mask)
                    .filter_map(|(value, keep)| keep.then_some(*value))
                    .collect(),
            ),
            Column::U64(values) => Column::U64(
                values
                    .iter()
                    .zip(mask)
                    .filter_map(|(value, keep)| keep.then_some(*value))
                    .collect(),
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryError {
    EmptyBatch,
    SchemaColumnMismatch,
    ColumnLengthMismatch,
    ColumnTypeMismatch { name: String },
    MissingColumn { name: String },
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecordBatch {
    pub schema: Vec<Field>,
    pub columns: Vec<Column>,
    pub rows: usize,
}

impl RecordBatch {
    pub fn try_new(schema: Vec<Field>, columns: Vec<Column>) -> Result<Self, QueryError> {
        if schema.is_empty() || columns.is_empty() {
            return Err(QueryError::EmptyBatch);
        }

        if schema.len() != columns.len() {
            return Err(QueryError::SchemaColumnMismatch);
        }

        let rows = columns[0].len();

        for (field, column) in schema.iter().zip(&columns) {
            if column.len() != rows {
                return Err(QueryError::ColumnLengthMismatch);
            }

            if field.dtype != column.dtype() {
                return Err(QueryError::ColumnTypeMismatch {
                    name: field.name.clone(),
                });
            }
        }

        Ok(Self {
            schema,
            columns,
            rows,
        })
    }

    pub fn column_index(&self, name: &str) -> Option<usize> {
        self.schema.iter().position(|field| field.name == name)
    }

    pub fn project(&self, names: &[&str]) -> Result<RecordBatch, QueryError> {
        let mut schema = Vec::with_capacity(names.len());
        let mut columns = Vec::with_capacity(names.len());

        for name in names {
            let index = self
                .column_index(name)
                .ok_or_else(|| QueryError::MissingColumn {
                    name: (*name).to_string(),
                })?;
            schema.push(self.schema[index].clone());
            columns.push(self.columns[index].clone());
        }

        RecordBatch::try_new(schema, columns)
    }

    pub fn filter_f64_gt(&self, name: &str, threshold: f64) -> Result<RecordBatch, QueryError> {
        let index = self
            .column_index(name)
            .ok_or_else(|| QueryError::MissingColumn {
                name: name.to_string(),
            })?;

        let Column::F64(values) = &self.columns[index] else {
            return Err(QueryError::ColumnTypeMismatch {
                name: name.to_string(),
            });
        };
        let mask = values
            .iter()
            .map(|value| *value > threshold)
            .collect::<Vec<_>>();
        let columns = self
            .columns
            .iter()
            .map(|column| column.filter(&mask))
            .collect();

        RecordBatch::try_new(self.schema.clone(), columns)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_batch() -> RecordBatch {
        RecordBatch::try_new(
            vec![
                Field::new("timestamp", ColumnType::U64),
                Field::new("close", ColumnType::F64),
                Field::new("volume", ColumnType::F64),
            ],
            vec![
                Column::U64(vec![1, 2, 3]),
                Column::F64(vec![100.0, 102.0, 99.0]),
                Column::F64(vec![10.0, 15.0, 9.0]),
            ],
        )
        .expect("valid batch")
    }

    #[test]
    fn projection_keeps_only_requested_columns() {
        let projected = sample_batch()
            .project(&["timestamp", "close"])
            .expect("projection should work");

        assert_eq!(projected.schema.len(), 2);
        assert_eq!(projected.rows, 3);
    }

    #[test]
    fn predicate_filter_keeps_matching_rows_across_columns() {
        let filtered = sample_batch()
            .filter_f64_gt("close", 100.0)
            .expect("filter should work");

        assert_eq!(filtered.rows, 1);
        assert_eq!(filtered.columns[0], Column::U64(vec![2]));
        assert_eq!(filtered.columns[1], Column::F64(vec![102.0]));
    }

    #[test]
    fn schema_validation_rejects_wrong_dtype() {
        let error = RecordBatch::try_new(
            vec![Field::new("close", ColumnType::F64)],
            vec![Column::U64(vec![1])],
        )
        .expect_err("dtype mismatch should fail");

        assert_eq!(
            error,
            QueryError::ColumnTypeMismatch {
                name: "close".to_string()
            }
        );
    }
}
