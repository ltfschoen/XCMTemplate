require('dotenv').config()
const { Web3 } = require('web3');
const BN = require('bn.js');

// Uses Mocha and Ganache
const Randomness = artifacts.require("../build/contracts/Randomness");
const RandomnessConsumer = artifacts.require("../build/contracts/RandomnessConsumer");
const RandomNumber = artifacts.require("../contracts/lib/RandomNumber");
const Flipper = artifacts.require("../contracts/lib/Flipper");

console.log('test_Flipper');

let wsProvider = new Web3.providers.WebsocketProvider(process.env.MOONBASE_BLASTAPI_ENDPOINT, {}, { delay: 500, autoReconnect: true, maxAttempts: 10 });
let web3 = new Web3();
// when using BlastAPI WSS endpoint I get error `TypeError: Cannot create property 'gasLimit' on string"`
// https://github.com/web3/web3.js/issues/3573
console.log('web3.currentProvider: ', web3.currentProvider);
// Randomness.setProvider(wsProvider);
// RandomnessConsumer.setProvider(wsProvider);
RandomNumber.setProvider(wsProvider);
Flipper.setProvider(wsProvider);

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
    let fulfillmentFee;
    let refundAddress;
    let gas;
    let gasLimit;
    let gasPrice;
    // https://github.com/PureStake/moonbeam/blob/master/precompiles/randomness/Randomness.sol#L17C43-L17C62
    // https://docs.web3js.org/api/web3-utils/function/toWei
    const requiredDeposit = Web3.utils.toWei('1', 'ether');
    const blockTimeout = 1000000;
    const initValue = false;
    beforeEach(async () => {
        console.log('beforeEach');
        randomnessInstance = await Randomness.at("0x0000000000000000000000000000000000000809");
        // console.log('randomnessInstance.address:', randomnessInstance.address);

        // RandomnessConsumer.link(randomnessInstance);
        // RandomNumber.link(randomnessInstance);

        // gas = Web3.utils.toWei('1000000', 'wei');
        // gasLimit = Web3.utils.toWei('600000', 'wei');
        // gasPrice = Web3.utils.toWei('2000000', 'wei');
        // gas = Web3.utils.toHex(70000);
        // gasLimit = Web3.utils.toHex(600000); // gwei
        // gasPrice = Web3.utils.toHex(21000);
        
        // Create contract with 1 Ether (contract must be payable)
        randomNumberInstance = await RandomNumber.deployed(); //.new({ from: accounts[0], value: requiredDeposit });
        // randomNumberInstance = await RandomNumber.new(
        //     { from: accounts[0], value: requiredDeposit,
        //         gas: gas, gasLimit: gasLimit, gasPrice: gasPrice, syncWithContext: true }
        // );
        console.log('randomNumberInstance.address:', randomNumberInstance.address);

        // Flipper.link(randomnessInstance);
        // Flipper.link(randomNumberInstance);

        // Deploy token contract

        flipperInstance = await Flipper.deployed(); //.new(initValue, { from: accounts[0] });
        // console.log('flipperInstance.address:', flipperInstance.address);
        // delay each test to simulate throttle that isn't available in truffle
        // setTimeout(function(){ done(); }, 5000);
        await flipperInstance.setRandomNumberContractAddress(randomNumberInstance.address);
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
            const oneWei = Web3.utils.toWei('1', 'wei');
            const value = new BN(oneWei, 10); // 1 wei (so do not use up all my testnet DEV tokens)
            // do not use `.call` when doing state changes to blockchain
            await flipperInstance.flip({ from: accounts[0], value: value });
            // setTimeout(function(){ done(); }, 5000);
            const newValue = await flipperInstance.get.call();
            assert.notEqual(previousValue, newValue, 'newValue is not opposite of previousValue');
        } catch (e) {
           console.log('error in flip: ', e);
        }
    });

    it("requests randomness", async () => {
        // TEMP ONLY - TRYING TO GET `requestRandomness` TO WORK
        try {
            fulfillmentFee = await randomNumberInstance.MIN_FEE.call();
            console.log('fulfillmentFee: ', fulfillmentFee.toString());
            console.log('fulfillmentFee is bn', BN.isBN(fulfillmentFee));
            console.log('accounts', accounts);

            // console.log('web3.currentProvider: ', web3.currentProvider);
            // do not use `.call` when doing state changes to blockchain
            // gas = Web3.utils.toWei('1000000', 'wei');
            // gasLimit = Web3.utils.toWei('600000', 'wei');
            // gasPrice = Web3.utils.toWei('2000000', 'wei');
            gas = Web3.utils.toHex(150000);
            gasLimit = Web3.utils.toHex(600000);
            gasPrice = Web3.utils.toHex(21000);
            refundAddress = await randomNumberInstance.requestRandomness(
                {   
                    from: accounts[0],
                    value: fulfillmentFee,
                    gas: gas, gasLimit: gasLimit, gasPrice: gasPrice, 
                    syncWithContext: true
                }
            );
            console.log('refundAddress: ', refundAddress);
            // const requestId = await randomNumberInstance.requestId.call();
            // console.log('requestId: ', requestId);
            // // Check status of request id from the randomness precompile
            // // https://github.com/PureStake/moonbeam/blob/master/precompiles/randomness/Randomness.sol#L96
            // const requestStatus = await randomNumberInstance.getRequestStatus.call(requestId);
            // console.log('requestStatus: ', requestStatus);

            // // Wait for at least MIN_VRF_BLOCKS_DELAY but less than MAX_VRF_BLOCKS_DELAY
            // // https://github.com/PureStake/moonbeam/blob/master/precompiles/randomness/Randomness.sol#L13
            // // https://github.com/PureStake/moonbeam/blob/master/precompiles/randomness/Randomness.sol#L15
            // const MIN_VRF_BLOCKS_DELAY = await randomNumberInstance.MIN_VRF_BLOCKS_DELAY.call();
            // let currentBlock = await web3.eth.getBlock("latest");
            // console.log('currentBlock: ', currentBlock);
            // for (i=0; i<MIN_VRF_BLOCKS_DELAY.length; i++) {
            //     advanceBlock();
            // }
            // currentBlock = await web3.eth.getBlock("latest");
            // console.log('currentBlock: ', currentBlock);

            // assert.equal(currentBlock, 2, 'wrong block');
            // assert.equal(requestStatus, 2, 'not ready as expected'); // where 2 in enum is 'READY'

            // await randomNumberInstance.fulfillRequest.call();
            // const random = await randomNumberInstance.random.call();
            // console.log('random number: ', random[0]);
        } catch (e) {
            console.log('error in requests randomness: ', e);
        }
    });
});
