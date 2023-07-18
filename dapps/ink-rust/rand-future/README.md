* Install latest Cargo Contract version
* Install Rustup 1.69. See https://github.com/paritytech/cargo-contract/blob/master/.github/workflows/ci.yml#L185. If you do not use 1.69 you will get error `ERROR: Loading of original wasm failed` when running `cargo contract build`
```
rustup update
rustup default stable
rustup install 1.69 
rustup default 1.69
rustup override set 1.69
rustup component add rust-src --toolchain 1.69
rustup toolchain list
rustup show
```
* Update Cargo Contract to latest version. Check latest version on Github then:
```
cargo-contract --version
cargo install --force --locked cargo-contract
```
* Run the following to start a contracts-node, upload and instantiate the contracts,
and call the relevant functions

* Links
    * use [rand-extension](https://github.com/paritytech/ink-examples/blob/main/rand-extension/lib.rs) to get random number on-chain
