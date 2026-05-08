#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BacktestRow {
    pub price: f64,
    pub target_weight: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BacktestResult {
    pub equity_curve: Vec<f64>,
    pub final_equity: f64,
}

pub fn run_single_asset_backtest(rows: &[BacktestRow], initial_cash: f64) -> BacktestResult {
    let mut cash = initial_cash;
    let mut quantity = 0.0;
    let mut equity_curve = Vec::with_capacity(rows.len());

    for row in rows {
        let equity = cash + quantity * row.price;
        let target_value = equity * row.target_weight;
        quantity = target_value / row.price;
        cash = equity - target_value;
        equity_curve.push(cash + quantity * row.price);
    }

    let final_equity = equity_curve.last().copied().unwrap_or(initial_cash);

    BacktestResult {
        equity_curve,
        final_equity,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_single_asset_target_weight_backtest() {
        let rows = [
            BacktestRow {
                price: 100.0,
                target_weight: 1.0,
            },
            BacktestRow {
                price: 110.0,
                target_weight: 1.0,
            },
        ];

        let result = run_single_asset_backtest(&rows, 1_000.0);

        assert_eq!(result.equity_curve, vec![1_000.0, 1_100.0]);
    }
}
