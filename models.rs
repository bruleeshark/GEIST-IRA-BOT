use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InterestRate {
    pub rate: f64,
    pub asset: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Balance {
    pub balance: f64,
    pub asset: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trade {
    pub buy_amount: f64,
    pub sell_amount: f64,
    pub buy_asset: String,
    pub sell_asset: String,
    pub timestamp: i64,
}
