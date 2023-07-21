# Block

Note: The following was based upon dapps/evm2/randomness

It is necessary since unable to get block hash using ink!
See https://github.com/paritytech/ink/issues/1849

## Getting started

* See installation steps for dapps/evm2/randomness

* Run with:
```
cd dapps/block
npm install -g truffle
npm install
truffle compile --compile-all
truffle migrate --reset --compile-all --network shibuya
```
Obtain the deployed contract address of Block.sol

```
BLOCK_CONTRACT_ADDRESS=<INSERT_DEPLOYED_BLOCK_CONTRACT_ADDRESS>
node ./scripts/demo-block-hash.js $BLOCK_CONTRACT_ADDRESS
```

* Get Shibuya Public Key SS58 from Moonbeam Public Key (hex) using either:
    * `https://kusama.subscan.io/tools/format_transform` OR
    * `docker run -it --pull=always docker.io/parity/subkey:latest generate --network astar`

* Faucet for Shibuya https://portal.astar.network/shibuya-testnet/assets

* Troubleshooting
    * If you get error `Error: The network id specified in the truffle config (81) does not match the one returned by the network (4261879)`, where it confuses the network id with the block number, or `TypeError: Cannot create property 'gasLimit' on string '0x410800'`
        * Restart or change your internet connection