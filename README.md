# STARFLEIT

Uniswap-inspired automated market-maker (AMM) protocol powered by Smart Contracts.

## Contracts

| Name                                               | Description                                  |
| -------------------------------------------------- | -------------------------------------------- |
| [`starfleit_factory`](contracts/starfleit_factory) |                                              |
| [`starfleit_pair`](contracts/starfleit_pair)       |                                              |
| [`starfleit_token`](contracts/starfleit_token)     | CW20 (ERC20 equivalent) token implementation |

* starfleit_factory

   Mainnet: ``

   Testnet: `fetch1xjn7ljgkzn8agscr8g6xnnhn3azu3kfkuga8uqufr36sc75f8s0sxyhnyq`

* starfleit_pair

   Mainnet (CodeID): 

   Testnet (CodeID): 711

* starfleit_token

   Mainnet (CodeID): 

   Testnet (CodeID): 712

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
