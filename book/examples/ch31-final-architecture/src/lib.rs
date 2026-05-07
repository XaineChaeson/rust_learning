#[derive(Debug, Clone, PartialEq)]
pub struct MarketBatch {
    pub symbol: String,
    pub prices: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FactorBatch {
    pub symbol: String,
    pub values: Vec<f64>,
}

pub trait FactorEngine {
    fn compute(&self, input: &MarketBatch) -> FactorBatch;
}

#[derive(Debug, Clone, Copy)]
pub struct ReturnEngine;

impl FactorEngine for ReturnEngine {
    fn compute(&self, input: &MarketBatch) -> FactorBatch {
        let values = input
            .prices
            .windows(2)
            .map(|window| window[1] / window[0] - 1.0)
            .collect();

        FactorBatch {
            symbol: input.symbol.clone(),
            values,
        }
    }
}

pub fn run_research_pipeline<E: FactorEngine>(engine: E, input: &MarketBatch) -> FactorBatch {
    engine.compute(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn final_architecture_uses_contracts_between_modules() {
        let batch = MarketBatch {
            symbol: String::from("AAPL"),
            prices: vec![100.0, 110.0],
        };

        let output = run_research_pipeline(ReturnEngine, &batch);

        assert_eq!(output.symbol, "AAPL");
        assert!((output.values[0] - 0.1).abs() < 1e-12);
    }
}
