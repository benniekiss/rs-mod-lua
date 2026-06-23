--- SPDX-License-Identifier: MIT

---@meta jsonschema

local jsonschema = {}

---@alias jsonschema.Draft
--- | "Draft201909"
--- | "Draft202012"
--- | "Draft4"
--- | "Draft6"
--- | "Draft7"

---@class (exact) jsonschema.EvaluationNode: table
---@field valid              bool
---@field evaluationPath     string
---@field schemaLocation     string
---@field instanceLocation   string
---@field annotations        integer
---@field droppedAnnotations boolean
---@field errors             table[]

--- Simple boolean validity indicator
---
---@class (exact) jsonschema.FlagOutput: table
---@field valid bool

--- Flat list of all evaluation units
---
---@see jsonschema.EvaluationNode
---
---@class (exact) jsonschema.ListOutput: table
---@field valid   bool
---@field details jsonschema.EvaluationNode[]

--- Nested tree structure of evaluation units
---
---@see jsonschema.EvaluationNode
---
---@class (exact) jsonschema.HierarchicalOutput: table
---@field valid              bool
---@field evaluationPath     string
---@field schemaLocation     string
---@field instanceLocation   string
---@field annotations        integer
---@field droppedAnnotations boolean
---@field errors             table[]
---@field details            jsonschema.EvaluationNode[]

--- Entry describing annotations emitted by a keyword during evaluation
---
---@class (exact) jsonschema.AnnotationEntry
---@field schema_location           string
---@field absolute_keyword_location table?
---@field instance_location         table?
---@field annotations               table?

--- Entry describing errors emitted by a keyword during evaluation
---
---@class (exact) jsonschema.ErrorEntry
---@field schema_location           string
---@field absolute_keyword_location table?
---@field instance_location         table?
---@field error                     table?

--- Result of evaluating a JSON instance against a schema
---
---@class (exact) jsonschema.Evaluation
jsonschema.Evaluation = {}

--- Get the flag output format
---
--- This is the simplest output format, containing only a boolean
--- indicating whether the instance is valid according to the schema
---@return jsonschema.FlagOutput
---
function jsonschema.Evaluation:flag() end

--- Get the list output format
---
--- This format provides a flat list of all evaluation units, where
--- each unit contains information about a specific validation step
--- including its location, validity, annotations, and errors.
---
---@return jsonschema.ListOutput
---
function jsonschema.Evaluation:list() end

--- Get the hierarchical output format
---
--- This format represents the evaluation as a tree structure that
--- mirrors the schema's logical structure. Each node contains its
--- validation result along with nested child nodes representing
--- sub-schema evaluations.
---
---@return jsonschema.HierarchicalOutput
---
function jsonschema.Evaluation:hierarchical() end

--- List all annotations produced during evaluation
---
---@return jsonschema.AnnotationEntry[]
---
function jsonschema.Evaluation:annotations() end

--- List all errors produced during evaluation
---
---@return jsonschema.ErrorEntry[]
---
function jsonschema.Evaluation:errors() end

--- A compiled JSON Schema validator.
---
---@class (exact) jsonschema.Validator
jsonschema.Validator = {}

--- Check if a JSON instance is valid
---
---@param json any
---
---@return boolean
function jsonschema.Validator:is_valid(json) end

--- Raises an error if the JSON instance is invalid
---
---@param json any
---
---@return bool, string?
function jsonschema.Validator:validate(json) end

--- Evaluate the JSON instance.
---
---@param json any
---
---@return jsonschema.Evaluation
---
function jsonschema.Validator:evaluate(json) end

--- List all errors raised when validating the JSON instance
---
---@param json any
---
---@return error[]
---
function jsonschema.Validator:errors(json) end

--- The JSON schema draft which was used to build the validator.
---
---@return jsonschema.Draft
---
function jsonschema.Validator:draft() end

--- A map of compiled JSON Schema validators keyed by URI-fragment JSON pointers
---
--- Each key is a URI-fragment JSON pointer (e.g. "#", "#/$defs/User").
--- The root schema is always present under the key "#"
---
---@class (exact) jsonschema.ValidatorMap
jsonschema.ValidatorMap = {}

--- Get the validator for the given URI-fragment pointer, or nil if not found
---
---@param pointer string
---
---@return jsonschema.Validator?
function jsonschema.ValidatorMap:get(pointer) end

--- Check if the map contains a validator for the given pointer
---
---@param pointer string
---
---@return boolean
function jsonschema.ValidatorMap:contains_key(pointer) end

--- List all URI-fragment pointers in the map.
---
---@return string[]
function jsonschema.ValidatorMap:keys() end

--- Validation for JSON meta schema
---
jsonschema.meta = {}

--- Validate a JSON Schema document against its meta-schema
---
---@param schema any
---
---@return boolean
function jsonschema.meta.is_valid(schema) end

--- Validate a JSON Schema document against its meta-schema and return the first error, if any
---
---@param schema any
---
---@return bool, string?
function jsonschema.meta.validate(schema) end

--- Build a validator for a JSON Schema's meta-schema.
---
---@param schema any
---
---@return jsonschema.Validator
function jsonschema.meta.validator_for(schema) end

--- Async validation functions
---
jsonschema.async = {}

--- Create a [`Validator`](lua-jsonschema.Validator) for the input schema with
--- automatic draft detection and default options, using non-blocking retrieval
--- for external references.
---
--- Async counterpart to validator_for. Note that only the construction is
--- asynchronous - validation itself is always synchronous.
---
---@async
---@param schema any
---
---@return jsonschema.Validator
function jsonschema.async.validator_for(schema) end

--- Create a [`ValidatorMap`](lua-jsonschema.ValidatorMap) from the input schema
--- using async retrieval for external references.
---
--- Async counterpart to validator_map_for. Note that only the construction is
--- asynchronous — validation itself is always synchronous.
---
---@async
---@param schema any
---
---@return jsonschema.ValidatorMap
function jsonschema.async.validator_map_for(schema) end

--- Bundle a JSON Schema into a Compound Schema Document, using async retrieval
--- for external references.
---
--- Async counterpart to [`bundle()`](lua-jsonschema.bundle).
---
---@async
---@param schema any
---
---@return any
function jsonschema.async.bundle(schema) end

--- Dereference a JSON Schema asynchronously.
---
--- Async counterpart to [`dereference()`](lua-jsonschema.dereference).
---
---@async
---@param schema any
---
---@return any
function jsonschema.async.dereference(schema) end

--- Validate a JSON instance against a schema
---
---@param schema any
---@param json   any
---
---@return boolean
---
function jsonschema.is_valid(schema, json) end

--- Validate a JSON instance against a schema and return the first error, if any
---
---@param schema any
---@param json   any
---
---@return bool, string?
function jsonschema.validate(schema, json) end

--- Evaluate a JSON instance against a schema
---
---@param schema any
---@param json   any
---
---@return jsonschema.Evaluation
---
function jsonschema.evaluate(schema, json) end

--- Create a [`Validator`](lua-jsonschema.Validator) for the input schema
---
---@param schema any
---
---@return jsonschema.Validator
---
function jsonschema.validator_for(schema) end

--- Create a [`ValidatorMap`](lua-jsonschema.ValidatorMap) from the input schema
---
---@param schema any
---
---@return jsonschema.ValidatorMap
---
function jsonschema.validator_map_for(schema) end

--- Embed all external $ref targets into a draft-appropriate container,
--- producing a Compound Schema Document that validates identically to the
--- original. Draft 4/6/7 use definitions; Draft 2019-09/2020-12 use $defs.
--- $ref values are preserved unchanged. For mixed-draft bundles, embedded
--- resources may include both id and $id to maximize interoperability with
--- downstream validators that differ in draft handling.
---
--- Limitation: $dynamicRef is not followed during bundling.
---
---@param schema any
---
---@return any
---
function jsonschema.bundle(schema) end

--- Dereference a JSON Schema by recursively replacing all $ref values with
--- the schemas they point to.
---
--- Circular references are left in place as $ref strings.
---
---@param schema any
---
---@return any
---
function jsonschema.dereference(schema) end
