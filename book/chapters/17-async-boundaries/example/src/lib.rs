#[derive(Debug, Clone, PartialEq)]
pub struct MarketEvent {
    pub sequence: u64,
    pub symbol: String,
    pub price: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IngestDecision {
    Accept,
    DropDuplicate,
    DropOutOfOrder,
}

#[derive(Debug, Default)]
pub struct IngestState {
    last_sequence: Option<u64>,
}

impl IngestState {
    pub fn accept(&mut self, event: &MarketEvent) -> IngestDecision {
        match self.last_sequence {
            None => {
                self.last_sequence = Some(event.sequence);
                IngestDecision::Accept
            }
            Some(previous) if event.sequence == previous => IngestDecision::DropDuplicate,
            Some(previous) if event.sequence < previous => IngestDecision::DropOutOfOrder,
            Some(_) => {
                self.last_sequence = Some(event.sequence);
                IngestDecision::Accept
            }
        }
    }
}

pub async fn normalize_event(mut event: MarketEvent) -> MarketEvent {
    event.symbol = event.symbol.trim().to_ascii_uppercase();
    event
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn async_boundary_keeps_ordering_rules_in_sync_code() {
        let mut state = IngestState::default();
        let first = MarketEvent {
            sequence: 10,
            symbol: String::from("aapl"),
            price: 100.0,
        };
        let older = MarketEvent {
            sequence: 9,
            symbol: String::from("aapl"),
            price: 99.0,
        };

        assert_eq!(state.accept(&first), IngestDecision::Accept);
        assert_eq!(state.accept(&older), IngestDecision::DropOutOfOrder);
    }
}
