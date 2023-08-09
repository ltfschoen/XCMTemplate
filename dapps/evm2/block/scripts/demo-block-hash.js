require('dotenv').config({ path: './.env'})
// note: change the below to '../.env' if run from in the ./scripts directory
// otherwise get error `TypeError: Cannot read properties of undefined (reading 'toHexString')`
// since unable to load variables from .env file
const process = require('process');
const ethers = require('ethers');
const { Wallet } = require('ethers');
const BN = require('bn.js');
const assert = require('assert');

const providerShibuyaWS = new ethers.providers.WebSocketProvider(
    // process.env.SHIBUYA_ENDPOINT,
    // "wss://rpc.shibuya.astar.network",
    "wss://shibuya-rpc.dwellir.com",
    {
        name: "shibuya",
        chainId: 81,
    },
);
console.log('shibuya provider WS: ', providerShibuyaWS);

// Signer
const signer = new Wallet(process.env.SHIBUYA_PRIVATE_KEY, providerShibuyaWS);
console.log('signer', signer);

const BlockContractBuilt = require('../build/contracts/Block.json');

const main = async () => {
    let contractAddressBlockShibuyaArg = process.argv[2];
    console.log('contractAddressBlockShibuyaArg: ', contractAddressBlockShibuyaArg);

    // Block
    const blockContractInstance = new ethers.Contract(
        contractAddressBlockShibuyaArg, BlockContractBuilt.abi, signer);
    console.log('blockContractInstance: ', blockContractInstance);
    console.log('blockContractInstance address: ', blockContractInstance.address);

    let currentBlockNumberFromCaller = await providerShibuyaWS.getBlockNumber();
    console.log('currentBlockNumberFromCaller: ', currentBlockNumberFromCaller.toString())
    let offset = 1; // previous block number's block hash

    const previousBlockHashByOffset = await blockContractInstance.getPreviousBlockHashByOffset.call(
        currentBlockNumberFromCaller,
        offset,
    );
    console.log('previousBlockHashByOffset is: ', previousBlockHashByOffset.toString());
}

function panic(error)
{
    console.error('error: ', error);
    process.exit(1);
}

main().catch(panic);
