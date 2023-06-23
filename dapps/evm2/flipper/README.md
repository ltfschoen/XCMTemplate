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

