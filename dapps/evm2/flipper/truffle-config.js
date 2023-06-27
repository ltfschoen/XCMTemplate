const HDWalletProvider = require('@truffle/hdwallet-provider');
require('dotenv').config() 

// Moonbeam Development Node Private Key
const privateKeyDev =
   '99B3C12287537E38C90A9219D4CB074A89A16E9CDB20BF85728EBD97C343E342';
// Moonbase Alpha Private Key --> Please change this to your own Private Key with funds
// NOTE: Do not store your private key in plaintext files
//       this is only for demostration purposes only
const privateKeyMoonbase = process.env.MOONBASE_PRIVATE_KEY;

const defaultMoonbaseEndpoint = 'https://rpc.api.moonbase.moonbeam.network';
const defaultMoonbaseNetworkId = 1287;

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
               // providerOrUrl: defaultMoonbaseEndpoint,
               providerOrUrl: process.env.MOONBASE_BLASTAPI_ENDPOINT,
               // pollingInterval: 20000, // default only 4000, modifying doesn't help
               retryTimeout: 10000, // default 400, modifying doesn't help
            };
            return new HDWalletProvider(args);
         },
         // Try to overcome error, changes below might help pass more tests but not as good as
         // using BlastApi
         // `Uncaught Error: PollingBlockTracker - encountered an error while attempting to update latest block:
         // undefined
         // https://ethereum.stackexchange.com/questions/97773/truffle-migrate-rinkeby-error-pollingblocktracker-encountered-an-error-whil
         // confirmations: 10,
         // timeoutBlocks: 900000,
         // skipDryRun: true,
         websocket: true,
         // gas: 5000000,
         // gasPrice: 50000000000, // 50 Gwei
         // gasLimit is required when using `WebsocketsProvider` instead of `HttpProvider`
         // else get error `TypeError: Cannot create property 'gasLimit' on string '0x467b05'`.
         // if this error appears even with the property set, then try changing to a different
         // internet connection
         gasLimit: 5000000,
         networkCheckTimeout: 1000000000,
         // deploymentPollingInterval: 8000,
         network_id: defaultMoonbaseNetworkId,
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
            // Fixes `"Migrations" -- evm error: InvalidCode(Opcode(95))`
            // https://docs.moonbeam.network/tutorials/eth-api/truffle-start-to-end/
            // https://docs.moonbeam.network/builders/build/eth-api/dev-env/remix/
            evmVersion: 'london',
         },
      },
   },
   // Moonbeam Truffle Plugin & Truffle Plugin for Verifying Smart Contracts
   plugins: ['moonbeam-truffle-plugin', 'truffle-plugin-verify'],
   // https://docs.moonbeam.network/builders/build/eth-api/verify-contracts/etherscan-plugins/#using-the-truffle-verify-plugin
   api_keys: {
      moonscan: process.env.MOONSCAN_API_KEY
   }
};
