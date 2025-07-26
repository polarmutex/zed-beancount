; Snippets for common Beancount patterns

; Transaction snippet
((identifier) @_start
 (#match? @_start "^txn$")
 (completion
  "txn" @name
  "Transaction"
  "2024-01-01 * \"${1:Payee}\" \"${2:Narration}\"\n  ${3:Assets:Checking}  ${4:100.00} ${5:USD}\n  ${6:Expenses:Food}$0" @template))

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