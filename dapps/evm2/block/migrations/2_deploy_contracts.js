require('dotenv').config()
const { Web3 } = require('web3');

console.log('deploying...');
module.exports = async function (deployer) {
  let providerInstance;
  let web3;
  if (deployer.network == "shibuya") {
    console.log('deploy_contracts to Shibuya');

    const Block = artifacts.require("../build/contracts/Block");
  
    providerInstance = new Web3.providers.WebsocketProvider(process.env.SHIBUYA_ENDPOINT, {}, { delay: 500, autoReconnect: true, maxAttempts: 100 });
    console.log('providerInstance: ', providerInstance);

    web3 = new Web3(providerInstance);
    console.log('web3.currentProvider: ', web3.currentProvider);

    deployer.deploy(Block);
  }
};
