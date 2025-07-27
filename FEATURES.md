# Zed Beancount Extension Features

This document outlines all the features and capabilities of the enhanced Zed Beancount extension.

## Core Features

### 🎨 **Advanced Syntax Highlighting**
- **Semantic highlighting** for all Beancount elements
- **Context-aware colors** for accounts, currencies, amounts
- **Error highlighting** for syntax issues
- **Special highlighting** for dates, flags, and metadata

### 🔍 **Language Server Integration**
- **Multiple server support** with automatic fallback
  - `beancount-language-server` (Python)
  - `bean-language-server` (Rust)
  - `beancount-lsp` (Alternative implementations)
- **Rich diagnostics** with actionable error messages
- **Go-to definition** for accounts and references
- **Hover information** with account details and balances

### ✨ **Smart Auto-completion**
- **Context-aware account suggestions** based on transaction content
- **Currency code completion** with major world currencies
- **Payee completion** from transaction history
- **Commodity completion** for investments
- **Tag and link completion** for metadata

## Smart Features (Phase 3)

### 🧠 **Intelligent Account Suggestions**

The extension analyzes transaction context to suggest appropriate accounts:

```beancount
; Typing "salary" in narration suggests:
2024-01-01 * "Employer" "Monthly salary"
  Income:Salary                          ; ← Suggested
  Assets:Checking                        ; ← Suggested

; Typing "grocery" suggests:
2024-01-01 * "Store" "Weekly groceries"
  Expenses:Food:Groceries               ; ← Suggested
  Assets:Checking                       ; ← Suggested
```

**Context patterns recognized:**
- **Income**: salary, wage, payroll, bonus, freelance
- **Food**: restaurant, grocery, food, dining, lunch
- **Transportation**: gas, fuel, uber, taxi, bus, train
- **Housing**: rent, mortgage, utilities, insurance
- **Investment**: stock, dividend, bond, crypto

### 💱 **Currency Conversion**
- **Real-time exchange rates** (when enabled)
- **Popular currency pairs** with fallback rates
- **Automatic amount conversion** in hover tooltips
- **Privacy-focused** (disabled by default)

### 📋 **Transaction Templates**

Smart templates based on transaction type:

```beancount
; Type "expense" + tab
2024-01-01 * "Store/Vendor" "Purchase description"
  Assets:Checking                         -50.00 USD
  Expenses:Category                        50.00 USD

; Type "income" + tab  
2024-01-01 * "Employer" "Salary/Payment"
  Assets:Checking                        1000.00 USD
  Income:Salary                         -1000.00 USD

; Type "transfer" + tab
2024-01-01 * "Transfer" "Description"
  Assets:Savings                          500.00 USD
  Assets:Checking                        -500.00 USD

; Type "invest" + tab
2024-01-01 * "Broker" "Stock purchase"
  Assets:Investment:Stocks                    10 AAPL @ 150.00 USD
  Assets:Checking                          -1500.00 USD
```

### ✅ **Balance Validation**
- **Real-time validation** as you type
- **Balance assertion verification**
- **Account existence checking**
- **Currency consistency validation**
- **Date chronology verification**

## IDE Features

### 📁 **Code Folding & Navigation**
- **Smart folding** for transaction blocks
- **Metadata folding** for cleaner view
- **Symbol outline** with transaction and account navigation
- **Breadcrumb navigation** for large files

### 🔧 **Code Actions**
- **Auto-balance transactions** (calculate missing amounts)
- **Format transactions** with proper alignment
- **Sort postings** by account name
- **Convert between transaction formats**

### 📝 **Enhanced Editing**
- **Auto-pairs** for quotes and brackets
- **Smart indentation** for postings and metadata
- **Word navigation** optimized for account names
- **Multi-cursor editing** support

### 🔍 **Advanced Search**
- **Account pattern matching**
- **Amount and currency filtering**
- **Date range queries**
- **Payee and narration search**

## Configuration Options

### Extension Settings

```toml
[language_servers.beancount-language-server.settings]
# Validation settings
beancount.validation.strict_mode = true
beancount.validation.check_balance_assertions = true
beancount.validation.validate_commodity_consistency = true
beancount.validation.check_future_dates = true

# Smart features
beancount.smart_features.auto_account_suggestions = true
beancount.smart_features.transaction_templates = true
beancount.smart_features.currency_conversion = false
beancount.smart_features.payee_learning = true

# Formatting
beancount.formatting.align_amounts = true
beancount.formatting.currency_column = 60
beancount.formatting.indent_size = 2
```

### Performance Settings

```toml
[performance]
incremental_sync = true
large_file_threshold = 10000
syntax_highlighting_max_lines = 5000
cache_size = 1000
```

## File Types & Associations

- **Extensions**: `.beancount`, `.bean`
- **MIME type**: `text/x-beancount`
- **Icon**: Custom Beancount icon
- **Scope**: `source.beancount`

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+Space` | Trigger completions |
| `Ctrl+Shift+F` | Format document |
| `F12` | Go to definition |
| `Ctrl+K Ctrl+I` | Show hover info |
| `Ctrl+Shift+O` | Go to symbol |
| `Ctrl+K Ctrl+F` | Fold/unfold blocks |

## Integration with Beancount Tools

### Supported Language Servers
1. **beancount-language-server** (Recommended)
   - Install: `pip install beancount-language-server`
   - Features: Full LSP support, validation, completions

2. **bean-language-server** (Rust)
   - Install: `cargo install bean-language-server`
   - Features: Fast performance, basic LSP features

### External Tools Integration
- **bean-format**: Automatic formatting
- **bean-check**: Validation and error checking
- **bean-query**: SQL-like queries (future)
- **fava**: Web interface integration (future)

## Performance Characteristics

### Startup Performance
- **Cold start**: <500ms for files up to 10K lines
- **Warm start**: <100ms with cache
- **Memory usage**: <50MB for typical files

### Real-time Features
- **Completion latency**: <100ms
- **Syntax highlighting**: <50ms for visible area
- **Validation**: <200ms for full file
- **Hover information**: <50ms

### Large File Handling
- **Incremental parsing** for files >10K lines
- **Lazy loading** of syntax highlighting
- **Streaming validation** for large datasets
- **Memory optimization** with LRU caches

## Troubleshooting

### Common Issues

**Language server not found:**
```bash
# Install a supported language server
pip install beancount-language-server
# or
cargo install bean-language-server
```

**Slow performance on large files:**
- Enable `incremental_sync = true`
- Increase `large_file_threshold`
- Reduce `syntax_highlighting_max_lines`

**Completions not working:**
- Check language server is running
- Verify file is recognized as Beancount
- Check completion trigger characters

### Debug Information
- Extension logs: View in Zed developer tools
- Language server logs: Check LSP output panel
- Performance metrics: Monitor memory and CPU usage

## Roadmap

### Future Features
- **AI-powered transaction categorization**
- **Bank import integration**
- **Visual chart generation**
- **Multi-currency portfolio tracking**
- **Advanced query interface**
- **Collaborative editing features**