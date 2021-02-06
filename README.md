# subDAO-contracts
contracts for subDAO, using [ink!](https://github.com/paritytech/ink).

## install cargo-contract
reference [here]().

## compile contracts
```bash
cargo +nightly contract build
```

## install by polkadot.js apps
visit [polkadot.js apps](https://polkadot.js.org/apps/), and connect subDAO node.
then `Develpoer`->`Contract`->`upload WASM`.

## build a new contract
```bash
cargo contract new test
```