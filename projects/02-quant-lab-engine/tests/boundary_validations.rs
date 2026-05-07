use quant_lab_engine::{
    BoundaryMode, Column, ColumnType, DType, EngineError, ExperimentResultBatch, Field,
    PythonArraySpec, partition_ranges, plan_python_boundary,
};

#[test]
fn columnar_rejects_schema_column_count_mismatch() {
    assert_eq!(
        ExperimentResultBatch::try_new(
            vec![Field::new("id", ColumnType::Utf8)],
            vec![Column::Utf8(Vec::new()), Column::F64(Vec::new())],
        ),
        Err(EngineError::LengthMismatch {
            context: "schema and columns",
            expected: 1,
            actual: 2,
        })
    );
}

#[test]
fn columnar_rejects_row_length_mismatch() {
    assert_eq!(
        ExperimentResultBatch::try_new(
            vec![
                Field::new("id", ColumnType::Utf8),
                Field::new("equity", ColumnType::F64),
            ],
            vec![
                Column::Utf8(vec!["a".to_string(), "b".to_string()]),
                Column::F64(vec![1.0]),
            ],
        ),
        Err(EngineError::LengthMismatch {
            context: "column lengths",
            expected: 2,
            actual: 1,
        })
    );
}

#[test]
fn columnar_rejects_dtype_mismatch() {
    assert_eq!(
        ExperimentResultBatch::try_new(
            vec![Field::new("equity", ColumnType::F64)],
            vec![Column::U64(vec![100])],
        ),
        Err(EngineError::InvalidConfig {
            context: "column dtype must match schema",
        })
    );
}

#[test]
fn projection_validation_rejects_empty_and_missing_columns() {
    let batch = ExperimentResultBatch::try_new(
        vec![
            Field::new("id", ColumnType::Utf8),
            Field::new("total_return", ColumnType::F64),
        ],
        vec![Column::Utf8(vec!["a".to_string()]), Column::F64(vec![0.01])],
    )
    .expect("valid batch");

    assert_eq!(
        batch.project(&[]),
        Err(EngineError::EmptyInput {
            context: "projection columns",
        })
    );
    assert_eq!(
        batch.project(&["missing"]),
        Err(EngineError::InvalidConfig {
            context: "projection column must exist",
        })
    );
}

#[test]
fn partition_ranges_validation_is_explicit() {
    assert_eq!(
        partition_ranges(0, 2),
        Err(EngineError::EmptyInput {
            context: "parallel input",
        })
    );
    assert_eq!(
        partition_ranges(3, 0),
        Err(EngineError::InvalidConfig {
            context: "parallel workers must be greater than zero",
        })
    );
    assert_eq!(
        partition_ranges(3, 10).expect("partitions are capped to input length"),
        vec![(0, 1), (1, 2), (2, 3)]
    );
}

#[test]
fn python_boundary_owned_output_forces_copy() {
    let plan = plan_python_boundary(
        PythonArraySpec {
            dtype: DType::F64,
            rows: 1,
            cols: 2,
            contiguous: true,
            contains_nan: false,
        },
        true,
    )
    .expect("valid boundary plan");

    assert_eq!(plan.mode, BoundaryMode::Copy);
    assert!(plan.release_gil_around_kernel);
}
