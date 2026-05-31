--- SPDX-License-Identifier: MIT

---@meta jsonschema

local jsonschema = {}

---
---
---@alias jsonschema.Draft "Draft201909" | "Draft202012" | "Draft4" | "Draft6" | "Draft7"

---
---
---@class (exact) jsonschema.AnnotationEntry
---@field schema_location           string
---@field absolute_keyword_location table?
---@field instance_location         table?
---@field annotations               table?

---
---
---@class (exact) jsonschema.ErrorEntry
---@field schema_location           string
---@field absolute_keyword_location table?
---@field instance_location         table?
---@field error                     table?

---
---
---@class (exact) jsonschema.Evaluation
jsonschema.Evaluation = {}

---
---
function jsonschema.Evaluation:flag() end

---
---
function jsonschema.Evaluation:list() end

---
---
function jsonschema.Evaluation:hierarchical() end

---
---@return jsonschema.AnnotationEntry[]
function jsonschema.Evaluation:annotations() end

---
---@return jsonschema.ErrorEntry[]
function jsonschema.Evaluation:errors() end

---
---
---@class (exact) jsonschema.Validator
jsonschema.Validator = {}

---
---
---@param json string
function jsonschema.Validator:is_valid(json) end

---
---
---@param json string
function jsonschema.Validator:validate(json) end

---
---
---@param json string
---
---@return jsonschema.Evaluation
function jsonschema.Validator:evaluate(json) end

---
---
---@param json string
---
---@return error[]
function jsonschema.Validator:errors(json) end

---
---
---@return jsonschema.Draft
function jsonschema.Validator:draft() end

---
---
---@class (exact) jsonschema.ValidatorMap
jsonschema.ValidatorMap = {}

---
---
---@param pointer string
---
---@return jsonschema.Validator
function jsonschema.ValidatorMap:get(pointer) end

---
---
---@param pointer string
---
---@return boolean
function jsonschema.ValidatorMap:contains_key(pointer) end

---
---
---@return string[]
function jsonschema.ValidatorMap:keys() end

--- Validation for JSON meta schema
---
jsonschema.meta = {}

---
---
---@param schema string
---@param json   string
---
---@return boolean
function jsonschema.meta.is_valid(schema, json) end

---
---
---@param schema string
---@param json   string
function jsonschema.meta.validate(schema, json) end

---
---
---@param schema string
---
---@return jsonschema.Validator
function jsonschema.meta.validator_for(schema) end

--- Async validation functions
---
jsonschema.async = {}

---
---
---@async
---@param schema string
---
---@return jsonschema.Validator
function jsonschema.async.validator_for(schema) end

---
---
---@async
---@param schema string
---
---@return jsonschema.ValidatorMap
function jsonschema.async.validator_map_for(schema) end

---
---
---@async
---@param schema string
---
---@return string
function jsonschema.async.bundle(schema) end

---
---
---@async
---@param schema string
---
---@return string
function jsonschema.async.dereference(schema) end

---
---
---@param schema string
---@param json   string
---
---@return boolean
function jsonschema.is_valid(schema, json) end

---
---
---@param schema string
---@param json   string
function jsonschema.validate(schema, json) end

---
---
---@param schema string
---@param json   string
---
---@return jsonschema.Evaluation
function jsonschema.evaluate(schema, json) end

---
---
---@param schema string
---
---@return jsonschema.Validator
function jsonschema.validator_for(schema) end

---
---
---@param schema string
---
---@return jsonschema.ValidatorMap
function jsonschema.validator_map_for(schema) end

---
---
---@param schema string
---
---@return string
function jsonschema.bundle(schema) end

---
---
---@param schema string
---
---@return string
function jsonschema.dereference(schema) end
