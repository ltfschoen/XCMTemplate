require('dotenv').config()
const { Web3 } = require('web3');

var RandomNumber = artifacts.require("../build/contracts/RandomNumber");
var Flipper = artifacts.require("../build/contracts/Flipper");

console.log('deploy_contracts');
let providerInstance = new Web3.providers.HttpProvider(process.env.MOONBASE_BLASTAPI_ENDPOINT, {}, { delay: 500, autoReconnect: true, maxAttempts: 10 });
console.log('providerInstance: ', providerInstance);
let web3 = new Web3(providerInstance);
console.log('web3.currentProvider: ', web3.currentProvider);
RandomNumber.setProvider(providerInstance);
Flipper.setProvider(providerInstance);
console.log('deploying...');
module.exports = function (deployer) {
  deployer.deploy(RandomNumber, { value: web3.utils.toWei('1', 'ether') });
  deployer.deploy(Flipper, false);
};
