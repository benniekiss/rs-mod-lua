# Usage

Load the module and validate Lua values:

```lua
local jsonschema = require("jsonschema")

local schema = {
    type = "object",
    properties = { name = { type = "string" } },
    required = { "name" },
}

assert(jsonschema.is_valid(schema, { name = "Lua" }))
assert(not jsonschema.is_valid(schema, { name = 42 }))

local valid, err = jsonschema.validate(schema, { name = 42 })
assert(not valid)
print(err) -- 42 is not of type "string"


-- String inputs are parsed as JSON. Quote JSON string values, or put them inside
-- Lua tables. Use `jsonschema.None` for JSON null:

local jsonschema = require("jsonschema")

assert(jsonschema.is_valid([[{"type": "string"}]], [["hello"]]))
assert(jsonschema.is_valid({ type = "null" }, jsonschema.None))

-- Compile a validator to reuse a schema. Evaluation errors include locations:

local jsonschema = require("jsonschema")

local validator = jsonschema.validator_for({
    type = "object",
    properties = { age = { type = "integer", minimum = 0 } },
})

assert(validator:is_valid({ age = 20 }))

local evaluation = validator:evaluate({ age = -1 })
assert(not evaluation:flag().valid)
for _, entry in ipairs(evaluation:errors()) do
    print(entry.instance_location, entry.error.message)
    -- /age   -1 is less than the minimum of 0
end

-- Use validation options to register a custom format. Option setters return new
-- userdata, so chain calls or retain the returned value:

local jsonschema = require("jsonschema")

local validator = jsonschema.ValidationOptions.new()
    :with_format("starts-with-lua", function (value)
        return value:sub(1, 3) == "lua"
    end)
    :should_validate_formats(true)
    :build({ type = "string", format = "starts-with-lua" })

assert(validator:is_valid([["lua-module"]]))
assert(not validator:is_valid([["other-module"]]))
```

The API is documented in the [`library/jsonschema.d.lua`](../library/jsonschema.d.lua) file,
which should work with LuaLS or EmmyluaLS.
