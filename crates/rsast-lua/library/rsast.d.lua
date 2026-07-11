-- SPDX-License-Identifier: MIT

---@meta rsast

local rsast = {}

---@alias rsast.TokenType "start" | "end"

---@class rsast.Token: table
---
---@field type rsast.TokenType
---@field rule string
---@field pos  integer

---@class Tokens: userdata
---
---@field next fun(self): rsast.Token

---@class rsast.Pair: userdata
---
---@generic R
---
---@field start       fun(self): integer
---@field end         fun(self): integer
---@field as_rule     fun(self): string
---@field as_str      fun(self): string
---@field as_node_tag fun(self): string
---@field get_input   fun(self): string
---@field line_col    fun(self): (integer, integer)
---@field pairs       fun(self, callback: fun(pairs: rsast.Pairs): R): R
---@field tokens      fun(self, callback: fun(tokens: rsast.Tokens): R): R
---@field dump        fun(self): table

---@class rsast.Pairs
---
---@generic R
---
---@field as_str        fun(self): string
---@field get_input     fun(self): string
---@field concat        fun(self): string
---@field is_empty      fun(self): boolean
---@field peek          fun(self, callback: fun(pair: rsast.Pair): R): R
---@field for_each      fun(self, callback: fun(pair: rsast.Pair): R): R
---@field for_each_flat fun(self, callback: fun(pair: rsast.Pair): R): R
---@field tokens        fun(self, callback: fun(tokens: rsast.Tokens): R): R
---@field tokens_flat   fun(self, callback: fun(tokens: rsast.Tokens): R): R
---@field dump          fun(self): table
---@field dump_flat     fun(self): table

---@class rsast.Ast: userdata
---
rsast.Ast = {}

---@param grammar string
---
---@return rsast.Ast
function rsast.Ast.new(grammar) end

---@generic R
---
---@param rule     string
---@param input    string
---@param callback fun(pairs: rsast.Pairs): R
---
---@return R
function rsast.Ast:parse(rule, input, callback) end

return rsast
