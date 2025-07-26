; Indentation rules for Beancount files

; Transaction entries and their postings
(txn
  (posting)* @indent
) @item

; Metadata blocks
(metadata
  (key_value)* @indent
) @item

; Posting indentation within transactions
(posting) @indent

; Plugin and include statements
(plugin) @item
(include) @item

; All directive types
[
  (open)
  (close)
  (balance)
  (pad)
  (note)
  (document)
  (price)
  (event)
  (query)
  (custom)
] @item

; Keep proper indentation for continued lines
(incomplete_amount) @indent