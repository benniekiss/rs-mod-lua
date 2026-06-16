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
---@field indent               number  The number of `prefix` to indent lines
---@field prefix               string  The string to use for indentation
---@field sort_keys            boolean Sort JSON keys
---@field empty_table_as_array boolean Convert empty tables to empty arrays
---@field detect_mixed_tables  boolean Detect mixed sequence and key tables
---@field error_unsupported    boolean Error on unsupported types (functions, userdata, etc)
---@field error_cycles         boolean Error on cycles
rsjson.EncodeConfig = {}

--- Create a new `rsjson.EncodeConfig`
---
---@return rsjson.EncodeConfig
function rsjson.EncodeConfig:new() end

---@class (exact) rsjson.DecodeConfig: userdata
---
---@field null            boolean Convert `nil` to `rsjson.null`
---@field cast_u64_to_f64 boolean Convert u64 numbers to f64 if they overflow i64
---@field set_array_mt    boolean Set the metatable of JSON array tables to `mlua::Lua::array_metatable`
rsjson.DecodeConfig = {}

--- Create a new `rsjson.DecodeConfig`
---
---@return rsjson.DecodeConfig
function rsjson.DecodeConfig:new() end

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
