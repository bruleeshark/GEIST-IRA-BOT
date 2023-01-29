use std::{env, error::Error};
use reqwest::Client;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::time::Instant;

#[derive(Deserialize, Debug)]
struct LlamaswapResponse {
    id: u64,
    quote_amount: f64,
    base_amount: f64,
    timestamp: String,
}

fn get_llamaswap_rate(client: &Client, from_token: &str, to_token: &str) -> Result<LlamaswapResponse, Box<dyn Error>> {
    let url = format!("https://api.llamaswap.com/v1/swap?from_token={}&to_token={}", from_token, to_token);
    let res = client.get(&url)
        .send()?
        .json::<LlamaswapResponse>()?;
    Ok(res)
}

fn main() {
    let client = Client::new();
    let min_spread = 0.005;
    let mut rate_cache: HashMap<(String, String), (LlamaswapResponse, Instant)> = HashMap::new();
    let mut borrow_token = "USDT".to_string();
    let mut deposit_token = "DAI".to_string();
    let cache_duration = std::time::Duration::from_secs(60 * 5);
    loop {
        let now = Instant::now();
        let rate = rate_cache.entry((borrow_token.clone(), deposit_token.clone())).or_insert_with(|| {
            (get_llamaswap_rate(&client, &borrow_token, &deposit_token).unwrap(), now)
        });
        if now.duration_since(rate.1) > cache_duration {
            let new_rate = get_llamaswap_rate(&client, &borrow_token, &deposit_token).unwrap();
            *rate = (new_rate, Instant::now());
        }
        if rate.0.quote_amount / rate.0.base_amount - 1.0 > min_spread {
            println!("Executing trade on Llamaswap: {} {} -> {} {}", rate.0.base_amount, borrow_token, rate.0.quote_amount, deposit_token);
        } else {
            println!("Spread too small to execute trade.");
        }
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}
