const HDWalletProvider = require('@truffle/hdwallet-provider');
require('dotenv').config() 
const { Web3 } = require('web3');

const privateKeyDev = '';
const privateKeyShibuya = process.env.SHIBUYA_PRIVATE_KEY;
// https://shibuya.subscan.io/tools/format_transform?input=XXyzohKoEwDWXvpo7CS8unT7rjU6KoiaRm8iE1eWSXMpf3i&type=All
const accountAddress = "0x46b03Bc771e0799dcBc20302e3054b7f34d17E44";

async function checksumShibuyaAccountAddress(_shibuyaAccountAddress) {
   let providerInstance = await new Web3.providers.WebsocketProvider(
      process.env.SHIBUYA_ENDPOINT, {}, { delay: 500, autoReconnect: true, maxAttempts: 100 });
   let web3 = await new Web3(providerInstance);
   console.log('web3.currentProvider: ', web3.currentProvider);
   let checksumShibuyaAccountAddress = web3.utils.toChecksumAddress(_shibuyaAccountAddress);
   return checksumShibuyaAccountAddress;
}
// const { Web3 } = require('web3');
// let providerInstance = new Web3.providers.WebsocketProvider(
//    process.env.SHIBUYA_ENDPOINT, {}, { delay: 500, autoReconnect: true, maxAttempts: 100 });
// let web3 = new Web3(providerInstance);
// const accounts = web3.eth.accounts.privateKeyToAccount(privateKeyShibuya);
// console.log('accounts:', accounts);

module.exports = {
   networks: {
      // faucet for SBY https://docs.astar.network/docs/build/environment/faucet
      shibuya: {
         provider: () => {
            if (!privateKeyShibuya.trim()) {
               throw new Error(
                  'Please enter a private key with funds to send transactions to TestNet'
               );
            }
            if (privateKeyDev == privateKeyShibuya) {
               throw new Error(
                  'Please change the private key used for Shibuya to your own with funds'
               );
            }
            let args = {
               privateKeys: [privateKeyShibuya],
               // HTTPS - https://evm.shibuya.astar.network
               // WSS - wss://rpc.shibuya.astar.network
               //       wss://shibuya-rpc.dwellir.com
               // providerOrUrl: 'wss://rpc.shibuya.astar.network', //process.env.SHIBUYA_ENDPOINT,
               // providerOrUrl: 'wss://shibuya-rpc.dwellir.com',
               providerOrUrl: process.env.SHIBUYA_ENDPOINT,
               retryTimeout: 10000,
            };
            return new HDWalletProvider(args);
         },
         websocket: true,
         // gasLimit: 5000000,
         // networkCheckTimeout: 1000000000,
         confirmations: 10,
         timeoutBlocks: 200,
         skipDryRun: true,
         from: async () => {
            let from = await checksumShibuyaAccountAddress(accountAddress);
            console.log('checksumShibuyaAccountAddress from: ', from)
            return from;
         },
         // from: accountAddress,
         network_id: 81,
      },
   },
   mocha: {
      timeout: 1000000000, // milliseconds
      enableTimeouts: false,
      bail: false,
      retries: 100,
   },
   // Solidity >=0.8.3 Compiler
   compilers: {
      solc: {
         version: '>=0.8.3',
         settings: {
            evmVersion: 'london',
         },
      },
   },
   // Truffle Plugin & Truffle Plugin for Verifying Smart Contracts
   //
   // Note: Shibuya does not currently have a Truffle Plugin.
   // All contract can be verified on the explorer itself, this can be Subscan or Blockscout.
   // Blockscout does have SDK's available to be implemented that will support Astar.
   plugins: ['truffle-plugin-verify'],
   api_keys: {
      etherscan: process.env.ETHERSCAN_API_KEY,
   }
};
