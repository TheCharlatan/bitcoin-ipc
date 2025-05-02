# Mining example

The demonstration in `example/mining.rs` connects to `bitcoin-node` via a Unix socket and makes multiple queries, one of which is to fetch the block header from the current block template. The main file of interest is `capnp/mining.capnp`, which defines the interface for the client to get block templates. You will see, for example, `create_new_block_request` in `mining.rs` corresponds to `createNewBlock` in the `capnp` file.

## Bitcoin Core

Checkout commit [06439a14c8](https://github.com/bitcoin/bitcoin/tree/06439a14c884d7f81f331646ad361e88b3037a51) or any other recent version of `master`, and build Bitcoin Core with IPC enabled:

```
cmake -B build -DENABLE_IPC=ON -DENABLE_WALLET=OFF
```

Build with parallel jobs

```
cmake --build build -j 16
```

Run `bitcoin-node` with IPC enabled, debugging, and listen for unix socket connections

```
./build/bin/bitcoin-node -chain=testnet4 -ipcbind=unix -debug=ipc
```

## This repository

You will need to provide the full file path to the `sock` file listening connections. This is found in the typical data directory. For example, on Linux, `/home/me/.bitcoin/testnet4/node.sock`

To run the example, provide this path as an argument:

```
cargo run --example mining <.sock path> --release
```
