require('dotenv').config()
const { Web3 } = require('web3');
const BN = require('bn.js');

// const contract = require("@truffle/contract");

var VRFD20 = artifacts.require("../build/contracts/VRFD20");
// var RandomNumber = artifacts.require("../build/contracts/RandomNumber");
// var Flipper = artifacts.require("../build/contracts/Flipper");

console.log('deploying...');
module.exports = async function (deployer) {
  let providerInstance;
  let web3;
  // console.log('deploy_contracts to Moonbase Alpha');
  
  // providerInstance = new Web3.providers.WebsocketProvider(process.env.MOONBASE_BLASTAPI_ENDPOINT, {}, { delay: 500, autoReconnect: true, maxAttempts: 100 });
  // console.log('providerInstance: ', providerInstance);
  
  // web3 = new Web3(providerInstance);
  // console.log('web3.currentProvider: ', web3.currentProvider);
  // deployer.deploy(RandomNumber, { value: web3.utils.toWei('0.000001', 'ether') });
  // deployer.deploy(Flipper, false);

  console.log('deploy_contracts to Chainlink Sepolia');
  providerInstance = new Web3.providers.WebsocketProvider(process.env.CHAINLINK_SEPOLIA_ENDPOINT, {}, { delay: 500, autoReconnect: true, maxAttempts: 100 });
  console.log('providerInstance: ', providerInstance);
  web3 = new Web3(providerInstance);
  console.log('web3.currentProvider: ', web3.currentProvider);
  const subscriptionId = 3217; // https://vrf.chain.link/
  // wants 128620983229604640 wei
  const value = web3.utils.toWei('0.000001', 'ether');
  const amount = new BN(value, 10);
  deployer.deploy(VRFD20, subscriptionId, { value: amount });
};
