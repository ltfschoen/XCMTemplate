require('dotenv').config()
const { Web3 } = require('web3');
const contract = require("@truffle/contract");

var RandomNumber = artifacts.require("../build/contracts/RandomNumber");
var Flipper = artifacts.require("../build/contracts/Flipper");

console.log('deploying...');
module.exports = async function (deployer) {

  console.log('deploy_contracts');
  let providerInstance = new Web3.providers.WebsocketProvider(process.env.MOONBASE_BLASTAPI_ENDPOINT, {}, { delay: 500, autoReconnect: true, maxAttempts: 100 });
  console.log('providerInstance: ', providerInstance);
  let web3 = new Web3(providerInstance);
  console.log('web3.currentProvider: ', web3.currentProvider);
  // let networkId = await web3.eth.net.getId();
  // console.log('networkId: ', networkId);
  // RandomNumber.setNetwork(networkId);
  // RandomNumber.setProvider(providerInstance);
  // Flipper.setNetwork(networkId);
  // Flipper.setProvider(providerInstance);
  // console.log('web3.eth', web3.eth);
  // const accounts = await web3.eth.getAccounts();
  // console.log('accounts: ', accounts);
  // deployer.deploy(RandomNumber, { from: accounts[0], value: Web3.utils.toWei('1', 'ether') });
  deployer.deploy(RandomNumber, { value: web3.utils.toWei('1', 'ether') });
  // deployer.link(RandomNumber, Flipper);
  deployer.deploy(Flipper, false);
};
