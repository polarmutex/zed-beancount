# Building the Zed Beancount Extension

This document explains how to build the Zed Beancount extension from source.

## Prerequisites

You need:
- Rust toolchain with WASM support
- C compiler (gcc/clang)
- pkg-config

## Build Methods

### Option 1: Using Nix (Recommended)

If you have Nix installed:

```bash
# Using the flake (preferred)
nix develop --impure
cargo build --release --target wasm32-wasip1

# Or using the simple shell.nix
nix-shell
cargo build --release --target wasm32-wasip1

# Or using nix commands directly
nix run .#build
```

### Option 2: Using Docker

```bash
# Build using Docker (includes all dependencies)
docker build -t zed-beancount-build .
docker create --name temp-container zed-beancount-build
docker cp temp-container:/app/extension.wasm .
docker rm temp-container
```

### Option 3: System Dependencies

If you have a system with gcc and Rust installed:

```bash
# Add WASM target
rustup target add wasm32-wasip1

# Build
./build.sh
```

### Option 4: GitHub Codespaces / DevContainer

Use the provided development environment:

```bash
# In a codespace or devcontainer
cargo build --release --target wasm32-wasip1
```

## Installation

Once built, install the extension:

```bash
# Create extension directory
mkdir -p ~/.config/zed/extensions/beancount

# Copy files
cp extension.toml ~/.config/zed/extensions/beancount/
cp -r languages ~/.config/zed/extensions/beancount/
cp target/wasm32-wasip1/release/beancount.wasm ~/.config/zed/extensions/beancount/extension.wasm

# Restart Zed
```

## Nix Development Commands

The flake.nix provides several useful commands:

```bash
# Build extension
nix run .#build

# Install extension locally
nix run .#install

# Watch for changes and rebuild
nix run .#watch
```

## Troubleshooting

### Missing C Compiler

If you get "linker `cc` not found":
- On Ubuntu/Debian: `sudo apt install build-essential`
- On Fedora: `sudo dnf install gcc`
- On macOS: `xcode-select --install`
- On NixOS: Use the provided flake.nix

### WASM Target Not Found

```bash
rustup target add wasm32-wasip1
```

### Build Script Execution Issues

```bash
chmod +x build.sh
bash build.sh  # Use bash explicitly
```

## File Structure

After building, you should have:

```
zed-beancount/
├── target/wasm32-wasip1/release/beancount.wasm  # Main extension binary
├── extension.toml                               # Extension manifest
├── languages/                                   # Language configuration
│   └── beancount/
│       ├── config.toml                         # Language settings
│       ├── highlights.scm                      # Syntax highlighting
│       ├── indents.scm                         # Indentation rules
│       ├── outline.scm                         # Symbol outline
│       ├── injections.scm                      # Code injection
│       └── snippets.scm                        # Code snippets
└── src/beancount.rs                            # Extension source code
```

## Development Workflow

1. Make changes to source code
2. Run `cargo build --release --target wasm32-wasip1`
3. Copy WASM file to Zed extensions directory
4. Restart Zed to test changes

Or use the watch command for automatic rebuilding:
```bash
nix run .#watch  # Nix users
cargo watch -x "build --release --target wasm32-wasip1"  # With cargo-watch
```