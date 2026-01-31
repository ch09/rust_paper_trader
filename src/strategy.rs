use log::{info, debug};

#[derive(Debug, PartialEq)]
pub enum Signal {
    Buy,
    Sell,
    None,
}

pub struct SmaStrategy {
    short_period: usize,
    long_period: usize,
    prices: Vec<f64>,
}

impl SmaStrategy {
    pub fn new(short_period: usize, long_period: usize) -> Self {
        Self {
            short_period,
            long_period,
            prices: Vec::new(),
        }
    }

    // Pre-load data to ensure we have enough points for SMA
    pub fn init_history(&mut self, history: Vec<f64>) {
        self.prices = history;
    }

    pub fn update(&mut self, price: f64) -> Signal {
        self.prices.push(price);
        
        // Keep buffer reasonable size (e.g., 2x long period)
        if self.prices.len() > self.long_period * 2 {
            self.prices.remove(0);
        }

        if self.prices.len() < self.long_period {
            debug!("Not enough data for SMA calculation yet. {}/{}", self.prices.len(), self.long_period);
            return Signal::None;
        }

        let short_sma = self.calculate_sma(self.short_period);
        let long_sma = self.calculate_sma(self.long_period);
        
        // We need previous SMAs to check for crossover
        // (This assumes we call update ONCE per new data point)
        // A simple crossover check requires checking the previous step's relationship.
        // For simplicity now, let's just trigger if Short > Long and we weren't before?
        // Or better: store previous state.
        // Actually, let's just calc current and prev.
        
        if self.prices.len() < self.long_period + 1 {
             return Signal::None;
        }

        let prev_short_sma = self.calculate_sma_at_offset(self.short_period, 1);
        let prev_long_sma = self.calculate_sma_at_offset(self.long_period, 1);

        debug!("Short SMA: {:.2} (Prev: {:.2}), Long SMA: {:.2} (Prev: {:.2})", short_sma, prev_short_sma, long_sma, prev_long_sma);

        // Golden Cross: Short crosses ABOVE Long
        if prev_short_sma <= prev_long_sma && short_sma > long_sma {
            info!("GOLDEN CROSS DETECTED: Short SMA {:.2} > Long SMA {:.2}", short_sma, long_sma);
            return Signal::Buy;
        }

        // Death Cross: Short crosses BELOW Long
        if prev_short_sma >= prev_long_sma && short_sma < long_sma {
            info!("DEATH CROSS DETECTED: Short SMA {:.2} < Long SMA {:.2}", short_sma, long_sma);
            return Signal::Sell;
        }

        Signal::None
    }

    fn calculate_sma(&self, period: usize) -> f64 {
        self.calculate_sma_at_offset(period, 0)
    }

    fn calculate_sma_at_offset(&self, period: usize, offset: usize) -> f64 {
        let len = self.prices.len();
        if len < period + offset {
            return 0.0;
        }
        let start = len - period - offset;
        let end = len - offset;
        let slice = &self.prices[start..end];
        let sum: f64 = slice.iter().sum();
        sum / period as f64
    }
}
