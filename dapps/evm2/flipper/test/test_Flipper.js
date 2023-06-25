var Web3 = require('web3');
const BN = require('bn.js');
// require('dotenv').config()

// Uses Mocha and Ganache
const Randomness = artifacts.require("../build/contracts/Randomness");
const RandomnessConsumer = artifacts.require("../build/contracts/RandomnessConsumer");
const RandomNumber = artifacts.require("../contracts/lib/RandomNumber");
const Flipper = artifacts.require("../contracts/lib/Flipper");

// let web3 = new Web3(new Web3.providers.WebsocketProvider(process.env.MOONBASE_BLASTAPI_ENDPOINT));
// // when using BlastAPI WSS endpoint I get error `TypeError: Cannot create property 'gasLimit' on string"`
// // https://github.com/web3/web3.js/issues/3573
// console.log('web3.currentProvider: ', web3.currentProvider);
// Randomness.setProvider(new Web3.providers.WebsocketProvider(process.env.MOONBASE_BLASTAPI_ENDPOINT));
// RandomnessConsumer.setProvider(new Web3.providers.WebsocketProvider(process.env.MOONBASE_BLASTAPI_ENDPOINT));
// RandomNumber.setProvider(new Web3.providers.WebsocketProvider(process.env.MOONBASE_BLASTAPI_ENDPOINT));
// Flipper.setProvider(new Web3.providers.WebsocketProvider(process.env.MOONBASE_BLASTAPI_ENDPOINT));

advanceBlock = () => {
    return new Promise((resolve, reject) => {
        web3.currentProvider.send({
            jsonrpc: '2.0',
            method: 'evm_mine',
            id: new Date().getTime()
        }, (err, result) => {
            console.log('result: ', result);
            console.log('err: ', err);
            if (err) { return reject(err) }
            const newBlockHash = web3.eth.getBlock('latest').hash;
            console.log('newBlockHash: ', newBlockHash);

            return resolve(newBlockHash);
        })
    })
}

contract('Flipper', accounts => {
    console.log('accounts: ', accounts);
    let randomnessInstance;
    let randomNumberInstance;
    let flipperInstance;
    // https://github.com/PureStake/moonbeam/blob/master/precompiles/randomness/Randomness.sol#L17C43-L17C62
    const requiredDeposit = "1000000000000000000"; // Wei (1 Ether)
    const blockTimeout = 1000000;
    const initValue = false;
    beforeEach(async () => {
        console.log('beforeEach');
        randomnessInstance = await Randomness.at("0x0000000000000000000000000000000000000809");
        // console.log('randomnessInstance.address:', randomnessInstance.address);
        RandomnessConsumer.link(randomnessInstance);
        RandomNumber.link(randomnessInstance);

        // Create contract with 1 Ether (contract must be payable)
        randomNumberInstance = await RandomNumber.deployed(); //.new({ from: accounts[0], value: requiredDeposit });
        // console.log('randomNumberInstance.address:', randomNumberInstance.address);
        Flipper.link(randomnessInstance);
        Flipper.link(randomNumberInstance);
        // Deploy token contract

        flipperInstance = await Flipper.deployed(); //.new(initValue, { from: accounts[0] });
        // console.log('flipperInstance.address:', flipperInstance.address);
        // delay each test to simulate throttle that isn't available in truffle
        // setTimeout(function(){ done(); }, 5000);
    });
    // Check stored value
    it("checks stored value", async () => {
        const value = await flipperInstance.get.call();
        console.log('value:', value);
        assert.equal(value, initValue, 'value stored does not match initial value');
    });

    // Set flipped value of existing value
    it("should flip the value", async () => {
        try {
            const previousValue = await flipperInstance.get.call();
            const value = "1000000000000000000";
            await flipperInstance.flip.call({ from: accounts[0], value: value });
            setTimeout(function(){ done(); }, 5000);
            const newValue = await flipperInstance.get.call();
            assert.notEqual(previousValue, newValue, 'newValue is not opposite of previousValue');
        } catch (e) {
           console.log('error in flip: ', e);
        }
    });

    it("requests randomness", async () => {
        // TEMP ONLY - TRYING TO GET `requestRandomness` TO WORK
        try {
            const randomnessPrecompileAddress = await randomNumberInstance.randomnessPrecompileAddress.call();
            console.log('randomnessPrecompileAddress: ', randomnessPrecompileAddress);
            const fulfillmentFee = await randomNumberInstance.MIN_FEE.call();
            console.log('fulfillmentFee: ', fulfillmentFee.toString());
            console.log('is bn', BN.isBN(fulfillmentFee));
            console.log('web3.currentProvider: ', web3.currentProvider);
            const refundAddress = await randomNumberInstance.requestRandomness.call(
                { from: accounts[0] },
                // same error with large amount
                // { value: "1000000000000000000" },
                { value: fulfillmentFee.toString() },
            );
            console.log('refundAddress: ', refundAddress);
            const requestId = await randomNumberInstance.requestId.call();
            console.log('requestId: ', requestId);
        } catch (e) {
            console.log('error in requests randomness: ', e);
        }

        // const fulfillmentFee = await randomNumberInstance.MIN_FEE.call();
        // const refundAddress = await randomNumberInstance.requestRandomness.call(
        //     { from: accounts[0] },
        //     { value: fulfillmentFee },
        // );
        // const requestId = await randomNumberInstance.requestId.call();
        // console.log('requestId: ', requestId);
        // // Check status of request id from the randomness precompile
        // // https://github.com/PureStake/moonbeam/blob/master/precompiles/randomness/Randomness.sol#L96
        // const requestStatus = await randomNumberInstance.getRequestStatus.call(requestId);

        // // Wait for at least MIN_VRF_BLOCKS_DELAY but less than MAX_VRF_BLOCKS_DELAY
        // // https://github.com/PureStake/moonbeam/blob/master/precompiles/randomness/Randomness.sol#L13
        // // https://github.com/PureStake/moonbeam/blob/master/precompiles/randomness/Randomness.sol#L15
        // const MIN_VRF_BLOCKS_DELAY = await randomNumberInstance.MIN_VRF_BLOCKS_DELAY.call();
        // for (i=0; i<MIN_VRF_BLOCKS_DELAY.length; i++) {
        //     advanceBlock();
        // }
        // const currentBlock = await web3.eth.getBlock("latest");
        // console.log('currentBlock: ', currentBlock);
        // assert.equal(currentBlock, 2, 'wrong block');
        // console.log('requestStatus: ', requestStatus);
        // assert.equal(requestStatus, 2, 'not ready as expected'); // where 2 in enum is 'READY'

        // await randomNumberInstance.fulfillRequest.call();
        // const random = await randomNumberInstance.random.call();
        // console.log('random number: ', random[0]);
        // assert.equal(random[0], '1000', 'not the expected random number');
    });
});
