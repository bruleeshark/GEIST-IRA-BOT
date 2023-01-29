use web3::types::{Address, H256, U256};

// Function for querying the interest rate of a token
fn query_interest_rate(token: Address, provider: &web3::Web3<web3::transports::Http>) -> Result<U256, web3::Error> {
    // Get the contract instance
    let contract_instance = get_contract_instance(token, provider)?;

    // Call the 'getInterestRate' function on the contract
    let result = contract_instance.query("getInterestRate", vec![], None, None)?;

    // Parse the result and return the interest rate
    let interest_rate: U256 = result.into_result()?;
    Ok(interest_rate)
}

// Function for making a deposit
fn make_deposit(token: Address, amount: U256, provider: &web3::Web3<web3::transports::Http>) -> Result<H256, web3::Error> {
    // Get the contract instance
    let contract_instance = get_contract_instance(token, provider)?;

    // Call the 'deposit' function on the contract
    let transaction = contract_instance.transact("deposit", vec![amount], None, None)?;
    let transaction_hash = transaction.transaction_hash;

    Ok(transaction_hash)
}

// Function for making a borrow
fn make_borrow(token: Address, amount: U256, provider: &web3::Web3<web3::transports::Http>) -> Result<H256, web3::Error> {
    // Get the contract instance
    let contract_instance = get_contract_instance(token, provider)?;

    // Call the 'borrow' function on the contract
    let transaction = contract_instance.transact("borrow", vec![amount], None, None)?;
    let transaction_hash = transaction.transaction_hash;

    Ok(transaction_hash)
}

// Helper function for getting the contract instance
fn get_contract_instance(token: Address, provider: &web3::Web3<web3::transports::Http>) -> Result<web3::contract::Contract<web3::transports::Http>, web3::Error> {
    // Define the ABI of the contract
    let abi = include_bytes!("contract_abi.json");

    // Create the contract instance
    let contract = web3::contract::Contract::from_json(provider, token, abi)?;

    Ok(contract)
}
