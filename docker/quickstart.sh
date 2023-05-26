#!/bin/bash

trap "echo; exit" INT
trap "echo; exit" HUP

# if they call this script from project root or from within docker/ folder then
# in both cases the PARENT_DIR will refer to the project root where the .env file is.
PARENT_DIR=$( echo $(dirname "$(dirname "$(realpath "${BASH_SOURCE[0]}")")") )

# kill the existing substrate-contracts-node that is running on port 30333
kill -9 $(lsof -ti:30333)

# delete the blockchain database so we don't get this error when we redeploy the
# smart contract `ERROR: This contract has already been uploaded with code hash:`
rm -rf /tmp/ink

cd ${PARENT_DIR}/docker/
# https://www.maketecheasier.com/run-bash-commands-background-linux/
nohup ./run-scn.sh &>/dev/null &
# wait for the blockchain node to start before we interact with it
# note: if you try to interact with it before it is ready then you will get error
# `ERROR: Rpc error: RPC error: Networking or low-level protocol error:
# Error when opening the TCP socket: Cannot assign requested address (os error 99)`
sleep 15

cd $PARENT_DIR/dapps/ink-rust/wasm-flipper/contract
cargo contract new flipper
cd flipper
PROJECT_ROOT=$PARENT_DIR/dapps/ink-rust/wasm-flipper/contract/flipper
cargo contract build \
    --manifest-path $PARENT_DIR/dapps/ink-rust/wasm-flipper/contract/flipper/Cargo.toml

# replace value of `abi` variable in ${PARENT_DIR}/dapps/ink-rust/wasm-flipper/ui/components/abi.ts
# with the latest built contents of ${PARENT_DIR}/target/ink/flipper/flipper.json file
cd ${PARENT_DIR}/docker/
ABI_FILE_PATH=${PARENT_DIR}/dapps/ink-rust/wasm-flipper/ui/components/abi.ts
JSON_FILE_PATH=${PARENT_DIR}/target/ink/flipper/flipper.json
node replaceAbi.js $ABI_FILE_PATH $JSON_FILE_PATH
cd $PROJECT_ROOT
OUTPUT_CODE_HASH=$(
    cargo contract upload --suri //Alice \
        --execute \
        --skip-confirm | tail -1
)
# example: '  Code hash "0x..."'
echo $OUTPUT_CODE_HASH
# remove text 'Code hash' and the outer double quotes of the code hash
OUTPUT_CODE_HASH_REMOVED_LABEL=$(echo "$OUTPUT_CODE_HASH" | sed 's/Code hash//;s/$//' | tr -d '"')
# trim whitespace
CODE_HASH=$(echo $OUTPUT_CODE_HASH_REMOVED_LABEL)
echo $CODE_HASH

# instantiate
OUTPUT_CONTRACT_ADDR=$(
    cargo contract instantiate \
        --suri //Bob \
        --constructor new \
        --args true \
        --execute \
        --skip-confirm \
        | tail -1
)
# example: '  Contract 5...'
echo $OUTPUT_CONTRACT_ADDR
# remove text 'Contract'
OUTPUT_CONTRACT_ADDR_REMOVED_LABEL=$(echo "$OUTPUT_CONTRACT_ADDR" | sed 's/Contract//;s/$//')
# trim whitespace
CONTRACT_ADDR=$(echo $OUTPUT_CONTRACT_ADDR_REMOVED_LABEL)
echo $CONTRACT_ADDR

cargo contract call \
	--suri //Charlie \
	--contract $CONTRACT_ADDR \
	--message flip \
	--execute \
	--skip-confirm

# highlight the `data` line in output containing the value of the emitted `Retrieve` event
cargo contract call \
	--suri //Charlie \
	--contract $CONTRACT_ADDR \
	--message get \
	--execute \
	--skip-confirm \
    | grep --color=always -z 'data'
