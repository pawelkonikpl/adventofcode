#!/bin/bash

# Installation script for MCP server
# Skrypt instalacyjny dla serwera MCP

set -e

echo "=== MCP Server Installation ==="
echo "=== Instalacja serwera MCP ==="
echo ""

# Build the server
echo "1. Building server in release mode..."
echo "1. Kompilowanie serwera w trybie release..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "Build failed!"
    echo "Kompilacja nie powiodła się!"
    exit 1
fi

echo "✓ Build successful!"
echo "✓ Kompilacja zakończona pomyślnie!"
echo ""

# Get the absolute path to the binary
BINARY_PATH="$(cd "$(dirname "$0")" && pwd)/target/release/mcp_server"

echo "2. Binary location:"
echo "2. Lokalizacja pliku binarnego:"
echo "   $BINARY_PATH"
echo ""

# Detect OS and show config file location
echo "3. Claude Desktop configuration:"
echo "3. Konfiguracja Claude Desktop:"
echo ""

if [[ "$OSTYPE" == "darwin"* ]]; then
    CONFIG_PATH="$HOME/Library/Application Support/Claude/claude_desktop_config.json"
    echo "   macOS detected"
    echo "   Wykryto macOS"
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    CONFIG_PATH="$APPDATA/Claude/claude_desktop_config.json"
    echo "   Windows detected"
    echo "   Wykryto Windows"
else
    CONFIG_PATH="$HOME/.config/Claude/claude_desktop_config.json"
    echo "   Linux detected"
    echo "   Wykryto Linux"
fi

echo "   Config file: $CONFIG_PATH"
echo "   Plik konfiguracyjny: $CONFIG_PATH"
echo ""

# Generate config snippet
echo "4. Add this to your Claude Desktop config:"
echo "4. Dodaj to do konfiguracji Claude Desktop:"
echo ""
cat <<EOF
{
  "mcpServers": {
    "rust-mcp-server": {
      "command": "$BINARY_PATH",
      "args": []
    }
  }
}
EOF
echo ""

echo "5. After adding the configuration:"
echo "5. Po dodaniu konfiguracji:"
echo "   - Restart Claude Desktop"
echo "   - Uruchom ponownie Claude Desktop"
echo "   - The MCP tools should be available"
echo "   - Narzędzia MCP powinny być dostępne"
echo ""

echo "Installation complete!"
echo "Instalacja zakończona!"
