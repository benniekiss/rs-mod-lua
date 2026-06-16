local json = require("rsjson")

describe("config", function ()
    describe("EncodeConfig#config", function ()
        it("EncodeConfig:new()#config", function ()
            assert.no_error(function ()
                json.EncodeConfig:new()
            end)
        end)

        local conf = json.EncodeConfig:new()

        it("EncodeConfig.indent#config", function ()
            conf.indent = nil
            assert.Nil(conf.indent)

            conf.indent = 4
            assert.Equal(4, conf.indent)
        end)

        it("EncodeConfig.prefix#config", function ()
            assert.Equal(conf.prefix, " ")

            conf.prefix = "foo"
            assert.Equal("foo", conf.prefix)
        end)

        it("EncodeConfig.sort_keys#config", function ()
            conf.sort_keys = true
            assert.True(conf.sort_keys)

            conf.sort_keys = false
            assert.False(conf.sort_keys)
        end)

        it("EncodeConfig.empty_table_as_array#config", function ()
            conf.empty_table_as_array = true
            assert.True(conf.empty_table_as_array)

            conf.empty_table_as_array = false
            assert.False(conf.empty_table_as_array)
        end)

        it("EncodeConfig.detect_mixed_tables#config", function ()
            conf.detect_mixed_tables = true
            assert.True(conf.detect_mixed_tables)

            conf.detect_mixed_tables = false
            assert.False(conf.detect_mixed_tables)
        end)

        it("EncodeConfig.error_unsupported#config", function ()
            conf.error_unsupported = true
            assert.True(conf.error_unsupported)

            conf.error_unsupported = false
            assert.False(conf.error_unsupported)
        end)

        it("EncodeConfig.error_cycles#config", function ()
            conf.error_cycles = true
            assert.True(conf.error_cycles)

            conf.error_cycles = false
            assert.False(conf.error_cycles)
        end)
    end)

    describe("DecodeConfig#config", function ()
        it("DecodeConfig:new()#config", function ()
            assert.no_error(function ()
                json.DecodeConfig:new()
            end)
        end)

        local conf = json.DecodeConfig:new()

        it("DecodeConfig.null#config", function ()
            conf.null = true
            assert.True(conf.null)

            conf.null = false
            assert.False(conf.null)
        end)

        it("DecodeConfig.cast_u64_to_f64#config", function ()
            conf.cast_u64_to_f64 = true
            assert.True(conf.cast_u64_to_f64)

            conf.cast_u64_to_f64 = false
            assert.False(conf.cast_u64_to_f64)
        end)

        it("DecodeConfig.set_array_mt#config", function ()
            conf.set_array_mt = true
            assert.True(conf.set_array_mt)

            conf.set_array_mt = false
            assert.False(conf.set_array_mt)
        end)
    end)
end)

describe("encode", function ()
    it("table#encode", function ()
        local te = { one = 1, two = 2, three = 3 }
        -- Since key order is not guaranteed, use substring matching
        local ex = {
            '"one":1',
            '"two":2',
            '"three":3',
        }

        for _, p in ipairs(ex) do
            assert.match(p, json.encode(te))
        end
    end)

    it("array#encode", function ()
        local te = { "one", 2, "three" }
        local ex = '["one",2,"three"]'

        assert.Equal(ex, json.encode(te))
    end)

    it("array_mt#encode", function ()
        local config = json.EncodeConfig:new()

        local te = { foo = "bar" }
        local ex = "[]"

        setmetatable(te, json.array_mt)

        local res = json.encode(te, config)

        assert.Equal(ex, res)
    end)

    it("no_array_mt#encode", function ()
        local config = json.EncodeConfig:new()

        local te = { foo = "bar" }
        local ex = '{"foo":"bar"}'
        local res = json.encode(te, config)

        assert.Equal(ex, res)
    end)

    it("string#encode", function ()
        local te = "a very 'long' string with ∆ unicode ∆"
        local ex = "\"a very 'long' string with ∆ unicode ∆\""

        assert.Equal(ex, json.encode(te))
    end)

    it("integer#encode", function ()
        local te = 123
        local ex = "123"

        assert.Equal(ex, json.encode(te))
    end)

    it("integer#encode", function ()
        local te = 123.999
        local ex = "123.999"

        assert.Equal(ex, json.encode(te))
    end)

    it("null#encode", function ()
        local te = json.null
        local ex = "null"

        assert.Equal(ex, json.encode(te))
    end)
end)

describe("decode", function ()
    it("table#decode", function ()
        local te = '{"one":1,"two":2,"three":3}'
        -- Since key order is not guaranteed, use substring matching
        local ex = { one = 1, two = 2, three = 3 }

        assert.Same(ex, json.decode(te))
    end)

    it("array#decode", function ()
        local te = '["one",2,"three"]'
        local ex = { "one", 2, "three" }

        assert.Same(ex, json.decode(te))
    end)

    it("array_mt#decode", function ()
        local config = json.DecodeConfig:new()
        config.set_array_mt = true

        local te = '["one",2,"three"]'
        local ex = { "one", 2, "three" }

        local res = json.decode(te, config)
        local mt = debug.getmetatable(res)

        assert.Same(ex, res)
        assert.Equal(mt, json.array_mt)
    end)

    it("no_array_mt#decode", function ()
        local config = json.DecodeConfig:new()
        config.set_array_mt = false

        local te = '["one",2,"three"]'
        local ex = { "one", 2, "three" }

        local res = json.decode(te, config)
        local mt = debug.getmetatable(res)

        assert.Same(ex, res)
        assert.Nil(mt)
    end)

    it("string#decode", function ()
        local te = "\"a very 'long' string with ∆ unicode ∆\""
        local ex = "a very 'long' string with ∆ unicode ∆"

        assert.Equal(ex, json.decode(te))
    end)

    it("integer#decode", function ()
        local te = "123"
        local ex = 123

        assert.Equal(ex, json.decode(te))
    end)

    it("number#decode", function ()
        local te = "123.999"
        local ex = 123.999

        assert.Equal(ex, json.decode(te))
    end)

    it("null#decode", function ()
        local te = "null"
        local ex = json.null

        assert.Equal(ex, json.decode(te))
    end)
end)
