use std::collections::BTreeMap;

use crate::{EngineError, MarketSeries, PipelineConfig, PipelineReport, run_pipeline};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MetricRegistry {
    counters: BTreeMap<String, u64>,
    gauges: BTreeMap<String, f64>,
}

impl MetricRegistry {
    pub fn increment(&mut self, name: &str, value: u64) {
        *self.counters.entry(name.to_string()).or_insert(0) += value;
    }

    pub fn set_gauge(&mut self, name: &str, value: f64) {
        self.gauges.insert(name.to_string(), value);
    }

    pub fn counter(&self, name: &str) -> u64 {
        self.counters.get(name).copied().unwrap_or(0)
    }

    pub fn gauge(&self, name: &str) -> Option<f64> {
        self.gauges.get(name).copied()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpanRecord {
    pub name: String,
    pub fields: BTreeMap<String, String>,
}

impl SpanRecord {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            fields: BTreeMap::new(),
        }
    }

    pub fn with_field(mut self, key: &str, value: impl ToString) -> Self {
        self.fields.insert(key.to_string(), value.to_string());
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObservedPipelineReport {
    pub report: PipelineReport,
    pub metrics: MetricRegistry,
    pub spans: Vec<SpanRecord>,
}

pub fn run_pipeline_observed(
    series: &MarketSeries,
    config: PipelineConfig,
) -> Result<ObservedPipelineReport, EngineError> {
    let mut metrics = MetricRegistry::default();
    let mut spans = Vec::new();

    metrics.increment("pipeline_runs_total", 1);
    metrics.increment("pipeline_input_rows_total", series.len() as u64);
    spans.push(
        SpanRecord::new("pipeline_start")
            .with_field("symbol", &series.symbol)
            .with_field("input_rows", series.len())
            .with_field("window", config.window),
    );

    let report = run_pipeline(series, config)?;

    metrics.increment("pipeline_signal_rows_total", report.signal_rows as u64);
    metrics.increment("pipeline_factor_rows_total", report.factor_rows as u64);
    metrics.set_gauge("pipeline_final_equity", report.final_equity);
    metrics.set_gauge("pipeline_total_return", report.total_return);
    metrics.set_gauge("pipeline_total_fees", report.total_fees);
    spans.push(
        SpanRecord::new("pipeline_finish")
            .with_field("symbol", &report.symbol)
            .with_field("signal_rows", report.signal_rows)
            .with_field("final_equity", format!("{:.2}", report.final_equity)),
    );

    Ok(ObservedPipelineReport {
        report,
        metrics,
        spans,
    })
}
