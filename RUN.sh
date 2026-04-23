#!/bin/bash
# All-in-One Runner for Polkadot Confidential Offchain Worker Demo
# This script starts everything you need in separate terminal windows

set -e

echo "╔═══════════════════════════════════════════════════════════╗"
echo "║   Polkadot Confidential Offchain Worker - Quick Start    ║"
echo "╚═══════════════════════════════════════════════════════════╝"
echo ""
echo "This will start:"
echo "  1. HTTP API Server (port 8080)"
echo "  2. Blockchain Node (port 9944)"
echo "  3. Setup API key and signing key"
echo "  4. Run test transaction"
echo ""

# Check if tmux is available
if ! command -v tmux &> /dev/null; then
    echo "⚠️  tmux not found. Installing recommended for better experience."
    echo "   Install with: sudo apt install tmux"
    echo ""
    echo "Continuing with sequential execution..."
    
    # Sequential execution
    echo "Step 1: Starting HTTP API server..."
    node http-api-server.js &
    API_PID=$!
    sleep 3
    
    echo "Step 2: Starting blockchain node..."
    ./start-node.sh &
    NODE_PID=$!
    sleep 15
    
    echo "Step 3: Setting up API key..."
    ./inject-api-key.sh my_secret_api_key_12345
    
    echo "Step 4: Inserting signing key..."
    ./insert-key.sh
    
    echo "Step 5: Running test..."
    npm test
    
    # Cleanup
    kill $API_PID $NODE_PID 2>/dev/null || true
    
else
    # tmux execution
    SESSION="offchain-worker-demo"
    
    # Kill existing session if it exists
    tmux kill-session -t $SESSION 2>/dev/null || true
    
    # Create new session
    tmux new-session -d -s $SESSION -n "API Server"
    
    # Window 1: API Server
    tmux send-keys -t $SESSION:0 "cd /home/chou/polkadot-confidential-offchain-worker" C-m
    tmux send-keys -t $SESSION:0 "npm run api" C-m
    
    # Window 2: Blockchain Node
    tmux new-window -t $SESSION -n "Blockchain"
    tmux send-keys -t $SESSION:1 "cd /home/chou/polkadot-confidential-offchain-worker" C-m
    tmux send-keys -t $SESSION:1 "sleep 3 && ./start-node.sh" C-m
    
    # Window 3: Setup and Test
    tmux new-window -t $SESSION -n "Test"
    tmux send-keys -t $SESSION:2 "cd /home/chou/polkadot-confidential-offchain-worker" C-m
    tmux send-keys -t $SESSION:2 "echo 'Waiting for services to start...'" C-m
    tmux send-keys -t $SESSION:2 "sleep 20" C-m
    tmux send-keys -t $SESSION:2 "echo ''" C-m
    tmux send-keys -t $SESSION:2 "echo '🔑 Injecting API key...'" C-m
    tmux send-keys -t $SESSION:2 "./inject-api-key.sh my_secret_api_key_12345" C-m
    tmux send-keys -t $SESSION:2 "echo ''" C-m
    tmux send-keys -t $SESSION:2 "echo '🔐 Inserting signing key...'" C-m
    tmux send-keys -t $SESSION:2 "./insert-key.sh" C-m
    tmux send-keys -t $SESSION:2 "echo ''" C-m
    tmux send-keys -t $SESSION:2 "echo '🚀 Running test...'" C-m
    tmux send-keys -t $SESSION:2 "npm test" C-m
    
    echo "✅ Started in tmux session: $SESSION"
    echo ""
    echo "To view the demo:"
    echo "  tmux attach -t $SESSION"
    echo ""
    echo "Navigate between windows:"
    echo "  Ctrl+B then 0 - API Server"
    echo "  Ctrl+B then 1 - Blockchain Node"
    echo "  Ctrl+B then 2 - Test Output"
    echo ""
    echo "To exit:"
    echo "  Ctrl+B then d (detach)"
    echo "  tmux kill-session -t $SESSION (kill)"
    echo ""
    
    # Attach to session
    tmux attach -t $SESSION
fi

