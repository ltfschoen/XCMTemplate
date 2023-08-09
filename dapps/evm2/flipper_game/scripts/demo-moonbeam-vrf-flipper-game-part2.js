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

const FlipperGameContractBuilt = require('../build/contracts/FlipperGame.json');
const FlipperGameRandomNumberContractBuilt = require('../build/contracts/FlipperGameRandomNumber.json'); 

const main = async () => {
    let contractAddressFlipperGameMoonbaseAlphaArg = process.argv[2];
    let contractAddressFlipperGameRandomNumberMoonbaseAlphaArg = process.argv[3];
    let gameId = process.argv[4];

    // FlipperGame
    const contractAddressFlipperGameMoonbaseAlpha = contractAddressFlipperGameMoonbaseAlphaArg;
    const flipperGameInstance = new ethers.Contract(
        contractAddressFlipperGameMoonbaseAlpha, FlipperGameContractBuilt.abi, signer);
    console.log('flipperGameInstance: ', flipperGameInstance);

    // FlipperGameRandomNumber
    const contractAddressFlipperGameRandomNumberMoonbaseAlpha = contractAddressFlipperGameRandomNumberMoonbaseAlphaArg;
    const flipperGameRandomNumberInstance = new ethers.Contract(
        contractAddressFlipperGameRandomNumberMoonbaseAlpha, FlipperGameRandomNumberContractBuilt.abi, signer);
    console.log('flipperGameRandomNumberInstance: ', flipperGameRandomNumberInstance);

    const requestId = await flipperGameRandomNumberInstance.requestId.call();
    console.log('requestId: ', requestId.toString());

    currentBlockNumber = await providerMoonbaseAlphaWS.getBlockNumber();
    console.log('currentBlockNumber: ', currentBlockNumber.toString());

    requestStatus = await flipperGameRandomNumberInstance.getRequestStatus.call();
    console.log('requestStatus: ', requestStatus.toString());
    assert.equal(requestStatus, 2, 'should be ready status after waiting a minute using local vrf'); // where 2 in enum is 'READY'    

    // Error: insufficient funds for gas * price + value
    tx = await flipperGameInstance.requestFulfillAnswerOfGame(
        gameId.toString(10),
        {
            from: signer.address,
            gasLimit: 600000,
            // gasPrice: 600000,
            maxPriorityFeePerGas: 2,
        }
    );
    console.log('Fulfilled. Please wait a few blocks until the random number is ready...');

    console.log('tx.hash requestFulfillAnswerOfGame: ', tx.hash);
    receipt = await tx.wait();
    console.log('receipt requestFulfillAnswerOfGame: ', receipt);

    let d20Value;
    for (let event of receipt.events) {
        if (event.event == 'FlipFulfilled') {
            console.log('requestFulfillAnswerOfGame with d20Value: ', event.args.d20Value.toString());
            d20Value = new BN(event.args.d20Value.toString(), 10);
        }
    }

    // Wait at least a few of blocks for it to be fulfilled
    await new Promise((resolve, reject) => setTimeout(resolve, 40000));

    currentBlockNumber = await providerMoonbaseAlphaWS.getBlockNumber();
    console.log('currentBlockNumber: ', currentBlockNumber.toString());

    requestStatus = await flipperGameRandomNumberInstance.getRequestStatus.call();
    console.log('requestStatus: ', requestStatus.toString());

    // tx = await flipperGameInstance.functions.fetchAndAddAnswerToGame(
    tx = await flipperGameInstance.fetchAndAddAnswerToGame(
        gameId.toString(10),
        {
            from: signer.address,
            gasLimit: 600000,
            maxPriorityFeePerGas: 2,
        }
    );
    console.log('tx.hash fetchAndAddAnswerToGame: ', tx.hash);
    receipt = await tx.wait();
    console.log('receipt fetchAndAddAnswerToGame: ', receipt);

    let answer;
    for (let event of receipt.events) {
        if (event.event == 'AddedAnswerForGame') {
            console.log('fetchAndAddAnswerToGame with answer: ', event.args.answer.toString());
            answer = new BN(event.args.answer.toString(), 10);
        }
    }

    console.log('answer: ', answer.toString(10));
}

function panic(error)
{
    console.error('error: ', error);
    process.exit(1);
}

main().catch(panic);
