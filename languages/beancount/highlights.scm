; Comments and documentation
(comment) @comment
(headline) @comment.documentation

; Strings and text
[
    (payee)
    (narration)
    (string)
] @string

; Special string types
(payee) @string.special
(narration) @string.quoted

; Numbers and amounts
(number) @number
(incomplete_amount) @number.float

; Dates
(date) @constant.builtin.date

; Currencies and commodities
(currency) @constant.builtin.currency
(commodity) @constant.builtin

; Accounts
(account) @type.account

; Directives and keywords
[
    (txn)
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
] @keyword.directive

; Options and includes
(option) @keyword.option
(include) @keyword.import
(plugin) @keyword.import

; Transaction flags
(flag) @constant.builtin.flag

; Metadata
(key) @property
(value) @string

; Tags and links
(tag) @tag
(link) @link

; Posting components
(posting) @markup.list
(posting_account) @type.account
(posting_amount) @number

; Error highlighting
(ERROR) @error

; Operators and punctuation
["=" "~" "@" "@@"] @operator
["(" ")" "{" "}" "[" "]"] @punctuation.bracket
["," ";"] @punctuation.delimiter
["\"" "'"] @punctuation.special

; Entry types with specific styling
(txn_line
  flag: (flag) @constant.builtin.flag
  payee: (payee)? @string.special
  narration: (narration)? @string.quoted)

; Account opening/closing with special highlighting
(open_line
  date: (date) @constant.builtin.date
  account: (account) @type.account.new)

(close_line
  date: (date) @constant.builtin.date
  account: (account) @type.account.closed)

; Balance assertions
(balance_line
  date: (date) @constant.builtin.date
  account: (account) @type.account
  amount: (amount) @number.assertion)

; Price entries
(price_line
  date: (date) @constant.builtin.date
  commodity: (commodity) @constant.builtin
  price: (amount) @number.price)
