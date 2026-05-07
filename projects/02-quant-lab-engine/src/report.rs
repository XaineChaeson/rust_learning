#[derive(Debug, Clone, PartialEq)]
pub struct PipelineReport {
    pub symbol: String,
    pub input_rows: usize,
    pub factor_rows: usize,
    pub signal_rows: usize,
    pub final_equity: f64,
    pub total_return: f64,
    pub max_drawdown: f64,
    pub turnover: f64,
    pub total_fees: f64,
}

impl PipelineReport {
    pub fn to_markdown(&self) -> String {
        format!(
            "# Quant Lab Report\n\n\
             ## System Goal\n\n\
             Train a small Rust quant pipeline with explicit data, factor, signal, backtest, and report boundaries.\n\n\
             ## Run Summary\n\n\
             symbol: {}\n\
             input_rows: {}\n\
             factor_rows: {}\n\
             signal_rows: {}\n\
             final_equity: {:.2}\n\
             total_return: {:.6}\n\
             max_drawdown: {:.6}\n\
             turnover: {:.6}\n\
             total_fees: {:.4}\n\n\
             ## Evidence\n\n\
             The report is generated after market-data validation, factor computation, signal alignment, and fee-aware backtesting.\n\n\
             ## Production Risks\n\n\
             Review data quality, zero-variance windows, benchmark noise, Python boundary copies, scheduler retries, and result idempotency before production use.\n",
            self.symbol,
            self.input_rows,
            self.factor_rows,
            self.signal_rows,
            self.final_equity,
            self.total_return,
            self.max_drawdown,
            self.turnover,
            self.total_fees
        )
    }
}
