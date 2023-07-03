require('dotenv').config({ path: '../.env'})
const ethers = require('ethers');
const { Wallet } = require('ethers');
const BN = require('bn.js');

// https://docs.moonbeam.network/builders/build/eth-api/libraries/ethersjs/
// Note: ethers v6.6.2 not yet supported by Moonbase Alpha, use v5
// https://docs.ethers.org/v5/api/providers/#WebSocketProvider
// so use `ethers.providers.JsonRpcProvider` instead of
// `ethers.JsonRpcProvider`
// const providerRPCMoonbaseAlphaConfig = {
//   moonbase: {
//     name: 'moonbase-alpha',
//     rpc: 'https://rpc.api.moonbase.moonbeam.network',
//     chainId: 1287, // 0x507 in hex,
//   },
// };
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

const setAsyncTimeout = (cb, timeout = 0) => new Promise(resolve => {
    setTimeout(() => {
        cb();
        resolve();
    }, timeout);
});

const main = async () => {
    const contractAddressMoonbaseAlpha = '0x4027755C05514421fe00f4Fde0bD3F8475ce8A6b';
    const randomNumberInstance = new ethers.Contract(
        contractAddressMoonbaseAlpha, RandomNumberContractBuilt.abi, signer);
    console.log('randomNumberInstance: ', randomNumberInstance);
    const fulfillmentFee = await randomNumberInstance.MIN_FEE.call();
    console.log('fulfillmentFee: ', fulfillmentFee.toString());
    console.log('fulfillmentFee is bn', BN.isBN(fulfillmentFee));

    console.log('x: ', ethers.utils.formatEther(fulfillmentFee));

    let roller = '0x1dd907ABb024E17d196de0D7Fe8EB507b6cCaae7';
    let res = await randomNumberInstance.requestRandomness(
        roller,
        {
            from: signer.address,
            gasLimit: 600000,
            maxPriorityFeePerGas: 2,
            value: fulfillmentFee
        }
    );
    console.log('res: ', await res);
    // debugging receipt
    console.log('res: ', await res.wait());
    
    const requestId = await randomNumberInstance.requestId.call();
    console.log('requestId: ', requestId.toString());
    // Check status of request id from the randomness precompile
    // https://github.com/PureStake/moonbeam/blob/master/precompiles/randomness/Randomness.sol#L96
    let requestStatus = await randomNumberInstance.getRequestStatus.call();
    console.log('requestStatus: ', requestStatus.toString());

    // Wait for at least MIN_VRF_BLOCKS_DELAY but less than MAX_VRF_BLOCKS_DELAY
    // https://github.com/PureStake/moonbeam/blob/master/precompiles/randomness/Randomness.sol#L13
    // https://github.com/PureStake/moonbeam/blob/master/precompiles/randomness/Randomness.sol#L15
    const MIN_VRF_BLOCKS_DELAY = await randomNumberInstance.VRF_BLOCKS_DELAY.call();
    console.log('MIN_VRF_BLOCKS_DELAY: ', MIN_VRF_BLOCKS_DELAY);

    let currentBlockNumber = await providerMoonbaseAlphaWS.getBlockNumber();
    console.log('currentBlockNumber: ', currentBlockNumber.toString());

    // assert.equal(requestStatus, 1, 'should still be pending'); // where 1 in enum is 'PENDING'

    // Wait a few blocks before fulfilling the request
    // by calling the consumer contract method fulfillRandomWords
    await setAsyncTimeout(async () => {
        console.log('fulfillRequest');

        try {
            // Error: insufficient funds for gas * price + value
            await randomNumberInstance.fulfillRequest(
                {
                    from: signer.address,
                    gasLimit: 600000,
                    // gasPrice: 600000,
                    maxPriorityFeePerGas: 2,
                }
            );
        } catch (e) {
            console.log('fulfillRequest error: ', e);
        }
    }, 10000);

    // requestStatus = await randomNumberInstance.getRequestStatus.call();
    // console.log('requestStatus: ', requestStatus.toString());

    // const random = await randomNumberInstance.random.call();
    // console.log('random number: ', random[0]);
}

main();
