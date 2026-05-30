local minijinja = require("minijinja")
local Environment = minijinja.Environment

describe("State tests", function ()
    local function state_func(state, _)
        assert.Equal("my_template", state:name())
        assert.Equal("none", state:auto_escape())
        assert.Equal("lenient", state:undefined_behavior())
        assert.Equal("test_block", state:current_block())
        assert.Equal(true, state:lookup("bar").baz)
        assert.Equal(42, state:lookup("func")())
        assert.Equal("my macro", state:call_macro("my_macro"))
        assert.Same({ "my_macro" }, state:exports())

        local known = state:known_variables()
        table.sort(known)
        assert.Same({ "bar", "foo", "func", "lua", "my_macro" }, known)

        assert.Equal("HELLO", state:apply_filter("state_filter", "hello"))
        assert.Equal(false, state:perform_test("state_test", "foo", "bar"))
        assert.Equal("[1, 2, 3]", state:format({ 1, 2, 3 }))

        local c, r = table.unpack(state:fuel_levels())
        local total = c + r
        assert.Equal(1000, total)
        assert.True(c < total)

        return true
    end

    local function setup(src)
        local env = Environment:empty()

        env.fuel = 1000

        env:add_global("lua", state_func)
        env:add_filter(
            "state_filter",
            function (val) return val:upper() end,
            false
        )
        env:add_test(
            "state_test",
            function (val, arg) return val == arg end,
            false
        )

        local name = "my_template"
        local source = "{% macro my_macro() %}my macro{% endmacro %}{% block test_block %}" .. src
            .. "{% endblock test_block %}"

        local ctx = { foo = "foo", func = function () return 42 end, bar = { baz = true } }

        return env, name, source, ctx
    end

    it("filter#State", function ()
        local env, name, source, ctx = setup("{{ foo | lua }}")
        env:add_filter("lua", state_func)

        env:render_str(source, ctx, name)
    end)

    it("test#State", function ()
        local env, name, source, ctx = setup("{{ foo is lua }}")
        env:add_test("lua", state_func)

        env:render_str(source, ctx, name)
    end)

    it("global#State", function ()
        local env, name, source, ctx = setup("{{ lua() }}")

        env:render_str(source, ctx, name)
    end)

    it("temps#State", function ()
        local env = Environment:empty()

        local first = true

        local function inc(state)
            if first then
                assert.Nil(state:get_temp("counter"))
                first = false
            end

            local new = state:get_or_set_temp("counter", function () return 0 end) + 1
            state:set_temp("counter", new)
            return new
        end

        env:add_global("inc", inc)
        local rv = env:render_str("{{ inc() }} {{ inc() }} {{ inc() }}")
        assert.Equal("1 2 3", rv)
    end)
end)
