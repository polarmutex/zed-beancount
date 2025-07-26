; Code injection rules for Beancount

; Inject SQL highlighting for query directives
((query
  query_string: (string) @injection.content)
 (#set! injection.language "sql"))

; Inject Python highlighting for plugin strings
((plugin
  name: (string) @injection.content)
 (#match? @injection.content ".*\\.py")
 (#set! injection.language "python"))

; Inject shell highlighting for include paths that look like scripts
((include
  file: (string) @injection.content)
 (#match? @injection.content ".*\\.(sh|bash)")
 (#set! injection.language "bash"))

; Inject markdown highlighting for long narrations or comments
((comment) @injection.content
 (#match? @injection.content "^;; ")
 (#set! injection.language "markdown"))

; Inject regex highlighting for account patterns in options
((option
  key: (string) @_key
  value: (string) @injection.content)
 (#match? @_key "account_.*_regexp")
 (#set! injection.language "regex"))

; Inject JSON highlighting for metadata values that look like JSON
((value) @injection.content
 (#match? @injection.content "^[\\{\\[].*[\\}\\]]$")
 (#set! injection.language "json"))