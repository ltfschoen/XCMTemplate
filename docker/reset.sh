#!/bin/bash

# restart the substrate-contracts-node and remove the chain database

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
