use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeError {
    InvalidConfig(&'static str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeConfig {
    pub service_name: String,
    pub max_in_flight: usize,
}

impl RuntimeConfig {
    pub fn validate(&self) -> Result<(), RuntimeError> {
        if self.service_name.trim().is_empty() {
            return Err(RuntimeError::InvalidConfig(
                "service_name must not be empty",
            ));
        }

        if self.max_in_flight == 0 {
            return Err(RuntimeError::InvalidConfig(
                "max_in_flight must be greater than zero",
            ));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeEvent {
    pub id: u64,
    pub bytes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpanRecord {
    pub name: String,
    pub fields: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MetricRegistry {
    counters: BTreeMap<String, u64>,
}

impl MetricRegistry {
    pub fn increment(&mut self, name: &str, value: u64) {
        *self.counters.entry(name.to_string()).or_insert(0) += value;
    }

    pub fn get(&self, name: &str) -> u64 {
        self.counters.get(name).copied().unwrap_or(0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeReport {
    pub accepted: usize,
    pub dropped: usize,
    pub metrics: MetricRegistry,
    pub spans: Vec<SpanRecord>,
}

pub fn process_events(
    config: &RuntimeConfig,
    events: &[RuntimeEvent],
) -> Result<RuntimeReport, RuntimeError> {
    config.validate()?;

    let mut metrics = MetricRegistry::default();
    let mut spans = Vec::new();
    let mut accepted = 0;
    let mut dropped = 0;

    for event in events {
        let mut fields = BTreeMap::new();
        fields.insert("event_id".to_string(), event.id.to_string());
        fields.insert("bytes".to_string(), event.bytes.to_string());

        if event.bytes == 0 || accepted >= config.max_in_flight {
            dropped += 1;
            metrics.increment("events_dropped_total", 1);
            fields.insert("decision".to_string(), "drop".to_string());
        } else {
            accepted += 1;
            metrics.increment("events_accepted_total", 1);
            metrics.increment("bytes_accepted_total", event.bytes as u64);
            fields.insert("decision".to_string(), "accept".to_string());
        }

        spans.push(SpanRecord {
            name: "ingest_event".to_string(),
            fields,
        });
    }

    Ok(RuntimeReport {
        accepted,
        dropped,
        metrics,
        spans,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_validation_rejects_empty_service_name() {
        let config = RuntimeConfig {
            service_name: String::new(),
            max_in_flight: 1,
        };

        assert_eq!(
            config.validate(),
            Err(RuntimeError::InvalidConfig(
                "service_name must not be empty"
            ))
        );
    }

    #[test]
    fn runtime_records_metrics_and_spans() {
        let config = RuntimeConfig {
            service_name: "quant-online".to_string(),
            max_in_flight: 2,
        };
        let report = process_events(
            &config,
            &[
                RuntimeEvent { id: 1, bytes: 10 },
                RuntimeEvent { id: 2, bytes: 0 },
                RuntimeEvent { id: 3, bytes: 20 },
                RuntimeEvent { id: 4, bytes: 30 },
            ],
        )
        .expect("valid config");

        assert_eq!(report.accepted, 2);
        assert_eq!(report.dropped, 2);
        assert_eq!(report.metrics.get("events_accepted_total"), 2);
        assert_eq!(report.metrics.get("events_dropped_total"), 2);
        assert_eq!(report.metrics.get("bytes_accepted_total"), 30);
        assert_eq!(report.spans.len(), 4);
    }
}
