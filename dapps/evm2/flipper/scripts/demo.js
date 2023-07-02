require('dotenv').config({ path: '../.env'})
// https://docs.alchemy.com/reference/alchemy-sdk-api-surface-overview#api-surface
const { Network, Alchemy, Contract, Utils, Wallet } = require('alchemy-sdk');
const VRFD20ContractBuilt = require("../build/contracts/VRFD20.json");

// https://www.npmjs.com/package/alchemy-sdk
const config = {
    apiKey: process.env.ALCHEMY_API_KEY,
    network: Network.ETH_SEPOLIA,
};

const alchemyProvider = new Alchemy(config);
console.log('alchemyProvider', alchemyProvider);

// Listen to all new pending transactions
// Note: Alchemy wraps ethers API
// https://docs.alchemy.com/reference/alchemy-sdk-api-surface-overview#alchemy-websockets
// https://docs.ethers.org/v6/api/providers/#WebSocketProvider
alchemyProvider.ws.on(
    {
        method: "alchemy_pendingTransactions",
        fromAddress: "0x1dd907ABb024E17d196de0D7Fe8EB507b6cCaae7"
    },
    (res) => console.log('detected pending tx from account:', res),
);

// Signer
const signer = new Wallet(process.env.MOONBASE_PRIVATE_KEY, alchemyProvider);
console.log('signer', signer);

// Contract
const VRFD20DeployedAtAddress = '0xe22cdfA9d8C8e942B498696ef54584426d2f5Dd6';
// https://docs.ethers.org/v5/api/contract/example/#example-erc-20-contract--connecting-to-a-contract
const VRFD20ContractRW = new Contract(VRFD20DeployedAtAddress, VRFD20ContractBuilt.abi, signer);
// console.log('VRFD20ContractRW', VRFD20ContractRW);

// VRFD20ContractRW.on('DiceRolled',
//     (res) => console.log('detected DiceRolled:', res));

// VRFD20ContractRW.on('DiceLanded',
//     (res) => console.log('detected DiceLanded:', res));

const setAsyncTimeout = (cb, timeout = 0) => new Promise(resolve => {
    setTimeout(() => {
        cb();
        resolve();
    }, timeout);
});

const getRolledValueForPlayer = async () => {
    // Get the latest block
    let latestBlock = await alchemyProvider.core.getBlockNumber();
    console.log('latestBlock', latestBlock);

    const valueRolled = await VRFD20ContractRW
        .getRolledValueForPlayer(signer.address);
    console.log(`The valueRolled by ${signer.address} is: `, valueRolled);
}

// Example of using the call method
const main = async () => {
    const ethersProvider = await alchemyProvider.config.getProvider();
    // console.log('ethersProvider: ', ethersProvider.formatter);

    // Interact with VRFD20
    const s_owner = await VRFD20ContractRW.s_owner();
    console.log("The s_owner is: ", s_owner);

    // let gasLimit = await VRFD20ContractRW.estimateGas.rollDice(signer.address);
    const overrides = {
        // gasLimit: gasLimit,
        gasLimit: 600000,
        gasPrice: 100000,
    };
    // Important: Must have added the latest VRFD20 contract as an approved consumer
    // contract so it can use the subscription balance when requesting randomness
    // https://vrf.chain.link/
    // tx 0xc1ccfd1ea081c846fa3912cb1f610c23fab3ef3ddc5151cea6fa8bd2297e9b84
    const requestId = await VRFD20ContractRW.rollDice(signer.address, overrides);
    console.log("The requestId is: ", requestId.value.toString());

    // Get the latest block
    let latestBlock = await alchemyProvider.core.getBlockNumber();
    console.log('latestBlock', latestBlock);

    // Wait a few blocks before getting the rolled value for a player
    await setAsyncTimeout(async () => {
        console.log('getRolledValueForPlayer');

        // Error: call revert exception; VM Exception while processing transaction: reverted with reason string "Dice not rolled"
        await getRolledValueForPlayer();
    }, 60000);
};
 
main();
