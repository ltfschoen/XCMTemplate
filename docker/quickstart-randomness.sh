#!/bin/bash

trap "echo; exit" INT
trap "echo; exit" HUP

# if they call this script from project root or from within docker/ folder then
# in both cases the PARENT_DIR will refer to the project root where the .env file is.
PARENT_DIR=$( echo $(dirname "$(dirname "$(realpath "${BASH_SOURCE[0]}")")") )

echo "Compiling contracts..."
cd $PARENT_DIR/dapps/evm2/flipper
PROJECT_ROOT=$PARENT_DIR/dapps/evm2/flipper
truffle compile --compile-all

echo "Migrating contracts..."
truffle migrate --reset --compile-all --network moonbase
