#!/bin/bash

trap "echo; exit" INT
trap "echo; exit" HUP

WITHOUT_NODE=$1

# if they call this script from project root or from within docker/ folder then
# in both cases the PARENT_DIR will refer to the project root where the .env file is.
PARENT_DIR=$( echo $(dirname "$(dirname "$(realpath "${BASH_SOURCE[0]}")")") )

# if calling from within docker/ then change to parent directory to run it.
if [ -e .env.example ]
then
    echo ".env.example detected. assuming already in project root"
else
	echo "no .env.example detected. assume in docker/ folder. switching to project root";
    cd $PARENT_DIR
fi

# generate .env file from .env.example if it does not exist
# https://stackoverflow.com/a/47677632/3208553
if [ -e .env ]
then
    echo ".env file exists"
else
	echo "generating .env file from .env.example since it does not exist";
	touch $PARENT_DIR/.env && cp $PARENT_DIR/.env.example $PARENT_DIR/.env;
fi

# assign fallback values for environment variables from .env.example incase
# not declared in .env file. alternative approach is `echo ${X:=$X_FALLBACK}`
source $PARENT_DIR/.env.example
source $PARENT_DIR/.env

# https://stackoverflow.com/a/25554904/3208553
set +e
bash -e <<TRY
  DOCKER_BUILDKIT=0 docker build \
    -f ${PARENT_DIR}/docker/Dockerfile \
    --build-arg WITHOUT_NODE=${WITHOUT_NODE} \
    --no-cache \
    --tag ink:latest ./
TRY
if [ $? -ne 0 ]; then
    printf "\n*** Detected error running 'docker build'. Trying 'docker buildx' instead...\n"
    DOCKER_BUILDKIT=0 docker buildx build \
        -f {PARENT_DIR}/docker/Dockerfile \
        --build-arg WITHOUT_NODE=${WITHOUT_NODE} \
        --no-cache \
        --tag ink:latest ./
fi

docker images
docker buildx ls
docker ps -a

# memory measured in bytes
# restart alternative "no"
docker run -it -d \
    --env-file "${PARENT_DIR}/.env" \
    --hostname ink \
    --name ink \
    --restart "on-failure" \
    --memory 750M \
    --memory-reservation 125M \
    --memory-swap 15G \
    --cpus 1 \
    --publish 0.0.0.0:8080:8080 \
    --publish 0.0.0.0:9933:9933 \
    --publish 0.0.0.0:9944:9944 \
    --publish 0.0.0.0:9615:9615 \
    --publish 0.0.0.0:30333:30333 \
    --publish 0.0.0.0:3000:3000 \
    --publish 0.0.0.0:443:443 \
    --publish 0.0.0.0:80:80 \
    --volume ${PARENT_DIR}:/app:rw \
    ink:latest
if [ $? -ne 0 ]; then
    kill "$PPID"; exit 1;
fi
CONTAINER_ID=$(docker ps -n=1 -q)
printf "\n*** Finished building Docker container ${CONTAINER_ID}.\n\n"
