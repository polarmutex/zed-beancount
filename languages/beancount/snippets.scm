; Snippets for common Beancount patterns

; Smart transaction snippets based on context

; Basic transaction snippet
((identifier) @_start
 (#match? @_start "^txn$")
 (completion
  "txn" @name
  "Transaction"
  "${1:2024-01-01} * \"${2:Payee}\" \"${3:Narration}\"\n  ${4:Assets:Checking}                        ${5:100.00} ${6:USD}\n  ${7:Expenses:Miscellaneous}$0" @template))

; Expense transaction templates
((identifier) @_start
 (#match? @_start "^expense$")
 (completion
  "expense" @name
  "Expense Transaction"
  "${1:2024-01-01} * \"${2:Store/Vendor}\" \"${3:Purchase description}\"\n  Assets:Checking                         -${4:50.00} ${5:USD}\n  Expenses:${6:Category}                   ${4:50.00} ${5:USD}$0" @template))

; Income transaction template
((identifier) @_start
 (#match? @_start "^income$")
 (completion
  "income" @name
  "Income Transaction"
  "${1:2024-01-01} * \"${2:Employer}\" \"${3:Salary/Payment}\"\n  Assets:Checking                         ${4:1000.00} ${5:USD}\n  Income:${6:Salary}                      -${4:1000.00} ${5:USD}$0" @template))

; Transfer between accounts
((identifier) @_start
 (#match? @_start "^transfer$")
 (completion
  "transfer" @name
  "Account Transfer"
  "${1:2024-01-01} * \"Transfer\" \"${2:Description}\"\n  ${3:Assets:Savings}                     ${4:500.00} ${5:USD}\n  ${6:Assets:Checking}                    -${4:500.00} ${5:USD}$0" @template))

; Investment purchase
((identifier) @_start
 (#match? @_start "^invest$")
 (completion
  "invest" @name
  "Investment Purchase"
  "${1:2024-01-01} * \"${2:Broker}\" \"${3:Stock purchase}\"\n  Assets:Investment:${4:Stocks}           ${5:10} ${6:AAPL} @ ${7:150.00} USD\n  Assets:Checking                         -${8:1500.00} USD$0" @template))

; Open account snippet  
((identifier) @_start
 (#match? @_start "^open$")
 (completion
  "open" @name
  "Open Account"
  "2024-01-01 open ${1:Assets:Checking} ${2:USD}$0" @template))

; Close account snippet
((identifier) @_start
 (#match? @_start "^close$")
 (completion
  "close" @name
  "Close Account"
  "2024-01-01 close ${1:Assets:Checking}$0" @template))

; Balance assertion snippet
((identifier) @_start
 (#match? @_start "^balance$")
 (completion
  "balance" @name
  "Balance Assertion"
  "2024-01-01 balance ${1:Assets:Checking} ${2:1000.00} ${3:USD}$0" @template))

; Price entry snippet
((identifier) @_start
 (#match? @_start "^price$")
 (completion
  "price" @name
  "Price Entry"
  "2024-01-01 price ${1:AAPL} ${2:150.00} ${3:USD}$0" @template))

; Option snippet
((identifier) @_start
 (#match? @_start "^option$")
 (completion
  "option" @name
  "Option Setting"
  "option \"${1:title}\" \"${2:My Beancount File}\"$0" @template))

; Include snippet
((identifier) @_start
 (#match? @_start "^include$")
 (completion
  "include" @name
  "Include File"
  "include \"${1:accounts.beancount}\"$0" @template))

; Plugin snippet
((identifier) @_start
 (#match? @_start "^plugin$")
 (completion
  "plugin" @name
  "Plugin Declaration"
  "plugin \"${1:beancount.plugins.auto_accounts}\"$0" @template))