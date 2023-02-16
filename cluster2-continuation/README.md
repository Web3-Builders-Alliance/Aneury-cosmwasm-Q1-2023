# Requirements For WBA Challenge

Last week's challenge:

Cluster 2 Challenge

You need two actors, Sender and Receiver.

The Sender:
- Receives native tokens from anyone and forwards them to the Receiver.
- Stores how much tokens have been received/forwarded, which can be returned in a Query.

The Receiver:
- Stores the received tokens until the owner of the contract claims them.
- The owner can claim part of the tokens held by the Receiver, or all at once.

Optional:
- The Sender gets notified when the Receiver has transferred the funds.
- The Sender gets notified when the Receiver funds have been claimed by its owner.

Assume happy paths, though minor validations are expected. Pass any relevant information you need on the messages.

Additional Resources:
- cw-template: https://github.com/CosmWasm/cw-template
- cw-storage-plus: https://github.com/CosmWasm/cw-storage-plus
- cw-plus: https://github.com/CosmWasm/cw-plus

# Cluster 2 challenge continuation:

- Validate that the amount of tokens being sent in the transaction match the ones the execute message intents to deposit (given that you are passing an amount in the execute message)
- Validate that the denom of the funds sent is uluna, considering we are gonna deploy this on terra.
- Validate only 1 type of coin is being sent in the tx.
- Create custom errors for those scenarios where things go wrong.

- Create a query on the sender to check how many tokens have been received/forwarded
- Create a query on the receiver to check if the tokens have been claimed by the owner

- Deploy to pisco-1, terra's testnet. Here's the information you need to achieve that:

```sh
  export CHAIN_ID="pisco-1"
  export DENOM="uluna"
  export BINARY="terrad"
  export RPC="https://terra-testnet-rpc.polkachu.com:443"
  export TXFLAG="--node $RPC --chain-id $CHAIN_ID --gas-prices 0.25$DENOM --gas auto --gas-adjustment 1.3 -y -b block --output json"
```
- Use terrad to interact with your contract(s), send queries and so on.

You need this to create the optimized wasm to deploy on chain:

```sh
docker run --rm -v "$(pwd)":/code \
--mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
cosmwasm/rust-optimizer:0.12.11
```

# Install Terrad
first install golang, just in case you don't have it installed.

```sh
git clone https://github.com/terra-money/core
cd core
git checkout [latest version]
make install
```

# Test Terra
```sh
terrad version --long
```
in case that this command show error related to that this command is not installed.
```
export PATH=$PATH:$(go env GOPATH)/bin
```


# TerraFlags
```sh
export TXFLAG="--node $RPC --chain-id $CHAIN_ID --gas-prices 0.25$DENOM --gas auto --gas-adjustment 1.3 -y -b block --output json"
```

# Common Command for Terrad
```sh
terrad tx
terrad tx wasm
terrad query

```

### using some explantion from Javier class

```sh
export CHAIN_ID="pisco-1"
export DENOM="uluna"
export BINARY="terrad"
export RPC="https://terra-testnet-rpc.polkachu.com:443"
export TXFLAG="--node $RPC --chain-id $CHAIN_ID --gas-prices 0.25$DENOM --gas auto --gas-adjustment 1.3 -y -b block --output json"
$BINARY status --node $RPC
```
# Store contract
```sh
terrad tx wasm store artifact/wasm.prefixed --from <my_address> $TX_FLAGS
...
terrad tx wasm instantiate --from ...

terrad tx wasm store wasm/file/path]
```


```sh
https://terrasco.pe/testnet/address/terra1s0df8dfdpve2vr2hg5p2dw9dqfmyct4klr4jspdhjew6ahyaddmst6df94
```

# Main CosmWasm Starter Pack

This is a template to build smart contracts in Rust to run inside a
[Cosmos SDK](https://github.com/cosmos/cosmos-sdk) module on all chains that enable it.
To understand the framework better, please read the overview in the
[cosmwasm repo](https://github.com/CosmWasm/cosmwasm/blob/master/README.md),
and dig into the [cosmwasm docs](https://www.cosmwasm.com).
This assumes you understand the theory and just want to get coding.

## Creating a new repo from template

Assuming you have a recent version of Rust and Cargo installed
(via [rustup](https://rustup.rs/)),
then the following should get you a new repo to start a contract:

Install [cargo-generate](https://github.com/ashleygwilliams/cargo-generate) and cargo-run-script.
Unless you did that before, run this line now:

```sh
cargo install cargo-generate --features vendored-openssl
cargo install cargo-run-script
```

Now, use it to create your new contract.
Go to the folder in which you want to place it and run:

**Latest**

```sh
cargo generate --git https://github.com/CosmWasm/cw-template.git --name PROJECT_NAME
```

For cloning minimal code repo:

```sh
cargo generate --git https://github.com/CosmWasm/cw-template.git --name PROJECT_NAME -d minimal=true
```

**Older Version**

Pass version as branch flag:

```sh
cargo generate --git https://github.com/CosmWasm/cw-template.git --branch <version> --name PROJECT_NAME
```

Example:

```sh
cargo generate --git https://github.com/CosmWasm/cw-template.git --branch 0.16 --name PROJECT_NAME
```

You will now have a new folder called `PROJECT_NAME` (I hope you changed that to something else)
containing a simple working contract and build system that you can customize.

## Create a Repo

After generating, you have a initialized local git repo, but no commits, and no remote.
Go to a server (eg. github) and create a new upstream repo (called `YOUR-GIT-URL` below).
Then run the following:

```sh
# this is needed to create a valid Cargo.lock file (see below)
cargo check
git branch -M main
git add .
git commit -m 'Initial Commit'
git remote add origin YOUR-GIT-URL
git push -u origin main
```

## CI Support

We have template configurations for both [GitHub Actions](.github/workflows/Basic.yml)
and [Circle CI](.circleci/config.yml) in the generated project, so you can
get up and running with CI right away.

One note is that the CI runs all `cargo` commands
with `--locked` to ensure it uses the exact same versions as you have locally. This also means
you must have an up-to-date `Cargo.lock` file, which is not auto-generated.
The first time you set up the project (or after adding any dep), you should ensure the
`Cargo.lock` file is updated, so the CI will test properly. This can be done simply by
running `cargo check` or `cargo unit-test`.

## Using your project

Once you have your custom repo, you should check out [Developing](./Developing.md) to explain
more on how to run tests and develop code. Or go through the
[online tutorial](https://docs.cosmwasm.com/) to get a better feel
of how to develop.

[Publishing](./Publishing.md) contains useful information on how to publish your contract
to the world, once you are ready to deploy it on a running blockchain. And
[Importing](./Importing.md) contains information about pulling in other contracts or crates
that have been published.

Please replace this README file with information about your specific project. You can keep
the `Developing.md` and `Publishing.md` files as useful referenced, but please set some
proper description in the README.



