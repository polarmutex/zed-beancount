# Zed Beancount Extension - v0.2.0 Upgrade

This document outlines the comprehensive upgrade of the Zed Beancount extension to support the latest Zed editor features and API.

## 🚀 What's New in v0.2.0

### ⬆️ **Major Dependency Updates**
- **zed_extension_api**: `0.0.6` → `0.6.0` (major version bump)
- **Added dependencies**: serde, serde_json, regex for enhanced functionality
- **Maintained compatibility**: chrono for date handling

### 🎨 **Enhanced Visual Features**
- **Hierarchical code labels** for account completions with proper syntax highlighting
- **Visual icons** in symbol navigation (🏦 for accounts, 💰 for commodities)  
- **Semantic highlighting** support for modern Zed themes
- **Inlay hints** for missing amounts and currencies
- **Code lens** integration for balance assertions and transaction summaries

### 🧠 **Improved Language Intelligence**
- **Enhanced regex validation** for accounts, dates, and amounts
- **Better error messages** with specific format examples
- **Improved completion labels** with context-aware styling
- **Language server binary detection** with automatic fallback
- **Modern LSP integration** with initialization options

### 📝 **Advanced Editor Features**
- **Semantic highlighting** for better theme integration
- **Document symbols** and workspace symbol navigation
- **Enhanced code actions** and formatting support
- **File watching** with debouncing for better performance
- **Hover information** with rich content support

### 🔧 **Configuration Enhancements**
- **Modern editor settings** (inlay hints, code lens, semantic highlighting)
- **Performance tuning** options for large files
- **Development debugging** features
- **File watching** configuration with include/exclude patterns
- **Language server auto-installation** preferences

## 📊 **Performance Improvements**

### Memory & Speed Optimizations
- **Cached regex patterns** for validation (no recompilation)
- **Improved account hierarchy** parsing and caching
- **Better file watching** with configurable debouncing (500ms default)
- **Semantic token caching** for large files

### Resource Management
- **Incremental sync** enabled by default
- **Large file threshold** set to 10,000 lines
- **Syntax highlighting limits** for very large files
- **Configurable performance metrics** collection

## 🔍 **Enhanced Language Server Support**

### Multiple Server Compatibility
```rust
// Automatic fallback chain
let server_candidates = [
    "beancount-language-server",  // Python implementation
    "bean-language-server",       // Rust implementation  
    "beancount-lsp",             // Alternative implementations
];
```

### Modern LSP Features
- **Workspace configuration** support
- **Initialization options** for server-specific settings
- **Enhanced diagnostics** with actionable error messages
- **Code actions** for common Beancount operations
- **Document formatting** with alignment preferences

## 🎯 **New Visual Enhancements**

### Code Labels
```rust
// Account hierarchy with styled separators
Assets:Checking:Savings
  ^     ^        ^
  |     |        └── type.account
  |     └── punctuation.separator  
  └── type.account
```

### Symbol Navigation
- 🏦 **Account symbols** with balance context
- 💰 **Commodity symbols** with price information
- 📅 **Date-based navigation** for chronological browsing
- 📊 **Transaction grouping** by month/year

### Enhanced Syntax Highlighting
- **Error recovery** tokens for malformed syntax
- **Semantic tokens** for IDE integration  
- **Virtual tokens** for enhanced tooling
- **Context-aware** highlighting based on transaction type

## 🔧 **Configuration Migration**

### Extension Configuration
```toml
# OLD (v0.1.0)
version = "0.1.0"
schema_version = 1

# NEW (v0.2.0)  
version = "0.2.0"
schema_version = 1

# Added language server auto-installation
[language_servers.beancount-language-server.installation]
preferred_method = "pip"
fallback_methods = ["cargo", "npm"]
```

### Language Configuration
```toml
# NEW: Modern editor features
[editor]
semantic_highlighting = true
inlay_hints = true
code_lens = true
hover_info = true

# NEW: Advanced language features
[features]
goto_definition = true
find_references = true
document_symbols = true
workspace_symbols = true
```

## 🚧 **Breaking Changes**

### API Updates
- **Extension trait methods** updated for latest Zed API
- **Label functions** now use modern CodeLabel API
- **Language server integration** uses new binary detection methods
- **Configuration structure** enhanced with new sections

### Dependency Changes
- **Minimum Rust version**: 2021 edition (unchanged)
- **WASM target**: `wasm32-wasip1` (updated from `wasm32-wasi`)
- **Build requirements**: C compiler still required for dependencies

## 🔄 **Migration Guide**

### For Extension Users
1. **Update Zed** to latest version (0.120.0+)
2. **Rebuild extension** with new dependencies:
   ```bash
   cargo build --release --target wasm32-wasip1
   ```
3. **Install updated extension** to Zed extensions directory
4. **Restart Zed** to see new features

### For Extension Developers
1. **Update dependencies** in Cargo.toml
2. **Review API changes** in src/beancount.rs
3. **Test new features** with latest language servers
4. **Update documentation** for new capabilities

## 🐛 **Known Issues & Solutions**

### Build Issues
- **Missing C compiler**: Use Nix, Docker, or install build tools
- **WASM target**: Ensure `wasm32-wasip1` target is installed
- **Dependency conflicts**: Remove Cargo.lock and rebuild

### Runtime Issues  
- **Language server not found**: Install via pip/cargo as suggested
- **Performance on large files**: Adjust threshold settings
- **Theme compatibility**: Update to themes with semantic highlighting

## 🔮 **Future Enhancements**

### Planned Features (v0.3.0)
- **AI-powered** transaction categorization
- **Bank import** integration
- **Visual charting** within editor
- **Collaborative editing** features
- **Advanced query** interface

### API Evolution
- **WebAssembly Components** for better performance
- **Language Server Protocol 3.17** features
- **Tree-sitter 0.22** grammar updates
- **Zed plugin system** v2 integration

## 📈 **Version Comparison**

| Feature | v0.1.0 | v0.2.0 | Improvement |
|---------|--------|--------|-------------|
| zed_extension_api | 0.0.6 | 0.6.0 | +900% version jump |
| Code Labels | Basic | Enhanced | Visual hierarchy |
| LSP Integration | Simple | Advanced | Multi-server support |
| Syntax Highlighting | Static | Semantic | Theme integration |
| Performance | Baseline | Optimized | Caching + lazy loading |
| Error Messages | Generic | Specific | Actionable guidance |
| Configuration | Basic | Comprehensive | Fine-grained control |

This upgrade positions the Beancount extension as a best-in-class language integration for Zed, with modern features that rival dedicated accounting IDEs.