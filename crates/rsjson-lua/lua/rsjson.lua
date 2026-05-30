--- SPDX-License-Identifier: MIT
---
--- A Lua interface to the rust `serde-json` crate.

local rsjson_lua = require("rsjson_lua")

local rsjson = {}

--- Low-level serde API
---
---@type table
rsjson.serde = rsjson_lua

--- Represents the JSON `null` value.
--- This can be used in the place of
--- `nil` to represent empty values.
---
---@alias rsjson.null lightuserdata

---@type rsjson.null
rsjson.null = rsjson.serde.null

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
rsjson.EncodeConfig = rsjson.serde.EncodeConfig

---@class (exact) rsjson.DecodeConfig: userdata
---
---@field null            boolean Convert `nil` to `rsjson.null`
---@field cast_u64_to_f64 boolean Convert u64 numbers to f64 if they overflow i64
---@field set_array_mt    boolean Set the metatable of JSON array tables to `mlua::Lua::array_metatable`
---
---@field new             fun(): rsjson.DecodeConfig
rsjson.DecodeConfig = rsjson.serde.DecodeConfig

--- Serialize a Lua object into a JSON string
---
---@param obj     any Any Lua object
---@param config? rsjson.EncodeConfig
---
---@return string # The serialized Lua object
function rsjson.encode(obj, config)
    return rsjson.serde.encode(obj, config)
end

--- Deserialize a JSON string into a Lua object
---
---@param str     string The JSON string
---@param config? rsjson.DecodeConfig
---
---@return any # The deserialized JSON object
function rsjson.decode(str, config)
    return rsjson.serde.decode(str, config)
end

--- Serialize a Lua object to a file
---
---@param obj     any    Any Lua object
---@param path    string The filepath to write to
---@param indent? number The indent level of lines
---
---@return string # The serialized Lua object
function rsjson.to_file(obj, path, indent)
    local file = io.open(path, "w+")
    if not file then
        error("Could not write file: " .. path)
    end

    local config = nil
    if indent then
        config = rsjson.EncodeConfig:new()
        config.indent = indent
    end

    local content = rsjson.encode(obj, config)
    file:write(content)
    file:flush()
    file:close()
    return content
end

--- Serialize a Lua object to a string
---
---@param obj     any    Any Lua object
---@param indent? number The indent level of lines
---
---@return string # The serialized Lua object
function rsjson.to_str(obj, indent)
    local config = nil
    if indent then
        config = rsjson.EncodeConfig:new()
        config.indent = indent
    end

    return rsjson.encode(obj, config)
end

--- Deserialize a JSON object from a file
---
---@param path string The filepath to load
---
---@return any # The deserialized JSON object
function rsjson.from_file(path)
    local file = io.open(path, "r")
    if not file then
        error("Could not read file: " .. path)
    end

    local content = file:read("a")
    file:close()

    return rsjson.decode(content)
end

--- Deserialize a JSON object from a string
---
---@param str string The string to load
---
---@return any # The deserialized JSON object
function rsjson.from_str(str)
    return rsjson.decode(str)
end

return rsjson
