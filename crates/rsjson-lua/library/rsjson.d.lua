--- SPDX-License-Identifier: MIT

---@meta rsjson

local rsjson = {}

--- Represents the JSON `null` value.
--- This can be used in the place of
--- `nil` to represent empty values.
---
---@class rsjson.null: lightuserdata
rsjson.null = nil

--- A metatable attachable to a Lua table to systematically encode
--- it as Array (instead of Map). As a result, encoded Array will
--- contain only sequence part of the table, with the same length
--- as the # operator on that table.
---
---@class rsjson.array_mt: table
rsjson.array_mt = nil

---@class (exact) rsjson.EncodeConfig: userdata
---
---@field indent                       number  The number of `prefix` to indent lines
---@field prefix                       string  The string to use for indentation
---@field sort_keys                    boolean Sort JSON keys
---@field encode_empty_tables_as_array boolean Convert empty tables to empty arrays
---@field detect_mixed_tables          boolean Detect mixed sequence and key tables
---@field deny_unsupported_types       boolean Error on unsupported types (functions, threads, etc)
---@field deny_recursive_tables        boolean Error on recursive tables.
rsjson.EncodeConfig = {}

--- Create a new `rsjson.EncodeConfig`
---
---@return rsjson.EncodeConfig
function rsjson.EncodeConfig:new() end

--- Set the indent level
---
---@param indent? integer
---
---@return self
function rsjson.EncodeConfig:set_indent(indent) end

--- Set the indent prefix string
---
---@param prefix? string
---
---@return self
function rsjson.EncodeConfig:set_prefix(prefix) end

--- Set whether to deny serializing unsupported Lua types.
---
--- This includes functions, threads, lightuserdata, and errors
---
---@param deny? boolean
---
---@return self
function rsjson.EncodeConfig:set_deny_unsupported_types(deny) end

--- Set whether to deny serializing recursive tables.
---
--- If true, an attempt to serialize a recursive table will cause an error.
--- Otherwise subsequent attempts to serialize the same table will be ignored.
---
---@param deny? boolean
---
---@return self
function rsjson.EncodeConfig:set_deny_recursive_tables(deny) end

--- Whether to sort keys in order.
---
---@param enable? boolean
---
---@return self
function rsjson.EncodeConfig:set_sort_keys(enable) end

--- Whether to encode empty tables as arrays.
---
--- If false, empty tables will be serialized as maps.
---
---@param enable? boolean
---
---@return self
function rsjson.EncodeConfig:set_encode_empty_tables_as_array(enable) end

--- Whether to detext mixed tables.
---
--- When false, a table with a non-zero length (with one or more borders) will
--- be always encoded as an array.
---@param enable? boolean
---
---@return self
function rsjson.EncodeConfig:set_detect_mixed_tables(enable) end

---@class (exact) rsjson.DecodeConfig: userdata
---
---@field null            boolean Convert `nil` to `rsjson.null`
---@field cast_u64_to_f64 boolean Convert u64 numbers to f64 if they overflow i64
---@field array_metatable boolean Set the metatable of JSON array tables to `mlua::Lua::array_metatable`
rsjson.DecodeConfig = {}

--- Create a new `rsjson.DecodeConfig`
---
---@return rsjson.DecodeConfig
function rsjson.DecodeConfig:new() end

--- Whether to decode JSON `null` to `rsjson.null` or `nil`
---
---@param enable boolean
---
---@return self
function rsjson.DecodeConfig:set_null(enable) end

--- Whether to cast u64 JSON numbers to floats.
---
---@param enable boolean
---
---@return self
function rsjson.DecodeConfig:set_cast_u64_to_f64(enable) end

--- Whether to set the metatable of JSON arrays to `rsjson.array_mt`.s
---
---@param enable boolean
---
---@return self
function rsjson.DecodeConfig:set_array_metatable(enable) end

--- Serialize a Lua object into a JSON string
---
---@param obj     any Any Lua object
---@param config? rsjson.EncodeConfig
---
---@return string # The serialized Lua object
function rsjson.encode(obj, config) end

--- Deserialize a JSON string into a Lua object
---
---@param str     string The JSON string
---@param config? rsjson.DecodeConfig
---
---@return any # The deserialized JSON object
function rsjson.decode(str, config) end

return rsjson
