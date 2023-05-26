#!/bin/bash
#
# Authored by Luke Schoen

trap "echo; exit" INT
trap "echo; exit" HUP

# if they call this script from project root or from within docker/ folder then
# in both cases the PARENT_DIR will refer to the project root
PARENT_DIR=$( echo $(dirname "$(dirname "$(realpath "${BASH_SOURCE[0]}")")") )

cd $PARENT_DIR

SCN_PATH="/usr/local/cargo/bin/substrate-contracts-node"
DB_STORAGE="/tmp/ink"
NAME="ink-template"
TCP_PORT=30333
RPC_PORT=9933
WS_PORT=9944
TELEMETRY_URL="wss://telemetry.polkadot.io/submit/ 0"
DETECT_LOG="tokio-runtime-worker runtime::contracts"

# run the node and highlight output that contains debug logs from smart contract
CMD="$SCN_PATH \
	--dev \
	--alice \
	--name $NAME \
	--base-path $DB_STORAGE \
	--force-authoring \
	--port $TCP_PORT \
	--rpc-port $RPC_PORT \
	--ws-port $WS_PORT \
	--unsafe-ws-external \
	--rpc-methods Unsafe \
	--unsafe-rpc-external \
	--rpc-cors all \
	--prometheus-external \
	--telemetry-url $TELEMETRY_URL \
	-lsync=debug,runtime::contracts=debug \
    | grep --color=always -z DETECT_LOG
"
echo "-----------------------"
echo "Executing: $CMD"
echo "----------------------"

$CMD
