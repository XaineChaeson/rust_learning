use quant_lab_engine::{
    EngineError, ExperimentConfig, MarketBar, MarketPartition, MarketSeries, MarketUniverse,
};

fn series(symbol: &str, closes: &[f64]) -> MarketSeries {
    let bars = closes
        .iter()
        .enumerate()
        .map(|(index, close)| MarketBar::new(index as u64 + 10, *close))
        .collect();

    MarketSeries::new(symbol, bars).expect("test market series is valid")
}

fn config() -> ExperimentConfig {
    ExperimentConfig {
        id: "exp-demo".to_string(),
        window: 3,
        entry_z: 0.8,
        exit_z: 0.2,
        fee_bps: 1.5,
        seed: 42,
    }
}

#[test]
fn market_series_rejects_empty_symbol() {
    assert_eq!(
        MarketSeries::new("  ", vec![MarketBar::new(1, 100.0)]),
        Err(EngineError::InvalidConfig {
            context: "symbol must not be empty",
        })
    );
}

#[test]
fn universe_rejects_duplicate_symbols() {
    assert_eq!(
        MarketUniverse::new(vec![
            series("AAA", &[100.0, 101.0]),
            series("AAA", &[99.0, 100.0]),
        ]),
        Err(EngineError::InvalidConfig {
            context: "market universe symbols must be unique",
        })
    );
}

#[test]
fn universe_builds_symbol_partitions_with_data_version() {
    let universe = MarketUniverse::new(vec![
        series("AAA", &[100.0, 101.0]),
        series("BBB", &[200.0, 199.0, 201.0]),
    ])
    .expect("unique universe");

    assert_eq!(universe.symbols(), vec!["AAA", "BBB"]);
    assert!(universe.series("BBB").is_some());

    let partitions = universe
        .partitions("vendor-snapshot-2026-05-07")
        .expect("valid data version");

    assert_eq!(
        partitions,
        vec![
            MarketPartition {
                symbol: "AAA".to_string(),
                start_timestamp: 10,
                end_timestamp: 11,
                data_version: "vendor-snapshot-2026-05-07".to_string(),
            },
            MarketPartition {
                symbol: "BBB".to_string(),
                start_timestamp: 10,
                end_timestamp: 12,
                data_version: "vendor-snapshot-2026-05-07".to_string(),
            },
        ]
    );
}

#[test]
fn market_partition_rejects_invalid_identity_fields() {
    assert_eq!(
        MarketPartition::new("", 1, 2, "v1"),
        Err(EngineError::InvalidConfig {
            context: "partition symbol must not be empty",
        })
    );
    assert_eq!(
        MarketPartition::new("AAA", 3, 2, "v1"),
        Err(EngineError::InvalidConfig {
            context: "partition start must be <= end",
        })
    );
    assert_eq!(
        MarketPartition::new("AAA", 1, 2, " "),
        Err(EngineError::InvalidConfig {
            context: "data_version must not be empty",
        })
    );
}

#[test]
fn experiment_task_key_includes_strategy_partition_data_version_and_parameters() {
    let partition = MarketPartition::new("AAA", 10, 20, "vendor-v3").expect("valid partition");
    let key = config()
        .task_key("mean-reversion-v2", &partition)
        .expect("valid task key");

    assert_eq!(
        key.deterministic_id(),
        "strategy=mean-reversion-v2|symbol=AAA|from=10|to=20|data=vendor-v3|seed=42|window=3|entry_z=8000|fee_bps=15000"
    );
}

#[test]
fn experiment_task_key_rejects_ambiguous_identity() {
    let partition = MarketPartition::new("AAA", 10, 20, "vendor-v3").expect("valid partition");

    assert_eq!(
        config().task_key(" ", &partition),
        Err(EngineError::InvalidConfig {
            context: "strategy_version must not be empty",
        })
    );

    assert_eq!(
        ExperimentConfig {
            window: 0,
            ..config()
        }
        .task_key("strategy-v1", &partition),
        Err(EngineError::InvalidConfig {
            context: "experiment window must be greater than zero",
        })
    );

    assert_eq!(
        ExperimentConfig {
            entry_z: f64::INFINITY,
            ..config()
        }
        .task_key("strategy-v1", &partition),
        Err(EngineError::InvalidConfig {
            context: "experiment task key numeric fields must be finite",
        })
    );
}
