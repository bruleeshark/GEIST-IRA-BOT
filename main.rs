use std::{env, error::Error};
use config::{Config, File};
use api::{get_interest_rate, make_borrow_transaction, make_deposit_transaction};
use models::{InterestRate, Trade};
use wallet::{sign_transaction, check_balance};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config = Config::default();
    config.merge(File::with_name("config")).unwrap();

    let threshold = config.get_float("threshold")?;
    let wallet_address = config.get_str("wallet_address")?;
    let web3_provider = config.get_str("web3_provider")?;

    loop {
        let interest_rates = get_interest_rate(web3_provider)?;
        let trade = get_best_trade(interest_rates, threshold)?;

        if let Some(t) = trade {
            let borrow_transaction = make_borrow_transaction(t.borrow_token, t.borrow_amount, web3_provider)?;
            let deposit_transaction = make_deposit_transaction(t.deposit_token, t.deposit_amount, web3_provider)?;
            let borrow_signed = sign_transaction(borrow_transaction, wallet_address)?;
            let deposit_signed = sign_transaction(deposit_transaction, wallet_address)?;
            execute_transactions(borrow_signed, deposit_signed, web3_provider)?;
        } else {
            println!("No profitable trade found.")
        }
        // Sleep for a certain period of time before checking again
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}

fn get_best_trade(interest_rates: Vec<InterestRate>, threshold: f32) -> Result<Option<Trade>, Box<dyn Error>> {
    // Iterate over the interest rates and check for a profitable trade
    // Return the trade struct if found, otherwise return None
}

fn execute_transactions(borrow_signed: String, deposit_signed: String, web3_provider: &str) -> Result<(), Box<dyn Error>> {
    // Send the signed transactions to the blockchain
}
