trading bot project is a command-line tool that allows users to automatically trade assets on the blockchain.

* The bot uses web3 providers to query the blockchain for interest rates and other data, and then makes trades based on user-specified thresholds and parameters.

The trading bot attempts to borrow one variety of stablecoin for less interest than can be earned from depositing a different stablecoin.
Thusly : single-market interest rate arbitrage.

* The config module is responsible for storing the configuration parameters of the bot, such as the threshold for making a trade, the wallet address, etc.

* The api module is responsible for interacting with the blockchain and web3 providers, such as the functions for querying the interest rates, making the borrow and deposit transactions, etc.

* The wallet module is responsible for interacting with the user's wallet, such as signing transactions, checking the balance, etc.

* The model module is responsible for creating structs or models representing the data returned by the blockchain, such as the interest rates, the balances, etc.

* The tests folder contains unit tests that are used to ensure that the bot functions correctly.
