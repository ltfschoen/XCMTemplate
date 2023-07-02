require('dotenv').config({ path: '../.env'})
const ethers = require('ethers');
const { Wallet } = require('ethers');
const BN = require('bn.js');

// https://docs.moonbeam.network/builders/build/eth-api/libraries/ethersjs/
const providerRPCMoonbaseAlphaConfig = {
  moonbase: {
    name: 'moonbase-alpha',
    rpc: 'https://rpc.api.moonbase.moonbeam.network',
    chainId: 1287, // 0x507 in hex,
  },
};
// Note: ethers v6.6.2 not yet supported by Moonbase Alpha, use v5
// https://docs.ethers.org/v5/api/providers/#WebSocketProvider
// so use `ethers.providers.JsonRpcProvider` instead of
// `ethers.JsonRpcProvider`
// const providerMoonbaseAlphaRPC = new ethers.providers.JsonRpcProvider(
//     providerRPCMoonbaseAlphaConfig.moonbase.rpc, 
//     {
//         chainId: providerRPCMoonbaseAlphaConfig.moonbase.chainId,
//         name: providerRPCMoonbaseAlphaConfig.moonbase.name,
//     }
// );
// console.log('moonbase alpha provider RPC: ', providerMoonbaseAlphaRPC);

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
    const contractAddressMoonbaseAlpha = '0x4027755C05514421fe00f4Fde0bD3F8475ce8A6b';
    const randomNumberInstance = new ethers.Contract(
        contractAddressMoonbaseAlpha, RandomNumberContractBuilt.abi, signer);
    console.log('randomNumberInstance: ', randomNumberInstance);
    const fulfillmentFee = await randomNumberInstance.MIN_FEE.call();
    console.log('fulfillmentFee: ', fulfillmentFee.toString());
    console.log('fulfillmentFee is bn', BN.isBN(fulfillmentFee));

    console.log('x: ', ethers.utils.formatEther(fulfillmentFee));
    // console.log('accounts', accounts);

    let roller = '0x1dd907ABb024E17d196de0D7Fe8EB507b6cCaae7';
    refundAddress = await randomNumberInstance.requestRandomness(
        roller,
        {
            from: signer.address,
            gasLimit: 600000,
            value: fulfillmentFee
        }
    );
    console.log('refundAddress: ', refundAddress);
    
    const requestId = await randomNumberInstance.requestId.call();
    console.log('requestId: ', requestId.toString());
    // Check status of request id from the randomness precompile
    // https://github.com/PureStake/moonbeam/blob/master/precompiles/randomness/Randomness.sol#L96
    const requestStatus = await randomNumberInstance.getRequestStatus.call();
    console.log('requestStatus: ', requestStatus.toString());

    // Wait for at least MIN_VRF_BLOCKS_DELAY but less than MAX_VRF_BLOCKS_DELAY
    // https://github.com/PureStake/moonbeam/blob/master/precompiles/randomness/Randomness.sol#L13
    // https://github.com/PureStake/moonbeam/blob/master/precompiles/randomness/Randomness.sol#L15
    const MIN_VRF_BLOCKS_DELAY = await randomNumberInstance.VRF_BLOCKS_DELAY.call();
    console.log('MIN_VRF_BLOCKS_DELAY: ', MIN_VRF_BLOCKS_DELAY);

    let currentBlockNumber = await providerMoonbaseAlphaWS.getBlockNumber();
    console.log('currentBlockNumber: ', currentBlockNumber.toString());


    // assert.equal(requestStatus, 1, 'should still be pending'); // where 1 in enum is 'PENDING'
    // evm_mine not defined, since can only do on Ganache not live testnet
    // for (i=0; i<MIN_VRF_BLOCKS_DELAY.length; i++) {
    //     advanceBlock();
    // }

    // TODO - not sure how to wait for next block number
    // while (firstBlockNumber != nextBlockNumber) {
        // TODO - wait for at least 2 blocks
        // setTimeout(async function(){
        //     console.log('setTimeout');
        //     return nextBlockNumber;
        // }, 20000);
        // nextBlockNumber = await web3.eth.getBlockNumber();
        // remove 'n' character from end of blocknumber
    //     nextBlockNumber = currentBlockNumber.toString().replace(/[^0-9.]/g, '');
    //     console.log('nextBlockNumber: ', nextBlockNumber);
    // }
    // console.log('found next block');

    // currentBlockNumber = await web3.eth.getBlockNumber();

    // assert.equal(parseNum(firstBlockNumber), parseNum(secondBlockNumber)+2, 'two blocks should have passed');
    // assert.equal(requestStatus, 2, 'not ready as expected'); // where 2 in enum is 'READY'

    await randomNumberInstance.fulfillRequest(
        {
            from: signer.address,
            gasLimit: 600000,
            gasPrice: 600000,
        }
    );
    const random = await randomNumberInstance.random.call();
    console.log('random number: ', random[0]);
}

main();
