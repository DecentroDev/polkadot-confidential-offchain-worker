# ⚡ Quick Start Guide

## 🎯 One-Command Start

```bash
./RUN.sh
```

This starts everything and runs the test automatically!

---

## 📋 Manual 3-Terminal Setup

### Terminal 1: HTTP API Server
```bash
npm install
npm run api
```

Wait for: `🌐 Server running on: http://0.0.0.0:8080`

### Terminal 2: Blockchain Node
```bash
./start-node.sh
```

Wait for: `Idle (0 peers), best: #1`

### Terminal 3: Test
```bash
# Setup
npm install
./inject-api-key.sh my_secret_api_key_12345
./insert-key.sh

# Run test
npm test
```

---

## ✅ What Should Happen

### API Server Output:
```
[2025-11-10T13:37:21.240Z] GET /api/data
  Authorization: Bearer my_secret_api_key_12345...
  ✅ Valid API key
  📤 Returning confidential data
```

### Blockchain Node Output:
```
[Offchain] API key retrieved: my_secret_api_key_12345
[Offchain] Response: {"authenticated":true,"message":"Success!..."}
```

### Test Output:
```
✅ Connected to blockchain!
✅ Transaction included in block
🎉 SUCCESS!
```

---

## 🔧 Customize

### Change API Endpoint

Edit `pallets/offchain-worker/src/lib.rs` line 172:
```rust
let url = "http://your-api.com/endpoint";
```

Rebuild:
```bash
cargo build --release
```

### Change API Key

```bash
./inject-api-key.sh YOUR_NEW_API_KEY
```

### Use Your Own API

1. Modify `http-api-server.js` or point to your API
2. Update the endpoint in `pallets/offchain-worker/src/lib.rs`
3. Rebuild and test

---

## 🐛 Troubleshooting

| Problem | Solution |
|---------|----------|
| Port 8080 in use | Change `PORT` in `http-api-server.js` |
| Port 9944 in use | `pkill -f parachain-template-node` |
| API key not found | Re-run `./inject-api-key.sh` |
| Build errors | `cargo clean && cargo build --release` |

---

## 📁 Key Files

| File | Purpose |
|------|---------|
| `RUN.sh` | All-in-one starter |
| `http-api-server.js` | Test API server |
| `start-node.sh` | Start blockchain |
| `inject-api-key.sh` | Store API key |
| `test-offchain-worker.js` | Test script |
| `pallets/offchain-worker/src/lib.rs` | Main logic |

---

## 🎓 Learn More

See [README.md](README.md) for full documentation.

---

**Need help?** Open an issue on GitHub!

