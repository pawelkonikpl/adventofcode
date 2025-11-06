#!/bin/bash

# Test script for MCP server
# Skrypt testowy dla serwera MCP

echo "Building MCP server..."
echo "Kompilowanie serwera MCP..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "Build failed!"
    echo "Kompilacja nie powiodła się!"
    exit 1
fi

echo ""
echo "Testing MCP server with sample requests..."
echo "Testowanie serwera MCP z przykładowymi żądaniami..."
echo ""

# Run the server with test requests
cat test_requests.jsonl | cargo run 2>&1 | grep -v "^MCP Server" | grep -v "Received:" | grep -v "Sent:"

echo ""
echo "Test completed!"
echo "Test zakończony!"
