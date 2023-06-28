require('dotenv').config()
const { Network, Alchemy } = require('alchemy-sdk');
const ethers = require('ethers');

// https://docs.alchemy.com/docs/interacting-with-a-smart-contract
const VRFD20ContractBuilt = require("../build/contracts/VRFD20.json");

const config = {
    apiKey: process.env.ALCHEMY_API_KEY,
    network: Network.ETH_SEPOLIA,
};

const alchemyProvider = new Alchemy(config);
console.log('alchemyProvider', alchemyProvider);

// // Provider
// // https://docs.ethers.org/v6/api/providers/#WebSocketProvider
// const alchemyProvider = new ethers.AlchemyProvider(
//     // _network: 11155111,
//     // url=process.env.CHAINLINK_SEPOLIA_ENDPOINT,
//     _network=Network.ETH_SEPOLIA,
//     // "eth-sepolia",
//     process.env.ALCHEMY_API_KEY
// );

// Signer
const signer = new ethers.Wallet(process.env.MOONBASE_PRIVATE_KEY, alchemyProvider);
console.log('signer', signer);

// Contract
const VRFD20DeployedAtAddress = '0xe22cdfA9d8C8e942B498696ef54584426d2f5Dd6';
const VRFD20Contract = new ethers.Contract(VRFD20DeployedAtAddress, VRFD20ContractBuilt.abi, signer);
console.log('VRFD20Contract', VRFD20Contract);

// // Listen to all new pending transactions
// alchemy.ws.on(
//     {
//         method: "alchemy_pendingTransactions",
//         fromAddress: "0x1dd907ABb024E17d196de0D7Fe8EB507b6cCaae7"
//     },
//     (res) => console.log('detected pending tx from account:', res),
// );

// Example of using the call method
const main = async () => {
    // // Get the latest block
    // const latestBlock = alchemy.core.getBlockNumber();
    // console.log('latestBlock', latestBlock);

    // Interact with VRFD20
    const s_owner = await VRFD20Contract.s_owner();
    console.log("The s_owner is: ", s_owner);
};
 
main();
