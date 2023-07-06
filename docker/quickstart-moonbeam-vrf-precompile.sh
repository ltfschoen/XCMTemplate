#!/bin/bash

trap "echo; exit" INT
trap "echo; exit" HUP

# if they call this script from project root or from within docker/ folder then
# in both cases the PARENT_DIR will refer to the project root where the .env file is.
PARENT_DIR=$( echo $(dirname "$(dirname "$(realpath "${BASH_SOURCE[0]}")")") )

# TODO - install all necessary dependencies as mentioned in the dapps/evm2/randomness/README.md

# echo "Compiling contracts..."
# cd $PARENT_DIR/dapps/evm2/randomness
# PROJECT_ROOT=$PARENT_DIR/dapps/evm2/randomness
# truffle compile --compile-all

# echo "Migrating contracts..."
# truffle migrate --reset --compile-all --network sepolia

# TODO - get the deployed contract address from the output
#        and pass that as a variable to the demo-chainlink-vrf-on-ethereum-sepolia.js script
#        (which gets the contract at that address and then call `rollDice`
#        and waits some blocks before getting the random number)

# echo "Request randomness then obtain a randomness status then a random number"
# cd dapps/evm2/randomness
# node ./scripts/demo-moonbeam-vrf-on-moonbase-alpha.js
