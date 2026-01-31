use log::{info, warn};
use colored::*;

#[derive(Debug, Clone)]
pub struct Position {
    pub entry_price: f64,
    pub quantity: f64,
    pub timestamp: i64,
}

#[derive(Debug)]
pub struct Wallet {
    pub usdt_balance: f64,
    pub crypto_balance: f64, // e.g., BTC balance
    pub positions: Vec<Position>,
}

impl Wallet {
    pub fn new(initial_balance: f64) -> Self {
        Self {
            usdt_balance: initial_balance,
            crypto_balance: 0.0,
            positions: Vec::new(),
        }
    }

    pub fn place_buy_order(&mut self, price: f64, quantity: f64) -> anyhow::Result<()> {
        let cost = price * quantity;
        if self.usdt_balance >= cost {
            self.usdt_balance -= cost;
            self.crypto_balance += quantity;
            self.positions.push(Position {
                entry_price: price,
                quantity,
                timestamp: chrono::Utc::now().timestamp(),
            });
            info!("{}", format!("BUY ORDER EXECUTED: {} @ ${:.2} (Cost: ${:.2})", quantity, price, cost).green());
            Ok(())
        } else {
            warn!("Insufficient funds to buy using ${}", cost);
            Err(anyhow::anyhow!("Insufficient funds"))
        }
    }

    pub fn place_sell_order(&mut self, price: f64, quantity: f64) -> anyhow::Result<()> {
        if self.crypto_balance >= quantity {
            let revenue = price * quantity;
            self.crypto_balance -= quantity;
            self.usdt_balance += revenue;
            
            // Remove positions (FIFO or simple reduction for simplicity)
            // For this simple bot, we just clear positions if we sell all, or reduce proportionally.
            // Simplified: If selling everything, clear positions.
            if self.crypto_balance <= 0.000001 {
                self.positions.clear();
            }

            info!("{}", format!("SELL ORDER EXECUTED: {} @ ${:.2} (Revenue: ${:.2})", quantity, price, revenue).red());
            Ok(())
        } else {
            warn!("Insufficient crypto to sell {}", quantity);
            Err(anyhow::anyhow!("Insufficient crypto balance"))
        }
    }

    pub fn get_total_equity(&self, current_price: f64) -> f64 {
        self.usdt_balance + (self.crypto_balance * current_price)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_creation() {
        let wallet = Wallet::new(1000.0);
        assert_eq!(wallet.usdt_balance, 1000.0);
        assert_eq!(wallet.crypto_balance, 0.0);
        assert!(wallet.positions.is_empty());
    }

    #[test]
    fn test_buy_order_success() {
        let mut wallet = Wallet::new(1000.0);
        let price = 100.0;
        let quantity = 1.0;
        
        // Buy 1 unit at $100
        let result = wallet.place_buy_order(price, quantity);
        assert!(result.is_ok());
        
        assert_eq!(wallet.usdt_balance, 900.0); // 1000 - 100
        assert_eq!(wallet.crypto_balance, 1.0);
        assert_eq!(wallet.positions.len(), 1);
        assert_eq!(wallet.positions[0].entry_price, 100.0);
    }

    #[test]
    fn test_buy_insufficient_funds() {
        let mut wallet = Wallet::new(100.0);
        let price = 200.0;
        let quantity = 1.0;
        
        // Buy 1 unit at $200 (cost $200) with only $100
        let result = wallet.place_buy_order(price, quantity);
        assert!(result.is_err());
        
        assert_eq!(wallet.usdt_balance, 100.0); // No change
        assert_eq!(wallet.crypto_balance, 0.0);
    }

    #[test]
    fn test_sell_order_success() {
        let mut wallet = Wallet::new(1000.0);
        // Setup: Buy 1 unit first
        let _ = wallet.place_buy_order(100.0, 1.0); 
        
        // Sell 0.5 units at $200
        let result = wallet.place_sell_order(200.0, 0.5);
        assert!(result.is_ok());
        
        // Revenue = 0.5 * 200 = 100
        // New USDT = 900 + 100 = 1000
        // New Crypto = 1.0 - 0.5 = 0.5
        assert_eq!(wallet.usdt_balance, 1000.0);
        assert_eq!(wallet.crypto_balance, 0.5);
    }

    #[test]
    fn test_sell_insufficient_funds() {
        let mut wallet = Wallet::new(1000.0);
        // Crypto balance is 0
        
        let result = wallet.place_sell_order(100.0, 1.0);
        assert!(result.is_err());
    }
}
