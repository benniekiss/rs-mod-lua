-- SPDX-License-Identifier: MIT

--[[
a pandoc filter utilizing minijinja.

minijinja can be configured through the `minijinja` metadata key. Render context
can be set directly in the metadata, or the `minijinja.context` key can be set
to the filepath of a json or yaml file containing the context.

```yaml
---
minijinja:
    context:
        foo: "FOO"
        bar:
            - one
            - two
            - three
    reload_before_render: true
    keep_trailing_newline: false
    trim_blocks: false
    lstrip_blocks: false
    debug: false
    fuel: null
    recursion_limit: 500
    undefined_behavior: "lenient"
    pycompat: true
---
```

When provided as a filter, the document will be converted to markdown, rendered,
then converted back to a `pandoc.Pandoc` object.
]]

local minijinja = require("minijinja")

local filter = {}

--- Settings to configure minijinja rendering
---
---@class (exact) minijinja_pandoc_filter.JinjaSettings
---
---@field reload_before_render? boolean
---@field keep_trailing_newline? boolean
---@field trim_blocks? boolean
---@field lstrip_blocks? boolean
---@field debug? boolean
---@field fuel? integer
---@field recursion_limit? integer
---@field undefined_behavior? minijinja.UndefinedBehavior
---@field pycompat? boolean
---@field context? table
local JinjaSettings = {
    reload_before_render = nil,
    keep_trailing_newline = nil,
    trim_blocks = nil,
    lstrip_blocks = nil,
    debug = nil,
    fuel = nil,
    recursion_limit = nil,
    undefined_behavior = nil,
    pycompat = nil,
    context = nil,
}

local JSON_EXTS = pandoc.List({ ".json" })

local YAML_EXTS = pandoc.List({ ".yaml", ".yml" })

--- Read a file and return the contents, or nil if the file could not be read.
---
---@param path string
---
---@return string|nil
local function read_file(path)
    local file = io.open(path, "r")
    if not file then
        pandoc.log.error("failed to read file: " .. path)
        return
    end

    local content = file:read("a")
    file:close()

    return content
end

--- Normalize a pandoc.Meta into a lua table
---
--- markdown formatting is supported for context values
---
---@param meta pandoc.Meta
---
---@return table
local function normalize_metadata(meta)
    local ctx = {}

    for k, v in pairs(meta) do
        local t = pandoc.utils.type(v)

        if t == "Inlines" or t == "Blocks" then
            ---@cast v pandoc.Inlines,pandoc.Blocks

            if t == "Inlines" then
                v = pandoc.Blocks(v)
            end

            v = pandoc.write(pandoc.Pandoc(v), "markdown"):gsub("\n$", "")
        elseif t == "table" then
            ---@cast v table
            v = normalize_metadata(v)
        end

        ctx[k] = v
    end

    return ctx
end

--- Check if an extension is a JSON extension
---
---@param ext string
---
---@return boolean
local function has_json_ext(ext)
    return JSON_EXTS:includes(ext)
end

--- Check if an extension is a YAML extension
---
---@param ext string
---
---@return boolean
local function has_yaml_ext(ext)
    return YAML_EXTS:includes(ext)
end

--- Load a JSON file
---
---@param path string
---
---@return table|nil
local function load_json(path)
    local json = read_file(path)
    if json == nil then return end

    local ctx = pandoc.json.decode(json, false)

    if pandoc.utils.type(ctx) ~= "table" then
        pandoc.error("invalid json: ", pandoc.utils.stringify(ctx))
        return
    end

    return ctx
end

--- Load a YAML file
---
---@param path string
---
---@return table|nil
local function load_yaml(path)
    local yaml = read_file(path)
    if yaml == nil then return end

    local ctx = pandoc.read(yaml, "markdown").meta

    if pandoc.utils.type(ctx) ~= "table" then
        pandoc.error("invalid yaml: ", pandoc.utils.stringify(ctx))
        return
    end

    return ctx
end

--- Load a context from a JSON or YAML file
---
---@param path string
---
---@return table|nil
local function load_context_from_file(path)
    if not pandoc.path.exists(path) then
        pandoc.log.error("file does not exist: " .. path)
        return
    end

    local _, ext = pandoc.path.split_extension(path)
    local is_json = has_json_ext(ext)
    local is_yaml = has_yaml_ext(ext)

    if not (is_json or is_yaml) then
        pandoc.log.error("only JSON and YAML files are supported: " .. path)
    end

    if is_json then
        return load_json(path)
    end

    if is_yaml then
        return load_yaml(path)
    end
end

--- Load a minijinja context
---
---@param context string|table
local function load_context(context)
    local is_string = pandoc.utils.type(context) == "string"
    local is_table = pandoc.utils.type(context) == "table"

    if not (is_string or is_table) then
        pandoc.log.error("`context` must be a filepath or a table: " .. context)
        return
    end

    local ctx
    if is_table then
        ---@cast context table
        ctx = context
    end

    if is_string then
        ---@cast context string
        ctx = load_context_from_file(context)
    end

    if ctx ~= nil then
        ctx = normalize_metadata(ctx)
    end

    return ctx
end

--- Parse a pandoc yaml metadata for minijinja settings
---
---@param meta pandoc.Meta
local function Meta(meta)
    local mj = meta.minijinja

    if mj == nil then return end

    local context = mj.context
    if context ~= nil then
        JinjaSettings.context = load_context(context)
    end

    JinjaSettings.reload_before_render = mj.reload_before_render
    JinjaSettings.keep_trailing_newline = mj.keep_trailing_newline
    JinjaSettings.trim_blocks = mj.trim_blocks
    JinjaSettings.lstrip_blocks = mj.lstrip_blocks
    JinjaSettings.debug = mj.debug
    JinjaSettings.fuel = mj.fuel
    JinjaSettings.recursion_limit = mj.recursion_limit
    JinjaSettings.undefined_behavior = mj.undefined_behavior
    JinjaSettings.pycompat = mj.pycompat
end

--- Render a document as a minijinja template
---
---@param doc pandoc.Pandoc
---
---@return pandoc.Pandoc
function filter.Pandoc(doc)
    doc = doc:walk({ Meta = Meta })

    local env = minijinja.Environment:new()

    if JinjaSettings.reload_before_render ~= nil then
        env.reload_before_render = JinjaSettings.reload_before_render
    end

    if JinjaSettings.keep_trailing_newline ~= nil then
        env.keep_trailing_newline = JinjaSettings.keep_trailing_newline
    end

    if JinjaSettings.trim_blocks ~= nil then
        env.trim_blocks = JinjaSettings.trim_blocks
    end

    if JinjaSettings.lstrip_blocks ~= nil then
        env.lstrip_blocks = JinjaSettings.lstrip_blocks
    end

    if JinjaSettings.debug ~= nil then
        env.debug = JinjaSettings.debug
    end

    if JinjaSettings.fuel ~= nil then
        env.fuel = JinjaSettings.fuel
    end

    if JinjaSettings.recursion_limit ~= nil then
        env.recursion_limit = JinjaSettings.recursion_limit
    end

    if JinjaSettings.undefined_behavior ~= nil then
        env.undefined_behavior = JinjaSettings.undefined_behavior
    end

    env:set_pycompat(JinjaSettings.pycompat)

    local source = pandoc.write(doc, "markdown")
    local rendered = env:render_str(source, JinjaSettings.context, PANDOC_STATE.output_file)

    return pandoc.read(rendered, "markdown")
end

return filter
