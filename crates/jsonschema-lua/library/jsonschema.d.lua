--- SPDX-License-Identifier: MIT

---@meta jsonschema

local jsonschema = {}

--- Represents JSON `null`.
---
---@class jsonschema.null: lightuserdata
jsonschema.null = nil

---@class (exact) jsonschema.EncodeConfig: userdata
---
---@field indent                       number  The number of `prefix` to indent lines
---@field prefix                       string  The string to use for indentation
---@field sort_keys                    boolean Sort JSON keys
---@field encode_empty_tables_as_array boolean Convert empty tables to empty arrays
---@field detect_mixed_tables          boolean Detect mixed sequence and key tables
---@field deny_unsupported_types       boolean Error on unsupported types (functions, threads, etc)
---@field deny_recursive_tables        boolean Error on recursive tables.
---@field recursion_limit              integer Maximum recursion dept for tables
jsonschema.EncodeConfig = {}

--- Create a new `jsonschema.EncodeConfig`
---
---@return jsonschema.EncodeConfig
function jsonschema.EncodeConfig.new() end

--- Set the indent level
---
---@param indent? integer
---
---@return jsonschema.EncodeConfig
function jsonschema.EncodeConfig:set_indent(indent) end

--- Set the indent prefix string
---
---@param prefix? string
---
---@return jsonschema.EncodeConfig
function jsonschema.EncodeConfig:set_prefix(prefix) end

--- Set whether to deny serializing unsupported Lua types.
---
--- This includes functions, threads, lightuserdata, and errors
---
---@param deny? boolean
---
---@return jsonschema.EncodeConfig
function jsonschema.EncodeConfig:set_deny_unsupported_types(deny) end

--- Set whether to deny serializing recursive tables.
---
--- If true, an attempt to serialize a recursive table will cause an error.
--- Otherwise subsequent attempts to serialize the same table will be ignored.
---
---@param deny? boolean
---
---@return jsonschema.EncodeConfig
function jsonschema.EncodeConfig:set_deny_recursive_tables(deny) end

--- Set whether to sort keys in order.
---
---@param enable? boolean
---
---@return jsonschema.EncodeConfig
function jsonschema.EncodeConfig:set_sort_keys(enable) end

--- Set whether to encode empty tables as arrays.
---
--- If false, empty tables will be serialized as maps.
---
---@param enable? boolean
---
---@return jsonschema.EncodeConfig
function jsonschema.EncodeConfig:set_encode_empty_tables_as_array(enable) end

--- Set whether to detext mixed tables.
---
--- When false, a table with a non-zero length (with one or more borders) will
--- be always encoded as an array.
---
---@param enable? boolean
---
---@return jsonschema.EncodeConfig
function jsonschema.EncodeConfig:set_detect_mixed_tables(enable) end

--- Set the maximum nesting depth for tables.
---
--- Increasing this limit may require a larger thread stack.
--- Zero rejects all tables.
---
---@param limit integer
---
---@return jsonschema.EncodeConfig
function jsonschema.EncodeConfig:set_recursion_limit(limit) end

---@class (exact) jsonschema.DecodeConfig: userdata
---
---@field null            boolean Convert `nil` to `jsonschema.null`
---@field cast_u64_to_f64 boolean Convert u64 numbers to f64 if they overflow i64
---@field array_metatable boolean Set the metatable of JSON array tables to `mlua::Lua::array_metatable`
---@field recursion_limit integer Maximum nesting depth for rust containers and newtype wrappers.
jsonschema.DecodeConfig = {}

--- Create a new `jsonschema.DecodeConfig`
---
---@return jsonschema.DecodeConfig
function jsonschema.DecodeConfig.new() end

--- Set whether to decode JSON `null` to `jsonschema.null` or `nil`
---
---@param enable boolean
---
---@return jsonschema.DecodeConfig
function jsonschema.DecodeConfig:set_null(enable) end

--- Set whether to cast u64 JSON numbers to floats.
---
---@param enable boolean
---
---@return jsonschema.DecodeConfig
function jsonschema.DecodeConfig:set_cast_u64_to_f64(enable) end

--- Set whether to set the metatable of JSON arrays to `jsonschema.array_mt`.s
---
---@param enable boolean
---
---@return jsonschema.DecodeConfig
function jsonschema.DecodeConfig:set_array_metatable(enable) end

--- Set the maximum nesting depth for containers and newtype wrappers.
---
--- Increasing this limit may require a larger thread stack.
--- Zero rejects all nesting.
---
---@param limit integer
---
---@return jsonschema.DecodeConfig
function jsonschema.DecodeConfig:set_recursion_limit(limit) end

---@class (exact) jsonschema.Uri: userdata
---
---@field scheme     string The URI scheme.
---@field authority? string The optional URI authority component
---@field path       string The URI path component
---@field query?     string The optional URI query component
---@field fragment?  string The optional URI fragment component
jsonschema.Uri = {}

--- Parse a URI.
---
---@param uri string
---
---@return jsonschema.Uri
function jsonschema.Uri.parse(uri) end

--- Normalize the URI.
---
---@return jsonschema.Uri
function jsonschema.Uri:normalize() end

--- Return a URI without the fragment component.
---
---@return jsonschema.Uri
function jsonschema.Uri:strip_fragment() end

--- Whether the URI has an authority.
---
---@return boolean
function jsonschema.Uri:has_authority() end

--- Whether the URI has a query.
---
---@return boolean
function jsonschema.Uri:has_query() end

--- Whether the URI has a fragment.
---
---@return boolean
function jsonschema.Uri:has_fragment() end

--- Set or remove the encoded fragment.
---
---@param fragment? string
function jsonschema.Uri:set_fragment(fragment) end

---@class (exact) jsonschema.DraftVariant: userdata
jsonschema.DraftVariant = {}

--- Detect the schema draft, falling back to this draft.
---
---@param schema   any
---@param options? jsonschema.EncodeConfig
---
---@return jsonschema.DraftVariant
function jsonschema.DraftVariant:detect(schema, options) end

--- Check whether the draft defines a keyword.
---
---@param keyword string
---
---@return boolean
function jsonschema.DraftVariant:is_known_keyword(keyword) end

---@class (exact) jsonschema.Draft: table
---
---@field DRAFT202012 jsonschema.DraftVariant
---@field DRAFT201909 jsonschema.DraftVariant
---@field DRAFT7      jsonschema.DraftVariant
---@field DRAFT6      jsonschema.DraftVariant
---@field DRAFT4      jsonschema.DraftVariant
---@field UNKNOWN     jsonschema.DraftVariant
jsonschema.Draft = {}

--- Get the draft for a schema URI.
---
---@param uri string
---
---@return jsonschema.DraftVariant
function jsonschema.Draft.from_schema_uri(uri) end

---@class (exact) jsonschema.EvaluationNode: table
---
---@field valid               boolean
---@field evaluationPath      string
---@field schemaLocation      string
---@field instanceLocation    string
---@field annotations?        any
---@field droppedAnnotations? any
---@field errors?             table<string, string>

---@class (exact) jsonschema.FlagOutput: table
---
---@field valid boolean

---@class (exact) jsonschema.ListOutput: table
---
---@field valid   boolean
---@field details jsonschema.EvaluationNode[]

---@class (exact) jsonschema.HierarchicalOutput: table
---
---@field valid               boolean
---@field evaluationPath      string
---@field schemaLocation      string
---@field instanceLocation    string
---@field annotations?        any
---@field droppedAnnotations? any
---@field errors?             table<string, string>
---@field details?            jsonschema.HierarchicalOutput[]

---@class (exact) jsonschema.AnnotationEntry: table
---
---@field schema_location            string
---@field absolute_keyword_location? string
---@field instance_location          string
---@field annotations                any

---@class jsonschema.ErrorEntry: table
---
---@field schema_location            string
---@field absolute_keyword_location? string
---@field instance_location          string
---@field error                      { keyword: string, message: string }

--- The result of evaluating an instance.
---
---@class jsonschema.Evaluation: userdata
jsonschema.Evaluation = {}

--- Get the validity flag.
---
---@return jsonschema.FlagOutput
function jsonschema.Evaluation:flag() end

--- Get the flat evaluation output.
---
---@return jsonschema.ListOutput
function jsonschema.Evaluation:list() end

--- Get the nested evaluation output.
---
---@return jsonschema.HierarchicalOutput
function jsonschema.Evaluation:hierarchical() end

--- List evaluation annotations.
---
---@return jsonschema.AnnotationEntry[]
function jsonschema.Evaluation:annotations() end

--- List evaluation errors.
---
---@return jsonschema.ErrorEntry[]
function jsonschema.Evaluation:errors() end

--- A compiled schema validator.
---
---@class (exact) jsonschema.Validator: userdata
jsonschema.Validator = {}

--- Check whether an instance is valid.
---
---@param json     any
---@param options? jsonschema.EncodeConfig
---
---@return boolean
function jsonschema.Validator:is_valid(json, options) end

--- Return validity and an optional error message.
---
---@param json     any
---@param options? jsonschema.EncodeConfig
---
---@return boolean, string?
function jsonschema.Validator:validate(json, options) end

--- Evaluate an instance.
---
---@param json     any
---@param options? jsonschema.EncodeConfig
---
---@return jsonschema.Evaluation
function jsonschema.Validator:evaluate(json, options) end

--- List validation errors.
---
---@param json     any
---@param options? jsonschema.EncodeConfig
---
---@return string[]
function jsonschema.Validator:errors(json, options) end

--- Get the draft used to compile the schema.
---
---@return jsonschema.DraftVariant
function jsonschema.Validator:draft() end

--- Validators keyed by URI-fragment pointers, with the root at "#".
---
---@class (exact) jsonschema.ValidatorMap: userdata
jsonschema.ValidatorMap = {}

--- Get a validator, or nil if absent.
---
---@param pointer string
---
---@return jsonschema.Validator?
function jsonschema.ValidatorMap:get(pointer) end

--- Check whether a pointer has a validator.
---
---@param pointer string
---
---@return boolean
function jsonschema.ValidatorMap:contains_key(pointer) end

--- List the URI-fragment pointers.
---
---@return string[]
function jsonschema.ValidatorMap:keys() end

--- Email format options. Setters consume the options and return new userdata.
---
---@class (exact) jsonschema.EmailOptions: userdata
jsonschema.EmailOptions = {}

--- Create email format options.
---
---@return jsonschema.EmailOptions
function jsonschema.EmailOptions.new() end

--- Set the minimum number of domain segments.
---
---@param min integer
---
---@return jsonschema.EmailOptions
function jsonschema.EmailOptions:with_minimum_sub_domains(min) end

--- Allow single-segment domains.
---
---@return jsonschema.EmailOptions
function jsonschema.EmailOptions:with_no_minimum_sub_domains() end

--- Require a top-level domain.
---
---@return jsonschema.EmailOptions
function jsonschema.EmailOptions:with_required_tld() end

--- Allow domain literals.
---
---@return jsonschema.EmailOptions
function jsonschema.EmailOptions:with_domain_literal() end

--- Reject domain literals.
---
---@return jsonschema.EmailOptions
function jsonschema.EmailOptions:without_domain_literal() end

--- Allow display text.
---
---@return jsonschema.EmailOptions
function jsonschema.EmailOptions:with_display_text() end

--- Reject display text.
---
---@return jsonschema.EmailOptions
function jsonschema.EmailOptions:without_display_text() end

--- Regex options. Setters consume the options and return new userdata.
---
---@class (exact) jsonschema.PatternOptions: userdata
jsonschema.PatternOptions = {}

--- Create fancy-regex options.
---
---@return jsonschema.PatternOptions
function jsonschema.PatternOptions.new() end

--- Set the backtracking limit.
---
---@param limit integer
---
---@return jsonschema.PatternOptions
function jsonschema.PatternOptions:backtrack_limit(limit) end

--- Set the compiled regex size limit.
---
---@param limit integer
---
---@return jsonschema.PatternOptions
function jsonschema.PatternOptions:size_limit(limit) end

--- Set the DFA cache size limit.
---
---@param limit integer
---
---@return jsonschema.PatternOptions
function jsonschema.PatternOptions:dfa_size_limit(limit) end

---@alias jsonschema.Format fun(value: string): boolean

---@alias jsonschema.Retriever fun(uri: jsonschema.Uri): any

---@alias jsonschema.KeywordCheck fun(instance: any): boolean

--- Raise a Lua error for an invalid instance.
---
---@alias jsonschema.KeywordValidate fun(instance: any)

---@alias jsonschema.KeywordFactory fun(parent: table<string, any>, value: any, location: string): jsonschema.KeywordCheck, jsonschema.KeywordValidate

--- Validation options. Setters consume the options and return new userdata.
---
---@class (exact) jsonschema.ValidationOptions: userdata
jsonschema.ValidationOptions = {}

--- Create validation options.
---
---@return jsonschema.ValidationOptions
function jsonschema.ValidationOptions.new() end

--- Compile a schema.
---
---@param schema   any
---@param options? jsonschema.EncodeConfig
---
---@return jsonschema.Validator
function jsonschema.ValidationOptions:build(schema, options) end

--- Compile a schema map.
---
---@param schema   any
---@param options? jsonschema.EncodeConfig
---
---@return jsonschema.ValidatorMap
function jsonschema.ValidationOptions:build_map(schema, options) end

--- Bundle external references into a Lua schema value.
---
---@param schema  any
---@param encode? jsonschema.EncodeConfig
---@param decode? jsonschema.DecodeConfig
---
---@return any
function jsonschema.ValidationOptions:bundle(schema, encode, decode) end

--- Replace references and return a Lua schema value.
---
---@param schema  any
---@param encode? jsonschema.EncodeConfig
---@param decode? jsonschema.DecodeConfig
---
---@return any
function jsonschema.ValidationOptions:dereference(schema, encode, decode) end

--- Set the schema draft.
---
---@param draft jsonschema.DraftVariant
---
---@return jsonschema.ValidationOptions
function jsonschema.ValidationOptions:with_draft(draft) end

--- Set the base URI for relative references.
---
---@param base_uri string
---
---@return jsonschema.ValidationOptions
function jsonschema.ValidationOptions:with_base_uri(base_uri) end

--- Set email format options.
---
---@param options jsonschema.EmailOptions
---
---@return jsonschema.ValidationOptions
function jsonschema.ValidationOptions:with_email_options(options) end

--- Set regex options, consuming the supplied userdata.
---
---@param options jsonschema.PatternOptions
---
---@return jsonschema.ValidationOptions
function jsonschema.ValidationOptions:with_pattern_options(options) end

--- Set a schema retrieval callback.
---
---@param retriever jsonschema.Retriever
---@param options?  jsonschema.EncodeConfig
---
---@return jsonschema.ValidationOptions
function jsonschema.ValidationOptions:with_retriever(retriever, options) end

--- Whether to validate formats.
---
---@param validate boolean
---
---@return jsonschema.ValidationOptions
function jsonschema.ValidationOptions:should_validate_formats(validate) end

--- Whether to ignore unknown formats.
---
---@param ignore boolean
---
---@return jsonschema.ValidationOptions
function jsonschema.ValidationOptions:should_ignore_unknown_formats(ignore) end

--- Register a format checker.
---
---@param name   string
---@param format jsonschema.Format
---
---@return jsonschema.ValidationOptions
function jsonschema.ValidationOptions:with_format(name, format) end

--- Register a keyword factory.
---
---@param name    string
---@param factory jsonschema.KeywordFactory
---
---@return jsonschema.ValidationOptions
function jsonschema.ValidationOptions:with_keyword(name, factory) end

--- Meta-schema validation.
jsonschema.meta = {}

--- Check whether a schema is valid.
---
---@param schema   any
---@param options? jsonschema.EncodeConfig
---
---@return boolean
function jsonschema.meta.is_valid(schema, options) end

--- Return schema validity and an optional error message.
---
---@param schema   any
---@param options? jsonschema.EncodeConfig
---
---@return boolean, string?
function jsonschema.meta.validate(schema, options) end

--- Compile the meta-schema validator.
---
---@param schema   any
---@param options? jsonschema.EncodeConfig
---
---@return jsonschema.Validator
function jsonschema.meta.validator_for(schema, options) end

--- Available when built with the async feature.
jsonschema.async = {}

--- Compile a schema validator.
---
---@async
---@param schema   any
---@param options? jsonschema.EncodeConfig
---
---@return jsonschema.Validator
function jsonschema.async.validator_for(schema, options) end

--- Compile a map of schema validators.
---
---@async
---@param schema   any
---@param options? jsonschema.EncodeConfig
---
---@return jsonschema.ValidatorMap
function jsonschema.async.validator_map_for(schema, options) end

--- Bundle external schema references. String input returns JSON text.
---
---@async
---@param schema  any
---@param encode? jsonschema.EncodeConfig
---@param decode? jsonschema.DecodeConfig
---
---@return any
function jsonschema.async.bundle(schema, encode, decode) end

--- Replace schema references, preserving circular references. String input returns JSON text.
---
---@async
---@param schema  any
---@param encode? jsonschema.EncodeConfig
---@param decode? jsonschema.DecodeConfig
---
---@return any
function jsonschema.async.dereference(schema, encode, decode) end

--- String inputs are parsed as JSON; other inputs are converted from Lua.
--- Check whether an instance is valid.
---
---@param schema   any
---@param json     any
---@param options? jsonschema.EncodeConfig
---
---@return boolean
function jsonschema.is_valid(schema, json, options) end

--- Return validity and an optional error message.
---
---@param schema   any
---@param json     any
---@param options? jsonschema.EncodeConfig
---
---@return boolean, string?
function jsonschema.validate(schema, json, options) end

--- Evaluate an instance against a schema.
---
---@param schema   any
---@param json     any
---@param options? jsonschema.EncodeConfig
---
---@return jsonschema.Evaluation
function jsonschema.evaluate(schema, json, options) end

--- Compile a schema validator.
---
---@param schema   any
---@param options? jsonschema.EncodeConfig
---
---@return jsonschema.Validator
function jsonschema.validator_for(schema, options) end

--- Compile a map of schema validators.
---
---@param schema   any
---@param options? jsonschema.EncodeConfig
---
---@return jsonschema.ValidatorMap
function jsonschema.validator_map_for(schema, options) end

--- Bundle external schema references.
---
--- String input returns JSON text.
---
---@param schema  any
---@param encode? jsonschema.EncodeConfig
---@param decode? jsonschema.DecodeConfig
---
---@return any
function jsonschema.bundle(schema, encode, decode) end

--- Replace schema references, preserving circular references.
---
--- String input returns JSON text.
---
---@param schema  any
---@param encode? jsonschema.EncodeConfig
---@param decode? jsonschema.DecodeConfig
---
---@return any
function jsonschema.dereference(schema, encode, decode) end

return jsonschema
