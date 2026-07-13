-- SPDX-License-Identifier: MIT

---@meta rsast

local rsast = {}

--- A callback for use with the `tokens` methods of [`rsast.Pair`](lua-rsast.Pair)
--- and [`rsast.Pairs`](lua-rsast.Pairs).
---
--- It is passed an [`rsast.Tokens`](lua-rsast.Tokens) as the only argument.
---
---@alias rsast.TokenCallback<R> fun(tokens: rsast.Tokens): R

--- A callback for use with the `peek`, `next`, and `next_back` methods of
--- [`rsast.Pairs`](lua-rsast.Pairs).
---
--- It is passed an [`rsast.Pair`](lua-rsast.Pair) as the only argument.
---
---@alias rsast.NodeCallback<R> fun(pair: rsast.Pair): R

--- A callback used with [`rsast.Pair:lines()`](lua-rsast.Pair.lines)
---
--- It is passed an [`rsast.Lines`](lua-rsast.Lines) as the only argument.
---
---@alias rsast.LineCallback<R> fun(lines: rsast.Lines): R

--- A callback for use with [`rsast.Ast:parse()`](lua-rsast.Ast.parse) or
--- [`rsast.Pair:pairs()`](lua-rsast.Pair.pairs)
---
--- It is passed an [`rsast.Pairs`](lua-rsast.Pairs) as the only argument.
---
---@alias rsast.PairsCallback<R> fun(pairs: rsast.Pairs): R

--- A callback for use with `fold`, `fold_flat`, `rfold`, and `rfold_flat` of
--- [`rsast.Pairs`](lua-rsast.Pairs).
---
--- It is passed an accumulator value and an [`rsast.Pair`](lua-rsast.Pair) as arguments.
---
--- This callback should return the accumulator value, and optionally, a boolean
--- to determine whether iteration should continue.
---
---@alias rsast.FoldCallback<T> fun(acc: T, pair: rsast.Pair): (T, boolean?)

--- A single AST node.
---
---@class rsast.Node: table
---
---@field rule  string              The double-quoted name of the rule
---@field pos   [integer, integer]  The start and stop position of the node
---@field inner string | rsast.Tree A list of child nodes, or the node text if there are no children

--- A complete AST tree
---
---@class rsast.Tree: table
---
---@field pairs rsast.Node[]       A list of nodes in the tree
---@field pos   [integer, integer] The start and stop position of the tree

---@alias rsast.TokenType "start" | "end"

--- A syntax token
---
---@class rsast.Token: table
---
---@field type rsast.TokenType The type of token, either 'start' or 'stop'
---@field rule string          The name of the rule
---@field pos  integer         The byte-offset position of the token

--- An iterator over [`rsast.Token`](lua-rsast.Token)
---
--- It can only be accessed and used within an [`rsast.TokenCallback`](lua-rsast.TokenCallback)
---
---@class rsast.Tokens: userdata
---
---@field peek      fun(self): rsast.Token? Get the next token without advancing the iterator
---@field next      fun(self): rsast.Token? Get the next token
---@field next_back fun(self): rsast.Token? Get the next token from the end

--- An iterator over the lines covered by an [`rsast.Pair`](lua-rsast.Pair)
---
--- It can only be accessed and used within an [`rsast.LineCallback`](lua-rsast.LineCallback)
---
---@class rsast.Lines: userdata
---
---@field peek fun(self): (string, integer, integer) Get the text, start, and stop position of the next line without advancing the iterator
---@field next fun(self): (string, integer, integer) Get the text, start, and stop position of the next line

--- A matching pair of [`rsast.Token`](lua-rsast.Token) and everything between them
---
--- It can only be accessed and used within an [`rsast.NodeCallback`](lua-rsast.NodeCallback)
--- or an [`rsast.FoldCallback`](lua-rsast.FoldCallback)
---
---@class rsast.Pair: userdata
---
---@field start       fun(self): integer            The start byte position of the pair
---@field stop        fun(self): integer            The end byte position of the pair
---@field as_rule     fun(self): string             The name of the rule which matched the pair
---@field as_str      fun(self): string             The text between `start` and `stop` of this pair
---@field as_node_tag fun(self): string             The current node tag
---@field get_input   fun(self): string             The input from which the pair was parsed
---@field line_col    fun(self): (integer, integer) The line and column number of `start`
---@field dump        fun(self): rsast.Node         Output the pair to an [`rsast.Node`](lua-rsast.Node)
---
rsast.Pair = {}

--- Invoke a callback with an [`rsast.Tokens`](lua-rsast.Tokens) iterator
---
---@generic R
---
---@param callback? rsast.TokenCallback<R>
---
---@return rsast.Token[] | R
---
function rsast.Pair:tokens(callback) end

--- Invoke a callback with an [`rsast.Lines`](lua-rsast.Lines) iterator
---
---@generic R
---
---@param callback? rsast.LineCallback<R>
---
---@return string[] | R
function rsast.Pair:lines(callback) end

--- Invoke a callback with an [`rsast.Pairs`](lua-rsast.Pairs) iterator
---
---@generic R
---
---@param callback? rsast.PairsCallback<R>
---
---@return rsast.Tree | R
---
function rsast.Pair:pairs(callback) end

--- An iterator over [`rsast.Pair`](lua-rsast.Pair)
---
--- It can only be accessed and used within an [`rsast.PairsCallback`](lua-rsast.PairsCallback)
---
---@class rsast.Pairs
---
---@field as_str    fun(self): string     The text between `start` of the first pair and `stop` of the last
---@field get_input fun(self): string     The input from which the pairs were parsed
---@field concat    fun(self): string     The concatenated text of the pairs
---@field is_empty  fun(self): boolean    Whether the iterator is empty
---@field dump      fun(self): rsast.Tree Output the pairs to an [`rsast.Tree`](lua.rsast.Tree)
---@field dump_flat fun(self): rsast.Tree Flatten nested pairs and output to an [`rsast.Tree`](lua.rsast.Tree)
---
rsast.Pairs = {}

--- Get the next pair without advancing the iterator.
---
--- Returns `nil` if the iterator is exhausted
---
---@generic R
---
---@param callback? rsast.NodeCallback<R>
---
---@return rsast.Node | R | nil
---
function rsast.Pairs:peek(callback) end

--- Get the next pair.
---
--- Returns `nil` if the iterator is exhausted
---
---@generic R
---
---@param callback? rsast.NodeCallback<R>
---
---@return rsast.Node | R | nil
---
function rsast.Pairs:next(callback) end

--- Get the next pair from the end.
---
--- Returns `nil` if the iterator is exhausted
---
---@generic R
---
---@param callback? rsast.NodeCallback<R>
---
---@return rsast.Node | R | nil
---
function rsast.Pairs:next_back(callback) end

---  Invoke a callback with an [`rsast.Tokens`](lua-rsast.Tokens) iterator
---
---@generic R
---
---@param callback? rsast.TokenCallback<R>
---
---@return rsast.Token[] | R
---
function rsast.Pairs:tokens(callback) end

--- Fold the pairs with `callback`
---
--- The method is called with an initial value, `acc`, which is passed to the
--- callback along with an [`rsast.Pair`](lua-rsast.Pair). The return value of
--- `callback` is then passed as the initial value on the next iteration
---
---@generic T
---
---@param acc       T
---@param callback? rsast.FoldCallback<T>
---
---@return T
---
function rsast.Pairs:fold(acc, callback) end

--- Flatten nested pairs and fold with `callback`
---
---@see rsast.Pairs.fold
---
---@generic T
---
---@param acc       T
---@param callback? rsast.FoldCallback<T>
---
---@return T
---
function rsast.Pairs:fold_flat(acc, callback) end

--- Reverse fold the pairs with `callback`
---
---@see rsast.Pairs.fold
---
---@generic T
---
---@param acc       T
---@param callback? rsast.FoldCallback<T>
---
---@return T
---
function rsast.Pairs:rfold(acc, callback) end

--- Flatten nested pairs and reverse fold with `callback`
---
---@see rsast.Pairs.fold
---
---@generic T
---
---@param acc       T
---@param callback? rsast.FoldCallback<T>
---
---@return T
---
function rsast.Pairs:rfold_flat(acc, callback) end

--- A PEG grammar parser
---
---@class rsast.Ast: userdata
---
rsast.Ast = {}

--- Load a grammar from a string for parsing.
---
--- Returns `rsast.Ast, nil` if the grammar was loaded successfully, or
--- `nil, string[]`, where `string[]` is a list of errors encountered
--- while loading the grammar.
---
---@param grammar string The grammar to load
---
---@return rsast.Ast? # A parser for the provided grammar, returns nil of it could not be loaded
---@return string[]?  # Errors encountered while loading the grammar, or nil if there were none.
---
function rsast.Ast.new(grammar) end

--- Parse an input with the loaded grammar.
---
--- If `callback` is provided, it is called with an [`rsast.Pairs`](lua-rsast.Pairs)
--- iterator as the first argument, and the return value of the function is returned
--- from [`parse`](lua-rsast.Ast.parse).
---
--- If `callback` is not provided, the input is parsed into an [`rsast.Tree`](lua-rsast.Tree).
---
---@generic R
---
---@param rule      string The rule to parse
---@param input     string The input to parse
---@param callback? rsast.PairsCallback<R>
---
---@return rsast.Tree | R # Returns the result of `callback`, or an [`rsast.Tree`](lua-rsast.Tree) if no callback was provided
---
function rsast.Ast:parse(rule, input, callback) end

return rsast
