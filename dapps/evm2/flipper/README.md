# Flipper

Note: The following was based upon Moonbeam Truffle Box https://docs.moonbeam.network/builders/build/eth-api/dev-env/truffle/

## Getting started

* Install Node.js v18.x (i.e. v18.16.0) since has LTS https://nodejs.org/en/download/releases
* Use an LTS version of NPM to avoid error `This version of µWS is not compatible with your Node.js build` https://stackoverflow.com/questions/71081725/this-version-of-%C2%B5ws-is-not-compatible-with-your-node-js-build-error-node-loade

```bash
apt-get remove -y nodejs
rm -rf /usr/lib/node_modules/
curl -fsSL https://deb.nodesource.com/setup_18.x | bash -
apt-get install -y nodejs
curl -sL https://dl.yarnpkg.com/debian/pubkey.gpg | gpg --dearmor | tee /usr/share/keyrings/yarnkey.gpg >/dev/null
echo "deb [signed-by=/usr/share/keyrings/yarnkey.gpg] https://dl.yarnpkg.com/debian stable main" | tee /etc/apt/sources.list.d/yarn.list
apt-get update && apt-get install yarn
npm config rm proxy
npm config rm https-proxy
```
* Install Truffle dependency:
```bash
npm install -g truffle
```
* Check if works
```bash
truffle --version
```
* Fix permissions error since /usr/bin/truffle symlinks to below but below didn't have execute permissions for any users
```bash
chmod 755 /usr/lib/node_modules/truffle/build/cli.bundled.js
```
* Setup Moonbeam Truffle Box template that this has been based upon
```bash
mkdir -p /app/dapps/evm/flipper
DAPP_PATH=/app/dapps/evm/flipper 
git clone https://github.com/PureStake/moonbeam-truffle-box $DAPP_PATH
cd $DAPP_PATH
```bash
* Replace template contracts with just https://github.com/hyperledger/solang/blob/main/examples/substrate/flipper.sol and add `pragma solidity ^0.8.0;`

* Update all dependencies in package.json to new major version
```bash
npm outdated
npm install -g npm-check-updates
ncu -u
npm update
```
* Install dependencies
```bash
npm install
```

* Install Docker in Docker container
```bash
apt-get remove docker docker-engine docker.io containerd runc
apt-get update && apt-get upgrade -y
apt-get install apt-transport-https ca-certificates curl gnupg-agent software-properties-common -y
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | apt-key add -
apt-key fingerprint 0EBFCD88
add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable"
apt-get update
apt-get install docker-ce docker-ce-cli containerd.io -y
adduser user
usermod -aG docker user
systemctl restart docker
systemctl enable docker
apt-get install -y docker-compose-plugin
apt-get install docker-compose
```

Note: It is necessary to run Docker within a Docker container https://devopscube.com/run-docker-in-docker/ in order to use `truffle run moonbeam ...` commands from https://trufflesuite.com/boxes/moonbeam-truffle-box/, which use the Moonbeam Truffle Box plugin https://github.com/PureStake/moonbeam-truffle-plugin, which uses images from Docker hub https://hub.docker.com/r/purestake/moonbeam/tags, or setup manually by following https://docs.moonbeam.network/node-operators/networks/run-a-node/flags/, as follows:

To run a local node run similar to the following:
```bash
mkdir /opt/moonbeam
git clone https://github.com/PureStake/moonbeam /opt/moonbeam
cd /opt/moonbeam
git checkout tags/$(git describe --tags)
cargo build
./target/debug/moonbeam --help
```

Then configure Truffle to connect to it Moonbeam Development `node_modules/.bin/truffle migrate --network dev`. Note that use of Ganache does not include pre-compiles https://docs.moonbeam.network/builders/build/eth-api/dev-env/truffle/.

Preferably use Moonbase Alpha TestNet `node_modules/.bin/truffle migrate --network moonbase` that requires testnet tokens from the faucet https://faucet.moonbeam.network/.

Note: When running tests against Moonbase Alpha TestNet. Disconnect VPN. Try to avoid encountering errors like `Too Many Requests`, `ProviderError`, `ETIMEDOUT`, `32603`, it is important to use a dedicated endpoint that you can get for free here https://blastapi.io/ by creating an account, creating a project, choosing Moonbase Alpha Testnet from their available endpoints and clicking "Activate", and then copying either the RPC endpoint. 
Note: I got error `TypeError: Cannot create property 'gasLimit' on string '0x464aff'` when tried using Blastapi WSS endpoint instead of RPC (https) endpoint. Note: If you change to WSS then you need to use `WebsocketProvider` instead of `HttpProvider`. Solved this error by using `gasLimit` in truffle-config.js (it was not necessary for HTTPS)

This is important because the public endpoint https://rpc.api.moonbase.moonbeam.network has stricter rate limiting. Ensure that you replace the public Moonbase Alpha endpoint in the truffle-config.js file with the dedicated endpoint.

Run tests
```
truffle test
```

* Generate account with Moonkey https://docs.moonbeam.network/node-operators/networks/collators/requirements/#generating-an-account-with-moonkey
```
cd /opt
wget https://github.com/PureStake/moonbeam/releases/download/v0.8.0/moonkey /opt
shasum -a 256 moonkey
```
* Verify output is `019c3de832ded3fccffae950835bb455482fca92714448cc0086a7c5f3d48d3e`
* Generate account `./moonkey --w12`
* Obtain Moonbase Alpha tokens from faucet
    * https://faucet.moonbeam.network/

* Compile contracts on network of choice (i.e. "moonbase") defined in truffle.js
    * Compile full `truffle compile --compile-all`
* Migrate
    * Migrate full `truffle migrate --reset --compile-all --network moonbase`
    * Migrate full `truffle migrate --reset --compile-all --network sepolia`
* Test
    * **Important:** It is necessary to first comment-out the code that is **not being compiled** in 2_deploy_contracts.j
    * `truffle test ./test/test_Flipper.js --verbose-rpc --network moonbase`
    * `truffle test ./test/test_ChainlinkVRF.js --network sepolia`
* Verify Contract - Moonbase Precompile

```
# truffle run verify Flipper --network moonbase
Verifying contracts on moonscan
   Verifying Flipper
   Pass - Verified: https://moonbase.moonscan.io/address/0x1c440D264DcCBe9b7AC84edCEC99De926Db98753#code
   Successfully verified 1 contract(s).
Verifying contracts on sourcify
   Failed to connect to Sourcify API at url https://sourcify.dev/server/chains
root@ink:/app/dapps/evm2/flipper# truffle run verify RandomNumber --network moonbase
Verifying contracts on moonscan
   Verifying RandomNumber
   Pass - Verified: https://moonbase.moonscan.io/address/0x4027755C05514421fe00f4Fde0bD3F8475ce8A6b#code
   Successfully verified 1 contract(s).
Verifying contracts on sourcify
   Failed to connect to Sourcify API at url https://sourcify.dev/server/chains
```

* Verify Contract - Chainlink VRF
```
# truffle run verify VRFD20 --network sepolia

Verifying contracts on etherscan
   No etherscan or sepolia_etherscan API Key provided
Verifying contracts on sourcify
   Verifying VRFD20
   Pass - Verified: https://sourcify.dev/#/lookup/0xe22cdfA9d8C8e942B498696ef54584426d2f5Dd6
   Successfully verified 1 contract(s).
```

* Chainlink VRF https://docs.chain.link/getting-started/intermediates-tutorial
    * View token balance https://sepolia.etherscan.io/address/0x1dd907abb024e17d196de0d7fe8eb507b6ccaae7
    * Create and fund a subscription https://docs.chain.link/vrf/v2/subscription/examples/get-a-random-number/#create-and-fund-a-subscription
    * Prepay Subscription https://vrf.chain.link/

* Run
    * node ./scripts/demo.js

* Chainlink VRF https://docs.chain.link/getting-started/intermediates-tutorial
    * View token balance https://sepolia.etherscan.io/address/0x1dd907abb024e17d196de0d7fe8eb507b6ccaae7
    * Create and fund a subscription https://docs.chain.link/vrf/v2/subscription/examples/get-a-random-number/#create-and-fund-a-subscription
    * Prepay Subscription https://vrf.chain.link/
        * Receipt https://sepolia.etherscan.io/tx/0xcc2cd9edf90e0f3351f3398b7013a7259c0acc7cfbfc38454192324fcfdb7d6a
    * Reference v2 contract (not implemented): https://remix.ethereum.org/#url=https://docs.chain.link/samples/VRF/VRFv2Consumer.sol&lang=en&optimize=false&runs=200&evmVersion=null&version=soljson-v0.8.18+commit.87f61d96.js

* Troubleshooting
    * `Client network socket disconnected before secure TLS connection was established`
        * Try fixing by running `unset https_proxy && unset http_proxy`, but this didn't actually work for me
    * If you get error `PollingBlockTracker` then try connecting to a different ISP and disable VPN and stop using a proxy and restart access to your internet
    * Sometimes when you run `truffle test --network moonbase` after changing some CLI options it outputs `Error: The network id specified in the truffle config (1287) does not match the one returned by the network (4619453).  Ensure that both the network and the provider are properly configured`, even though the network id in the truffle-config.js is in fact 1287 (for some reason it confuses the block number for the network id), but when you run it again it might works. So just keep running the command again until it. Or run `truffle test --network sepolia` instead of 
    `truffle test --verbose-rpc --network sepolia`
    works or change internet connection.
* References
    * Hackathon submission
        * https://moonbeam.hackerearth.com/challenges/hackathon/moonbeam-hackathon-2/dashboard
    * https://github.com/trufflesuite/truffle/blob/develop/packages/contract/README.md
    * https://docs.web3js.org/
    * https://evmdocs.acala.network/tutorials/hardhat-tutorials/precompiledtoken-tutorial
    * Chainlink
        * https://vrf.chain.link/
        * VRF https://docs.chain.link/getting-started/intermediates-tutorial
        * Faucet https://faucets.chain.link/
        * https://docs.chain.link/getting-started/advanced-tutorial
        * https://docs.chain.link/vrf/v2/subscription/supported-networks/#configurations
        * https://docs.chain.link/vrf/v2/subscription/examples/get-a-random-number/#create-and-fund-a-subscription
    * Alchemy 
        * https://dashboard.alchemy.com/apps
        * https://docs.alchemy.com/reference/api-overview
        * https://github.com/alchemyplatform/alchemy-sdk-js
        * https://docs.alchemy.com/docs/smart-contract-basics
        * https://docs.alchemy.com/docs/ethers-js-provider
        * https://docs.alchemy.com/docs/interacting-with-a-smart-contract
    * Ethers
        * https://docs.ethers.org/v6/api/providers/#WebSocketProvider
    * XCM 
        * https://docs.moonbeam.network/builders/interoperability/xcm/overview/
        * https://github.com/AstarNetwork/ink-xvm-sdk/tree/main
    * Polkadot.js
        * https://docs.moonbeam.network/builders/build/substrate-api/polkadot-js-api/

    * Other
        * https://github.com/HCastano/urban-planning-in-the-paraverse-with-ink
        * https://www.rob.tech/blog/hybrid-chains/