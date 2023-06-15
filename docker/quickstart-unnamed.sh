#!/bin/bash

# compatibility cargo-contract v3.0.1
#
# start a fresh substrate-contracts-node and upload the "unnamed" ink!
# smart contracts to it and then instantiate and call a method
#
# if you want to sky dry run then find/replace `--skip-dry-run` with `#--skip-dry-run`
# to comment those lines out

trap "echo; exit" INT
trap "echo; exit" HUP

# if they call this script from project root or from within docker/ folder then
# in both cases the PARENT_DIR will refer to the project root where the .env file is.
PARENT_DIR=$( echo $(dirname "$(dirname "$(realpath "${BASH_SOURCE[0]}")")") )

echo "Killing previous Substrate contracts node..."
# kill the existing substrate-contracts-node that is running on port 30333
kill -9 $(lsof -ti:30333) &>/dev/null &

echo "Removing previous database..."
# delete the blockchain database so we don't get this error when we redeploy the
# smart contract `ERROR: This contract has already been uploaded with code hash:`
rm -rf /tmp/ink

echo "Running new Substrate contracts node..."
cd ${PARENT_DIR}/docker/
# https://www.maketecheasier.com/run-bash-commands-background-linux/
nohup ./run-scn.sh &>/dev/null &
# wait for the blockchain node to start before we interact with it
# note: if you try to interact with it before it is ready then you will get error
# `ERROR: Rpc error: RPC error: Networking or low-level protocol error:
# Error when opening the TCP socket: Cannot assign requested address (os error 99)`
sleep 25
echo "Building contracts..."
cd $PARENT_DIR/dapps/xcm/unnamed
PROJECT_ROOT=$PARENT_DIR/dapps/xcm/unnamed
cargo contract build \
    --manifest-path $PARENT_DIR/dapps/xcm/unnamed/Cargo.toml
cargo contract build \
    --manifest-path $PARENT_DIR/dapps/xcm/unnamed/oracle_contract/Cargo.toml

cd $PROJECT_ROOT
echo "Uploading sub-contract..."
# upload sub-contract
#
# note: provide args variable so able to comment out all `--skip-dry-run` options in bulk
# since it breaks command if comment out a multiline command option `#--skip-dry-run \`
# see https://stackoverflow.com/a/9522766/3208553
args=(
    --suri //Alice
	--execute
    --skip-dry-run
	--skip-confirm
    $PARENT_DIR/target/ink/oracle_contract/oracle_contract.wasm
)
OUTPUT_CODE_HASH_SUB=$(
    cargo contract upload "${args[@]}" | tail -1
)
echo "Finished uploading contract..."
# example: '  Code hash "0x..."'
echo $OUTPUT_CODE_HASH_SUB
# remove text 'Code hash' and the outer double quotes of the code hash
OUTPUT_CODE_HASH_SUB_REMOVED_LABEL=$(echo "$OUTPUT_CODE_HASH_SUB" | sed 's/Code hash//;s/$//' | tr -d '"')
# trim whitespace
CODE_HASH_SUB=$(echo $OUTPUT_CODE_HASH_SUB_REMOVED_LABEL)
echo $CODE_HASH_SUB

echo "Uploading main-contract..."
# upload main-contract
#
args=(
    --suri //Alice
	--execute
    # --skip-dry-run
	--skip-confirm
    $PARENT_DIR/target/ink/unnamed/unnamed.wasm
)
OUTPUT_CODE_HASH_MAIN=$(
    cargo contract upload "${args[@]}" | tail -1
)
echo "Finished uploading contract..."
# example: '  Code hash "0x..."'
echo $OUTPUT_CODE_HASH_MAIN
# remove text 'Code hash' and the outer double quotes of the code hash
OUTPUT_CODE_HASH_MAIN_REMOVED_LABEL=$(echo "$OUTPUT_CODE_HASH_MAIN" | sed 's/Code hash//;s/$//' | tr -d '"')
# trim whitespace
CODE_HASH_MAIN=$(echo $OUTPUT_CODE_HASH_MAIN_REMOVED_LABEL)
echo $CODE_HASH_MAIN

ARG_ID_MARKET="\"my_id\""

# Note: The id_market is stored as Vec<u8> format instead of String.
# Paste the following at https://play.rust-lang.org/?version=stable&mode=debug&edition=2021
# then press the "Run" button and it will output `[109, 121, 95, 105, 100]`
# ```rust
# fn main() -> Result<(), std::io::Error> {
#     let s = "my_id".to_string();
#     let b = s.as_bytes();
#     println!("{:?}", b);
#     let _y = String::from_utf8(b.to_vec());
#     Ok(())
# }
# ```

cd $PROJECT_ROOT

# instantiate "main" contract, providing the code hash generated from uploading the "sub" contract
echo "Instantiating main-contract..."

args=(
    --manifest-path $PARENT_DIR/dapps/xcm/unnamed/Cargo.toml
    --suri //Alice
    --constructor new
    --args $CODE_HASH_SUB $ARG_ID_MARKET "100" "228" "500"
    --execute
    --gas 100000000000
    --proof-size 100000000000
    # --skip-dry-run
    --skip-confirm
)
OUTPUT_CONTRACT_ADDR_MAIN=$(
    cargo contract instantiate "${args[@]}" | tail -1
)

# example: '  Contract 5...'
echo $OUTPUT_CONTRACT_ADDR_MAIN
# remove text 'Contract'
OUTPUT_CONTRACT_ADDR_MAIN_REMOVED_LABEL=$(echo "$OUTPUT_CONTRACT_ADDR_MAIN" | sed 's/Contract//;s/$//')
# trim whitespace using `echo ...`
CONTRACT_ADDR_MAIN=$(echo $OUTPUT_CONTRACT_ADDR_MAIN_REMOVED_LABEL)
echo $CONTRACT_ADDR_MAIN

echo "Instantiating sub-contract..."
args=(
    --manifest-path $PARENT_DIR/dapps/xcm/unnamed/oracle_contract/Cargo.toml
    --suri //Alice
    --constructor new
    --args $ARG_ID_MARKET "100" "228" "500"
    --execute
    --gas 100000000000
    --proof-size 100000000000
    # --skip-dry-run
    --skip-confirm
)
OUTPUT_CONTRACT_ADDR_SUB=$(
    cargo contract instantiate "${args[@]}" | tail -1
)

# example: '  Contract 5...'
echo $OUTPUT_CONTRACT_ADDR_SUB
# remove text 'Contract'
OUTPUT_CONTRACT_ADDR_SUB_REMOVED_LABEL=$(echo "$OUTPUT_CONTRACT_ADDR_SUB" | sed 's/Contract//;s/$//')
# trim whitespace using `echo ...`
CONTRACT_ADDR_SUB=$(echo $OUTPUT_CONTRACT_ADDR_SUB_REMOVED_LABEL)
echo $CONTRACT_ADDR_SUB

ARG_BLOCK_HASH_ENTROPY="\"aef6eca62ae61934a7ab5ad3814f6e319abd3e4e4aa1a3386466ad197d1c4dea\""

echo "Calling contract method set_block_for_entropy_for_market_id..."
args=(
	--suri //Alice
	--contract $CONTRACT_ADDR_MAIN
	--message set_block_for_entropy_for_market_id
    --args $ARG_ID_MARKET "228" $ARG_BLOCK_HASH_ENTROPY
	--execute
    --gas 100000000000
    --proof-size 100000000000
    # --skip-dry-run
	--skip-confirm
)
cargo contract call "${args[@]}" | grep --color=always -z 'data'

echo "Calling contract method get_entropy_for_market_id ..."
args=(
	--suri //Alice
	--contract $CONTRACT_ADDR_MAIN
	--message get_entropy_for_market_id
    --args $ARG_ID_MARKET
	--execute
    # --skip-dry-run
	--skip-confirm
)
cargo contract call "${args[@]}" | grep --color=always -z 'data'

# TODO - consider removing. maybe use to override results incase of missed blocks?
#
# args=(
# 	--suri //Alice
# 	--contract $CONTRACT_ADDR_MAIN
# 	--message set_entropy_for_market_id
#     --args $ARG_ID_MARKET "228" $ARG_BLOCK_HASH_ENTROPY "0" "0"
# 	--execute
#     # --skip-dry-run
# 	--skip-confirm
# )
# cargo contract call "${args[@]}" | grep --color=always -z 'data'

# highlight the `data` line in output containing the value of the emitted `Retrieve` event
echo "Calling contract method get_oracle_contract_address ..."
args=(
	--suri //Alice
	--contract $CONTRACT_ADDR_MAIN
	--message get_oracle_contract_address
	--execute
    # --skip-dry-run
	--skip-confirm
)
# FIXME - why doesn't this return anything?
cargo contract call "${args[@]}" | grep --color=always -z 'data'
