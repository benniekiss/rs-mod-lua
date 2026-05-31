--- SPDX-License-Identifier: MIT

---@meta rsjson

local rsjson = {}

--- Represents the JSON `null` value.
--- This can be used in the place of
--- `nil` to represent empty values.
---
---@alias rsjson.null lightuserdata

---@class (exact) rsjson.EncodeConfig: userdata
---
---@field indent               number  The number of `prefix` to indent lines
---@field prefix               string  The string to use for indentation
---@field sort_keys            boolean Sort JSON keys
---@field empty_table_as_array boolean Convert empty tables to empty arrays
---@field detect_mixed_tables  boolean Detect mixed sequence and key tables
---@field error_unsupported    boolean Error on unsupported types (functions, userdata, etc)
---@field error_cycles         boolean Error on cycles
---
---@field new                  fun(): rsjson.EncodeConfig

---@class (exact) rsjson.DecodeConfig: userdata
---
---@field null            boolean Convert `nil` to `rsjson.null`
---@field cast_u64_to_f64 boolean Convert u64 numbers to f64 if they overflow i64
---@field set_array_mt    boolean Set the metatable of JSON array tables to `mlua::Lua::array_metatable`
---
---@field new             fun(): rsjson.DecodeConfig

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
