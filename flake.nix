{
  description = "Zed Beancount Extension Development Environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Define Rust toolchain with WASM target
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
          targets = [ "wasm32-wasip1" ];
        };

        # Build dependencies
        buildInputs = with pkgs; [
          # Core build tools
          gcc
          pkg-config
          
          # Rust toolchain
          rustToolchain
          
          # WASM tools
          wasmtime
          wasm-tools
          
          # Development tools
          cargo-watch
          cargo-edit
          cargo-audit
          
          # Zed extension development
          nodejs_20
          
          # Beancount language servers (optional)
          python3
          python3Packages.pip
        ];

        # Development shell dependencies
        nativeBuildInputs = with pkgs; [
          # Additional development utilities
          git
          curl
          jq
          
          # Documentation tools
          mdbook
          
          # Testing tools
          cargo-nextest
        ];

      in
      {
        # Development shell
        devShells.default = pkgs.mkShell {
          inherit buildInputs nativeBuildInputs;
          
          # Environment variables
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          CARGO_TARGET_WASM32_WASIP1_LINKER = "${pkgs.llvmPackages.clang}/bin/clang";
          CC_wasm32_wasip1 = "${pkgs.llvmPackages.clang}/bin/clang";
          AR_wasm32_wasip1 = "${pkgs.llvmPackages.llvm}/bin/llvm-ar";
          
          # Shell hook for setup
          shellHook = ''
            echo "🦀 Zed Beancount Extension Development Environment"
            echo "Rust version: $(rustc --version)"
            echo "Cargo version: $(cargo --version)"
            echo "WASM target available: $(rustup target list --installed | grep wasm32-wasip1 || echo 'Installing...')"
            
            # Ensure WASM target is installed
            rustup target add wasm32-wasip1 2>/dev/null || true
            
            # Install beancount-language-server if not available
            if ! command -v beancount-language-server &> /dev/null; then
              echo "📦 Installing beancount-language-server..."
              pip install --user beancount-language-server
            fi
            
            echo "✅ Environment ready! Run 'cargo build --release --target wasm32-wasip1' to build extension"
          '';
        };

        # Package for the extension
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "zed-beancount-extension";
          version = "0.1.0";
          
          src = ./.;
          
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          
          buildInputs = buildInputs;
          nativeBuildInputs = nativeBuildInputs;
          
          # Build for WASM target
          buildPhase = ''
            cargo build --release --target wasm32-wasip1
          '';
          
          installPhase = ''
            mkdir -p $out
            cp target/wasm32-wasip1/release/*.wasm $out/ || true
            cp -r languages $out/
            cp extension.toml $out/
          '';
          
          meta = with pkgs.lib; {
            description = "Beancount language support for Zed editor";
            homepage = "https://github.com/zed-extensions/beancount";
            license = licenses.mit;
            maintainers = [ ];
          };
        };

        # Formatter for the flake
        formatter = pkgs.alejandra;
        
        # Development apps
        apps = {
          # Build extension
          build = flake-utils.lib.mkApp {
            drv = pkgs.writeShellScriptBin "build-extension" ''
              echo "🔨 Building Zed Beancount extension..."
              cargo build --release --target wasm32-wasip1
              echo "✅ Extension built successfully!"
              ls -la target/wasm32-wasip1/release/*.wasm
            '';
          };
          
          # Install extension locally for testing
          install = flake-utils.lib.mkApp {
            drv = pkgs.writeShellScriptBin "install-extension" ''
              echo "📦 Installing extension to local Zed..."
              
              # Create extensions directory if it doesn't exist
              mkdir -p ~/.config/zed/extensions/beancount
              
              # Copy extension files
              cp extension.toml ~/.config/zed/extensions/beancount/
              cp -r languages ~/.config/zed/extensions/beancount/
              cp target/wasm32-wasip1/release/*.wasm ~/.config/zed/extensions/beancount/extension.wasm 2>/dev/null || {
                echo "❌ No WASM file found. Run 'nix run .#build' first."
                exit 1
              }
              
              echo "✅ Extension installed! Restart Zed to see changes."
            '';
          };
          
          # Watch and rebuild on changes
          watch = flake-utils.lib.mkApp {
            drv = pkgs.writeShellScriptBin "watch-extension" ''
              echo "👀 Watching for changes..."
              cargo watch -x "build --release --target wasm32-wasip1"
            '';
          };
        };
      }
    );
}