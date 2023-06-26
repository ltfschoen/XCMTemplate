require('dotenv').config()
const { Web3 } = require('web3');

var RandomNumber = artifacts.require("../build/contracts/RandomNumber");
var Flipper = artifacts.require("../build/contracts/Flipper");

console.log('deploy_contracts');
let wsProvider = new Web3.providers.WebsocketProvider(process.env.MOONBASE_BLASTAPI_ENDPOINT, {}, { delay: 500, autoReconnect: true, maxAttempts: 10 });
console.log('wsProvider: ', wsProvider);
let web3 = new Web3(wsProvider);
console.log('web3.currentProvider: ', web3.currentProvider);
RandomNumber.setProvider(wsProvider);
Flipper.setProvider(wsProvider);

module.exports = function (deployer) {
  deployer.deploy(RandomNumber, { value: web3.utils.toWei('1', 'ether') });
  deployer.deploy(Flipper, false);
};
