// Uses Mocha and Ganache
const RandomNumber = artifacts.require("RandomNumber");
const Flipper = artifacts.require("Flipper");

advanceBlock = () => {
    return new Promise((resolve, reject) => {
        web3.currentProvider.send({
            jsonrpc: '2.0',
            method: 'evm_mine',
            id: new Date().getTime()
        }, (err, result) => {
            if (err) { return reject(err) }
            const newBlockHash = web3.eth.getBlock('latest').hash;

            return resolve(newBlockHash);
        })
    })
}

contract('Flipper', accounts => {
    let randomNumberInstance;
    let flipperInstance;
    // https://github.com/PureStake/moonbeam/blob/master/precompiles/randomness/Randomness.sol#L17C43-L17C62
    const requiredDeposit = "1000000000000000000"; // Wei (1 Ether)
    const initValue = false;
    beforeEach(async () => {
        // Create contract with 1 Ether (contract must be payable)
        randomNumberInstance = await RandomNumber.new({ from: accounts[0], value: requiredDeposit });
        // Deploy token contract
        flipperInstance = await Flipper.new(initValue, { from: accounts[0] });
    });
    // Check stored value
    it("checks stored value", async () => {
        const value = await flipperInstance.get.call();
        assert.equal(value, initValue, 'value stored does not match initial value');
    });

    // Set flipped value of existing value
    it("should flip the value", async () => {
        const previousValue = await flipperInstance.get.call();
        await flipperInstance.flip.call({ from: accounts[0] });
        const newValue = await flipperInstance.flip.call();
        assert.notEqual(previousValue, newValue, 'newValue is not opposite of previousValue');
    });

    it("requests randomness", async () => {
        const fulfillmentFee = await flipperInstance.MIN_FEE.call();
        const refundAddress = await randomNumberInstance.requestRandomness.call(
            { from: accounts[0] },
            { value: fulfillmentFee },
        );
        const requestId = await randomNumberInstance.requestId.call();

        // Check status of request id from the randomness precompile
        // https://github.com/PureStake/moonbeam/blob/master/precompiles/randomness/Randomness.sol#L96
        const requestStatus = await randomNumberInstance.getRequestStatus.call(requestId);

        // Wait for at least MIN_VRF_BLOCKS_DELAY but less than MAX_VRF_BLOCKS_DELAY
        // https://github.com/PureStake/moonbeam/blob/master/precompiles/randomness/Randomness.sol#L13
        // https://github.com/PureStake/moonbeam/blob/master/precompiles/randomness/Randomness.sol#L15
        const MIN_VRF_BLOCKS_DELAY = await randomNumberInstance.MIN_VRF_BLOCKS_DELAY.call();
        for (i=0; i<MIN_VRF_BLOCKS_DELAY.length; i++) {
            advanceBlock();
        }
        const currentBlock = await web3.eth.getBlock("latest");
        console.log('currentBlock: ', currentBlock);
        assert.equal(currentBlock, 2, 'wrong block');
        console.log('requestStatus: ', requestStatus);
        assert.equal(requestStatus, 2, 'not ready as expected'); // where 2 in enum is 'READY'

        await randomNumberInstance.fulfillRequest.call();
        const random = await randomNumberInstance.random.call();
        console.log('random number: ', random[0]);
        assert.equal(random[0], '1000', 'not the expected random number');
    });
});
