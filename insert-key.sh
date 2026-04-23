#!/bin/bash
# Insert signing key for offchain worker

set -e

NODE_URL="${NODE_URL:-http://localhost:9944}"

echo "╔═══════════════════════════════════════════════════════════╗"
echo "║   Insert Offchain Worker Signing Key                     ║"
echo "╚═══════════════════════════════════════════════════════════╝"
echo ""

echo "🔑 Inserting Alice's key for offchain worker signing..."
echo "   Key type: btc!"
echo "   Node URL: $NODE_URL"
echo ""

# Insert Alice's key with the 'btc!' key type
curl -s -X POST "$NODE_URL" \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "author_insertKey",
    "params": [
      "btc!",
      "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice",
      "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
    ]
  }' | jq '.'

echo ""
echo "✅ Key inserted!"
echo ""
echo "Verifying key..."
curl -s -X POST "$NODE_URL" \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "author_hasKey",
    "params": [
      "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
      "btc!"
    ]
  }' | jq '.'

echo ""

