-- SPDX-License-Identifier: MIT

---@meta rsast

local rsast = {}

---@class rsast.Node: table
---
---@field rule  string
---@field pos   [integer, integer]
---@field inner string | rsast.Ast

---@class rsast.Ast: table
---
---@field pairs rsast.Node[]
---@field pos   [integer, integer]

---@alias rsast.TokenType "start" | "end"

---@class rsast.Token: table
---
---@field type rsast.TokenType
---@field rule string
---@field pos  integer

---@class Tokens: userdata
---
---@field peek      fun(self): rsast.Token?
---@field next      fun(self): rsast.Token?
---@field next_back fun(self): rsast.Token?

---@class rsast.Pair: userdata
---
---@generic R: any
---
---@field start       fun(self): integer
---@field end         fun(self): integer
---@field as_rule     fun(self): string
---@field as_str      fun(self): string
---@field as_node_tag fun(self): string
---@field get_input   fun(self): string
---@field line_col    fun(self): (integer, integer)
---@field dump        fun(self): rsast.Node
---@field pairs       fun(self, callback?: fun(pairs: rsast.Pairs): R): R | rsast.Ast
---@field tokens      fun(self, callback?: fun(tokens: rsast.Tokens): R): R | rsast.Token[]

---@class rsast.Pairs
---
---@generic R: any
---
---@field as_str        fun(self): string
---@field get_input     fun(self): string
---@field concat        fun(self): string
---@field is_empty      fun(self): boolean
---@field peek          fun(self, callback?: fun(pair: rsast.Pair): R): R | rsast.Node | nil
---@field next          fun(self, callback?: fun(pair: rsast.Pair): R): R | rsast.Node | nil
---@field next_back     fun(self, callback?: fun(pair: rsast.Pair): R): R | rsast.Node | nil
---@field tokens        fun(self, callback?: fun(tokens: rsast.Tokens): R): R | rsast.Token[]
---@field tokens_flat   fun(self, callback?: fun(tokens: rsast.Tokens): R): R | rsast.Token[]
---@field for_each      fun(self, callback: fun(pair: rsast.Pair): bool?): bool
---@field for_each_flat fun(self, callback: fun(pair: rsast.Pair): bool?): bool
---@field dump          fun(self): rsast.Ast
---@field dump_flat     fun(self): rsast.Ast

---@class rsast.Ast: userdata
---
rsast.Ast = {}

---@param grammar string
---
---@return rsast.Ast?, string[]?
function rsast.Ast.new(grammar) end

---@generic R: any
---
---@param rule      string
---@param input     string
---@param callback? fun(pairs: rsast.Pairs): R
---
---@return R | rsast.Ast
function rsast.Ast:parse(rule, input, callback) end

return rsast
