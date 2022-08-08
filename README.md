# STARFLEIT
[![starfleit on crates.io](https://img.shields.io/crates/v/starfleit.svg)](https://crates.io/crates/starfleit)
[![workflow](https://github.com/starfleit/starfleit-contracts/actions/workflows/tests.yml/badge.svg)](https://github.com/starfleit/starfleit-contracts/actions/workflows/tests.yml)
[![codecov](https://codecov.io/gh/starfleit/starfleit-contracts/branch/main/graph/badge.svg?token=RKTBOBPCCW)](https://codecov.io/gh/starfleit/starfleit-contracts)

Uniswap-inspired automated market-maker (AMM) protocol powered by Smart Contracts.

## Contracts

| Name                                               | Description                                  |
| -------------------------------------------------- | -------------------------------------------- |
| [`starfleit_factory`](contracts/starfleit_factory) |                                              |
| [`starfleit_pair`](contracts/starfleit_pair)       |                                              |
| [`starfleit_token`](contracts/starfleit_token)     | CW20 (ERC20 equivalent) token implementation |

* starfleit_factory

   Mainnet: `fetch1slz6c85kxp4ek5ufmcakfhnscv9r2snlemxgwz6cjhklgh7v2hms8rgt5v`

   Testnet: `fetch1kmag3937lrl6dtsv29mlfsedzngl9egv5c3apnr468q50gu04zrqea398u`

* starfleit_pair

   Mainnet (CodeID): 22

   Testnet (CodeID): 1464

* starfleit_token

   Mainnet (CodeID): 21

   Testnet (CodeID): 1463

## Running this contract

You will need Rust 1.44.1+ with wasm32-unknown-unknown target installed.

You can run unit tests on this on each contracts directory via :

```
cargo unit-test
cargo integration-test
```

Once you are happy with the content, you can compile it to wasm on each contracts directory via:

```
RUSTFLAGS='-C link-arg=-s' cargo wasm
cp ../../target/wasm32-unknown-unknown/release/cw1_subkeys.wasm .
ls -l cw1_subkeys.wasm
sha256sum cw1_subkeys.wasm
```

Or for a production-ready (compressed) build, run the following from the repository root:

```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.12.6
```

The optimized contracts are generated in the artifacts/ directory.
