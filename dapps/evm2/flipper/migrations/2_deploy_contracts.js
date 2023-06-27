require('dotenv').config()
const { Web3 } = require('web3');
const contract = require("@truffle/contract");

var VRFChainlink = artifacts.require("../build/contracts/VRFChainlink");
var RandomNumber = artifacts.require("../build/contracts/RandomNumber");
var Flipper = artifacts.require("../build/contracts/Flipper");

console.log('deploying...');
module.exports = async function (deployer) {
  let providerInstance;
  let web3;
  console.log('deploy_contracts to Moonbase Alpha');
  
  providerInstance = new Web3.providers.WebsocketProvider(process.env.MOONBASE_BLASTAPI_ENDPOINT, {}, { delay: 500, autoReconnect: true, maxAttempts: 100 });
  console.log('providerInstance: ', providerInstance);
  
  web3 = new Web3(providerInstance);
  console.log('web3.currentProvider: ', web3.currentProvider);
  deployer.deploy(RandomNumber, { value: web3.utils.toWei('1', 'ether') });
  deployer.deploy(Flipper, false);

  console.log('deploy_contracts to Chainlink Sepolia');
  providerInstance = new Web3.providers.WebsocketProvider(process.env.CHAINLINK_SEPOLIA_ENDPOINT, {}, { delay: 500, autoReconnect: true, maxAttempts: 100 });
  console.log('providerInstance: ', providerInstance);
  web3 = new Web3(providerInstance);
  console.log('web3.currentProvider: ', web3.currentProvider);
  const subscriptionId = 3217; // https://vrf.chain.link/
  deployer.deploy(VRFChainlink, { subscriptionId: subscriptionId, value: web3.utils.toWei('0.001', 'ether') });
};
