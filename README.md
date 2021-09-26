# subDAO-contracts
contracts for subDAO, using [ink! 3.0.0.0-rc3](https://github.com/paritytech/ink/tree/v3.0.0-rc3).

## rust

```
curl https://sh.rustup.rs -sSf | sh
source ~/.cargo/env
rustup toolchain install 1.51.0
rustup toolchain install nightly-2021-07-23
rustup component add rust-src --toolchain nightly-2021-07-23
rustup target add wasm32-unknown-unknown --toolchain nightly-2021-07-23
rustup default 1.51.0
```

## binaryen
As a pre-requisite for the tool you need to install the [binaryen](https://github.com/WebAssembly/binaryen) package, which is used to optimize the WebAssembly bytecode of the contract.

binaryen version **must be >=99**.

## cargo-contract
Please **use version newer than  0.11**!  
```
cargo install cargo-contract --vers ^0.11 --force --locked
```

reference [here](https://substrate.dev/substrate-contracts-workshop/#/0/setup).

## compile contracts
### single contract
```bash
cargo +nightly-2021-07-23 contract build
```

### all contracts
```
./build.sh
```
the ABI, wasm, and contract files are copied in `./release` dir.

## install by polkadot.js apps
visit [polkadot.js apps](https://polkadot.js.org/apps/), and connect subDAO node.
then `Develpoer`->`Contract`->`upload WASM`.