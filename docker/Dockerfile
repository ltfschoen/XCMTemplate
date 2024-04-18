# Source: https://github.com/paritytech/scripts/blob/master/dockerfiles/base-ci-linux/Dockerfile
# Source: https://github.com/paritytech/scripts/blob/master/dockerfiles/contracts-ci-linux/Dockerfile
# Changes shown in Git commits

FROM docker.io/library/debian:bullseye-slim
ARG DEBIAN_CODENAME=bullseye
ARG RUST_NIGHTLY="2024-02-17"
ARG NODE_VERSION=v20.12.0
ARG NODE_VERSION_MAJOR=20
ARG CARGO_CONTRACT_VERSION=4.1.0
ARG SUBSTRATE_CONTRACTS_NODE_VERSION=v0.40.0

# build args
ARG WITHOUT_CONTRACTS_NODE=${WITHOUT_CONTRACTS_NODE}
# https://stackoverflow.com/questions/71040681/qemu-x86-64-could-not-open-lib64-ld-linux-x86-64-so-2-no-such-file-or-direc
# https://stackoverflow.com/a/66900911/3208553
ARG DOCKER_DEFAULT_PLATFORM=${DOCKER_DEFAULT_PLATFORM}

WORKDIR /app

COPY ./docker/run.sh /app/docker/run.sh

ENV SHELL /bin/bash
# use ARG so not only set during build time
ARG DEBIAN_FRONTEND=noninteractive
ARG DEBCONF_NOWARNINGS="yes"

# config for wasm32-unknown-unknown & clang
COPY docker/utility/base-ci-linux-config /root/.cargo/config
COPY docker/utility/debian-llvm-clang.key /etc/apt/trusted.gpg.d/debian-llvm-clang.gpg

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    CC=clang-14 \
    CXX=clang-14 \
    LS_CONTRACT=${LS_CONTRACT} \
    DOCKER_DEFAULT_PLATFORM=${DOCKER_DEFAULT_PLATFORM}
    # \ 
    # DEBIAN_CODENAME=${DEBIAN_CODENAME} \
    # WITHOUT_CONTRACTS_NODE=${WITHOUT_CONTRACTS_NODE}

# https://docs.docker.com/engine/reference/builder/#here-documents
# install tools and dependencies
RUN ./docker/run.sh ${DEBIAN_CODENAME} ${HOME} ${RUST_NIGHTLY} ${CARGO_CONTRACT_VERSION} ${SUBSTRATE_CONTRACTS_NODE_VERSION} ${NODE_VERSION_MAJOR} ${WITHOUT_CONTRACTS_NODE}
# show backtraces
ENV	RUST_BACKTRACE=1

EXPOSE 8080 30333 9933 9944 9615 3000 443 80

CMD tail -f /dev/null

# set this at end of Dockerfile, with `ENV DEBIAN_FRONTEND noninteractive` at beginning
# so both interactive and non-interactive modes get set.
ARG DEBIAN_FRONTEND teletype
