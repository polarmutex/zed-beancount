{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Core build tools
    gcc
    pkg-config
    
    # Rust toolchain
    rustc
    cargo
    
    # WASM tools
    wasmtime
    wasm-tools
    
    # Development tools
    git
    curl
  ];

  shellHook = ''
    echo "🦀 Simple Zed Beancount Extension Build Environment"
    echo "Rust version: $(rustc --version)"
    echo "Cargo version: $(cargo --version)"
    
    # Ensure WASM target is installed
    rustup target add wasm32-wasip1 2>/dev/null || true
    
    echo "✅ Environment ready! Run 'cargo build --release --target wasm32-wasip1' to build extension"
  '';
}