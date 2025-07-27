#!/bin/bash

# Build script for Zed Beancount extension

set -e

echo "🦀 Building Zed Beancount Extension"

# Check if we have the WASM target
if ! rustup target list --installed | grep -q wasm32-wasip1; then
    echo "📦 Installing WASM target..."
    rustup target add wasm32-wasip1
fi

# Clean previous builds
echo "🧹 Cleaning previous builds..."
cargo clean

# Build the extension
echo "🔨 Building extension..."
cargo build --release --target wasm32-wasip1

# Check if build was successful
if [ -f target/wasm32-wasip1/release/beancount.wasm ]; then
    echo "✅ Extension built successfully!"
    echo "📄 WASM file: target/wasm32-wasip1/release/beancount.wasm"
    echo "📊 File size: $(ls -lh target/wasm32-wasip1/release/beancount.wasm | awk '{print $5}')"
else
    echo "❌ Build failed - no WASM file found"
    exit 1
fi

echo ""
echo "🚀 To install the extension:"
echo "1. Copy the files to your Zed extensions directory:"
echo "   mkdir -p ~/.config/zed/extensions/beancount"
echo "   cp extension.toml ~/.config/zed/extensions/beancount/"
echo "   cp -r languages ~/.config/zed/extensions/beancount/"
echo "   cp target/wasm32-wasip1/release/beancount.wasm ~/.config/zed/extensions/beancount/extension.wasm"
echo "2. Restart Zed"