use web3::types::{Address, H256, U256};
use web3::Transport;

pub struct Wallet {
    pub address: Address,
    pub transport: Box<dyn Transport>,
}

impl Wallet {
    pub fn new(address: Address, transport: Box<dyn Transport>) -> Self {
        Wallet { address, transport }
    }

    pub fn balance(&self) -> Result<U256, web3::Error> {
        let balance = web3::eth::get_balance(&self.transport, self.address, None)?;
        Ok(balance)
    }

    pub fn sign_transaction(&self, to: Address, value: U256, gas: U256, gas_price: U256, nonce: U256) -> Result<H256, web3::Error> {
        let signed_transaction = web3::eth::accounts::sign_transaction(&self.transport, self.address, to, value, gas, gas_price, nonce)?;
        Ok(signed_transaction.hash)
    }
}
