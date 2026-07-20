-- SPDX-License-Identifier: MIT

---@meta rsast

local rsast = {}

---@class rsast.Node: table
---
---@field start     integer
---@field stop      integer
---@field rule      string
---@field node_tag? string
---@field pairs?    rsast.Tree

---@class rsast.Tree: table
---
---@field start integer
---@field stop  integer
---@field pairs rsast.Node[]

--- A matching pair of [`rsast.Token`](lua-rsast.Token) and everything between them
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
---@field dump        fun(self): rsast.Node
---
rsast.Pair = {}

--- Invoke a callback with an [`rsast.Pairs`](lua-rsast.Pairs) iterator
---
---@return rsast.Pairs
---
function rsast.Pair:pairs() end

--- An iterator over [`rsast.Pair`](lua-rsast.Pair)
---
---@class rsast.Pairs
---
---@field as_str    fun(self): string     The text between `start` of the first pair and `stop` of the last
---@field get_input fun(self): string     The input from which the pairs were parsed
---@field is_empty  fun(self): boolean    Whether the iterator is empty
---@field dump      fun(self): rsast.Tree
---
rsast.Pairs = {}

--- Get the next pair without advancing the iterator.
---
--- Returns `nil` if the iterator is exhausted
---
---@return rsast.Pair?
---
function rsast.Pairs:peek() end

--- Get the next pair.
---
--- Returns `nil` if the iterator is exhausted
---
---@return rsast.Pair?
---
function rsast.Pairs:next() end

--- Get the next pair from the end.
---
--- Returns `nil` if the iterator is exhausted
---
---@return rsast.Pair?
---
function rsast.Pairs:next_back() end

--- Iterate over the pairs
---
---@return fun(): rsast.Pair
---
function rsast.Pairs:iter() end

--- Iterate over the pairs in reverse order
---
---@return fun(): rsast.Pair
---
function rsast.Pairs:reviter() end

--- Flatten nested nodes into a single iterator
---
---@return rsast.Pairs
---
function rsast.Pairs:flatten() end

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

--- Validate in input against the grammar
---
---
---@param rule  string The rule to parse
---@param input string The input to parse
---
---@return boolean, string? # Whether the input was valid, and any errors if it was not.
---
function rsast.Ast:validate(rule, input) end

--- Parse an input with the loaded grammar.
---
--- `callback` is called with an [`rsast.Pairs`](lua-rsast.Pairs) iterator as
--- the first argument, and the return value of the function is returned
--- from [`parse`](lua-rsast.Ast.parse).
---
---@generic R
---
---@param rule     string                        The rule to parse
---@param input    string                        The input to parse
---@param callback fun(pairs: rsast.Pairs): R...
---
---@return R... # Returns the result of `callback`
---
function rsast.Ast:parse(rule, input, callback) end

return rsast
