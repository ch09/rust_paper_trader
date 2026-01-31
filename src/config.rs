use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct TradingConfig {
    pub pair: String,
    pub interval: String,
    pub sma_short_period: usize,
    pub sma_long_period: usize,
    pub stop_loss_pct: f64,
    pub take_profit_pct: f64,
    pub initial_balance: f64,
}

impl TradingConfig {
    pub fn default() -> Self {
        Self {
            pair: "BTCUSDT".to_string(),
            interval: "1m".to_string(),
            sma_short_period: 10,
            sma_long_period: 50,
            stop_loss_pct: 0.02, // 2%
            take_profit_pct: 0.05, // 5%
            initial_balance: 10000.0,
        }
    }
}
