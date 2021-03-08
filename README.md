# PolkaMusic

PolkaMusic is a public, [substrate](https://github.com/paritytech/substrate) based music streaming platform built using [substrate-node-template](https://github.com/substrate-developer-hub/substrate-node-template), with on-chain governance by $POLM holders and forkless blockchain upgrade.

## Development

The network is not ready for use, under development :hammer_and_wrench:

For more information about the project check the website :link:[PolkaMusic](https://polkamusic.io)

### Build & Run Node

#### Prerequisites

- [x] Clone this repo and update the submodules:

```
git clone https://github.com/polkamusic/polkamusic
cd polkamusic:
```
- [x] Install RustLang with necessary dependencies

### Commands

```
cargo build
cargo run
```


### Embedded Docs

Once the project has been built, the following command can be used to explore all parameters and
subcommands:

```sh
./target/release/node-template -h
```

## Run

The provided `cargo run` command will launch a temporary node and its state will be discarded after
you terminate the process. After the project has been built, there are other ways to launch the
node.

### Single-Node Development Chain

This command will start the single-node development chain with persistent state:

```bash
./target/release/node-template --dev
```

Purge the development chain's state:

```bash
./target/release/node-template purge-chain --dev
```




