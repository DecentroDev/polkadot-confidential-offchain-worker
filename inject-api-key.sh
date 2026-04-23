#!/bin/bash
# Inject API key into the blockchain's offchain local storage

set -e

API_KEY="${1:-my_secret_api_key_12345}"
STORAGE_KEY="api_key"
NODE_URL="${NODE_URL:-http://localhost:9944}"

echo "╔═══════════════════════════════════════════════════════════╗"
echo "║   Inject API Key into Offchain Storage                   ║"
echo "╚═══════════════════════════════════════════════════════════╝"
echo ""

# Convert API key to hex
API_KEY_HEX=$(echo -n "$API_KEY" | xxd -p | tr -d '\n')
STORAGE_KEY_HEX=$(echo -n "$STORAGE_KEY" | xxd -p | tr -d '\n')

echo "🔑 Injecting API key into offchain storage..."
echo "   Storage key: $STORAGE_KEY"
echo "   API key: $API_KEY"
echo "   Node URL: $NODE_URL"
echo ""

# Inject into offchain local storage
RESPONSE=$(curl -s -X POST "$NODE_URL" \
  -H "Content-Type: application/json" \
  -d "{
    \"jsonrpc\": \"2.0\",
    \"id\": 1,
    \"method\": \"offchain_localStorageSet\",
    \"params\": [
      \"PERSISTENT\",
      \"0x${STORAGE_KEY_HEX}\",
      \"0x${API_KEY_HEX}\"
    ]
  }")

echo "Response: $RESPONSE"
echo ""

# Verify it was stored
echo "🔍 Verifying storage..."
VERIFY=$(curl -s -X POST "$NODE_URL" \
  -H "Content-Type: application/json" \
  -d "{
    \"jsonrpc\": \"2.0\",
    \"id\": 1,
    \"method\": \"offchain_localStorageGet\",
    \"params\": [
      \"PERSISTENT\",
      \"0x${STORAGE_KEY_HEX}\"
    ]
  }")

echo "Verification: $VERIFY"
echo ""
echo "✅ API key injected successfully!"
echo ""
echo "The offchain worker can now use this API key to make authenticated requests."

