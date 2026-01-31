use reqwest::Client;
use serde::Deserialize;
use anyhow::Result;

#[derive(Deserialize, Debug)]
struct TickerPrice {
    symbol: String,
    price: String,
}

pub struct MarketData {
    client: Client,
    base_url: String,
}

impl MarketData {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "https://api.binance.com/api/v3".to_string(),
        }
    }

    pub async fn fetch_latest_price(&self, symbol: &str) -> Result<f64> {
        let url = format!("{}/ticker/price?symbol={}", self.base_url, symbol);
        let resp = self.client.get(&url).send().await?;
        
        let ticker: TickerPrice = resp.json().await?;
        let price = ticker.price.parse::<f64>()?;
        
        Ok(price)
    }
    
    // Optional: fetch candles if we wanted to backfill SMA, 
    // but for simplicity in "real-time" paper trading, we might just build it up.
    // Ideally we fetch a few candles to start. Let's add that.
    
    pub async fn fetch_recent_closes(&self, symbol: &str, interval: &str, limit: usize) -> Result<Vec<f64>> {
        // api/v3/klines
        let url = format!("{}/klines?symbol={}&interval={}&limit={}", self.base_url, symbol, interval, limit);
        let resp = self.client.get(&url).send().await?;
        
        // Binance kline format is [[timestamp, open, high, low, close, volume, ...], ...]
        let klines: Vec<serde_json::Value> = resp.json().await?;
        
        let mut closes = Vec::new();
        for kline in klines {
            if let Some(close_str) = kline.get(4).and_then(|v| v.as_str()) {
                if let Ok(price) = close_str.parse::<f64>() {
                    closes.push(price);
                }
            }
        }
        
        Ok(closes)
    }
}
