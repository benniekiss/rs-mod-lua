--- SPDX-License-Identifier: MIT

---@meta rs_mod_lua_core

local rs_mod_lua_core = {}

---@class (exact) rs_mod_lua_core.EncodeConfig: userdata
---
---@field indent                       number  The number of `prefix` to indent lines
---@field prefix                       string  The string to use for indentation
---@field sort_keys                    boolean Sort JSON keys
---@field encode_empty_tables_as_array boolean Convert empty tables to empty arrays
---@field detect_mixed_tables          boolean Detect mixed sequence and key tables
---@field deny_unsupported_types       boolean Error on unsupported types (functions, threads, etc)
---@field deny_recursive_tables        boolean Error on recursive tables.
---@field recursion_limit              integer Set the maximum recursion dept for tables
rs_mod_lua_core.EncodeConfig = {}

--- Create a new `EncodeConfig`
---
---@return self
function rs_mod_lua_core.EncodeConfig.new() end

--- Set the indent level
---
---@param indent? integer
---
---@return self
function rs_mod_lua_core.EncodeConfig:set_indent(indent) end

--- Set the indent prefix string
---
---@param prefix? string
---
---@return self
function rs_mod_lua_core.EncodeConfig:set_prefix(prefix) end

--- Set whether to deny serializing unsupported Lua types.
---
--- This includes functions, threads, lightuserdata, and errors
---
---@param deny? boolean
---
---@return self
function rs_mod_lua_core.EncodeConfig:set_deny_unsupported_types(deny) end

--- Set whether to deny serializing recursive tables.
---
--- If true, an attempt to serialize a recursive table will cause an error.
--- Otherwise subsequent attempts to serialize the same table will be ignored.
---
---@param deny? boolean
---
---@return self
function rs_mod_lua_core.EncodeConfig:set_deny_recursive_tables(deny) end

--- Set whether to sort keys in order.
---
---@param enable? boolean
---
---@return self
function rs_mod_lua_core.EncodeConfig:set_sort_keys(enable) end

--- Set whether to encode empty tables as arrays.
---
--- If false, empty tables will be serialized as maps.
---
---@param enable? boolean
---
---@return self
function rs_mod_lua_core.EncodeConfig:set_encode_empty_tables_as_array(enable) end

--- Set whether to detext mixed tables.
---
--- When false, a table with a non-zero length (with one or more borders) will
--- be always encoded as an array.
---
---@param enable? boolean
---
---@return self
function rs_mod_lua_core.EncodeConfig:set_detect_mixed_tables(enable) end

--- Set the maximum nesting depth for tables.
---
--- Increasing this limit may require a larger thread stack.
--- Zero rejects all tables.
---
---@param limit integer
---
---@return self
function rs_mod_lua_core.EncodeConfig:set_recursion_limit(limit) end

---@class (exact)rs_mod_lua_core.DecodeConfig: userdata
---
---@field null            boolean Convert `nil` to `null`
---@field cast_u64_to_f64 boolean Convert u64 numbers to f64 if they overflow i64
---@field array_metatable boolean Set the metatable of JSON array tables to `mlua::Lua::array_metatable`
---@field recursion_limit integer Maximum nesting depth for rust containers and newtype wrappers.
rs_mod_lua_core.DecodeConfig = {}

--- Create a new `DecodeConfig`
---
---@return self
function rs_mod_lua_core.DecodeConfig.new() end

--- Set whether to decode JSON `null` to `null` or `nil`
---
---@param enable boolean
---
---@return self
function rs_mod_lua_core.DecodeConfig:set_null(enable) end

--- Set whether to cast u64 JSON numbers to floats.
---
---@param enable boolean
---
---@return self
function rs_mod_lua_core.DecodeConfig:set_cast_u64_to_f64(enable) end

--- Set whether to set the metatable of JSON arrays to `array_mt`.s
---
---@param enable boolean
---
---@return self
function rs_mod_lua_core.DecodeConfig:set_array_metatable(enable) end

--- Set the maximum nesting depth for containers and newtype wrappers.
---
--- Increasing this limit may require a larger thread stack.
--- Zero rejects all nesting.
---
---@param limit integer
---
---@return self
function rs_mod_lua_core.DecodeConfig:set_recursion_limit(limit) end

return rs_mod_lua_core
