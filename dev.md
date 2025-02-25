<div align="center">

# Confidential Offchain Worker Dev Tutorial

<img height="70px" alt="Polkadot SDK Logo" src="https://github.com/paritytech/polkadot-sdk/raw/master/docs/images/Polkadot_Logo_Horizontal_Pink_Black.png#gh-light-mode-only"/>

> This template is based on [parachain template](https://github.com/paritytech/polkadot-sdk/tree/master/templates/parachain) based on Polkadot SDK.
> 
> The tutorial below explains how to run and use it.

</div>

## Getting Started

- 🦀 The template is using the Rust language.

- 👉 Check the
  [Rust installation instructions](https://www.rust-lang.org/tools/install) for your system.

- 🛠️ Depending on your operating system and Rust version, there might be additional
  packages required to compile this template - please take note of the Rust compiler output.

- ℹ️ Recommended toolchains (at the time of writing):
  - `stable-aarch64-apple-darwin (default)`
  - `nightly-aarch64-apple-darwin (override)`


### Build Node Template

🔨 Use the following command to build the node without launching it:

```sh
cargo build --release
```

🐳 Alternatively, build the docker image:

```sh
docker build . -t polkadot-sdk-parachain-template
```

### Run tests

In order to execute the tests, run:

```sh
cargo test
```

### Install `polkadot-omni-node`

1. Download & expose it via `PATH`.

    ℹ️ **Info:** You can find the stable version [here](https://github.com/paritytech/polkadot-sdk/releases)
  
    ```bash
    # Download and set it on PATH.
    wget https://github.com/paritytech/polkadot-sdk/releases/download/<stable_release_tag>/polkadot-omni-node
    chmod +x polkadot-omni-node
    export PATH="$PATH:`pwd`"
    ```

2. Compile & install via `cargo`:

    ```bash
    # Assuming ~/.cargo/bin is on the PATH
    cargo install polkadot-omni-node
    ```

### Run Node Template

1. Install `chain-spec-builder`

    **Note**: `chain-spec-builder` binary is published on [`crates.io`](https://crates.io) under
    [`staging-chain-spec-builder`](https://crates.io/crates/staging-chain-spec-builder) due to a name conflict.
    Install it with `cargo` like bellow :
    
    ```bash
    cargo install staging-chain-spec-builder
    ```

2. Generate a chain spec

    Omni Node expects for the chain spec to contain parachains related fields like `relay_chain` and `para_id`.
    These fields can be introduced by running [`staging-chain-spec-builder`](https://crates.io/crates/staging-chain-spec-builder)
    with additional flags:
    
    ```
    chain-spec-builder create -t development \
    --relay-chain paseo \
    --para-id 1000 \
    --runtime ./target/release/wbuild/parachain-template-runtime/parachain_template_runtime.compact.compressed.wasm \
    named-preset development
    ```

3. Run the Omni Node

    And now with the generated chain spec we can start the node in development mode like so:
    
    ```bash
    polkadot-omni-node --chain ./chain_spec.json --offchain-worker always --dev
    ```

### Test the Node Template


#### 1. Connect with the Polkadot-JS Apps Front-End

- 🌐 You can interact with your local node using the
  hosted version of the Polkadot/Substrate Portal:
  [relay chain](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944)
  and [parachain](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9988).

- 🪐 A hosted version is also
  available on [IPFS](https://dotapps.io/).

- 🧑‍🔧 You can also find the source code and instructions for hosting your own instance in the
  [`polkadot-js/apps`](https://github.com/polkadot-js/apps) repository.

### 2. Updating Your API Key

We use **offchain local storage** to store the API key. You can update it using one of the following methods:

#### Using Polkadot-JS Apps
1. Open **Polkadot-JS Apps**.
2. Navigate to the **"RPC"** tab.
3. Select **"offchain"** → **"localStorageSet"**.
4. Enter the key and value for your API key.
5. Submit the transaction.

#### Using an RPC Call
You can also update the API key programmatically using an **RPC call**:

```json
{
  "jsonrpc": "2.0",
  "method": "offchain_localStorageSet",
  "params": ["persistent", "api_key", "your_new_api_key"],
  "id": 1
}
```
Replace "your_new_api_key" with your actual API key.

### 3. Protecting Your API Key

You can use **Subway** as a **JSON RPC Gateway** to enhance security by protecting your API key with middleware and configuration settings.  

🔗 **Check out the documentation for more details:** [Subway](https://github.com/AcalaNetwork/subway)

## Useful links

- [`Omni Node Polkadot SDK Docs`](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/omni_node/index.html)
- [`Chain Spec Genesis Reference Docs`](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/chain_spec_genesis/index.html)
- [`polkadot-parachain-bin`](https://crates.io/crates/polkadot-parachain-bin)
- [`polkadot-sdk-parachain-template`](https://github.com/paritytech/polkadot-sdk-parachain-template)
- [`frame-omni-bencher`](https://crates.io/crates/frame-omni-bencher)
- [`staging-chain-spec-builder`](https://crates.io/crates/staging-chain-spec-builder)

## Troubleshooting

ℹ️ Unfortunately, the current `polkadot-omni-node` version is NOT compatible with the Offchain worker template. We hope this will be fixed in an upcoming release.

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
