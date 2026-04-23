# 🚀 Polkadot Confidential Offchain Worker - Quick Start

This repository contains a **Polkadot SDK-based blockchain** with a **confidential offchain worker** that securely stores API keys and makes authenticated HTTP requests to external APIs.

## 📋 Prerequisites

- Rust toolchain (latest stable)
- Node.js v18+ (for testing scripts)
- jq (for JSON parsing in shell scripts)

## 🎯 What This Does

The offchain worker:
1. **Securely stores** API keys in the blockchain's local offchain storage
2. **Retrieves** the API key when needed
3. **Makes authenticated HTTP requests** to external APIs
4. **Logs responses** without exposing the API key on-chain

This is useful for:
- Fetching data from authenticated APIs
- Price oracles with API keys
- Weather data services
- Any external API that requires authentication

## 🏃 Quick Start (3 Steps)

### Step 1: Start the HTTP API Server

The API server provides a test endpoint that requires authentication.

```bash
# Terminal 1: Start the API server
npm install
npm run api

# Or with custom port/API key:
PORT=8080 API_KEY=my_secret_key_123 node http-api-server.js
```

The server will start on `http://localhost:8080` with endpoint `/api/data`.

### Step 2: Start the Blockchain Node

```bash
# Terminal 2: Start the blockchain node
chmod +x start-node.sh
./start-node.sh
```

Wait for the node to start (you'll see "Idle" messages). The node runs on `ws://localhost:9944`.

### Step 3: Inject API Key & Test

```bash
# Terminal 3: Setup and test

# Install dependencies for test script
npm install

# Inject the API key into offchain storage
chmod +x inject-api-key.sh
./inject-api-key.sh my_secret_api_key_12345

# Insert signing key for offchain worker
chmod +x insert-key.sh
./insert-key.sh

# Trigger the offchain worker
npm test
```

## 📊 What You'll See

### API Server Output:
```
[2025-01-10T10:30:15.123Z] GET /api/data
  Authorization: Bearer my_secret_api_key_12345...
  ✅ Valid API key
  📤 Returning confidential data
```

### Blockchain Node Output (with -lruntime::offchain=trace):
```
[Offchain] API key retrieved: my_secret_api_key_12345
[Offchain] Making HTTP request to http://localhost:8080/api/data
[Offchain] Response: {"authenticated":true,"message":"Success!..."}
```

## 🔧 Configuration

### Change the API Endpoint

Edit `pallets/offchain-worker/src/lib.rs`:

```rust
// Line 171: Change this URL
let url = "http://localhost:8080/api/data";
```

Then rebuild:
```bash
cargo build --release
```

### Change the API Key Storage Key

The API key is stored under the key `"api_key"` in offchain storage. To change this, edit:

```rust
// In lib.rs, line 132
match CustomApiKeyFetcher::fetch_api_key_for_request("api_key") {
```

## 📁 Project Structure

```
polkadot-confidential-offchain-worker/
├── pallets/
│   └── offchain-worker/      # The offchain worker pallet
│       └── src/lib.rs         # Main offchain worker logic
├── offchain-utils/            # Utility library for offchain operations
│   └── src/
│       ├── offchain_api_key.rs  # API key management
│       └── fetcher.rs           # HTTP request handling
├── runtime/                   # Blockchain runtime
├── node/                      # Blockchain node implementation
├── http-api-server.js         # Test HTTP API server
├── start-node.sh             # Start blockchain script
├── inject-api-key.sh         # Inject API key script
├── insert-key.sh             # Insert signing key script
└── test-offchain-worker.js   # Test script
```

## 🛠️ Development

### Build

```bash
cargo build --release
```

### Run Tests

```bash
cargo test
```

### Clean Build

```bash
cargo clean
cargo build --release
```

## 🔐 Security Notes

1. **API keys are stored locally** in the offchain worker's storage, not on-chain
2. **Keys never appear in blocks** or on-chain state
3. **Each node** maintains its own offchain storage
4. **HTTPS recommended** for production API endpoints

## 📝 Customization

### Add Your Own API Integration

1. Edit `pallets/offchain-worker/src/lib.rs`
2. Modify the `offchain_worker` function (line 152)
3. Change the URL and request handling
4. Rebuild and test

Example:
```rust
fn offchain_worker(block_number: BlockNumberFor<T>) {
    let api_key = match Self::get_api_key() {
        Ok(key) => key,
        Err(_) => return,
    };

    // Your custom API endpoint
    let url = "https://api.example.com/v1/data";
    
    let request = HttpRequest::new(url)
        .add_header("Authorization", &format!("Bearer {}", api_key))
        .add_header("Content-Type", "application/json");

    let response = DefaultOffchainFetcher::fetch_string(request);
    
    match response {
        Ok(data) => {
            log::info!("Received data: {}", data);
            // Process the data...
        }
        Err(e) => {
            log::error!("API request failed: {:?}", e);
        }
    }
}
```

## 🤝 Contributing

Feel free to open issues or submit pull requests!

## 📄 License

MIT License - see LICENSE file for details

## 🔗 Resources

- [Polkadot SDK Documentation](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/index.html)
- [Offchain Workers Guide](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/async_backing_guide/index.html)
- [Substrate Documentation](https://docs.substrate.io/)

## 💡 Tips

- Use `RUST_LOG=runtime=debug` for detailed runtime logs
- Use `-lruntime::offchain=trace` for offchain worker traces
- Check logs to verify API key retrieval and HTTP requests
- The offchain worker runs on **every block** by default

---

**Ready to build confidential offchain integrations? Start with Step 1 above! 🚀**

