
# Biconomy SDK Social Login + Gasless Transactions

This is a starter kit that implements the Biconomy SDK into a basic React application built with Vite. This Kit comes with everything you need for social login and for making gasless transactions to a contract with the Biconomy SDK. 


## Installation

Navigate to this directory in your terminal and then 

```bash
  cd smartContract
```

You can use npm to install but for these instructions I will be using yarn. It will work the same way with npm or pnpm. 

```bash
  yarn
```

Once dependencies are installed run the following command: 

```bash
  cp .env.example .env;
```

You will need to update the newly created .env file with your wallet's PRIVATE_KEY. Make sure to get API keys from [Etherscan](https://etherscan.io/) or [PolygonScan](https://polygonscan.com/). Create an account to get your API keys, these are free. 

Run the command below to deploy your contract to the Polygon Mumbai Network. Make sure the wallet you got your private key from has a small amount of test matic for deployment. You can get test matic [here](https://faucet.polygon.technology/).

```bash
  npx hardhat run scripts/deploy.ts --network mumbai
```

Once deployed copy the contract address on the terminal and run the command below: 

```bash
  npx hardhat verify --network mumbai <your-contract-address>
```

This will verify your contract on Polygonscan and give you a link to your code on Polygonscan!

## Biconomy Dashboard Registration

Now head over to the [Biconomy SDK Dashboard](https://dashboard.biconomy.io/)

Follow the instructionso on the docs [here](https://docs.biconomy.io/guides/biconomy-dashboard) to register your contract, load your gas tank, and grab your api key.

## Frontend Configuration

In your terminal navigate to the root folder of the project and install dependencies and configure the frontend variables as we did for the contract folder. 

```bash
  yarn
```

```bash
  cp .env.example .env;
```

Add your Biconomy API Key and your contract address in the .env file, and use the dev script to run: 

```bash
  yarn dev
```

You should now be all set to try out the starter kit and begin making changes for your own project! 


## Pull requests welcome

Have any improvements to be made? Feel free to make a Pull Request! 
