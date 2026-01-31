# Rust Paper Trader

I wanted to learn more about Rust and async programming, so I built this text-based paper trading bot. It connects to Binance to get real price data but runs only simulated trades, so you can test out strategies without risking any actual money.

## Features

*   **Real Data**: It pulls live prices for Bitcoin (or other pairs) directly from Binance.
*   **Safe Simulation**: Managing a fake wallet with USDT and Crypto balances.
*   **Simple Strategy**: Uses a classic Moving Average crossover (SMA) to decide when to buy or sell.
*   **Safety Nets**: I added basic Stop-Loss and Take-Profit logic to protect the fake gains.
*   **Tech Stack**: Written in Rust using `tokio` for the async parts and `reqwest` for API calls.

## Prerequisites

You'll just need Rust installed on your machine. If you don't have it yet, you can get it from [rust-lang.org](https://www.rust-lang.org/tools/install).

## How to Run It

1.  Clone this repo:
    ```bash
    git clone https://github.com/ch09/rust_paper_trader.git
    cd rust_paper_trader
    ```

2.  Run the bot:
    ```bash
    cargo run
    ```

It should start compiling and then you'll see it printing out the current price and what it's doing.

## Configuration

If you want to play around with the settings, check out `src/config.rs`. You can change the trading pair, the intervals, or how much "money" you start with:

```rust
pub fn default() -> Self {
    Self {
        pair: "BTCUSDT".to_string(),
        interval: "1m".to_string(), 
        sma_short_period: 10,       
        sma_long_period: 50,       
        stop_loss_pct: 0.02,        
        initial_balance: 10000.0,   
    }
}
```

## How the Strategy Works

Right now, it's pretty simple:
*   **Buy**: When the short-term average price goes slightly above the long-term average.
*   **Sell**: When the short-term average drops below the long-term average.
*   It also automatically sells if the price drops by 2% (Stop Loss) or goes up by 5% (Take Profit).

## Disclaimer

This is just a fun learning project. Please don't try to hook this up to a real wallet with real money without doing a **lot** more testing and security work!
