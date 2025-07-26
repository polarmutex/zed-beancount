; Symbol outline for Beancount files

; Transaction entries
(txn 
  date: (date) @context
  narration: (narration)? @name
) @item

(txn 
  date: (date) @context
  payee: (payee) @name
) @item

; Account operations
(open 
  date: (date) @context
  account: (account) @name
) @item

(close 
  date: (date) @context  
  account: (account) @name
) @item

(balance
  date: (date) @context
  account: (account) @name
) @item

(pad
  date: (date) @context
  account: (account) @name
) @item

; Options and configuration
(option
  key: (string) @name
) @item

(plugin
  name: (string) @name
) @item

(include
  file: (string) @name
) @item

; Price entries
(price
  date: (date) @context
  commodity: (commodity) @name
) @item

; Events and notes
(event
  date: (date) @context
  name: (string) @name
) @item

(note
  date: (date) @context
  account: (account) @name
) @item

; Custom directives
(custom
  date: (date) @context
  name: (string) @name
) @item