require('dotenv').config({ path: './.env'})
// note: change the below to '../.env' if run from in the ./scripts directory
// otherwise get error `TypeError: Cannot read properties of undefined (reading 'toHexString')`
// since unable to load variables from .env file
const process = require('process');
const ethers = require('ethers');
const { Wallet } = require('ethers');
const BN = require('bn.js');
const assert = require('assert');

const providerMoonbaseAlphaWS = new ethers.providers.WebSocketProvider(
    // process.env.MOONBASE_BLASTAPI_ENDPOINT, // need auth for this endpoint
    "wss://moonbeam-alpha.api.onfinality.io/public-ws",
    {
        name: "moonbase-alphanet", // or "moonbase-alpha"
        chainId: 1287, // 0x507 in hex,
    },
);
console.log('moonbase alpha provider WS: ', providerMoonbaseAlphaWS);

// Signer
const signer = new Wallet(process.env.MOONBASE_PRIVATE_KEY, providerMoonbaseAlphaWS);
console.log('signer', signer);

const RandomNumberContractBuilt = require('../build/contracts/RandomNumber.json'); 

const main = async () => {
    let contractAddressArg = process.argv[2];

    const contractAddressMoonbaseAlpha = contractAddressArg; // uses local VRF
    
    const randomNumberInstance = new ethers.Contract(
        contractAddressMoonbaseAlpha, RandomNumberContractBuilt.abi, signer);
    console.log('randomNumberInstance: ', randomNumberInstance);

    const requestId = await randomNumberInstance.requestId.call();
    console.log('requestId: ', requestId.toString());

    let currentBlockNumber = await providerMoonbaseAlphaWS.getBlockNumber();
    console.log('currentBlockNumber: ', currentBlockNumber.toString());

    let requestStatus = await randomNumberInstance.getRequestStatus.call();
    console.log('requestStatus: ', requestStatus.toString());
    assert.equal(requestStatus, 2, 'should be ready status after waiting a minute using local vrf'); // where 2 in enum is 'READY'    

    // Error: insufficient funds for gas * price + value
    let fulfilled = await randomNumberInstance.fulfillRequest(
        {
            from: signer.address,
            gasLimit: 600000,
            // gasPrice: 600000,
            maxPriorityFeePerGas: 2,
        }
    );
    console.log('Fulfilled. Please wait a few blocks until the random number is ready...');

    // Wait at least a few of blocks for it to be fulfilled
    await new Promise((resolve, reject) => setTimeout(resolve, 40000));

    currentBlockNumber = await providerMoonbaseAlphaWS.getBlockNumber();
    console.log('currentBlockNumber: ', currentBlockNumber.toString());

    requestStatus = await randomNumberInstance.getRequestStatus.call();
    console.log('requestStatus: ', requestStatus.toString());

    // `random` is a non-payable method
    const random = await randomNumberInstance.functions.random(0);
    console.log('random number: ', random.length && random[0].toString());

    const randomUsingModulus = await randomNumberInstance.functions.getRolledValueForPlayer(signer.address);
    console.log('randomUsingModulus: ', randomUsingModulus.toString());
}

function panic(error)
{
    console.error('error: ', error);
    process.exit(1);
}

main().catch(panic);