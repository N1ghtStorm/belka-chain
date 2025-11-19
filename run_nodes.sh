#!/bin/bash

echo "Building project..."
cargo build

echo ""
echo "Starting Node 1 on port 8080..."
cargo run -r -- --port 18080 &
NODE1_PID=$!

sleep 2

echo ""
echo "Starting Node 2 on port 18081 (connecting to Node 1)..."
cargo run -r -- --port 18081 --peer 127.0.0.1:18080 &
NODE2_PID=$!

echo ""
echo "Nodes started!"
echo "Node 1 PID: $NODE1_PID"
echo "Node 2 PID: $NODE2_PID"
echo ""
echo "Press Ctrl+C to stop all nodes"

trap "kill $NODE1_PID $NODE2_PID 2>/dev/null; exit" INT
wait

