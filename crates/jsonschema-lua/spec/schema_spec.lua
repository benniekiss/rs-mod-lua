local jsonschema = require("jsonschema")

local schema = {
    type = "object",
    properties = { count = { type = "integer", minimum = 0 } },
    required = { "count" },
    additionalProperties = false,
}

local reference_schema = {
    ["$defs"] = { count = { type = "integer", minimum = 0 } },
    type = "object",
    properties = { count = { ["$ref"] = "#/$defs/count" } },
}

local function check_validator(validator)
    assert.True(validator:is_valid({ count = 2 }))
    assert.False(validator:is_valid({ count = -1 }))
    local valid, err = validator:validate({ count = 2 })
    assert.True(valid)
    assert.Nil(err)
    valid, err = validator:validate({ count = -1 })
    assert.False(valid)
    assert.Equal("-1 is less than the minimum of 0", err)
    assert.Same({}, validator:errors({ count = 2 }))
    assert.Same({ err }, validator:errors({ count = -1 }))
    assert.True(validator:evaluate({ count = 2 }):flag().valid)
    assert.False(validator:evaluate({ count = -1 }):flag().valid)
    assert.Equal(jsonschema.Draft.DRAFT202012, validator:draft())
end

local function check_map(map)
    assert.True(map:contains_key("#"))
    assert.True(map:contains_key("#/properties/count"))
    assert.False(map:contains_key("#/properties/missing"))
    assert.Nil(map:get("#/properties/missing"))
    check_validator(map:get("#"))
    local count = map:get("#/properties/count")
    assert.True(count:is_valid(2))
    assert.False(count:is_valid(-1))
    local keys = map:keys()
    assert.True(#keys > 0)
    for _, key in ipairs(keys) do
        assert.is_string(key)
        assert.True(map:contains_key(key))
        assert.is_userdata(map:get(key))
    end
end

describe("meta#jsonschema", function ()
    it("is_valid#meta", function ()
        assert.True(jsonschema.meta.is_valid(schema))
        assert.True(jsonschema.meta.is_valid(true))
        assert.True(jsonschema.meta.is_valid(false))
        assert.False(jsonschema.meta.is_valid({ type = "unknown" }))
    end)

    it("validate#meta", function ()
        local valid, err = jsonschema.meta.validate(schema)
        assert.True(valid)
        assert.Nil(err)
        valid, err = jsonschema.meta.validate({ type = "unknown" })
        assert.False(valid)
        assert.is_string(err)
        assert.matches("unknown", err)
    end)

    it("validator_for#meta", function ()
        local validator = jsonschema.meta.validator_for({
            ["$schema"] = "http://json-schema.org/draft-07/schema#",
        })
        assert.True(validator:is_valid(schema))
        assert.False(validator:is_valid({ type = "unknown" }))
    end)

    it("accepts JSON text#meta", function ()
        assert.True(jsonschema.meta.is_valid('{"type":"integer"}'))
        assert.False(jsonschema.meta.validate('{"type":"unknown"}'))
        assert.True(jsonschema.meta.validator_for("{}"):is_valid(schema))
    end)
end)

describe("schema#jsonschema", function ()
    it("is_valid#schema", function ()
        assert.True(jsonschema.is_valid(schema, { count = 2 }))
        assert.False(jsonschema.is_valid(schema, { count = -1 }))
        assert.True(jsonschema.is_valid(true, jsonschema.None))
        assert.False(jsonschema.is_valid(false, 2))
        assert.True(jsonschema.is_valid({ type = "null" }, jsonschema.None))
    end)

    it("validate#schema", function ()
        local valid, err = jsonschema.validate(schema, { count = 2 })
        assert.True(valid)
        assert.Nil(err)
        valid, err = jsonschema.validate(schema, { count = -1 })
        assert.False(valid)
        assert.Equal("-1 is less than the minimum of 0", err)
    end)

    it("accepts JSON text#schema", function ()
        assert.True(jsonschema.is_valid('{"type":"string"}', '"hello"'))
        assert.False(jsonschema.is_valid('{"type":"integer"}', '"hello"'))
        assert.True(jsonschema.validate(schema, '{"count":2}'))
        assert.True(jsonschema.validator_for('{"type":"integer"}'):is_valid("2"))
    end)

    it("passes encode options to schema and validator functions#schema", function ()
        local options = jsonschema.EncodeConfig.new()
        options:set_encode_empty_tables_as_array(true)
        local array_schema = { type = "array" }
        assert.False(jsonschema.is_valid(array_schema, {}))
        assert.True(jsonschema.is_valid(array_schema, {}, options))
        assert.True(jsonschema.validate(array_schema, {}, options))
        assert.True(jsonschema.evaluate(array_schema, {}, options):flag().valid)
        local validator = jsonschema.validator_for(array_schema)
        assert.True(validator:is_valid({}, options))
        assert.True(validator:validate({}, options))
        assert.Same({}, validator:errors({}, options))
        assert.True(validator:evaluate({}, options):flag().valid)
    end)

    it("rejects malformed JSON and invalid schemas#schema", function ()
        assert.has_error(function () jsonschema.is_valid("{", 2) end)
        assert.has_error(function () jsonschema.validate(schema, "{") end)
        assert.has_error(function () jsonschema.validator_for({ type = "unknown" }) end)
        assert.has_error(function () jsonschema.validator_map_for({ type = "unknown" }) end)
    end)

    it("evaluate#schema", function ()
        local validator = jsonschema.validator_for(schema)
        for _, instance in ipairs({ { count = 2 }, { count = -1 } }) do
            local evaluation = jsonschema.evaluate(schema, instance)
            local expected = validator:evaluate(instance)
            assert.Same(expected:flag(), evaluation:flag())
            assert.Same(expected:list(), evaluation:list())
            assert.Same(expected:hierarchical(), evaluation:hierarchical())
            assert.Same(expected:errors(), evaluation:errors())
            assert.Same(expected:annotations(), evaluation:annotations())
        end
    end)

    it("validator_for#schema", function () check_validator(jsonschema.validator_for(schema)) end)
    it("validator_map_for#schema", function () check_map(jsonschema.validator_map_for(schema)) end)

    it("bundle#schema", function ()
        assert.Same(reference_schema, jsonschema.bundle(reference_schema))
        assert.Equal('{"type":"integer"}', jsonschema.bundle('{"type":"integer"}'))
    end)

    it("dereference#schema", function ()
        local result = jsonschema.dereference(reference_schema)
        assert.Same({ type = "integer", minimum = 0 }, result.properties.count)
        assert.Nil(result.properties.count["$ref"])
        assert.Equal("#/$defs/count", reference_schema.properties.count["$ref"])
        assert.True(jsonschema.is_valid(result, { count = 2 }))
        assert.False(jsonschema.is_valid(result, { count = -1 }))
        assert.Equal('{"type":"integer"}', jsonschema.dereference('{"type":"integer"}'))
    end)

    it("passes decode options to reference operations#schema", function ()
        local input = { default = jsonschema.None }
        local decode = jsonschema.DecodeConfig.new()
        decode:set_null(true)
        for _, operation in ipairs({ jsonschema.bundle, jsonschema.dereference }) do
            assert.Equal(jsonschema.None, operation(input, nil, decode).default)
            decode:set_null(false)
            assert.Nil(operation(input, nil, decode).default)
            decode:set_null(true)
        end
    end)

    it("rejects unresolved references#schema", function ()
        local unresolved = { ["$ref"] = "#/$defs/missing" }
        assert.has_error(function () jsonschema.validator_for(unresolved) end)
        assert.has_error(function () jsonschema.dereference(unresolved) end)
    end)
end)

-- The module build does not enable the optional async feature. Local references
-- complete without an IO runtime when this suite runs against an async build.
if jsonschema.async then
    describe("async#jsonschema", function ()
        local function run(func, value)
            local thread = coroutine.create(function () return func(value) end)
            local ok, result = coroutine.resume(thread)
            assert.True(ok, tostring(result))
            assert.Equal("dead", coroutine.status(thread))
            return result
        end

        it("bundle#async", function ()
            assert.Same(
                jsonschema.bundle(reference_schema),
                run(jsonschema.async.bundle, reference_schema)
            )
        end)

        it("dereference#async", function ()
            assert.Same(
                jsonschema.dereference(reference_schema),
                run(jsonschema.async.dereference, reference_schema)
            )
        end)

        it(
            "validator_for#async",
            function () check_validator(run(jsonschema.async.validator_for, schema)) end
        )
        it(
            "validator_map_for#async",
            function () check_map(run(jsonschema.async.validator_map_for, schema)) end
        )
    end)
end
