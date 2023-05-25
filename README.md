### Smart Contract in ink!

#### Docker

##### Setup Container

* Note:
	* The docker/Dockerfile and its dependencies in docker/utility are modified copies of files in https://github.com/paritytech/scripts/blob/master/dockerfiles to have the flexibility to develop locally using ink! in a Docker container that uses Linux without any dependency issues, and where changes replicated on the host machine.
	* This is useful if you're on an old macOS Catalina and Apple won't allow you to do further software updates, so you cannot install `brew install protobuf` to install necessary dependencies.

* Install and run [Docker](https://www.docker.com/)
* Generate .env file from sample file
	* Generate a Substrate-based account on an air-gapped machine using [Subkey](https://support.polkadot.network/support/solutions/articles/65000180519-how-to-create-an-account-in-subkey) or by installing [Subkey in Docker](https://github.com/paritytech/substrate/tree/master/docker) on an air-gapped machine
	* Add the mnemonic phrase of the Substrate-based account to the value of `LS_CONTRACTS` in the .env file.
	* Obtain testnet tokens from faucet at https://use.ink/faucet/
* Check versions used in Dockerfile:
	* Rust nightly version
	* Node.js version
	* Cargo Contract
	* Substrate Contracts Node
* Update dependencies in ./dapps/ink-rust/wasm-flipper/package.json https://stackoverflow.com/a/70588930/3208553
	```
	cd ./dapps/ink-rust/wasm-flipper/
	yarn upgrade
	```
* Update dependencies in ./dapps/ink-rust/wasm-flipper/contract/flipper/Cargo.toml
	```
	cd ./dapps/ink-rust/wasm-flipper/contract/flipper/
	cargo update
	```
* Run Docker container and follow the terminal log instructions.
	* Note: Optionally **exclude** installing substrate-contracts-node by running `time ./docker/docker.sh "without_node"` since including it will increase build time substantially and may not be necessary if you are deploying to remote testnets

	```bash
	touch .env && cp .env.example .env
	time ./docker/docker.sh
	```

* Check Memory & CPU usage. Update memory limits in docker-compose.yml
	```bash
	docker stats
	```
* Enter Docker container
	```bash
	docker exec -it ink /bin/bash
	```
* Optional [Attach to Container in Visual Studio Code (VSCode)](https://code.visualstudio.com/docs/devcontainers/attach-container#_attach-to-a-docker-container)
	* Open folder /app
		```
		cd /app
		```

* Check versions. Note: 
```bash
rustup toolchain list
rustup update
rustup show
cargo-contract --version
```
* Check if substrate-contracts-node was installed if you used the "with-node" argument
```bash
substrate-contracts-node --version
```

##### Run Cargo Contract Node in Docker Container

* **Important** This is only available if you did not run ./docker/run.sh using "without_node" argument

* Run Cargo Contract Node
	* Note: Use either `--tmp` or `--base-path "/tmp/ink"`
	* Note: Delete chain database `rm -rf /tmp/ink`.
	* Note: Check disk space used by database `du /tmp/ink`

```bash
substrate-contracts-node \
	--dev \
	--alice \
	--name "ink-test" \
	--base-path "/tmp/ink" \
	--force-authoring \
	--port 30333 \
	--rpc-port 9933 \
	--ws-port 9944 \
	--ws-external \
	--rpc-methods Unsafe \
	--rpc-cors all \
	--telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
	-lsync=debug
```

* Leave that terminal tab running the node. Enter the terminal again in a new tab with `docker exec -it ink /bin/bash` and run the following:
* Attach to the running terminal with VSCode if necessary. See [here](https://code.visualstudio.com/docs/devcontainers/attach-container)

##### Build & Upload Smart Contract to Local Testnet (using Cargo Contract)

* Create Rust project with template
```
cd dapps/ink-rust/wasm-flipper/contract
cargo contract new flipper
cd flipper
```
* Optionally build with VSCode by adding the project `"dapps/ink-rust/wasm-flipper/contract/flipper"` to the list of members in the Cargo.toml file in the project root, and running "Terminal > Run Task > Build Contract" to build all the contract using the configuration in ./.vscode/launch.json
* Generate .contract, .wasm, and metadata.json code. Note: Use `--release` to deploy
```
cargo contract build --manifest-path /app/dapps/ink-rust/wasm-flipper/contract/flipper/Cargo.toml
```
* Copy ./target/ink/flipper/flipper.json
	* Paste this as the ABI value of `const abi = ` ./dapps/ink-rust/wasm-flipper/ui/components/abi.ts

* Upload Contract (note: prefer to use contracts-ui to avoid exposing private key)
```
cargo contract upload --suri //Alice
```

* Actual output:

	* Terminal #1
		```
		Result Success!
		Code hash "0xec3a66a8f99674ecf25d180fc39ee8e620d45e8de459277e353ece20753d6c53"
			Deposit 434925000000
		Your upload call has not been executed.
		To submit the transaction and execute the call on chain, add -x/--execute flag to the command
		```

	* Terminal #2
		```
		2023-05-11 05:49:47.389  INFO tokio-runtime-worker substrate: 💤 Idle (0 peers), best: #0 (0x18c5…59af), finalized #0 (0x18c5…59af), ⬇ 0 ⬆ 0    
		2023-05-11 05:49:48.124 DEBUG tokio-runtime-worker sync: Propagating transactions    
		2023-05-11 05:49:48.437  INFO tokio-runtime-worker jsonrpsee_ws_server::server: Accepting new connection 1/100
		2023-05-11 05:49:50.006  INFO tokio-runtime-worker jsonrpsee_ws_server::server: Accepting new connection 2/100
		2023-05-11 05:49:51.027 DEBUG tokio-runtime-worker sync: Propagating transactions    
		2023-05-11 05:49:52.392  INFO tokio-runtime-worker substrate: 💤 Idle (0 peers), best: #0 (0x18c5…59af), finalized #0 (0x18c5…59af), ⬇ 0 ⬆ 0
		```

* Upload and Execute it. Optionally `--skip-dry-run`
```
cargo contract upload --suri //Alice \
	--execute \
	--skip-confirm
```

* Note: The output format should be:
	```
	CodeStored event
	code_hash: 0x......
	```
  * Note: only one copy of the code is stored, but there can be many instance of one code blob, differs from other EVM chains where each node has a copy

* Actual output:

	* Terminal #1
		```
			Events
			Event Balances ➜ Withdraw
				who: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
				amount: 2.068216063mUNIT
			Event Balances ➜ Reserved
				who: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
				amount: 434.925mUNIT
			Event Contracts ➜ CodeStored
				code_hash: 0xec3a66a8f99674ecf25d180fc39ee8e620d45e8de459277e353ece20753d6c53
			Event TransactionPayment ➜ TransactionFeePaid
				who: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
				actual_fee: 2.068216063mUNIT
				tip: 0UNIT
			Event System ➜ ExtrinsicSuccess
				dispatch_info: DispatchInfo { weight: Weight { ref_time: 2068203465, proof_size: 0 }, class: Normal, pays_fee: Yes }

		Code hash "0xec3a66a8f99674ecf25d180fc39ee8e620d45e8de459277e353ece20753d6c53"
		```

	* Terminal #2
		```
		2023-05-11 05:56:10.582  INFO tokio-runtime-worker substrate: 💤 Idle (0 peers), best: #0 (0x18c5…59af), finalized #0 (0x18c5…59af), ⬇ 0 ⬆ 0    
		2023-05-11 05:56:11.793  INFO tokio-runtime-worker jsonrpsee_ws_server::server: Accepting new connection 1/100
		2023-05-11 05:56:11.797 DEBUG tokio-runtime-worker sync: Propagating transactions    
		2023-05-11 05:56:13.276 DEBUG tokio-runtime-worker sync: Propagating transaction [0x27855e94096938dcb8a0b54aa6eb10f1c7fc4b6a4691cbe8022218600c8a5b30]    
		2023-05-11 05:56:13.320  INFO tokio-runtime-worker sc_basic_authorship::basic_authorship: 🙌 Starting consensus session on top of parent 0x18c5ce867c31a60bfdd97f1cc6656e684370cea155a1556c9340f18d47ac59af    
		2023-05-11 05:56:13.960  INFO tokio-runtime-worker sc_basic_authorship::basic_authorship: 🎁 Prepared block for proposing at 1 (431 ms) [hash: 0x9bc460edc15179afa81912948bbb0f537add47434c9dea3a08ef52154165be22; parent_hash: 0x18c5…59af; extrinsics (2): [0xa8a7…4455, 0x2785…5b30]]    
		2023-05-11 05:56:14.046 DEBUG tokio-runtime-worker sync: Reannouncing block 0x9bc460edc15179afa81912948bbb0f537add47434c9dea3a08ef52154165be22 is_best: true    
		2023-05-11 05:56:14.047 DEBUG tokio-runtime-worker sync: New best block imported 0x9bc460edc15179afa81912948bbb0f537add47434c9dea3a08ef52154165be22/#1    
		2023-05-11 05:56:14.053  INFO tokio-runtime-worker sc_consensus_manual_seal::rpc: Instant Seal success: CreatedBlock { hash: 0x9bc460edc15179afa81912948bbb0f537add47434c9dea3a08ef52154165be22, aux: ImportedAux { header_only: false, clear_justification_requests: false, needs_justification: false, bad_justification: false, is_new_best: true } }    
		2023-05-11 05:56:14.102  INFO tokio-runtime-worker substrate: ✨ Imported #1 (0x9bc4…be22)    
		2023-05-11 05:56:14.699 DEBUG tokio-runtime-worker sync: Propagating transactions    
		2023-05-11 05:56:15.591  INFO tokio-runtime-worker substrate: 💤 Idle (0 peers), best: #1 (0x9bc4…be22), finalized #0 (0x18c5…59af), ⬇ 0 ⬆ 0
		```

##### Build & Upload Smart Contract to Local Testnet (using Swanky CLI)

Install Swanky CLI https://github.com/AstarNetwork/swanky-cli
```bash
cd dapps/ink-rust/wasm-flipper
nvm use
yarn global add @astar-network/swanky-cli@2.1.2
```

0. Init

```bash
cd contract
swanky init flipper
```
Note: Choose `ink` as a contract language and `flipper` as template and a chosen contract name.
Optionally choose from `Y/N` when asking to download the Swanky Node (NOT required if already using Substrate Contracts Node).

1. Start the local node

If you chose to download the Swanky Node then run it in your local environment:
```bash
cd flipper
swanky node start
```

2. Build the contract

Build the contract in a new tab
```bash
swanky contract compile flipper
```
Note: Try `rustup update` if you face error

3. Deploy the contract

Local Testnet
```bash
swanky contract deploy flipper --account alice -g 100000000000 -a true
```

Shibuya Testnet
```bash
swanky contract deploy flipper --account alice --gas 100000000000 --args true --network shibuya
```
Copy paste the contract address.

4. Update `WS_PROVIDER` to check if it connects to Shibuya or localhost in ./dapps/ink-rust/wasm-flipper/ui/components/app.tsx

5. View in block explorer if deploy on Astar https://astar.subscan.io/wasm_contract_dashboard?tab=contract

##### Interact with Contract using ink! Python

* Note: This assumes

```bash
cd dapps/ink-python/example
pip3 install --no-cache-dir -r requirements.txt
python3 ./src/app.py
```

##### Interact with Contract using Flipper and Polkadot.js API

```bash
cd dapps/ink-rust/wasm-flipper
yarn
yarn dev
```

* Go to http://localhost:3000

* FIXME: Currently getting error:
```
API-WS: disconnected from ws://127.0.0.1:9944: 1006:: Abnormal Closure
```
* Tried these solutions unsuccessfully:
	* https://substrate.stackexchange.com/questions/5966/connecting-to-a-chain-on-ws-localhost-with-polkadot-js-app-fails-when-running

* Reference https://polkadot.js.org/docs/api-contract/start/basics

##### Interact with ink! Contracts using Contracts Node

###### Cargo Contracts

* Instantiate Contract
```
cargo contract instantiate \
	--suri //Bob \
	--constructor new \
	--args true \
	--execute \
	--skip-confirm
```

* Wait for response

```
...
Event System => NewAccount
	account: 5G...
...
Event Contracts + Instantiated
	deployer: 5F...
	contract: 5G.... (new contract account address to interact with contract)
...
```

* Check value was assigned correctly
* If use `--skip-dry-run` and execute as a transaction then we won't see the return value
```
cargo contract call \
	--suri //Charlie \
	--contract 5G... \
	--message get \
	--execute \
	--skip-confirm
```

* Interact to flip the boolean value, not a dry run so no response but we get a gas limit response

```
cargo contract call \
	--suri //Charlie \
	--contract 5G... \
	--message flip \
	--execute \
	--skip-confirm
```

* Check it flipped the boolean value

```
cargo contract call \
	--suri //Charlie \
	--contract 5G... \
	--message get \
	--execute \
	--skip-confirm
```

* Note: only works in debug mode `cargo build` (not release)
```
ink::env::debug_println("inc by {}, new value {}", by, self.value);
```

* Note: it should output on substrate-contracts-node too as `tokio-runtime-worker runtime::contracts Execution finished with debug buffer...`
* Note: it should show in contracts-ui website too
* Note: events are not emitted in a dry-run (why wouldn't we want this in debugging mode?)

### Useful Docker Commands

* List Docker containers
```bash
docker ps -a
```

* List Docker images
```bash
docker images -a
docker buildx ls
```

* Enter Docker container shell
```bash
docker exec -it $CONTAINER_ID /bin/sh
```

* View Docker container logs
```bash
docker logs -f $CONTAINER_ID
```

* Remove Docker container
```bash
docker stop $CONTAINER_ID; docker rm $CONTAINER_ID;
```

* Remove Docker image
```bash
docker rmi $IMAGE_ID
docker buildx rm --all-inactive
```

* Reduce space used by Docker Desktop
	* Docker Preferences -> Resources -> Advanced -> Virtual Disk Limit
		* e.g. 64Gb reduce to 32Gb
		* Note: This deletes all Docker images similar to `docker system prune -a --volumes`

### Notes

* Strategy:
	* Why use smart contract instead of blockchain?
		* Faster iterations of design, development, testing, and release of applications to market
		* Provide core functionality for the base layer of a general purpose blockchain that is being built
		* Allow smart contracts to interact with an application-specific blockchain pallet logic and use them to expose some logic to users since smart contracts treat all user input as untrusted and potentially adversarial
		* Example:
			* Building an application where most logic in Substrate pallets
				* Allow users to upload their own trading algorithms using smart contracts
				* Smart contracts require gas fees to execute so users would pay for the execution time of those trading algorithms
				* Expose relevant primitives similar to the [Chain extension primitive](https://ink.substrate.io/macros-attributes/chain-extension/) of the Contracts pallet 
	* What types of smart contracts may be deployed on Substrate runtime?
		* WebAssembly
		* EVM-compatible
	* What is a smart contract?
		* Instructions that are instantiated and executed on a host platform using a specific smart contract chain account address 
		* Instructions written in a language
	* What Substrate pallet is best to use when building a runtime to host smart contracts that are being built?
		* Contracts pallet allows deployment and execution of WebAssembly-based smart contracts
	* What trait do smart contract accounts used in the Contracts pallet of Substrate extend?
		* `Currency` trait


* Link
	* https://docs.substrate.io/build/smart-contracts-strategy/

TODO - continue summarising from "Smart contract accounts" section

### Links

#### Ink

* https://use.ink/
* https://github.com/paritytech/awesome-ink
* https://github.com/paritytech/ink-examples
* https://substrate.stackexchange.com/questions/tagged/ink?tab=Votes
* https://www.youtube.com/@ink-lang
* https://github.com/paritytech/cargo-contract

#### Docker

* https://docs.docker.com/engine/reference/builder/#here-documents