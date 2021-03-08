# subDAO-contracts
contracts for subDAO, using [ink!](https://github.com/paritytech/ink).

## install cargo-contract
reference [here]().

**Warning!!**  
Please use version 0.8.0!  
```
cargo install cargo-contract --vers 0.8.0 --force --locked
```

## compile contracts
```bash
cargo +nightly contract build
```

## install by polkadot.js apps
visit [polkadot.js apps](https://polkadot.js.org/apps/), and connect subDAO node.
then `Develpoer`->`Contract`->`upload WASM`.

## generate a new contract
```bash
cargo contract new test
```

## build all contracts
```bash
./build.sh
```
the ABI and wasm copied in `./traget` dir.