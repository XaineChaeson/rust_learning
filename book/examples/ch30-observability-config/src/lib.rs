#[derive(Debug, Clone, PartialEq)]
pub struct EngineConfig {
    pub worker_threads: usize,
    pub batch_size: usize,
    pub fail_on_nan: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConfigError {
    ZeroWorkers,
    ZeroBatch,
}

impl EngineConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.worker_threads == 0 {
            return Err(ConfigError::ZeroWorkers);
        }

        if self.batch_size == 0 {
            return Err(ConfigError::ZeroBatch);
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Metric {
    pub name: &'static str,
    pub value: f64,
}

pub fn compute_throughput(rows: usize, elapsed_seconds: f64) -> Metric {
    Metric {
        name: "rows_per_second",
        value: rows as f64 / elapsed_seconds,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_validation_rejects_ambiguous_runtime_state() {
        let config = EngineConfig {
            worker_threads: 0,
            batch_size: 1024,
            fail_on_nan: true,
        };

        assert_eq!(config.validate(), Err(ConfigError::ZeroWorkers));
    }

    #[test]
    fn metrics_have_stable_names() {
        assert_eq!(compute_throughput(100, 2.0).value, 50.0);
    }
}
