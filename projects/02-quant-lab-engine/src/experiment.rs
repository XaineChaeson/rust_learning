use crate::{
    BacktestConfig, EngineError, MarketPartition, MarketSeries, PipelineConfig, SignalConfig,
    run_pipeline,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ExperimentConfig {
    pub id: String,
    pub window: usize,
    pub entry_z: f64,
    pub exit_z: f64,
    pub fee_bps: f64,
    pub seed: u64,
}

impl ExperimentConfig {
    pub fn task_key(
        &self,
        strategy_version: &str,
        partition: &MarketPartition,
    ) -> Result<ExperimentTaskKey, EngineError> {
        ExperimentTaskKey::new(strategy_version, partition, self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExperimentTaskKey {
    pub strategy_version: String,
    pub symbol: String,
    pub start_timestamp: u64,
    pub end_timestamp: u64,
    pub data_version: String,
    pub seed: u64,
    pub window: usize,
    pub entry_z_scaled: i64,
    pub fee_bps_scaled: i64,
}

impl ExperimentTaskKey {
    pub fn new(
        strategy_version: &str,
        partition: &MarketPartition,
        config: &ExperimentConfig,
    ) -> Result<Self, EngineError> {
        let strategy_version = strategy_version.trim();

        if strategy_version.is_empty() {
            return Err(EngineError::InvalidConfig {
                context: "strategy_version must not be empty",
            });
        }

        if config.window == 0 {
            return Err(EngineError::InvalidConfig {
                context: "experiment window must be greater than zero",
            });
        }

        if !config.entry_z.is_finite() || !config.fee_bps.is_finite() {
            return Err(EngineError::InvalidConfig {
                context: "experiment task key numeric fields must be finite",
            });
        }

        Ok(Self {
            strategy_version: strategy_version.to_string(),
            symbol: partition.symbol.clone(),
            start_timestamp: partition.start_timestamp,
            end_timestamp: partition.end_timestamp,
            data_version: partition.data_version.clone(),
            seed: config.seed,
            window: config.window,
            entry_z_scaled: scale_float(config.entry_z),
            fee_bps_scaled: scale_float(config.fee_bps),
        })
    }

    pub fn deterministic_id(&self) -> String {
        format!(
            "strategy={}|symbol={}|from={}|to={}|data={}|seed={}|window={}|entry_z={}|fee_bps={}",
            self.strategy_version,
            self.symbol,
            self.start_timestamp,
            self.end_timestamp,
            self.data_version,
            self.seed,
            self.window,
            self.entry_z_scaled,
            self.fee_bps_scaled,
        )
    }
}

fn scale_float(value: f64) -> i64 {
    (value * 10_000.0).round() as i64
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExperimentResult {
    pub id: String,
    pub window: usize,
    pub entry_z: f64,
    pub fee_bps: f64,
    pub seed: u64,
    pub final_equity: f64,
    pub total_return: f64,
    pub max_drawdown: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExperimentGrid {
    pub windows: Vec<usize>,
    pub entry_z_values: Vec<f64>,
    pub fee_bps_values: Vec<f64>,
    pub seed: u64,
}

impl ExperimentGrid {
    pub fn expand(&self) -> Result<Vec<ExperimentConfig>, EngineError> {
        if self.windows.is_empty()
            || self.entry_z_values.is_empty()
            || self.fee_bps_values.is_empty()
        {
            return Err(EngineError::EmptyInput {
                context: "experiment grid",
            });
        }

        let mut configs = Vec::new();
        let mut index = 0;

        for &window in &self.windows {
            for &entry_z in &self.entry_z_values {
                for &fee_bps in &self.fee_bps_values {
                    index += 1;
                    configs.push(ExperimentConfig {
                        id: format!("exp-{index:04}-w{window}-z{entry_z:.2}-fee{fee_bps:.1}"),
                        window,
                        entry_z,
                        exit_z: entry_z / 3.0,
                        fee_bps,
                        seed: self.seed,
                    });
                }
            }
        }

        Ok(configs)
    }
}

pub fn run_experiment(
    series: &MarketSeries,
    config: &ExperimentConfig,
) -> Result<ExperimentResult, EngineError> {
    let report = run_pipeline(
        series,
        PipelineConfig {
            window: config.window,
            signal: SignalConfig {
                entry_z: config.entry_z,
                exit_z: config.exit_z,
                long_weight: 1.0,
                short_weight: -1.0,
            },
            backtest: BacktestConfig {
                initial_cash: 1_000_000.0,
                fee_bps: config.fee_bps,
            },
        },
    )?;

    Ok(ExperimentResult {
        id: config.id.clone(),
        window: config.window,
        entry_z: config.entry_z,
        fee_bps: config.fee_bps,
        seed: config.seed,
        final_equity: report.final_equity,
        total_return: report.total_return,
        max_drawdown: report.max_drawdown,
    })
}

pub fn run_grid(
    series: &MarketSeries,
    grid: &ExperimentGrid,
) -> Result<Vec<ExperimentResult>, EngineError> {
    grid.expand()?
        .iter()
        .map(|config| run_experiment(series, config))
        .collect()
}
