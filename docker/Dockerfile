# Source: https://github.com/paritytech/scripts/blob/master/dockerfiles/base-ci-linux/Dockerfile
# Source: https://github.com/paritytech/scripts/blob/master/dockerfiles/contracts-ci-linux/Dockerfile
# Changes shown in Git commits

FROM docker.io/library/debian:bullseye-slim
ARG DEBIAN_CODENAME=bullseye
ARG RUST_NIGHTLY="2023-03-21"
ARG NODE_VERSION=v20.1.0
ARG NODE_VERSION_MAJOR=20
ARG CARGO_CONTRACT_VERSION=3.0.1
ARG SUBSTRATE_CONTRACTS_NODE_VERSION=v0.25.0

ARG WITHOUT_NODE=${WITHOUT_NODE}

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
        CXX=clang-14

# https://docs.docker.com/engine/reference/builder/#here-documents
# install tools and dependencies
RUN ./docker/run.sh ${WITHOUT_NODE}

# show backtraces
ENV	RUST_BACKTRACE=1

EXPOSE 8080 30333 9933 9944 9615 3000 443 80

CMD tail -f /dev/null

	# cargo clean up
	# removes compilation artifacts cargo install creates (>250M)
	# rm -rf "${CARGO_HOME}/registry" "${CARGO_HOME}/git" /root/.cache/sccache && \

	# apt clean up
	# apt-get autoremove -y && \
	# apt-get clean && \
	# rm -rf /var/lib/apt/lists/*

# set this at end of Dockerfile, with `ENV DEBIAN_FRONTEND noninteractive` at beginning
# so both interactive and non-interactive modes get set.
ARG DEBIAN_FRONTEND teletype
