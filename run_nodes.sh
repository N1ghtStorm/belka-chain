#!/bin/bash

# Script to run two nodes for ping-pong testing

echo "Building project..."
cargo build

echo ""
echo "Starting Node 1 on port 18080..."
cargo run -- 18080 &
NODE1_PID=$!

sleep 2

echo ""
echo "Starting Node 2 on port 18081..."
cargo run -- 18081 &
NODE2_PID=$!

echo ""
echo "Nodes started!"
echo "Node 1 PID: $NODE1_PID"
echo "Node 2 PID: $NODE2_PID"
echo ""
echo "Press Ctrl+C to stop all nodes"

# Wait for interrupt
trap "kill $NODE1_PID $NODE2_PID 2>/dev/null; exit" INT
wait

