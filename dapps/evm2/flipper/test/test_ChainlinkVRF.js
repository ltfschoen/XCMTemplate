require('dotenv').config()
const { Web3 } = require('web3');
const BN = require('bn.js');

// Uses Mocha and Ganache
const VRFChainlink = artifacts.require("../build/contracts/VRFChainlink");

console.log('test_ChainlinkVRF');

let providerInstance = new Web3.providers.WebsocketProvider(process.env.CHAINLINK_SEPOLIA_ENDPOINT, {}, { delay: 500, autoReconnect: true, maxAttempts: 100 });
console.log('providerInstance: ', providerInstance);
let web3 = new Web3(providerInstance);
console.log('web3.currentProvider: ', web3.currentProvider);

contract('VRFChainlink', accounts => {
    console.log('accounts: ', accounts);
    let vrfChainlinkInstance;
    let gas;
    let gasLimit;
    let gasPrice;

    beforeEach(async () => {
        console.log('beforeEach');

        vrfChainlinkInstance = await VRFChainlink.deployed();
        console.log('vrfChainlinkInstance.address:', vrfChainlinkInstance.address);
    });

    it("requests random VRF", async () => {
        try {
            let s_subscriptionId = await vrfChainlinkInstance.s_subscriptionId.call();
            console.log('s_subscriptionId: ', s_subscriptionId.toString());
            console.log('s_subscriptionId is bn', BN.isBN(s_subscriptionId));
            console.log('accounts', accounts);

            let s_owner = await vrfChainlinkInstance.s_owner.call();
            console.log('s_owner: ', s_owner.toString());

            let roller = '0x1dd907ABb024E17d196de0D7Fe8EB507b6cCaae7';
            requestId = await randomNumberInstance.rollDice(roller, { from: accounts[0], value: fulfillmentFee });
            console.log('requestId: ', requestId);
        } catch (e) {
            console.log('error in requests random VRF: ', e);
        }
    });
});
