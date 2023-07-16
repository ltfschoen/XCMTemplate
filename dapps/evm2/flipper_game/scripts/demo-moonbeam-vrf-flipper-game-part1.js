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
    console.log('contractAddressFlipperGameMoonbaseAlphaArg: ', contractAddressFlipperGameMoonbaseAlphaArg);
    console.log('contractAddressFlipperGameRandomNumberMoonbaseAlphaArg: ', contractAddressFlipperGameRandomNumberMoonbaseAlphaArg);

    // FlipperGame
    const flipperGameInstance = new ethers.Contract(
        contractAddressFlipperGameMoonbaseAlphaArg, FlipperGameContractBuilt.abi, signer);
    console.log('flipperGameInstance: ', flipperGameInstance);
    console.log('flipperGameInstance address: ', flipperGameInstance.address);
    console.log('contractAddressFlipperGameMoonbaseAlphaArg: ', contractAddressFlipperGameMoonbaseAlphaArg);

    // FlipperGameRandomNumber
    const flipperGameRandomNumberInstance = new ethers.Contract(
        contractAddressFlipperGameRandomNumberMoonbaseAlphaArg, FlipperGameRandomNumberContractBuilt.abi, signer);
    console.log('flipperGameRandomNumberInstance: ', flipperGameRandomNumberInstance);
    console.log('flipperGameRandomNumberInstance address: ', flipperGameRandomNumberInstance.address);
    console.log('contractAddressFlipperGameRandomNumberMoonbaseAlphaArg: ', contractAddressFlipperGameRandomNumberMoonbaseAlphaArg);

    let tx = await flipperGameRandomNumberInstance.setFlipperGameContractAddress(
        flipperGameInstance.address,
        {
            from: signer.address,
            gasLimit: 600000,
            maxPriorityFeePerGas: 2
        }
    );
    console.log('tx.hash setFlipperGameContractAddress: ', tx.hash);
    let receipt = await tx.wait();
    console.log('receipt setFlipperGameContractAddress: ', receipt);

    const flipperGameContractAddress = await flipperGameRandomNumberInstance.flipperGameContractAddress.call();
    console.log('flipperGameContractAddress is: ', flipperGameContractAddress.toString());

    tx = await flipperGameInstance.setFlipperGameRandomNumberContractAddress(
        flipperGameRandomNumberInstance.address,
        {
            from: signer.address,
            gasLimit: 600000,
            maxPriorityFeePerGas: 2
        }
    );
    console.log('tx.hash setFlipperGameRandomNumberContractAddress: ', tx.hash);
    receipt = await tx.wait();
    console.log('receipt setFlipperGameRandomNumberContractAddress: ', receipt);

    const flipperGameRandomNumberContractAddress = await flipperGameInstance.flipperGameRandomNumberContractAddress.call();
    console.log('flipperGameRandomNumberContractAddress is: ', flipperGameRandomNumberContractAddress.toString());

    let guessValue = new BN(10, 10);
    console.log('guessValue: ', guessValue.toString(10));
    tx = await flipperGameInstance.createGame(
        guessValue.toString(10),
        {
            from: signer.address,
            gasLimit: 600000,
            maxPriorityFeePerGas: 2,
        }
    );

    console.log('tx.hash createGame: ', tx.hash);
    receipt = await tx.wait();
    console.log('receipt createGame: ', receipt);

    let gameId;
    for (let event of receipt.events) {
        if (event.event == 'CreatedGame') {
            console.log('createGame finished with gameId: ', event.args.gameId.toString());
            gameId = new BN(event.args.gameId.toString(), 10);
        }
    }

    guessValue = new BN(15, 10);
    tx = await flipperGameInstance.addPlayerToGame(
        gameId.toString(10),
        guessValue.toString(10),
        {
            from: signer.address,
            gasLimit: 600000,
            maxPriorityFeePerGas: 2,
        }
    );

    console.log('tx.hash addPlayerToGame: ', tx.hash);
    receipt = await tx.wait();
    console.log('receipt addPlayerToGame: ', receipt);

    for (let event of receipt.events) {
        if (event.event == 'AddedGuessForPlayerOfGame') {
            console.log('AddedGuessForPlayerOfGame with guess: ', event.args.guess.toString());
        }

        if (event.event == 'AddedPlayerToGame') {
            console.log('AddedPlayerToGame with playerAddress: ', event.args.playerAddress.toString());
        }
    }

    // // TODO - change this to a different player address with a different guess
    // // since each player can only guess once
    //
    // guessValue = new BN(17, 10);
    // guessValue = await flipperGameInstance.addGuessForPlayerOfGame(
    //     gameId,
    //     guessValue,
    //     {
    //         from: signer.address,
    //         gasLimit: 600000,
    //         maxPriorityFeePerGas: 2,
    //     }
    // );
    // console.log('addGuessForPlayerOfGame finished with guess value: ', guessValue);
    const fulfillmentFee = await flipperGameRandomNumberInstance.MIN_FEE.call();
    const fulfillmentFeeBN = new BN(fulfillmentFee.toString(), 10);

    console.log('x: ', ethers.utils.formatEther(fulfillmentFee));

    tx = await flipperGameInstance.requestRandomessAnswerOfGame(
        gameId.toString(10),
        {
            from: signer.address,
            gasLimit: 600000,
            maxPriorityFeePerGas: 2,
            value: fulfillmentFeeBN.toString(10)
        }
    );
    console.log('tx.hash requestRandomessAnswerOfGame: ', tx.hash);
    receipt = await tx.wait();
    console.log('receipt requestRandomessAnswerOfGame: ', receipt);

    let requestId;
    for (let event of receipt.events) {
        if (event.event == 'FlipStarted') {
            console.log('requestRandomessAnswerOfGame with requestId: ', event.args.requestId.toString());
            requestId = new BN(event.args.requestId.toString(), 10);
        }
    }

    // // NOTE: below should not be required since we get the `requestId` from the events above 
    // const requestId = await flipperGameRandomNumberInstance.requestId.call();
    // console.log('requestId: ', requestId.toString());

    // Wait for at least MIN_VRF_BLOCKS_DELAY but less than MAX_VRF_BLOCKS_DELAY
    const MIN_VRF_BLOCKS_DELAY = await flipperGameRandomNumberInstance.VRF_BLOCKS_DELAY.call();
    console.log('MIN_VRF_BLOCKS_DELAY: ', MIN_VRF_BLOCKS_DELAY);

    let currentBlockNumber = await providerMoonbaseAlphaWS.getBlockNumber();
    console.log('currentBlockNumber: ', currentBlockNumber.toString());

    // Check status of request id from the randomness precompile
    // https://github.com/PureStake/moonbeam/blob/master/precompiles/randomness/Randomness.sol#L96
    let requestStatus = await flipperGameRandomNumberInstance.getRequestStatus.call();
    console.log('requestStatus: ', requestStatus.toString());
    assert.equal(requestStatus, 1, 'should still be pending'); // where 1 in enum is 'PENDING'

    console.log('Please wait...');
    // Wait a few blocks before fulfilling the request
    // and calling the consumer contract method fulfillRandomWords
    await new Promise((resolve, reject) => setTimeout(resolve, 70000));
    
    console.log('Ready to proceed with fulfillRequest process...');
}

function panic(error)
{
    console.error('error: ', error);
    process.exit(1);
}

main().catch(panic);