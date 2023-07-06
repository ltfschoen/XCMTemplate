require('dotenv').config()
const { Web3 } = require('web3');
const BN = require('bn.js');

// Uses Mocha and Ganache
const Flipper = artifacts.require("../contracts/lib/Flipper");

console.log('test_Flipper');

let providerInstance = new Web3.providers.WebsocketProvider(process.env.MOONBASE_BLASTAPI_ENDPOINT, {}, { delay: 500, autoReconnect: true, maxAttempts: 100 });
console.log('providerInstance: ', providerInstance);

contract('Flipper', accounts => {
    console.log('accounts: ', accounts);

    let flipperInstance;
    // https://docs.web3js.org/api/web3-utils/function/toWei
    const initValue = false;
    beforeEach(async () => {
        console.log('beforeEach');

        flipperInstance = await Flipper.deployed(); //.new(initValue, { from: accounts[0] });
        // console.log('flipperInstance.address:', flipperInstance.address);
        // delay each test to simulate throttle that isn't available in truffle
        // setTimeout(function(){ done(); }, 5000);
        // await flipperInstance.setRandomNumberContractAddress(randomNumberInstance.address);
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
});
