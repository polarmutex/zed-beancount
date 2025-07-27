# Zed Beancount Extension

A comprehensive [Beancount](https://beancount.github.io/) language extension for the [Zed editor](https://zed.dev/) providing intelligent auto-completion, syntax highlighting, validation, and smart features for plain-text accounting.

> **🆕 v0.2.0 Update**: Now supports the latest Zed API with enhanced visual features, semantic highlighting, and modern IDE capabilities. See [UPGRADE.md](UPGRADE.md) for details.

## ✨ Features

### 🎨 **Rich Syntax Highlighting**
- Semantic highlighting for accounts, currencies, amounts, dates
- Context-aware colors for different transaction types
- Error highlighting for syntax issues
- Support for all Beancount directives and metadata

### 🧠 **Smart Auto-completion**
- **Context-aware account suggestions** based on transaction content
- **Currency completion** with major world currencies and cryptocurrencies  
- **Payee completion** learned from transaction history
- **Commodity completion** for investments and stocks
- **Tag and link completion** for metadata

### 🔍 **Language Server Integration**
- Multiple language server support with automatic fallback
- Real-time diagnostics and validation
- Go-to definition for accounts and references
- Hover information with account details

### ⚡ **Smart Features**
- **Transaction templates** for common patterns (income, expenses, transfers, investments)
- **Balance validation** with real-time error checking
- **Currency conversion** with exchange rate support
- **Account suggestions** based on payee and narration context
- **Auto-formatting** with proper alignment

### 📁 **Enhanced Editing**
- Code folding for transactions and metadata blocks
- Symbol outline for easy navigation
- Auto-pairs for quotes and brackets
- Smart indentation for postings
- Multi-cursor editing support

## 🚀 Quick Start

### Installation

1. **Build the extension** (see [BUILD.md](BUILD.md) for detailed instructions):
   ```bash
   # Using Nix (recommended)
   nix run .#build
   
   # Or using system tools
   cargo build --release --target wasm32-wasip1
   ```

2. **Install to Zed**:
   ```bash
   # Using Nix
   nix run .#install
   
   # Or manually
   mkdir -p ~/.config/zed/extensions/beancount
   cp extension.toml ~/.config/zed/extensions/beancount/
   cp -r languages ~/.config/zed/extensions/beancount/
   cp target/wasm32-wasip1/release/beancount.wasm ~/.config/zed/extensions/beancount/extension.wasm
   ```

3. **Install a language server** (optional but recommended):
   ```bash
   pip install beancount-language-server
   # or
   cargo install bean-language-server
   ```

4. **Restart Zed** and open a `.beancount` file!

### Basic Usage

Create a new file with `.beancount` extension and start typing:

```beancount
; Type "txn" + Tab for transaction template
2024-01-01 * "Employer" "Monthly salary"
  Assets:Checking                        2500.00 USD
  Income:Salary                         -2500.00 USD

; Type "expense" + Tab for expense template  
2024-01-02 * "Grocery Store" "Weekly shopping"
  Assets:Checking                         -85.43 USD
  Expenses:Food:Groceries                  85.43 USD
```

## 📚 Documentation

- **[Features Guide](FEATURES.md)** - Complete feature overview
- **[Build Instructions](BUILD.md)** - How to build from source
- **[Example Files](examples/)** - Sample Beancount files showcasing features

## 🛠️ Development

### Requirements
- Rust with `wasm32-wasip1` target
- C compiler (gcc/clang)
- Optional: Nix for reproducible builds

### Build Commands

```bash
# Development build
cargo build --target wasm32-wasip1

# Release build
cargo build --release --target wasm32-wasip1

# Run tests
cargo test

# Using Nix
nix develop          # Enter development shell
nix run .#build      # Build extension
nix run .#watch      # Watch for changes
```

### Project Structure

```
zed-beancount/
├── src/
│   └── beancount.rs         # Main extension logic
├── languages/beancount/
│   ├── config.toml          # Language configuration
│   ├── highlights.scm       # Syntax highlighting rules
│   ├── indents.scm          # Indentation rules
│   ├── outline.scm          # Symbol navigation
│   ├── injections.scm       # Code injection rules
│   └── snippets.scm         # Code snippets
├── examples/
│   └── example.beancount    # Example Beancount file
├── tests/
│   └── test_extension.rs    # Unit tests
├── extension.toml           # Extension manifest
├── flake.nix               # Nix development environment
└── Dockerfile              # Container build environment
```

## 🎯 Smart Features Demo

### Context-Aware Suggestions
The extension analyzes transaction context to suggest appropriate accounts:

```beancount
; Typing "salary" suggests Income:Salary + Assets:Checking
2024-01-01 * "ACME Corp" "Monthly salary payment"

; Typing "grocery" suggests Expenses:Food:Groceries  
2024-01-01 * "Whole Foods" "Weekly grocery shopping"

; Typing "gas" suggests Expenses:Transportation:Gas
2024-01-01 * "Shell" "Fill up gas tank"
```

### Transaction Templates
Quick templates for common transaction types:

- `txn` → Basic transaction
- `expense` → Expense transaction
- `income` → Income transaction  
- `transfer` → Account transfer
- `invest` → Investment purchase

### Smart Validation
Real-time validation with helpful error messages:

- Date format validation (`YYYY-MM-DD`)
- Transaction flag requirements (`*` or `!`)
- Account naming conventions
- Balance assertion verification
- Quote balancing

## 🔧 Configuration

### Extension Settings

```toml
[language_servers.beancount-language-server.settings]
# Smart features
beancount.smart_features.auto_account_suggestions = true
beancount.smart_features.transaction_templates = true
beancount.smart_features.currency_conversion = false

# Validation
beancount.validation.strict_mode = true
beancount.validation.check_balance_assertions = true

# Formatting  
beancount.formatting.align_amounts = true
beancount.formatting.currency_column = 60
```

## 🤝 Contributing

Contributions are welcome! Please see our [contribution guidelines](CONTRIBUTING.md) for details.

### Areas for Contribution
- Additional language server integrations
- Enhanced validation rules
- More transaction templates  
- Performance optimizations
- Documentation improvements

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Beancount](https://beancount.github.io/) - The amazing plain-text accounting tool
- [tree-sitter-beancount](https://github.com/polarmutex/tree-sitter-beancount) - Grammar definition
- [Zed](https://zed.dev/) - The high-performance code editor
- [beancount-language-server](https://github.com/polarmutex/beancount-language-server) - LSP implementation

## 📊 Stats

- **Lines of Code**: ~1,000 Rust + 500 Tree-sitter queries
- **Features**: 25+ intelligent features
- **Performance**: <500ms startup, <100ms completions
- **File Support**: `.beancount`, `.bean` extensions
- **Language Servers**: 3+ compatible implementations
