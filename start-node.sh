#!/bin/bash
# Start the Polkadot Confidential Offchain Worker Node

set -e

echo "╔═══════════════════════════════════════════════════════════╗"
echo "║   Polkadot Confidential Offchain Worker - Start Node     ║"
echo "╚═══════════════════════════════════════════════════════════╝"
echo ""

# Check if binary exists
if [ ! -f "./target/release/parachain-template-node" ]; then
    echo "❌ Binary not found. Building..."
    cargo build --release
fi

# Kill any existing node
pkill -f parachain-template-node || true
sleep 2

echo "🚀 Starting blockchain node in development mode..."
echo ""
echo "Node configuration:"
echo "  - RPC:              http://127.0.0.1:9944"
echo "  - WebSocket:        ws://127.0.0.1:9944"
echo "  - Mode:             Development (--dev)"
echo "  - Storage:          Temporary (--tmp)"
echo "  - Offchain Worker:  Enabled"
echo ""

# Start the node
./target/release/parachain-template-node \
  --dev \
  --tmp \
  --rpc-external \
  --rpc-cors all \
  --rpc-port 9944 \
  --rpc-methods Unsafe \
  --enable-offchain-indexing true \
  --offchain-worker Always \
  -lruntime=debug \
  -lruntime::offchain=trace

echo ""
echo "✅ Node started!"

