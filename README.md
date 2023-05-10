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

* Run from a Docker container and follow the terminal log instructions.
```bash
cp .env.example .env
./docker.sh
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
	--tmp \
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
cargo contract new mydapp
mkdir -p dapps/mydapp && mv mydapp dapps/mydapp
cd dapps/mydapp
```
* Optionally build with VSCode by adding the project `"dapps/mydapp"` to the list of members in the Cargo.toml file in the project root, and running "Terminal > Run Task > Build Contract" to build all the DApps using the configuration in ./.vscode/launch.json
* Generate .contract, .wasm, and metadata.json code. Note: Use `--release` to deploy
```
cargo contract build
```
* Upload Contract (note: prefer to use contracts-ui to avoid exposing private key)
```
cargo contract upload --suri //Alice
```

* Actual output:
	* Terminal #1:
```
2023-05-10 18:28:43.609  INFO tokio-runtime-worker substrate: 💤 Idle (0 peers), best: #0 (0xac6a…5c4f), finalized #0 (0xac6a…5c4f), ⬇ 0 ⬆ 0    
2023-05-10 18:28:44.188 DEBUG tokio-runtime-worker sync: Propagating transactions    
2023-05-10 18:28:47.088 DEBUG tokio-runtime-worker sync: Propagating transactions    
2023-05-10 18:28:47.088  INFO tokio-runtime-worker jsonrpsee_server::server: Accepting new connection 1/100
2023-05-10 18:28:47.967  INFO tokio-runtime-worker jsonrpsee_server::server: Accepting new connection 2/100
2023-05-10 18:28:48.086 ERROR tokio-runtime-worker jsonrpsee_server::transport::ws: WS transport error: i/o error: Transport endpoint is not connected (os error 107); terminate connection: 0
```

	* Terminal #2
```
# cargo contract upload --suri //Alice
      Result Success!
   Code hash "0xa0c52cd7843da761236263923f454d5860db1794855de29396cc9908c58bd4b9"
     Deposit 434790000000
Your upload call has not been executed.
To submit the transaction and execute the call on chain, add -x/--execute flag to the command.
```

* It should output:
```
CodeStored event
code_hash: 0x......
```
  * Note: only one copy of the code is stored, but there can be many instance of one code blob, differs from other EVM chains where each node has a copy


##### Interact with ink! Contracts using Contracts Node

###### Cargo Contracts

* Instantiate Contract
```
cargo contract instantiate \
	--suri //Bob \
	--constructor new \
	--args 10
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
* Use `dry-run` because if execute as a transaction then we won't see the return value
```
cargo contract call \
	--suri //Charlie \
	--contract 5G... \
	--message get \
	--dry-run
```

* Interact to increment by 5, not a dry run so no response but we get a gas limit response

```
cargo contract call \
	--suri //Charlie \
	--contract 5G... \
	--message inc \
	--args 5
```

* Check it incremented

```
cargo contract call \
	--suri //Charlie \
	--contract 5G... \
	--message get \
	--dry-run
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
