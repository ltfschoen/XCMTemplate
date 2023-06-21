#!/bin/bash

trap "echo; exit" INT
trap "echo; exit" HUP

# if they call this script from project root or from within docker/ folder then
# in both cases the PARENT_DIR will refer to the project root where the .env file is.
PARENT_DIR=$( echo $(dirname "$(dirname "$(realpath "${BASH_SOURCE[0]}")")") )

echo "Building contracts..."
cd $PARENT_DIR/dapps/basic_contract_caller
PROJECT_ROOT=$PARENT_DIR/dapps/ink-rust/IPSP22/contract/IPSP22
cargo +nightly-2023-03-21-x86_64-unknown-linux-gnu contract build \
    --manifest-path $PARENT_DIR/dapps/ink-rust/IPSP22/contract/IPSP22/Cargo.toml

cd $PROJECT_ROOT

echo "Uploading contract..."
args=(
    --suri //Alice
	--execute
    # --skip-dry-run
	--skip-confirm
    $PARENT_DIR/target/ink/my_token/my_token.wasm
)
OUTPUT_CODE_HASH=$(
    cargo contract upload "${args[@]}" | tail -1
)
echo "Finished uploading contract..."
# example: '  Code hash "0x..."'
echo $OUTPUT_CODE_HASH
# remove text 'Code hash' and the outer double quotes of the code hash
OUTPUT_CODE_HASH_REMOVED_LABEL=$(echo "$OUTPUT_CODE_HASH" | sed 's/Code hash//;s/$//' | tr -d '"')
# trim whitespace
CODE_HASH_MAIN=$(echo $OUTPUT_CODE_HASH_REMOVED_LABEL)
echo $CODE_HASH_MAIN

cd $PROJECT_ROOT

echo "Instantiating contract..."

INITIAL_SUPPLY="100000000"
args=(
    --manifest-path $PARENT_DIR/dapps/ink-rust/IPSP22/contract/IPSP22/Cargo.toml
    --suri //Alice
    --constructor new
    --args $INITIAL_SUPPLY
    --execute
    --gas 1000000000000
    --proof-size 1000000000000
    # --skip-dry-run
    --skip-confirm
)
OUTPUT_CONTRACT_ADDR=$(
    cargo contract instantiate "${args[@]}" | tail -1
)

# example: '  Contract 5...'
echo $OUTPUT_CONTRACT_ADDR
# remove text 'Contract'
OUTPUT_CONTRACT_ADDR_REMOVED_LABEL=$(echo "$OUTPUT_CONTRACT_ADDR" | sed 's/Contract//;s/$//')
# trim whitespace using `echo ...`
CONTRACT_ADDR_MAIN=$(echo $OUTPUT_CONTRACT_ADDR_REMOVED_LABEL)
echo $CONTRACT_ADDR_MAIN
