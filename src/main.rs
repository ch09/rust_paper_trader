mod config;
mod wallet;
mod market;
mod strategy;
mod engine;

use crate::config::TradingConfig;
use crate::engine::TradingEngine;
use env_logger::Env;
use log::info;

#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("========================================");
    info!("   RUST PAPER TRADING BOT STARTED       ");
    info!("========================================");

    // Load Config
    // In a real app, we might load from file/env. Here we use defaults.
    let mut config = TradingConfig::default();
    
    // Customize config for demo purposes to see action faster if desired
    // config.sma_short_period = 5;
    // config.sma_long_period = 10;
    
    info!("Configuration Loaded:");
    info!("Pair: {}", config.pair);
    info!("Initial Balance: ${}", config.initial_balance);
    info!("Strategy: SMA Cross ({} / {})", config.sma_short_period, config.sma_long_period);

    let mut engine = TradingEngine::new(config);
    engine.run().await;
}
