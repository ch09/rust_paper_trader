use crate::config::TradingConfig;
use crate::market::MarketData;
use crate::strategy::{SmaStrategy, Signal};
use crate::wallet::Wallet;
use log::{error, info, warn};
use std::time::Duration;
use colored::*;

pub struct TradingEngine {
    config: TradingConfig,
    market: MarketData,
    wallet: Wallet,
    strategy: SmaStrategy,
}

impl TradingEngine {
    pub fn new(config: TradingConfig) -> Self {
        let wallet = Wallet::new(config.initial_balance);
        let market = MarketData::new();
        let strategy = SmaStrategy::new(config.sma_short_period, config.sma_long_period);
        
        Self {
            config,
            market,
            wallet,
            strategy,
        }
    }

    pub async fn run(&mut self) {
        info!("Starting Trading Engine for {}...", self.config.pair);
        
        // 1. Pre-load history to warm up SMA
        info!("Fetching historical data to warm up SMA strategy...");
        // Fetch enough candles for Long SMA + buffer
        let required_history = self.config.sma_long_period + 10;
        match self.market.fetch_recent_closes(&self.config.pair, &self.config.interval, required_history).await {
            Ok(closes) => {
                info!("Loaded {} historical candles.", closes.len());
                self.strategy.init_history(closes);
            },
            Err(e) => {
                error!("Failed to fetch history: {}. Starting with empty history.", e);
            }
        }

        loop {
            // 2. Fetch current price
            match self.market.fetch_latest_price(&self.config.pair).await {
                Ok(price) => {
                    self.process_tick(price).await;
                },
                Err(e) => {
                    error!("Error fetching price: {}", e);
                }
            }

            // 3. Sleep
            // For a "real" bot, we'd aim to align with candle close, but for simple loop:
            tokio::time::sleep(Duration::from_secs(10)).await;
        }
    }

    async fn process_tick(&mut self, price: f64) {
        let equity = self.wallet.get_total_equity(price);
        info!("Current Price: ${:.2} | Equity: ${:.2}", price, equity);

        // Check Risk Management (SL/TP)
        if !self.wallet.positions.is_empty() {
             self.check_risk_management(price);
        }

        // Run Strategy
        let signal = self.strategy.update(price);
        match signal {
            Signal::Buy => {
                self.execute_buy(price);
            },
            Signal::Sell => {
                self.execute_sell(price);
            },
            Signal::None => {}
        }
    }

    fn check_risk_management(&mut self, current_price: f64) {
        // Simple logic: Check the aggregate position or individual positions.
        // Let's check the *average* entry price or just if *any* position hits SL/TP.
        // For simplicity, we sell ALL if SL/TP is hit.
        
        if self.wallet.positions.is_empty() { return; }
        
        // Calculate average entry price
        let total_qty: f64 = self.wallet.positions.iter().map(|p| p.quantity).sum();
        let total_cost: f64 = self.wallet.positions.iter().map(|p| p.quantity * p.entry_price).sum();
        
        if total_qty == 0.0 { return; }
        
        let avg_entry = total_cost / total_qty;
        let pct_change = (current_price - avg_entry) / avg_entry;

        if pct_change <= -self.config.stop_loss_pct {
            warn!("STOP LOSS HIT! Change: {:.2}% (SL: {:.2}%)", pct_change * 100.0, self.config.stop_loss_pct * 100.0);
            self.execute_sell(current_price);
        } else if pct_change >= self.config.take_profit_pct {
            info!("TAKE PROFIT HIT! Change: {:.2}% (TP: {:.2}%)", pct_change * 100.0, self.config.take_profit_pct * 100.0);
            self.execute_sell(current_price);
        }
    }

    fn execute_buy(&mut self, price: f64) {
        // Buy with 50% of available USDT balance for simple money management
        let amount_to_spend = self.wallet.usdt_balance * 0.50;
        if amount_to_spend < 10.0 {
             warn!("Not enough funds to buy (Min $10). Balance: ${:.2}", self.wallet.usdt_balance);
             return;
        }
        
        let quantity = amount_to_spend / price;
        if let Err(e) = self.wallet.place_buy_order(price, quantity) {
            error!("Buy failed: {}", e);
        }
    }

    fn execute_sell(&mut self, price: f64) {
        // Sell 100% of crypto balance
        let quantity = self.wallet.crypto_balance;
        if quantity <= 0.0 { return; } // Nothing to sell

        if let Err(e) = self.wallet.place_sell_order(price, quantity) {
             error!("Sell failed: {}", e);
        }
    }
}
