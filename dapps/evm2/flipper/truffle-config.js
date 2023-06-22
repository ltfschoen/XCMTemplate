const HDWalletProvider = require('@truffle/hdwallet-provider');
require('dotenv').config() 

// Moonbeam Development Node Private Key
const privateKeyDev =
   '99B3C12287537E38C90A9219D4CB074A89A16E9CDB20BF85728EBD97C343E342';
// Moonbase Alpha Private Key --> Please change this to your own Private Key with funds
// NOTE: Do not store your private key in plaintext files
//       this is only for demostration purposes only
const privateKeyMoonbase = process.env.MOONBASE_PRIVATE_KEY;

module.exports = {
   networks: {
      // Moonbeam Development Network
      dev: {
         provider: () => {
            if (!privateKeyDev.trim()) {
               throw new Error(
                  'Please enter a private key with funds, you can use the default one'
               );
            }
            let args = {
               privateKeys: [privateKeyDev],
               providerOrUrl: 'http://localhost:9944/',
            };
            return new HDWalletProvider(args);
         },
         network_id: 1281,
      },
      // Moonbase Alpha TestNet
      moonbase: {
         provider: () => {
            if (!privateKeyMoonbase.trim()) {
               throw new Error(
                  'Please enter a private key with funds to send transactions to TestNet'
               );
            }
            if (privateKeyDev == privateKeyMoonbase) {
               throw new Error(
                  'Please change the private key used for Moonbase to your own with funds'
               );
            }
            // First argument to new HDWalletProvider() must be a mnemonic phrase,
            // a single private key, or a list of private keys.
            // Expected private key is a Uint8Array with length 32
            // https://github.com/trufflesuite/truffle/tree/develop/packages/hdwallet-provider
            let args = {
               privateKeys: [privateKeyMoonbase],
               providerOrUrl: 'https://rpc.api.moonbase.moonbeam.network',
            };
            return new HDWalletProvider(args);
         },
         network_id: 1287,
      },
      // faucet for SBY https://docs.astar.network/docs/build/environment/faucet
      astar_shibuya: {
         provider: () => {
            let args = {
               privateKeys: [privateKeyAstarShibuya],
               providerOrUrl: 'https://evm.shibuya.astar.network',
            };
            return new HDWalletProvider(args);
         },
         network_id: 81,
      },
   },
   // Solidity 0.8.20 Compiler
   compilers: {
      solc: {
         version: '^0.8.20',
         settings: {
            // Fixes `"Migrations" -- evm error: InvalidCode(Opcode(95))`
            // https://docs.moonbeam.network/tutorials/eth-api/truffle-start-to-end/
            // https://docs.moonbeam.network/builders/build/eth-api/dev-env/remix/
            evmVersion: 'london',
         },
      },
   },
   // Moonbeam Truffle Plugin & Truffle Plugin for Verifying Smart Contracts
   plugins: ['moonbeam-truffle-plugin', 'truffle-plugin-verify'],
};
