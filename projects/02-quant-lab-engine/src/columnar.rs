use crate::{EngineError, ExperimentResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnType {
    Utf8,
    U64,
    F64,
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
    Utf8(Vec<String>),
    U64(Vec<u64>),
    F64(Vec<f64>),
}

impl Column {
    pub fn len(&self) -> usize {
        match self {
            Column::Utf8(values) => values.len(),
            Column::U64(values) => values.len(),
            Column::F64(values) => values.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn dtype(&self) -> ColumnType {
        match self {
            Column::Utf8(_) => ColumnType::Utf8,
            Column::U64(_) => ColumnType::U64,
            Column::F64(_) => ColumnType::F64,
        }
    }

    fn filter(&self, mask: &[bool]) -> Column {
        match self {
            Column::Utf8(values) => Column::Utf8(
                values
                    .iter()
                    .zip(mask)
                    .filter(|(_, keep)| **keep)
                    .map(|(value, _)| value.clone())
                    .collect(),
            ),
            Column::U64(values) => Column::U64(
                values
                    .iter()
                    .zip(mask)
                    .filter_map(|(value, keep)| keep.then_some(*value))
                    .collect(),
            ),
            Column::F64(values) => Column::F64(
                values
                    .iter()
                    .zip(mask)
                    .filter_map(|(value, keep)| keep.then_some(*value))
                    .collect(),
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExperimentResultBatch {
    pub schema: Vec<Field>,
    pub columns: Vec<Column>,
    pub rows: usize,
}

impl ExperimentResultBatch {
    pub fn from_results(results: &[ExperimentResult]) -> Result<Self, EngineError> {
        if results.is_empty() {
            return Err(EngineError::EmptyInput {
                context: "experiment result batch",
            });
        }

        Self::try_new(
            vec![
                Field::new("id", ColumnType::Utf8),
                Field::new("window", ColumnType::U64),
                Field::new("entry_z", ColumnType::F64),
                Field::new("fee_bps", ColumnType::F64),
                Field::new("seed", ColumnType::U64),
                Field::new("final_equity", ColumnType::F64),
                Field::new("total_return", ColumnType::F64),
                Field::new("max_drawdown", ColumnType::F64),
            ],
            vec![
                Column::Utf8(results.iter().map(|result| result.id.clone()).collect()),
                Column::U64(results.iter().map(|result| result.window as u64).collect()),
                Column::F64(results.iter().map(|result| result.entry_z).collect()),
                Column::F64(results.iter().map(|result| result.fee_bps).collect()),
                Column::U64(results.iter().map(|result| result.seed).collect()),
                Column::F64(results.iter().map(|result| result.final_equity).collect()),
                Column::F64(results.iter().map(|result| result.total_return).collect()),
                Column::F64(results.iter().map(|result| result.max_drawdown).collect()),
            ],
        )
    }

    pub fn try_new(schema: Vec<Field>, columns: Vec<Column>) -> Result<Self, EngineError> {
        if schema.is_empty() || columns.is_empty() {
            return Err(EngineError::EmptyInput {
                context: "columnar batch",
            });
        }

        if schema.len() != columns.len() {
            return Err(EngineError::LengthMismatch {
                context: "schema and columns",
                expected: schema.len(),
                actual: columns.len(),
            });
        }

        let rows = columns[0].len();

        for (field, column) in schema.iter().zip(&columns) {
            if column.len() != rows {
                return Err(EngineError::LengthMismatch {
                    context: "column lengths",
                    expected: rows,
                    actual: column.len(),
                });
            }

            if field.dtype != column.dtype() {
                return Err(EngineError::InvalidConfig {
                    context: "column dtype must match schema",
                });
            }
        }

        Ok(Self {
            schema,
            columns,
            rows,
        })
    }

    pub fn project(&self, names: &[&str]) -> Result<Self, EngineError> {
        if names.is_empty() {
            return Err(EngineError::EmptyInput {
                context: "projection columns",
            });
        }

        let mut schema = Vec::with_capacity(names.len());
        let mut columns = Vec::with_capacity(names.len());

        for name in names {
            let index = self.column_index(name).ok_or(EngineError::InvalidConfig {
                context: "projection column must exist",
            })?;
            schema.push(self.schema[index].clone());
            columns.push(self.columns[index].clone());
        }

        Self::try_new(schema, columns)
    }

    pub fn filter_total_return_gt(&self, threshold: f64) -> Result<Self, EngineError> {
        let index = self
            .column_index("total_return")
            .ok_or(EngineError::InvalidConfig {
                context: "total_return column must exist",
            })?;
        let Column::F64(values) = &self.columns[index] else {
            return Err(EngineError::InvalidConfig {
                context: "total_return must be f64",
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

        Self::try_new(self.schema.clone(), columns)
    }

    pub fn column_index(&self, name: &str) -> Option<usize> {
        self.schema.iter().position(|field| field.name == name)
    }
}
