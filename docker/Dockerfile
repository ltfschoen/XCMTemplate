# Source: https://github.com/paritytech/scripts/blob/master/dockerfiles/base-ci-linux/Dockerfile
# Source: https://github.com/paritytech/scripts/blob/master/dockerfiles/contracts-ci-linux/Dockerfile
# Changes shown in Git commits

FROM docker.io/library/debian:bullseye-slim
ARG DEBIAN_CODENAME=bullseye
ARG RUST_NIGHTLY="2023-03-21"

WORKDIR /app

ENV SHELL /bin/bash
ENV DEBIAN_FRONTEND=noninteractive

# config for wasm32-unknown-unknown & clang
COPY docker/utility/base-ci-linux-config /root/.cargo/config
COPY docker/utility/debian-llvm-clang.key /etc/apt/trusted.gpg.d/debian-llvm-clang.gpg

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
        CC=clang-14 \
        CXX=clang-14

# install tools and dependencies
RUN set -eux; \
    apt-get -y update; \
    apt-get install -y --no-install-recommends \
        libssl-dev make cmake graphviz \
        git pkg-config curl time rhash ca-certificates jq \
        python3 python3-pip lsof ruby ruby-bundler git-restore-mtime xz-utils zstd unzip gnupg protobuf-compiler && \
    apt-get install -y vim wget net-tools && \
# add clang 14 repo
    echo "deb http://apt.llvm.org/${DEBIAN_CODENAME}/ llvm-toolchain-${DEBIAN_CODENAME}-14 main" >> /etc/apt/sources.list.d/llvm-toochain-${DEBIAN_CODENAME}-14.list; \
    echo "deb-src http://apt.llvm.org/${DEBIAN_CODENAME}/ llvm-toolchain-${DEBIAN_CODENAME}-14 main" >> /etc/apt/sources.list.d/llvm-toochain-${DEBIAN_CODENAME}-14.list; \
    apt-get -y update; \
    apt-get install -y --no-install-recommends \
        clang-14 lldb-14 lld-14 libclang-14-dev && \
# add non-root user
    # groupadd -g 1000 nonroot && \
    # useradd -u 1000 -g 1000 -s /bin/bash -m nonroot && \
# install specific minio client version (2023-04-06)
    curl -L "https://dl.min.io/client/mc/release/linux-amd64/archive/mc.RELEASE.2023-04-06T16-51-10Z" -o /usr/local/bin/mc && \
    chmod 755 /usr/local/bin/mc && \
# set a link to clang
    update-alternatives --install /usr/bin/cc cc /usr/bin/clang-14 100; \
# set a link to ldd
    update-alternatives --install /usr/bin/ld ld /usr/bin/ld.lld-14 100; \
# install rustup, use minimum components
    curl -L "https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init" \
        -o rustup-init; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --profile minimal --default-toolchain stable; \
    # rm rustup-init; \
    # chown -R root:nonroot ${RUSTUP_HOME} ${CARGO_HOME}; \
    # chmod -R g+w ${RUSTUP_HOME} ${CARGO_HOME}; \
    pip install yq; \
# install sccache
    cargo install sccache; \
# versions
    rustup show; \
    cargo --version; \
# cargo clean up
# removes compilation artifacts cargo install creates (>250M)
    # rm -rf "${CARGO_HOME}/registry" "${CARGO_HOME}/git" /root/.cache/sccache; \
# apt clean up
    # apt-get autoremove -y; \
    # apt-get clean; \
    # rm -rf /var/lib/apt/lists/*
	apt-get install -y --no-install-recommends zlib1g-dev npm wabt && \
	npm install --ignore-scripts -g yarn && \
	# `binaryen` is needed by `cargo-contract` for optimizing Wasm files.
	# We fetch the latest release which contains a Linux binary.
	curl -L $(curl --silent https://api.github.com/repos/WebAssembly/binaryen/releases \
		 | jq -r '.[0].assets | [.[] | .browser_download_url] | map(select(match("x86_64-linux\\.tar\\.gz$"))) | .[0]' \
		 ) | tar -xz -C /usr/local/bin/ --wildcards --strip-components=2 'binaryen-*/bin/wasm-opt' && \

	# The stable toolchain is used to build ink! contracts through the use of the
	# `RUSTC_BOOSTRAP=1` environment variable. We also need to install the
	# `wasm32-unknown-unknown` target since that's the platform that ink! smart contracts
	# run on.
	rustup target add wasm32-unknown-unknown --toolchain stable && \
	rustup component add rust-src --toolchain stable && \
	rustup default stable && \
	# We also use the nightly toolchain for linting. We perform checks using RustFmt, and
	# Cargo Clippy.
	#
	# Note that we pin the nightly toolchain since it often creates breaking changes during
	# the RustFmt and Clippy stages of the CI.
	rustup toolchain install nightly-${RUST_NIGHTLY} --target wasm32-unknown-unknown \
		--profile minimal --component rustfmt clippy rust-src && \

	# Alias pinned toolchain as nightly, otherwise it appears as though we
	# don't have a nightly toolchain (i.e rustc +nightly --version is empty)
	ln -s "/usr/local/rustup/toolchains/nightly-${RUST_NIGHTLY}-x86_64-unknown-linux-gnu" \
		/usr/local/rustup/toolchains/nightly-x86_64-unknown-linux-gnu && \

	# `cargo-dylint` and `dylint-link` are dependencies needed to run `cargo-contract`.
	cargo install cargo-dylint dylint-link && \

	# Install the latest `cargo-contract` to allow you to compile and interact with contracts
	# (published on crates.io)
	# Note: To install a specific version run `cargo install cargo-contract --version x.x.x`
	# cargo install --git https://github.com/paritytech/cargo-contract \
    #     --locked --branch master --force && \
	cargo install cargo-contract --version 2.2.1 && \

	# Download the latest `substrate-contracts-node` binary
	# Note: To install a specific version run
	# `cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node/ --version x.x.x`
	# curl -L -o substrate-contracts-node.zip 'https://gitlab.parity.io/parity/mirrors/substrate-contracts-node/-/jobs/artifacts/main/download?job=build-linux' && \
	# unzip substrate-contracts-node.zip && \
	# mv artifacts/substrate-contracts-node-linux/substrate-contracts-node /usr/local/cargo/bin/substrate-contracts-node && \
	# rm -r artifacts substrate-contracts-node.zip && \
	# chmod +x /usr/local/cargo/bin/substrate-contracts-node && \

	cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git --tag v0.23.0 --force --locked && \
	chmod +x /usr/local/cargo/bin/substrate-contracts-node && \

	# We use `estuary` as a lightweight cargo registry in the CI to test if
	# publishing `cargo-contract` to it and installing it from there works.
	cargo install --git https://github.com/onelson/estuary.git --force && \

	# Versions
	yarn --version && \
	rustup show && \
	cargo --version && \
	echo $( substrate-contracts-node --version | awk 'NF' ) && \
	estuary --version

# show backtraces
ENV	RUST_BACKTRACE=1

COPY ./README.md .
COPY ./dapps .

EXPOSE 8080 30333 9933 9944 9615 443 80

CMD tail -f /dev/null

	# cargo clean up
	# removes compilation artifacts cargo install creates (>250M)
	# rm -rf "${CARGO_HOME}/registry" "${CARGO_HOME}/git" /root/.cache/sccache && \

	# apt clean up
	# apt-get autoremove -y && \
	# apt-get clean && \
	# rm -rf /var/lib/apt/lists/*