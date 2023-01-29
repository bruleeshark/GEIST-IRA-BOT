Our trading bot project is a command-line tool that allows users to automatically trade assets on the Ethereum blockchain. 

The bot uses web3 providers to query the blockchain for interest rates and other data, and then makes trades based on user-specified thresholds and parameters. 

The bot also includes functions for interacting with the user's wallet, such as signing transactions and checking balances. 

The bot is written in Rust programming language and uses the Ethereum blockchain. 

The project is split into several modules, each with a specific function. 

The config module is responsible for storing the configuration parameters of the bot, such as the threshold for making a trade, the wallet address, etc. 

The api module is responsible for interacting with the blockchain and web3 providers, such as the functions for querying the interest rates, making the borrow and deposit transactions, etc. 

The wallet module is responsible for interacting with the user's wallet, such as signing transactions, checking the balance, etc. 

The model module is responsible for creating structs or models representing the data returned by the blockchain, such as the interest rates, the balances, etc. 

The tests folder contains unit tests that are used to ensure that the bot functions correctly.
