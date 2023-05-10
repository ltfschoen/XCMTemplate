#!/bin/bash

trap "echo; exit" INT
trap "echo; exit" HUP

# assign fallback values for environment variables from .env.example incase
# not declared in .env file. alternative approach is `echo ${X:=$X_FALLBACK}`
source $(dirname "$0")/.env.example
source $(dirname "$0")/.env

printf "\n*** Started building Docker container."
printf "\n*** Please wait... \n***"
DOCKER_BUILDKIT=0 docker compose -f docker-compose.yml up --build -d
if [ $? -ne 0 ]; then
    kill "$PPID"; exit 1;
fi
printf "\n*** Finished building Docker container.\n"
