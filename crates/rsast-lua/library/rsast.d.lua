-- SPDX-License-Identifier: MIT

---@meta rsast

local rsast = {}

---
---@alias rsast.NodeCallback<R> fun(pair: rsast.Pair): R

---
---@alias rsast.TokenCallback<R> fun(tokens: rsast.Tokens): R

---
---@alias rsast.FoldCallback<T> fun(acc: T, pair: rsast.Pair): (T, boolean?)

---
---@alias rsast.PairsCallback<R> fun(pairs: rsast.Pairs): R

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
---@field tokens      fun(self, callback?: rsast.TokenCallback<R>): R | rsast.Token[]
---@field pairs       fun(self, callback?: rsast.PairsCallback<R>): R | rsast.Ast

---@class rsast.Pairs
---
---@generic R: any, T: any
---
---@field as_str      fun(self): string
---@field get_input   fun(self): string
---@field concat      fun(self): string
---@field is_empty    fun(self): boolean
---@field peek        fun(self, callback?: rsast.NodeCallback<R>): R | rsast.Node | nil
---@field next        fun(self, callback?: rsast.NodeCallback<R>): R | rsast.Node | nil
---@field next_back   fun(self, callback?: rsast.NodeCallback<R>): R | rsast.Node | nil
---@field tokens      fun(self, callback?: rsast.TokenCallback<R>): R | rsast.Token[]
---@field tokens_flat fun(self, callback?: rsast.TokenCallback<R>): R | rsast.Token[]
---@field fold        fun(self, acc: T, callback: rsast.FoldCallback<T>): T
---@field fold_flat   fun(self, acc: T, callback: rsast.FoldCallback<T>): T
---@field rfold       fun(self, acc: T, callback: rsast.FoldCallback<T>): T
---@field rfold_flat  fun(self, acc: T, callback: rsast.FoldCallback<T>): T
---@field dump        fun(self): rsast.Ast
---@field dump_flat   fun(self): rsast.Ast

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
---@param callback? rsast.PairsCallback<R>
---
---@return R | rsast.Ast
function rsast.Ast:parse(rule, input, callback) end

return rsast
