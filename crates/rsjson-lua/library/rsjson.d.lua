--- SPDX-License-Identifier: MIT

---@meta rsjson

local rsjson = {}

---@alias rsjson.EncodeConfig rs_mod_lua_core.EncodeConfig
---@alias rsjson.DecodeConfig rs_mod_lua_core.DecodeConfig

---@type rsjson.EncodeConfig
rsjson.EncodeConfig = nil

---@type rsjson.DecodeConfig
rsjson.DecodeConfig = nil

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
---@class rsjson.array_metatable: table
rsjson.array_metatable = nil

--- Serialize a Lua object into a JSON string
---
---@param obj     any                 Any Lua object
---@param config? rsjson.EncodeConfig
---
---@return string # The serialized Lua object
function rsjson.encode(obj, config) end

--- Deserialize a JSON string into a Lua object
---
---@param str     string              The JSON string
---@param config? rsjson.DecodeConfig
---
---@return any # The deserialized JSON object
function rsjson.decode(str, config) end

return rsjson
