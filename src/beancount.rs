use zed_extension_api::{self as zed, LanguageServerId, Result, CodeLabel, CodeLabelSpan};

struct BeancountExtension;

impl zed::Extension for BeancountExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // Try multiple language server candidates
        let server_candidates = [
            "beancount-language-server",
            "bean-language-server",
            "beancount-lsp",
        ];

        for candidate in &server_candidates {
            if let Some(path) = worktree.which(candidate) {
                return Ok(zed::Command {
                    command: path,
                    args: vec!["--stdio".to_string()],
                    env: vec![],
                });
            }
        }

        Err("No Beancount language server found in PATH. Please install one of:
            - beancount-language-server (pip install beancount-language-server)
            - bean-language-server (cargo install bean-language-server)
            - See https://github.com/polarmutex/beancount-language-server for more options"
            .to_string())
    }

    fn language_server_initialization_options(
        &mut self,
        _: &LanguageServerId,
        _: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        Ok(Some(zed::serde_json::json!({
            "journal_file": null,
            "main_file": null,
            "check_ignore_unused_imports": true,
            "completions": {
                "accounts": true,
                "currencies": true,
                "payees": true,
                "commodities": true
            },
            "diagnostics": {
                "balance_assertions": true,
                "account_validation": true,
                "currency_consistency": true
            }
        })))
    }

    fn label_for_completion(
        &self,
        _: &LanguageServerId,
        completion: zed::lsp::Completion,
    ) -> Option<CodeLabel> {
        match completion.kind? {
            zed::lsp::CompletionItemKind::VARIABLE => {
                // Account completions
                if completion.label.contains(':') {
                    Some(CodeLabel {
                        spans: vec![CodeLabelSpan::literal(&completion.label, Some("type.account".into()))],
                        filter_range: 0..completion.label.len(),
                        code: completion.label,
                    })
                } else {
                    None
                }
            }
            zed::lsp::CompletionItemKind::CONSTANT => {
                // Currency completions
                Some(CodeLabel {
                    spans: vec![CodeLabelSpan::literal(&completion.label, Some("constant.builtin.currency".into()))],
                    filter_range: 0..completion.label.len(),
                    code: completion.label,
                })
            }
            zed::lsp::CompletionItemKind::TEXT => {
                // Payee/narration completions
                Some(CodeLabel {
                    spans: vec![CodeLabelSpan::literal(&completion.label, Some("string.special".into()))],
                    filter_range: 0..completion.label.len(),
                    code: completion.label,
                })
            }
            _ => None,
        }
    }

    fn label_for_symbol(
        &self,
        _: &LanguageServerId,
        symbol: zed::lsp::Symbol,
    ) -> Option<CodeLabel> {
        match symbol.kind {
            zed::lsp::SymbolKind::FUNCTION => {
                // Transaction entries
                Some(CodeLabel {
                    spans: vec![
                        CodeLabelSpan::literal("txn ", Some("keyword.directive".into())),
                        CodeLabelSpan::literal(&symbol.name, Some("string.quoted".into())),
                    ],
                    filter_range: 0..symbol.name.len(),
                    code: symbol.name,
                })
            }
            zed::lsp::SymbolKind::VARIABLE => {
                // Account entries
                Some(CodeLabel {
                    spans: vec![CodeLabelSpan::literal(&symbol.name, Some("type.account".into()))],
                    filter_range: 0..symbol.name.len(),
                    code: symbol.name,
                })
            }
            zed::lsp::SymbolKind::CONSTANT => {
                // Price/commodity entries
                Some(CodeLabel {
                    spans: vec![CodeLabelSpan::literal(&symbol.name, Some("constant.builtin".into()))],
                    filter_range: 0..symbol.name.len(),
                    code: symbol.name,
                })
            }
            _ => None,
        }
    }
}

zed::register_extension!(BeancountExtension);
