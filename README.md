### Smart Contract in ink!

#### Docker

##### Setup Container

* Note:
	* The docker/Dockerfile and its dependencies in docker/utility are modified copies of files in https://github.com/paritytech/scripts/blob/master/dockerfiles to have the flexibility to develop locally using ink! in a Docker container that uses Linux without any dependency issues, and where changes replicated on the host machine.
	* This is useful if you're on an old macOS Catalina and Apple won't allow you to do further software updates, so you cannot install `brew install protobuf` to install necessary dependencies.

* Install and run [Docker](https://www.docker.com/)
* Generate .env file from sample file
* Check versions used in Dockerfile:
	* Rust version
	* Cargo Contract
	* Substrate Contracts Node

* Run Docker container and follow the terminal log instructions.
```bash
touch .env && cp .env.example .env
./docker.sh
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

* Check versions
```
rustup toolchain list
rustup update
rustup show
cargo-contract --version
substrate-contracts-node --version
```
* Run Cargo Contract Node
```
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
	* Note: Use either `--tmp` or `--base-path "/tmp/ink"`

* Leave that terminal tab running the node. Enter the terminal again in a new tab with `docker exec -it ink /bin/bash` and run the following:
* Attach to the running terminal with VSCode if necessary. See [here](https://code.visualstudio.com/docs/devcontainers/attach-container)

* Create Rust project with template
```
cargo contract new flipper
mkdir -p contract/ && mv flipper contract/ && cd contract/flipper
```
* Optionally build with VSCode by adding the project `"contract/flipper"` to the list of members in the Cargo.toml file in the project root, and running "Terminal > Run Task > Build Contract" to build all the contract using the configuration in ./.vscode/launch.json
* Generate .contract, .wasm, and metadata.json code. Note: Use `--release` to deploy
```
cargo contract build
```
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

##### Interact with Contract using Polkadot.js API

* Reference https://polkadot.js.org/docs/api-contract/start/basics


### Useful Docker Commands

* List Docker containers
```bash
docker ps -a
```

* List Docker images
```bash
docker images -a
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
```
