#!/bin/bash

trap "echo; exit" INT
trap "echo; exit" HUP

# if they call this script from project root or from within docker/ folder then
# in both cases the PARENT_DIR will refer to the project root where the .env file is.
PARENT_DIR=$( echo $(dirname "$(dirname "$(realpath "${BASH_SOURCE[0]}")")") )

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
