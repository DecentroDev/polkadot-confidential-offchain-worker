<div align="center">

# Polkadot SDK's Parachain Template

<img height="70px" alt="Polkadot SDK Logo" src="https://github.com/paritytech/polkadot-sdk/raw/master/docs/images/Polkadot_Logo_Horizontal_Pink_White.png#gh-dark-mode-only"/>
<img height="70px" alt="Polkadot SDK Logo" src="https://github.com/paritytech/polkadot-sdk/raw/master/docs/images/Polkadot_Logo_Horizontal_Pink_Black.png#gh-light-mode-only"/>

> This is a template for creating a [parachain](https://wiki.polkadot.network/docs/learn-parachains) based on Polkadot SDK.
>
> This template is automatically updated after releases in the main [Polkadot SDK monorepo](https://github.com/paritytech/polkadot-sdk).

</div>

## Getting Started

- 🦀 The template is using the Rust language.

- 👉 Check the
  [Rust installation instructions](https://www.rust-lang.org/tools/install) for your system.

- 🛠️ Depending on your operating system and Rust version, there might be additional
  packages required to compile this template - please take note of the Rust compiler output.

### Build

🔨 Use the following command to build the node without launching it:

```sh
cargo build --release
```

🐳 Alternatively, build the docker image:

```sh
docker build . -t polkadot-sdk-parachain-template
```

## Local Development Chain

## Polkadot-omni-node

## Installation

Download & expose it via `PATH`:

```bash
# Download and set it on PATH.
wget https://github.com/paritytech/polkadot-sdk/releases/download/<stable_release_tag>/polkadot-omni-node
chmod +x polkadot-omni-node
export PATH="$PATH:`pwd`"
```

Compile & install via `cargo`:

```bash
# Assuming ~/.cargo/bin is on the PATH
cargo install polkadot-omni-node
```

## Usage

A basic example for an Omni Node run starts from a runtime which implements the [`sp_genesis_builder::GenesisBuilder`](https://docs.rs/sp-genesis-builder/latest/sp_genesis_builder/trait.GenesisBuilder.html).
The interface mandates the runtime to expose a [`named-preset`](https://docs.rs/staging-chain-spec-builder/latest/staging_chain_spec_builder/#generate-chain-spec-using-runtime-provided-genesis-config-preset).

### 1. Install chain-spec-builder

**Note**: `chain-spec-builder` binary is published on [`crates.io`](https://crates.io) under
[`staging-chain-spec-builder`](https://crates.io/crates/staging-chain-spec-builder) due to a name conflict.
Install it with `cargo` like bellow :

```bash
cargo install staging-chain-spec-builder
```

### 2. Generate a chain spec

Omni Node expects for the chain spec to contain parachains related fields like `relay_chain` and `para_id`.
These fields can be introduced by running [`staging-chain-spec-builder`](https://crates.io/crates/staging-chain-spec-builder)
with additional flags:

```bash
chain-spec-builder create --relay-chain <relay_chain_id> --para-id <id> -r <runtime.wasm> named-preset <preset_name>
```

### 3. Run Omni Node

And now with the generated chain spec we can start the node in development mode like so:

```bash
polkadot-omni-node --dev --chain <chain_spec.json>
```

## Useful links

- [`Omni Node Polkadot SDK Docs`](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/omni_node/index.html)
- [`Chain Spec Genesis Reference Docs`](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/chain_spec_genesis/index.html)
- [`polkadot-parachain-bin`](https://crates.io/crates/polkadot-parachain-bin)
- [`polkadot-sdk-parachain-template`](https://github.com/paritytech/polkadot-sdk-parachain-template)
- [`frame-omni-bencher`](https://crates.io/crates/frame-omni-bencher)
- [`staging-chain-spec-builder`](https://crates.io/crates/staging-chain-spec-builder)

## Troubleshooting

Current `polkadot-omni-node` version not working with Offchain worker template. We will wait for the new release.

For now we can clone the [Polkadot SDK monorepo](https://github.com/paritytech/polkadot-sdk) and build that project to get the latest `polkadot-omni-node`.

#### 1. Clone the repo [Polkadot SDK monorepo](https://github.com/paritytech/polkadot-sdk)

#### 2. Assuming you've built the binary using:

```bash
cargo build --release
```

you'll find the executable in the target/release directory.

#### 3. Make It Executable

Though Cargo usually sets the execute permission, you can ensure it’s executable by running:

```bash
chmod +x target/release/polkadot-omni-node
```

#### 4. Move the Binary to a Global PATH Directory

For example, copy it to /usr/local/bin (you might need sudo privileges):

```bash
sudo cp target/release/polkadot-omni-node /usr/local/bin/
```

#### 5. Verify Global Installation

After copying, verify that it’s accessible globally by running:

```bash
polkadot-omni-node --help
```

If everything is set up correctly, you should see the help output for the polkadot-omni-node command.

### Connect with the Polkadot-JS Apps Front-End

- 🌐 You can interact with your local node using the
  hosted version of the Polkadot/Substrate Portal:
  [relay chain](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944)
  and [parachain](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9988).

- 🪐 A hosted version is also
  available on [IPFS](https://dotapps.io/).

- 🧑‍🔧 You can also find the source code and instructions for hosting your own instance in the
  [`polkadot-js/apps`](https://github.com/polkadot-js/apps) repository.

## Contributing

- 🔄 This template is automatically updated after releases in the main [Polkadot SDK monorepo](https://github.com/paritytech/polkadot-sdk).

- ➡️ Any pull requests should be directed to this [source](https://github.com/paritytech/polkadot-sdk/tree/master/templates/parachain).

- 😇 Please refer to the monorepo's
  [contribution guidelines](https://github.com/paritytech/polkadot-sdk/blob/master/docs/contributor/CONTRIBUTING.md) and
  [Code of Conduct](https://github.com/paritytech/polkadot-sdk/blob/master/docs/contributor/CODE_OF_CONDUCT.md).

## Getting Help

- 🧑‍🏫 To learn about Polkadot in general, [Polkadot.network](https://polkadot.network/) website is a good starting point.

- 🧑‍🔧 For technical introduction, [here](https://github.com/paritytech/polkadot-sdk#-documentation) are
  the Polkadot SDK documentation resources.

- 👥 Additionally, there are [GitHub issues](https://github.com/paritytech/polkadot-sdk/issues) and
  [Substrate StackExchange](https://substrate.stackexchange.com/).
