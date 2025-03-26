<div align="center">

# Confidential Offchain Worker Dev Tutorial

<img height="70px" alt="Polkadot SDK Logo" src="https://github.com/paritytech/polkadot-sdk/raw/master/docs/images/Polkadot_Logo_Horizontal_Pink_Black.png#gh-light-mode-only"/>

> This template is based on the Polkadot SDK's [parachain template](https://github.com/paritytech/polkadot-sdk/tree/master/templates/parachain).
>
> The tutorial below explains how to run and use the Confidential Offchain Worker template, allowing you to access protected HTTP resources through Substrate's offchain worker feature.

</div>

## Getting Started 

### MacOS
**Install**
First, we will need Homebrew - the macOS package manager. Run the following in your Terminal.

```
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install.sh)"

```

Make sure brew installed properly.

```
brew --version
```

And let's make sure brew is up-to-date with the latest packages.

```
brew update
```

Now let's install the necessary packages.

```
brew install cmake openssl protobuf
```

Now let's install Rust.

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Don't forget to update the shell to include the necessary environment variables.

```
source ~/.cargo/env
```

You should now have Rust installed. Double check.

```
rustc --version
```

Now let's configure the Rust toolchain.

```
rustup default stable
rustup update
rustup target add wasm32-unknown-unknown
```

And add the nightly release channel.

```
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

You can confirm your Rust toolchain was installed properly with the following commands.

```
rustup show
rustup +nightly show
```

### Linux

These instructions will guide you on installing the necessary prerequisites for Linux to install Pop CLI and use all its features.

Help improve this documentation. Check our contribution guidelines.

**For Fedora**
```
sudo dnf update
sudo dnf install clang curl git openssl-devel make protobuf-compiler
```

**For OpenSUSE**

```
sudo zypper install clang curl git openssl-devel llvm-devel libudev-devel make protobuf
```

**Install Rust**


```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```


Don't forget to update the shell to include the necessary environment variables.

```
source $HOME/.cargo/env
```

You should now have Rust installed. Double check.

```
rustc --version

```

Now let's configure the Rust toolchain.

```
rustup default stable
rustup update
rustup target add wasm32-unknown-unknown
```

And add the nightly release channel.

```
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

You can confirm your Rust toolchain was installed properly with the following commands.

```
rustup show
rustup +nightly show
```

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

### `polkadot-omni-node` Installation

> ⚠️ **IMPORTANT:** The stable release doesn't currently support offchain-worker functionality. As a temporary fix, we have to manually clone and build the repository. Please head to the [Manual `polkadot-omni-node` Installation](#manual-polkadot-omni-node-installation) section and follow the instructions there.

1. Download & expose it via `PATH`.

   > ℹ️ **Info:** You can find the stable version [here](https://github.com/paritytech/polkadot-sdk/releases)

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

> ℹ️ Before running the template, make sure `polkadot-omni-node` has been installed (see previous chapter).

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

   or

   ```
   chain-spec-builder create --raw-storage --relay-chain "rococo-local" --para-id 1000 --runtime \
   target/release/wbuild/parachain-template-runtime/parachain_template_runtime.wasm named-preset development
   ```

3. Run the Omni Node

   And now with the generated chain spec we can start the node in development mode.

   > ℹ️ We're including trace logs here so we get more insights about the offchain-worker activity

   ```bash
   polkadot-omni-node --chain ./chain_spec.json --offchain-worker always --dev --log offchain=trace,logger=trace
   ```

   Note how the protected API is currently not accessible by the off-chain worker, hence ERRORs are being shown:

   <img width="1504" alt="image" src="https://github.com/user-attachments/assets/fa52cb47-db40-43f7-b0ba-d01d117fe660" />

   ➡️ In the next chapter we're going to explain how to configure the API key.

### Configure the API Key

#### 1. Connect with the Polkadot-JS Apps Front-End

- Connect to your local node using the Polkadot/Substrate Portal:
  https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944.

### 2. Update Your API Key

We use **offchain local storage** to store the API key. You can update it using one of the following methods:

#### Using Polkadot-JS Apps

1. Head to the [**"RPC"** tab](https://polkadot.js.org/apps/#/rpc).
2. Select **"offchain"** → **"localStorageSet"**.
3. Enter the key and value for your API key. For this demo, we'll use a [protected Postman demo endpoint](https://www.postman.com/postman/published-postman-templates/request/mj0cy1z/basic-auth) that can be accessed with `Basic cG9zdG1hbjpwYXNzd29yZA==`:

<img width="751" alt="image" src="https://github.com/user-attachments/assets/020b2359-57c2-47b6-8f7e-bdd3fe98a012" />

4. Submit the transaction.
5. Verify that the authorization ERROR doesn't occur anymore and the protected API resrouce can be accessed by the offchain-worker:

   <img width="1503" alt="image" src="https://github.com/user-attachments/assets/e737b729-7730-4991-8653-03c3dd39f1a2" />

#### Using an RPC Call

Alternatively to using Polkadot.js apps, you can also update the API key programmatically using an **RPC call**:

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

It's important to understand that per default, the `offchain.localStorageSet(kind, key, value)` endpoint is publicly callable without any restrictions (see also: https://github.com/paritytech/substrate/issues/2303). For this reason, an attacker could easily overwrite the api key if he knew how the `key` is called.

To mitigate this problem, you can use Acala's **Subway** as a **JSON RPC Gateway** to enhance security by protecting your API key with middleware and configuration settings. This solutions allows you to whitelist clients that are allowed to call the endpoint you want to protect (in this case, `offchain.localStorageSet(kind, key, value)`).

🔗 **Check out the documentation for more details:** [Subway](https://github.com/AcalaNetwork/subway)

## Useful links

- [`Omni Node Polkadot SDK Docs`](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/omni_node/index.html)
- [`Chain Spec Genesis Reference Docs`](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/chain_spec_genesis/index.html)
- [`polkadot-parachain-bin`](https://crates.io/crates/polkadot-parachain-bin)
- [`polkadot-sdk-parachain-template`](https://github.com/paritytech/polkadot-sdk-parachain-template)
- [`frame-omni-bencher`](https://crates.io/crates/frame-omni-bencher)
- [`staging-chain-spec-builder`](https://crates.io/crates/staging-chain-spec-builder)

## Troubleshooting

### Manual `polkadot-omni-node` Installation

> ℹ️ Unfortunately, the currently stable `polkadot-omni-node` version is NOT compatible with the Offchain worker template. We expect this to be fixed in an upcoming release (see [this PR](https://github.com/paritytech/polkadot-sdk/pull/7479)).

In order to use the offchain features on the `polkadot-omni-node`, we can clone the [Polkadot SDK monorepo](https://github.com/paritytech/polkadot-sdk) and build it manually (at commit height [87f4f3f](https://github.com/paritytech/polkadot-sdk/commit/87f4f3f0df5fc0cc72f69e612909d4d213965820) or higher).

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

> ⚠️ **IMPORTANT:** If you've already installed an older version of `polkadot-omni-node`, you'll have to remove it manually first by executing `cargo uninstall polkadot-omni-node`.

Copy it to /usr/local/bin (you might need sudo privileges):

```bash
sudo cp target/release/polkadot-omni-node /usr/local/bin/
```

#### 5. Verify Global Installation

After copying, verify that it’s accessible globally by running:

```bash
polkadot-omni-node --help
```

Make sure that you're using v1.17.1 or above:

```bash
 % polkadot-omni-node --version
polkadot-omni-node 1.17.1-4a400dc1866
```

If everything is set up correctly, you should see the help output for the polkadot-omni-node command.
