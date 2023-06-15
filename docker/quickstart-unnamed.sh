#!/bin/bash

# start a fresh substrate-contracts-node and upload the "unnamed" ink!
# smart contracts to it and then instantiate and call a method

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
sleep 5

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
OUTPUT_CODE_HASH=$(
    cargo contract upload --suri //Alice \
	--execute \
	--skip-confirm \
    $PARENT_DIR/target/ink/oracle_contract/oracle_contract.wasm | tail -1
)
echo "Uploading contract..."
# upload main-contract
OUTPUT_CODE_HASH_MAIN=$(
    cargo contract upload --suri //Alice \
        --execute \
        --skip-dry-run \
        --skip-confirm \
        $PARENT_DIR/target/ink/unnamed/unnamed.wasm | tail -1
)
echo "Finished uploading contracts..."
# example: '  Code hash "0x..."'
echo $OUTPUT_CODE_HASH
# remove text 'Code hash' and the outer double quotes of the code hash
OUTPUT_CODE_HASH_REMOVED_LABEL=$(echo "$OUTPUT_CODE_HASH" | sed 's/Code hash//;s/$//' | tr -d '"')
# trim whitespace
CODE_HASH=$(echo $OUTPUT_CODE_HASH_REMOVED_LABEL)
echo $CODE_HASH

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

# instantiate "main" contract, providing the code hash generated from uploading the "sub" contract

echo "Instantiating contract..."
OUTPUT_CONTRACT_ADDR=$(
    cargo contract instantiate \
        --manifest-path $PARENT_DIR/dapps/xcm/unnamed/Cargo.toml \
        --suri //Alice \
        --constructor new \
        --args "$CODE_HASH" "my_id" "100" "228" "500" \
        --execute \
        --skip-dry-run \
        --gas 100000000000 \
	    --proof-size 100000000000 \
        --skip-confirm \
        | tail -1
)

sleep 5

# example: '  Contract 5...'
echo $OUTPUT_CONTRACT_ADDR
# remove text 'Contract'
OUTPUT_CONTRACT_ADDR_REMOVED_LABEL=$(echo "$OUTPUT_CONTRACT_ADDR" | sed 's/Contract//;s/$//')
# trim whitespace
CONTRACT_ADDR=$(echo $OUTPUT_CONTRACT_ADDR_REMOVED_LABEL)
echo $CONTRACT_ADDR
# echo "Calling contract method..."
# note: an example hash value has been used
# TODO - should the hash be '0x' prefixed?
# cargo contract call \
# 	--suri //Alice \
# 	--contract $CONTRACT_ADDR \
# 	--message set_block_for_entropy_for_market_id \
#     --args "my_id" "228" "aef6eca62ae61934a7ab5ad3814f6e319abd3e4e4aa1a3386466ad197d1c4dea" \
# 	--execute \
#     --skip-dry-run \
#     --gas 100000000000 \
#     --proof-size 100000000000 \
# 	--skip-confirm \
#     | grep --color=always -z 'data'

# cargo contract call \
# 	--suri //Alice \
# 	--contract $CONTRACT_ADDR \
# 	--message get_entropy_for_market_id \
#     --args "my_id" \
# 	--execute \
# 	--skip-confirm \
#     | grep --color=always -z 'data'

# # TODO - why is this method even necessary in the code? was it to override results incase of missed blocks?
# #
# # cargo contract call \
# # 	--suri //Alice \
# # 	--contract $CONTRACT_ADDR \
# # 	--message set_entropy_for_market_id \
# #     --args "my_id" "228" "aef6eca62ae61934a7ab5ad3814f6e319abd3e4e4aa1a3386466ad197d1c4dea" "0" "0" \
# # 	--execute \
# # 	--skip-confirm \
# #     | grep --color=always -z 'data'

# # highlight the `data` line in output containing the value of the emitted `Retrieve` event
# cargo contract call \
# 	--suri //Alice \
# 	--contract $CONTRACT_ADDR \
# 	--message get_oracle_contract_address \
# 	--execute \
# 	--skip-confirm \
#     | grep --color=always -z 'data'
