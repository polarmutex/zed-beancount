// Test module for Beancount extension
use std::collections::HashMap;

// Mock implementation of BeancountExtension for testing
struct BeancountExtension {
    account_cache: HashMap<String, Vec<String>>,
    currency_cache: Vec<String>,
    payee_cache: Vec<String>,
}

impl Default for BeancountExtension {
    fn default() -> Self {
        Self {
            account_cache: HashMap::new(),
            currency_cache: vec![
                "USD".to_string(), "EUR".to_string(), "GBP".to_string(), 
                "JPY".to_string(), "CAD".to_string(), "AUD".to_string(),
                "BTC".to_string(), "ETH".to_string(), "AAPL".to_string(),
            ],
            payee_cache: Vec::new(),
        }
    }
}

impl BeancountExtension {
    fn suggest_accounts(&self, context: &str) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        if context.contains("salary") || context.contains("wage") || context.contains("payroll") {
            suggestions.extend_from_slice(&[
                "Income:Salary".to_string(),
                "Assets:Checking".to_string(),
            ]);
        }
        
        if context.contains("food") || context.contains("restaurant") || context.contains("grocery") {
            suggestions.extend_from_slice(&[
                "Expenses:Food:Restaurant".to_string(),
                "Expenses:Food:Groceries".to_string(),
                "Assets:Checking".to_string(),
            ]);
        }
        
        if context.contains("gas") || context.contains("fuel") || context.contains("transportation") {
            suggestions.extend_from_slice(&[
                "Expenses:Transportation:Gas".to_string(),
                "Assets:Checking".to_string(),
            ]);
        }
        
        if context.contains("stock") || context.contains("investment") || context.contains("dividend") {
            suggestions.extend_from_slice(&[
                "Assets:Investment:Stocks".to_string(),
                "Income:Dividends".to_string(),
                "Assets:Checking".to_string(),
            ]);
        }
        
        if suggestions.is_empty() {
            suggestions.extend_from_slice(&[
                "Assets:Checking".to_string(),
                "Expenses:Miscellaneous".to_string(),
                "Income:Other".to_string(),
            ]);
        }
        
        suggestions
    }
    
    fn convert_currency(&self, amount: f64, from: &str, to: &str) -> Option<f64> {
        match (from, to) {
            ("USD", "EUR") => Some(amount * 0.85),
            ("EUR", "USD") => Some(amount * 1.18),
            ("USD", "GBP") => Some(amount * 0.73),
            ("GBP", "USD") => Some(amount * 1.37),
            _ => None,
        }
    }
    
    fn generate_transaction_template(&self, payee: &str, narration: &str) -> String {
        let accounts = self.suggest_accounts(&format!("{} {}", payee, narration));
        let account1 = accounts.get(0).unwrap_or(&"Assets:Checking".to_string());
        let account2 = accounts.get(1).unwrap_or(&"Expenses:Miscellaneous".to_string());
        
        format!(
            r#"2024-01-01 * "{}" "{}"
  {}  
  {} "#,
            payee, narration, account1, account2
        )
    }
    
    fn validate_beancount_file(&self, content: &str) -> Vec<String> {
        let mut errors = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        
        for (i, line) in lines.iter().enumerate() {
            let line_num = i + 1;
            let trimmed = line.trim();
            
            if trimmed.is_empty() || trimmed.starts_with(';') {
                continue;
            }
            
            if trimmed.starts_with("202") && trimmed.len() >= 10 {
                if !trimmed.contains('*') && !trimmed.contains('!') {
                    errors.push(format!("Line {}: Missing transaction flag (* or !)", line_num));
                }
                
                let date_part = &trimmed[..10];
                if date_part.matches('-').count() != 2 {
                    errors.push(format!("Line {}: Invalid date format. Use YYYY-MM-DD", line_num));
                }
            }
            
            let quote_count = trimmed.matches('"').count();
            if quote_count % 2 != 0 {
                errors.push(format!("Line {}: Unbalanced quotes", line_num));
            }
            
            if trimmed.contains(':') && !trimmed.starts_with("  ") && !trimmed.starts_with("option") {
                if let Some(account_start) = trimmed.find(char::is_alphabetic) {
                    let account_end = trimmed[account_start..].find(char::is_whitespace).unwrap_or(trimmed.len() - account_start);
                    let account = &trimmed[account_start..account_start + account_end];
                    
                    if !account.chars().next().unwrap_or('a').is_uppercase() {
                        errors.push(format!("Line {}: Account name '{}' should start with uppercase letter", line_num, account));
                    }
                    
                    if account.chars().any(|c| !c.is_alphanumeric() && c != ':' && c != '-') {
                        errors.push(format!("Line {}: Account name '{}' contains invalid characters", line_num, account));
                    }
                }
            }
        }
        
        errors
    }
    
    fn update_cache(&mut self, content: &str) {
        let mut accounts = Vec::new();
        let mut payees = Vec::new();
        
        for line in content.lines() {
            let trimmed = line.trim();
            
            if trimmed.starts_with("  ") && trimmed.contains(':') {
                if let Some(account_end) = trimmed.find(char::is_whitespace) {
                    let account = trimmed[2..account_end].trim();
                    if !accounts.contains(&account.to_string()) {
                        accounts.push(account.to_string());
                    }
                }
            }
            
            if trimmed.contains('*') || trimmed.contains('!') {
                if let Some(start) = trimmed.find('"') {
                    if let Some(end) = trimmed[start + 1..].find('"') {
                        let payee = &trimmed[start + 1..start + 1 + end];
                        if !payees.contains(&payee.to_string()) && !payee.is_empty() {
                            payees.push(payee.to_string());
                        }
                    }
                }
            }
        }
        
        self.account_cache.insert("recent".to_string(), accounts);
        self.payee_cache = payees;
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_account_suggestions() {
        let extension = BeancountExtension::default();
        
        // Test salary context
        let suggestions = extension.suggest_accounts("Monthly salary from employer");
        assert!(suggestions.contains(&"Income:Salary".to_string()));
        assert!(suggestions.contains(&"Assets:Checking".to_string()));
        
        // Test food context
        let suggestions = extension.suggest_accounts("Dinner at restaurant");
        assert!(suggestions.contains(&"Expenses:Food:Restaurant".to_string()));
        
        // Test transportation context
        let suggestions = extension.suggest_accounts("Gas station fill up");
        assert!(suggestions.contains(&"Expenses:Transportation:Gas".to_string()));
        
        // Test investment context
        let suggestions = extension.suggest_accounts("Buy stocks and shares");
        assert!(suggestions.contains(&"Assets:Investment:Stocks".to_string()));
        assert!(suggestions.contains(&"Income:Dividends".to_string()));
    }

    #[test]
    fn test_currency_conversion() {
        let extension = BeancountExtension::default();
        
        // Test USD to EUR conversion
        let result = extension.convert_currency(100.0, "USD", "EUR");
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 85.0);
        
        // Test EUR to USD conversion
        let result = extension.convert_currency(100.0, "EUR", "USD");
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 118.0);
        
        // Test unsupported conversion
        let result = extension.convert_currency(100.0, "XYZ", "ABC");
        assert!(result.is_none());
    }

    #[test]
    fn test_transaction_template_generation() {
        let extension = BeancountExtension::default();
        
        let template = extension.generate_transaction_template("Grocery Store", "Weekly shopping");
        
        // Should contain proper structure
        assert!(template.contains("* \"Grocery Store\""));
        assert!(template.contains("\"Weekly shopping\""));
        assert!(template.contains("Assets:Checking"));
        assert!(template.contains("Expenses:Food:Groceries"));
        
        // Should have proper date format (YYYY-MM-DD)
        assert!(template.starts_with("202")); // Should start with current year
    }

    #[test]
    fn test_file_validation() {
        let extension = BeancountExtension::default();
        
        // Test valid content
        let valid_content = r#"
2024-01-01 * "Test" "Valid transaction"
  Assets:Checking  100.00 USD
  Expenses:Test
"#;
        let errors = extension.validate_beancount_file(valid_content);
        assert!(errors.is_empty());
        
        // Test invalid date format
        let invalid_date = r#"
24-01-01 * "Test" "Invalid date"
  Assets:Checking  100.00 USD
  Expenses:Test
"#;
        let errors = extension.validate_beancount_file(invalid_date);
        assert!(!errors.is_empty());
        
        // Test missing transaction flag
        let missing_flag = r#"
2024-01-01 "Test" "Missing flag"
  Assets:Checking  100.00 USD
  Expenses:Test
"#;
        let errors = extension.validate_beancount_file(missing_flag);
        assert!(!errors.is_empty());
        
        // Test unbalanced quotes
        let unbalanced_quotes = r#"
2024-01-01 * "Test "Unbalanced quotes"
  Assets:Checking  100.00 USD
  Expenses:Test
"#;
        let errors = extension.validate_beancount_file(unbalanced_quotes);
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_cache_functionality() {
        let mut extension = BeancountExtension::default();
        
        let content = r#"
2024-01-01 * "Employer" "Salary payment"
  Assets:Checking  2000.00 USD
  Income:Salary

2024-01-02 * "Store" "Groceries"
  Assets:Checking  -50.00 USD
  Expenses:Food:Groceries

2024-01-03 * "Gas Station" "Fuel"
  Assets:Checking  -40.00 USD
  Expenses:Transportation:Gas
"#;
        
        extension.update_cache(content);
        
        // Check that accounts were extracted
        if let Some(accounts) = extension.account_cache.get("recent") {
            assert!(accounts.contains(&"Assets:Checking".to_string()));
            assert!(accounts.contains(&"Income:Salary".to_string()));
            assert!(accounts.contains(&"Expenses:Food:Groceries".to_string()));
            assert!(accounts.contains(&"Expenses:Transportation:Gas".to_string()));
        }
        
        // Check that payees were extracted
        assert!(extension.payee_cache.contains(&"Employer".to_string()));
        assert!(extension.payee_cache.contains(&"Store".to_string()));
        assert!(extension.payee_cache.contains(&"Gas Station".to_string()));
    }

    #[test]
    fn test_currency_cache() {
        let extension = BeancountExtension::default();
        
        // Check that major currencies are pre-loaded
        assert!(extension.currency_cache.contains(&"USD".to_string()));
        assert!(extension.currency_cache.contains(&"EUR".to_string()));
        assert!(extension.currency_cache.contains(&"GBP".to_string()));
        assert!(extension.currency_cache.contains(&"BTC".to_string()));
        assert!(extension.currency_cache.contains(&"AAPL".to_string()));
    }

    #[test]
    fn test_account_validation() {
        let extension = BeancountExtension::default();
        
        // Test valid account names
        let valid_content = r#"
open Assets:Checking
open Expenses:Food:Restaurant
open Income:Salary:Base
"#;
        let errors = extension.validate_beancount_file(valid_content);
        // Should not produce account naming errors
        let account_errors: Vec<_> = errors.iter()
            .filter(|e| e.contains("Account name"))
            .collect();
        assert!(account_errors.is_empty());
        
        // Test invalid account names
        let invalid_content = r#"
open assets:checking
open Expenses:food@restaurant
"#;
        let errors = extension.validate_beancount_file(invalid_content);
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_smart_suggestions_edge_cases() {
        let extension = BeancountExtension::default();
        
        // Test empty context
        let suggestions = extension.suggest_accounts("");
        assert!(!suggestions.is_empty()); // Should provide default suggestions
        assert!(suggestions.contains(&"Assets:Checking".to_string()));
        
        // Test unknown context
        let suggestions = extension.suggest_accounts("unknown random context");
        assert!(suggestions.contains(&"Assets:Checking".to_string()));
        assert!(suggestions.contains(&"Expenses:Miscellaneous".to_string()));
        
        // Test multiple contexts
        let suggestions = extension.suggest_accounts("salary food transportation");
        // Should contain suggestions from all contexts
        assert!(suggestions.len() > 3);
    }
}