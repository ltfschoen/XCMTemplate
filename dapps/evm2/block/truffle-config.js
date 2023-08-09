const HDWalletProvider = require('@truffle/hdwallet-provider');
require('dotenv').config() 

const privateKeyDev = '';
const privateKeyShibuya = process.env.SHIBUYA_PRIVATE_KEY;

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
               providerOrUrl: 'wss://shibuya-rpc.dwellir.com',
               retryTimeout: 10000,
            };
            return new HDWalletProvider(args);
         },
         websocket: true,
         gasLimit: 5000000,
         networkCheckTimeout: 1000000000,
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
   plugins: ['truffle-plugin-verify'],
   api_keys: {
      etherscan: process.env.ETHERSCAN_API_KEY,
   }
};
