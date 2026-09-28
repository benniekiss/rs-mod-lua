local jsonschema = require("jsonschema")

local Options = jsonschema.ValidationOptions

describe("options#jsonschema", function ()
    it("build#options", function ()
        local options = Options.new()
        local validator = options:build({ type = "integer" })
        assert.True(validator:is_valid(1))
        assert.False(validator:is_valid(1.5))
        assert.True(options:build('{"type":"boolean"}'):is_valid(true))
        assert.has_error(function () options:build({ type = "unknown" }) end)
    end)

    it("build_map#options", function ()
        local map = Options.new():build_map({ properties = { count = { type = "integer" } } })
        assert.True(map:contains_key("#"))
        assert.True(map:get("#/properties/count"):is_valid(1))
        assert.False(map:get("#/properties/count"):is_valid(false))
        assert.Nil(map:get("/missing"))
    end)

    it("with_draft#options", function ()
        local options = Options.new()
        local draft4 = options:with_draft(jsonschema.Draft.DRAFT4)
        assert.Equal(jsonschema.Draft.DRAFT4, draft4:build({}):draft())
        assert.has_error(function () options:build({}) end)
    end)

    it("should_validate_formats#options", function ()
        local schema = { format = "ipv4" }
        assert.True(
            Options.new():should_validate_formats(false):build(schema):is_valid('"invalid"')
        )
        local validator = Options.new()
            :should_validate_formats(true)
            :build(schema)
        assert.False(validator:is_valid('"invalid"'))
        assert.True(validator:is_valid('"127.0.0.1"'))
    end)

    it("should_ignore_unknown_formats#options", function ()
        local schema = { format = "not-a-format" }
        local options = Options.new():should_validate_formats(true)
        assert.True(
            options:should_ignore_unknown_formats(true):build(schema):is_valid('"anything"')
        )
        assert.has_error(function ()
            Options.new()
                :should_validate_formats(true)
                :should_ignore_unknown_formats(false)
                :build(schema)
        end)
    end)

    it("with_format#options", function ()
        local seen = {}
        local validator = Options.new():with_format("even-length", function (value)
            table.insert(seen, value)
            return #value % 2 == 0
        end):should_validate_formats(true):build({ format = "even-length" })
        assert.True(validator:is_valid('"ab"'))
        assert.False(validator:is_valid('"abc"'))
        assert.True(validator:validate('"abcd"'))
        assert.False(validator:validate('"abcde"'))
        assert.Same({ "ab", "abc", "abcd", "abcde" }, seen)
    end)

    it("handles format callback errors#options", function ()
        local validator = Options.new()
            :with_format("broken", function () error("format failed") end)
            :should_validate_formats(true)
            :build({ format = "broken" })
        assert.False(validator:is_valid('"value"'))
        assert.False(validator:validate('"value"'))
    end)

    it("with_keyword#options", function ()
        local calls = 0
        local factory = function (parent, value, location)
            calls = calls + 1
            assert.Same({ multipleOfCustom = 3 }, parent)
            assert.Equal(3, value)
            assert.Equal("/multipleOfCustom", location)
            return function (instance) return instance % value == 0 end, function (instance)
                    if instance % value ~= 0 then error("not a custom multiple") end
                end
        end
        local validator = Options.new():with_keyword(
            "multipleOfCustom",
            factory
        ):build({ multipleOfCustom = 3 })
        assert.Equal(1, calls)
        assert.True(validator:is_valid(6))
        assert.False(validator:is_valid(7))
        assert.True(validator:validate(6))
        local valid, err = validator:validate(7)
        assert.False(valid)
        assert.matches("not a custom multiple", err)
        assert.Equal(1, #validator:errors(7))
    end)

    it("with_retriever and with_base_uri#options", function ()
        local seen = {}
        local options = Options.new()
            :with_base_uri("https://example.com/schemas/")
            :with_retriever(function (uri)
                table.insert(seen, tostring(uri))
                assert.Equal("/schemas/count.json", uri.path)
                return { type = "integer", minimum = 0 }
            end)
        local schema = { ["$ref"] = "count.json" }
        local validator = options:build(schema)
        assert.True(validator:is_valid(2))
        assert.False(validator:is_valid(-1))
        assert.Same({ "https://example.com/schemas/count.json" }, seen)
        assert.True(options:build_map(schema):get("#"):is_valid(2))
        local bundled = options:bundle(schema)
        assert.is_table(bundled)
        assert.is_table(bundled["$defs"])
        local dereferenced = options:dereference(schema)
        assert.Nil(dereferenced["$ref"])
        assert.Equal("integer", dereferenced.type)
    end)

    it("propagates retriever errors#options", function ()
        local options = Options.new():with_retriever(function () error("missing schema") end)
        local ok, err = pcall(function ()
            options:build({ ["$ref"] = "https://example.com/missing" })
        end)
        assert.False(ok)
        assert.matches("missing schema", tostring(err))
    end)

    it("bundle and dereference#options", function ()
        local schema = {
            ["$defs"] = { count = { type = "integer" } },
            properties = { count = { ["$ref"] = "#/$defs/count" } },
        }
        local options = Options.new()
        assert.Same(schema, options:bundle(schema))
        assert.Same({ type = "integer" }, options:dereference(schema).properties.count)
    end)
end)

describe("email#options", function ()
    local function validator(options)
        return Options.new():with_email_options(options):should_validate_formats(true):build({
            format = "email",
        })
    end

    it("minimum subdomains#email", function ()
        local strict = validator(jsonschema.EmailOptions.new():with_minimum_sub_domains(3))
        assert.False(strict:is_valid('"user@example.com"'))
        assert.True(strict:is_valid('"user@sub.example.com"'))
        assert.True(
            validator(jsonschema.EmailOptions.new():with_no_minimum_sub_domains()):is_valid('"user@localhost"')
        )
        local tld = validator(jsonschema.EmailOptions.new():with_required_tld())
        assert.False(tld:is_valid('"user@localhost"'))
        assert.True(tld:is_valid('"user@example.com"'))
    end)

    it("domain literals#email", function ()
        local address = '"user@[127.0.0.1]"'
        assert.True(
            validator(jsonschema.EmailOptions.new():with_domain_literal()):is_valid(address)
        )
        assert.False(
            validator(jsonschema.EmailOptions.new():without_domain_literal()):is_valid(address)
        )
    end)

    it("display text#email", function ()
        local address = '"User <user@example.com>"'
        assert.True(validator(jsonschema.EmailOptions.new():with_display_text()):is_valid(address))
        assert.False(
            validator(jsonschema.EmailOptions.new():without_display_text()):is_valid(address)
        )
    end)
end)

describe("pattern#options", function ()
    it("configures regex limits#pattern", function ()
        local pattern = jsonschema.PatternOptions.new()
            :backtrack_limit(10000)
            :size_limit(1000000)
            :dfa_size_limit(1000000)
        local validator = Options.new()
            :with_pattern_options(pattern)
            :build({ pattern = "^a+$" })
        assert.True(validator:is_valid('"aaa"'))
        assert.False(validator:is_valid('"aba"'))
    end)

    it("rejects invalid patterns#pattern", function ()
        assert.has_error(function ()
            Options.new():with_pattern_options(jsonschema.PatternOptions.new()):build({
                pattern = "[",
            })
        end)
    end)
end)
