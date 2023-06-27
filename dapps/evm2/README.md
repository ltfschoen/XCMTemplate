
# Flipper DApp + Biconomy SDK Social Login + Gasless Transactions

This uses the Biconomy starter kit that implements the Biconomy SDK into a basic React application built with Vite. This Kit comes with everything you need for social login and for making gasless transactions to a contract with the Biconomy SDK. It has been used to implement Flipper with a random number generator.

## Deploy Smart Contracts

See the smart contract setup [README](./flipper/README.md)

## Biconomy Dashboard Registration

Now head over to the [Biconomy SDK Dashboard](https://dashboard.biconomy.io/)

Follow the instruction on the docs [here](https://docs.biconomy.io/guides/biconomy-dashboard) to register your contract, load your gas tank, and grab your api key.

* Add Paymaster (e.g. flipper / Ethereum)
* Copy API key to dapps/evm2/.env

## Frontend Configuration

Install Vite
```
apt-get install -y vite
```

Install dependencies
```bash
yarn
```

```bash
cp .env.example .env;
```

Add your Biconomy API Key and your contract address in the .env file

Build and run:
```bash
yarn build
yarn dev
```

Go to http://localhost:5000
