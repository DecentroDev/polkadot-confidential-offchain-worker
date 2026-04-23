# 🔐 Polkadot Confidential Offchain Worker

A **Polkadot SDK-based blockchain** with a **confidential offchain worker** that securely stores API keys and makes authenticated HTTP requests to external APIs.

<div align="center">

[![Polkadot SDK](https://img.shields.io/badge/Polkadot%20SDK-Latest-E6007A)](https://github.com/paritytech/polkadot-sdk)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

</div>

## 🎯 What This Does

This project demonstrates how to build a secure offchain worker that:

- 🔐 **Stores API keys securely** in local offchain storage (never on-chain)
- 🔑 **Retrieves credentials** safely when making external API calls
- 🌐 **Makes authenticated HTTP requests** to external services
- ✅ **Processes responses** without exposing sensitive data on-chain

**Use Cases:**
- Price oracles with authenticated API access
- Weather data services requiring API keys
- Any external API integration that needs credentials
- Confidential data fetching for DeFi protocols

## 🚀 Quick Start (3 Steps)

### Prerequisites
```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Node.js v18+
# jq for JSON parsing
```

### Step 1: Build
```bash
cargo build --release
```

### Step 2: Start Everything
```bash
./RUN.sh
```

This starts:
- ✅ HTTP API Server (port 8080)
- ✅ Blockchain Node (port 9944)
- ✅ Injects API key
- ✅ Runs test transaction

### Step 3: Watch the Magic ✨

The offchain worker will:
1. Retrieve the API key from offchain storage
2. Make an authenticated request to the HTTP API
3. Log the response

Check the logs to see it in action!

## 📖 Manual Setup (Step-by-Step)

### Terminal 1: Start API Server
```bash
npm install
npm run api
```

Output:
```
🌐 Server running on: http://0.0.0.0:8080
🔑 API Key: my_secret_api_key_12345
```

### Terminal 2: Start Blockchain
```bash
./start-node.sh
```

Wait for: `Idle (0 peers), best: #1`

### Terminal 3: Setup & Test
```bash
# Install dependencies
npm install

# Inject API key into offchain storage
./inject-api-key.sh my_secret_api_key_12345

# Insert signing key
./insert-key.sh

# Run test
npm test
```

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Blockchain Node                          │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  Runtime (WASM)                                       │  │
│  │  ┌────────────────────────────────────────────────┐  │  │
│  │  │  Offchain Worker Pallet                        │  │  │
│  │  │  - Retrieves API key from offchain storage     │  │  │
│  │  │  - Makes HTTP request with authentication      │  │  │
│  │  │  - Processes response                          │  │  │
│  │  └────────────────────────────────────────────────┘  │  │
│  └──────────────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  Offchain Local Storage (PERSISTENT)                 │  │
│  │  Key: "api_key"                                      │  │
│  │  Value: "my_secret_api_key_12345"                   │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                          │
                          │ HTTP Request
                          │ Authorization: Bearer <api_key>
                          ▼
                ┌──────────────────────┐
                │   HTTP API Server    │
                │   (port 8080)        │
                │  - Validates API key │
                │  - Returns data      │
                └──────────────────────┘
```

## 📁 Project Structure

```
polkadot-confidential-offchain-worker/
├── pallets/
│   └── offchain-worker/          # Offchain worker pallet
│       └── src/lib.rs             # Main logic
├── offchain-utils/                # Utility library
│   └── src/
│       ├── offchain_api_key.rs    # API key management
│       └── fetcher.rs             # HTTP client
├── runtime/                       # Blockchain runtime
├── node/                          # Node implementation
├── http-api-server.js            # Test HTTP API server
├── start-node.sh                 # Start blockchain
├── inject-api-key.sh             # Inject API key
├── insert-key.sh                 # Insert signing key
├── test-offchain-worker.js       # Test script
├── RUN.sh                        # All-in-one runner
└── README.md                     # This file
```

## 🔧 Configuration

### Change the API Endpoint

Edit `pallets/offchain-worker/src/lib.rs` (line 172):

```rust
let url = "http://localhost:8080/api/data";
```

Then rebuild:
```bash
cargo build --release
```

### Change the API Key Storage Key

Edit `pallets/offchain-worker/src/lib.rs` (line 132):

```rust
CustomApiKeyFetcher::fetch_api_key_for_request("api_key")
```

### Use Different API Key

```bash
./inject-api-key.sh YOUR_CUSTOM_API_KEY
```

## 🧪 Testing

### Test the HTTP API Server
```bash
curl -H "Authorization: Bearer my_secret_api_key_12345" \
  http://localhost:8080/api/data
```

Expected response:
```json
{
  "authenticated": true,
  "message": "Success! This is confidential data from the API.",
  "data": {
    "temperature": 23.5,
    "humidity": 65,
    "status": "operational"
  }
}
```

### Test Offchain Worker
```bash
npm test
```

Expected output:
```
✅ Connected to blockchain!
✅ Transaction included in block
🎉 SUCCESS!

Check the node logs to see:
  1. API key retrieval from offchain storage
  2. HTTP request with authentication
  3. Response from the API server
```

### View Detailed Logs

Start the node with trace logging:
```bash
./target/release/parachain-template-node \
  --dev \
  --tmp \
  --rpc-external \
  --rpc-port 9944 \
  -lruntime::offchain=trace
```

You'll see:
```
[Offchain] API key retrieved: my_secret_api_key_12345
[Offchain] Making HTTP request to http://localhost:8080/api/data
[Offchain] Response: {"authenticated":true,"message":"Success!..."}
```

## 🔐 Security Features

| Feature | Implementation |
|---------|---------------|
| **API Key Storage** | Offchain local storage (PERSISTENT) |
| **Key Visibility** | Never exposed on-chain or in blocks |
| **Access Control** | Only accessible by the node's offchain worker |
| **Transport** | HTTPS recommended for production |
| **Key Rotation** | Update via RPC call to offchain storage |

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

### Check Code
```bash
cargo clippy
cargo fmt --check
```

## 📚 How It Works

### 1. API Key Storage
```rust
// Store API key in offchain local storage
CustomApiKeyFetcher::store_api_key("api_key", "my_secret_api_key_12345");
```

### 2. Offchain Worker Execution
```rust
fn offchain_worker(block_number: BlockNumberFor<T>) {
    // Retrieve API key
    let api_key = Self::get_api_key().expect("API key not found");
    
    // Construct authenticated request
    let mut auth_value = String::from("Bearer ");
    auth_value.push_str(&api_key);
    
    let request = HttpRequest::new("http://localhost:8080/api/data")
        .add_header("Authorization", &auth_value);
    
    // Make HTTP request
    let response = DefaultOffchainFetcher::fetch_string(request);
    
    // Process response
    match response {
        Ok(data) => log::info!("Received: {}", data),
        Err(e) => log::error!("Failed: {:?}", e),
    }
}
```

### 3. HTTP Request Flow
1. **Offchain worker** runs on every new block
2. **Retrieves API key** from local storage
3. **Constructs HTTP request** with Authorization header
4. **Sends request** to external API
5. **Processes response** and logs result

## 🌐 HTTP API Endpoints

The included test server provides:

| Endpoint | Method | Auth | Description |
|----------|--------|------|-------------|
| `/api/data` | GET | ✅ Required | Returns confidential data |
| `/api/health` | GET | ❌ No | Health check |

## 🐛 Troubleshooting

### API Server Not Responding
```bash
# Check if server is running
curl http://localhost:8080/api/health

# Restart server
pkill -f http-api-server.js
npm run api
```

### Blockchain Node Not Starting
```bash
# Check port 9944
lsof -i :9944

# Clean and restart
pkill -f parachain-template-node
./start-node.sh
```

### API Key Not Found
```bash
# Re-inject API key
./inject-api-key.sh my_secret_api_key_12345

# Verify storage
curl -X POST http://localhost:9944 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","id":1,"method":"offchain_localStorageGet","params":["PERSISTENT","0x6170695f6b6579"]}'
```

### Offchain Worker Not Triggering
```bash
# Make sure you call an extrinsic to trigger block production
npm test

# Check if offchain worker is enabled
./target/release/parachain-template-node --help | grep offchain
```

## 🎓 Learn More

- [Polkadot SDK Documentation](https://paritytech.github.io/polkadot-sdk/master/)
- [Offchain Workers Guide](https://docs.substrate.io/learn/offchain-operations/)
- [Substrate Documentation](https://docs.substrate.io/)

## 🤝 Contributing

Contributions welcome! Please open an issue or submit a PR.

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

Built on [Polkadot SDK](https://github.com/paritytech/polkadot-sdk) by Parity Technologies.

---

<div align="center">

**Ready to build secure offchain integrations?**

⭐ Star this repo if it helped you!

</div>
