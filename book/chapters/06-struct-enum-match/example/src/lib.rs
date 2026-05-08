#[derive(Debug, Clone, PartialEq)]
pub struct Bar {
    pub symbol: String,
    pub timestamp: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

impl Bar {
    pub fn typical_price(&self) -> f64 {
        (self.high + self.low + self.close) / 3.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Signal {
    Buy,
    Sell,
    Hold,
    TargetWeight(f64),
}

pub fn signal_to_weight(signal: Signal) -> f64 {
    match signal {
        Signal::Buy => 1.0,
        Signal::Sell => -1.0,
        Signal::Hold => 0.0,
        Signal::TargetWeight(weight) => weight,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signal_maps_to_weight() {
        assert_eq!(signal_to_weight(Signal::Buy), 1.0);
        assert_eq!(signal_to_weight(Signal::Sell), -1.0);
        assert_eq!(signal_to_weight(Signal::Hold), 0.0);
        assert_eq!(signal_to_weight(Signal::TargetWeight(0.25)), 0.25);
    }
}
