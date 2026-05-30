local minijinja = require("minijinja")
local Environment = minijinja.Environment

describe("Environment tests", function ()
    describe("expressions#Environment", function ()
        it("basic#expressions", function ()
            local env = Environment:new()

            local rv = env:eval("1 + b", { b = 42 })
            assert.Equal(43, rv)
        end)

        it("globals#expressions", function ()
            local env = Environment:new()
            local rv

            env:add_global("life", 42)
            env:add_global("hello", function () return "hello world" end)

            rv = env:eval("life")
            assert.Equal(42, rv)

            rv = env:eval("hello()")
            assert.Equal("hello world", rv)
        end)

        it("callable#expressions", function ()
            local env = Environment:new()

            local magic = function () return { 1, 2, 3 } end

            local rv = env:eval("x()", { x = magic })
            assert.Same({ 1, 2, 3 }, rv)
        end)

        it("callable object#expressions", function ()
            local env = Environment:new()

            local foo = {}
            setmetatable(foo, foo)
            function foo.__call()
                return { 1, 2, 3 }
            end

            local rv = env:eval("x()", { x = foo })
            assert.Same({ 1, 2, 3 }, rv)
        end)

        it("methods#expressions", function ()
            local env = Environment:new()

            local foo = {}
            function foo:bar()
                return "bar"
            end

            local rv = env:eval("foo.bar()", { foo = foo })
            assert.Equal("bar", rv)
        end)

        it("attribute#expressions", function ()
            local env = Environment:new()

            local foo = {}
            foo.bar = 99

            local rv = env:eval("foo.bar", { foo = foo })
            assert.Equal(99, rv)
        end)

        it("nested attribute#expressions", function ()
            local env = Environment:new()

            local foo = {}
            function foo:bar()
                return "bar"
            end

            local baz = {}
            baz.buzz = foo

            local rv = env:eval("baz.buzz.bar()", { baz = baz })
            assert.Equal("bar", rv)
        end)

        it("undefined attribute#expressions", function ()
            local env = Environment:new()

            local foo = {}

            local rv = env:eval("foo.biz", { foo = foo })
            assert.Nil(rv)
        end)

        it("index array#expressions", function ()
            local env = Environment:new()
            local rv

            local foo = { 1, 2, 3 }

            -- bracket notation
            rv = env:eval("foo[0]", { foo = foo })
            assert.Equal(1, rv)

            rv = env:eval("foo[1]", { foo = foo })
            assert.Equal(2, rv)

            rv = env:eval("foo[2]", { foo = foo })
            assert.Equal(3, rv)

            -- dot notation
            rv = env:eval("foo.0", { foo = foo })
            assert.Equal(1, rv)

            rv = env:eval("foo.1", { foo = foo })
            assert.Equal(2, rv)

            rv = env:eval("foo.2", { foo = foo })
            assert.Equal(3, rv)
        end)

        it("index table#expressions", function ()
            local env = Environment:new()
            local rv

            local foo = { one = 1, two = 2, three = 3 }

            -- bracket notation
            rv = env:eval("foo['one']", { foo = foo })
            assert.Equal(1, rv)

            rv = env:eval("foo['two']", { foo = foo })
            assert.Equal(2, rv)

            rv = env:eval("foo['three']", { foo = foo })
            assert.Equal(3, rv)

            -- dot notation
            rv = env:eval("foo.one", { foo = foo })
            assert.Equal(1, rv)

            rv = env:eval("foo.two", { foo = foo })
            assert.Equal(2, rv)

            rv = env:eval("foo.three", { foo = foo })
            assert.Equal(3, rv)
        end)

        it("types#expressions", function ()
            local env = Environment:new()
            local x

            x = {}
            assert.Same(x, env:eval("x", { x = x }))

            x = { 1, 2, 3 }
            assert.Same(x, env:eval("x", { x = x }))

            x = { ["a"] = 1, ["b"] = 2, ["c"] = 3 }
            assert.Same(x, env:eval("x", { x = x }))

            x = { ["a"] = 42, ["b"] = 42.5, ["c"] = "blah" }
            assert.Same(x, env:eval("x", { x = x }))

            x = false
            assert.False(x, env:eval("x", { x = x }))

            x = nil
            assert.Nil(x, env:eval("x", { x = x }))

            x = minijinja.None
            assert.Same(x, env:eval("x", { x = x }))

            x = 99
            assert.Equal(x, env:eval("x", { x = x }))

            x = 99.99
            assert.Equal(x, env:eval("x", { x = x }))

            x = "foobar"
            assert.Equal(x, env:eval("x", { x = x }))
        end)

        it("filters#expressions", function ()
            local env = Environment:new()

            local filter = function (_, val) return val:upper() end

            env:add_filter("lua", filter)

            local rv = env:eval("'hello'|lua")
            assert.Equal("HELLO", rv)
        end)

        it("tests#expressions", function ()
            local env = Environment:new()

            local test = function (_, val) return val == "lua" end
            env:add_test("lua", test)

            assert.True(env:eval("'lua' is lua"))
            assert.False(env:eval("'LUA' is lua"))
        end)
    end)

    describe("templates#Environment", function ()
        it("custom-syntax#templates", function ()
            local env = Environment:new()

            env:set_syntax({
                variable_delimiters = { "${", "}" },
                block_delimiters = { "<%", "%>" },
                comment_delimiters = { "<!--", "-->" },
            })

            local rv = env:render_str("<% if true %>${ value }<% endif %><!-- nothing -->", {
                value = 42,
            })

            assert.Equal("42", rv)
        end)

        it("line-statements#templates", function ()
            local env = Environment:new()
            env:set_syntax({
                line_statement_prefix = "#",
                line_comment_prefix = "##",
            })

            local rv = env:render_str("# for x in range(3)\n{{ x }}\n# endfor")
            assert.Equal("0\n1\n2\n", rv)
        end)

        it("keep-trailing-newlines#templates", function ()
            local env = Environment:new()
            local source = "foo\n"

            assert.Equal("foo", env:render_str(source))

            env.keep_trailing_newline = true
            assert.Equal("foo\n", env:render_str(source))
        end)

        it("trim-blocks#templates", function ()
            local env = Environment:new()
            local source = "{% if true %}\nfoo{% endif %}"

            assert.Equal("\nfoo", env:render_str(source))

            env.trim_blocks = true
            assert.Equal("foo", env:render_str(source))
        end)

        it("lstrip-blocks#templates", function ()
            local env = Environment:new()
            local source = "  {% if true %}\nfoo{% endif %}"

            assert.Equal("  \nfoo", env:render_str(source))

            env.lstrip_blocks = true
            assert.Equal("\nfoo", env:render_str(source))
        end)

        it("trim_and_lstrip_blocks#templates", function ()
            local env = Environment:new()
            local source = "  {% if true %}\nfoo{% endif %}"

            assert.Equal("  \nfoo", env:render_str(source))

            env.trim_blocks = true
            env.lstrip_blocks = true
            assert.Equal("foo", env:render_str(source))
        end)

        it("fuel#templates", function ()
            local env = Environment:new()
            local source = "{% for i in ['' * 10] %}{{ i | fuel_check }}{% endfor %}"
            local fuel = 10

            local function fuel_check(state, _)
                local c, r = table.unpack(state:fuel_levels())
                local total = c + r
                assert.Equal(fuel, total)
                assert.True(c < total)
            end

            env:add_filter("fuel_check", fuel_check)

            env.fuel = fuel
            assert.Equal(fuel, env.fuel)
            assert.Not.Error(function ()
                env:render_str(source)
            end)

            env.fuel = fuel / 2
            assert.Equal(fuel / 2, env.fuel)
            assert.match_error(function ()
                env:render_str(source)
            end, "engine ran out of fuel")
        end)

        it("undeclared-variables#templates", function ()
            local env = Environment:new()

            env:add_template("foo.txt", "{{ x }} {{ bar.x }}")
            env:add_template("bar.txt", "{{ x }}")

            local udv

            udv = env:undeclared_variables("foo.txt")
            table.sort(udv)
            assert.Same({ "bar", "x" }, udv)

            -- nested
            udv = env:undeclared_variables("foo.txt", true)
            table.sort(udv)
            assert.Same({ "bar.x", "x" }, udv)

            assert.Same({ "x" }, env:undeclared_variables("bar.txt"))
        end)

        it("loop-controls#templates", function ()
            local env = Environment:new()

            local rv = env:render_str(
                [[
				{% for x in [1, 2, 3, 4, 5] %}
				{% if x == 1 %}
					{% continue %}
				{% elif x == 3 %}
					{% break %}
				{% endif %}
				{{ x }}
				{% endfor %}
			]]
            )

            assert.Equal("2", rv:match("^%s*(2)%s*$"))
        end)

        it("iterate array#templates", function ()
            local env = Environment:new()

            local foo = { 1, 2, 3 }

            local rv = env:render_str("{% for i in foo %}{{ i }}{% endfor %}", { foo = foo })
            assert.Equal("123", rv)
        end)

        it("iterate table#templates", function ()
            local env = Environment:new()
            local rv

            local foo = { one = 1, two = 2, three = 3 }

            -- since key order is not guaranteed, do a simple pattern match on the output
            rv = env:render_str("{% for k in foo %}{{ k }} {% endfor %}", { foo = foo })
            assert.match("one ", rv)
            assert.match("two ", rv)
            assert.match("three ", rv)

            rv = env:render_str("{% for k,v in foo | items %}{{ k }}: {{ v }}\n{% endfor %}", {
                foo = foo,
            })
            assert.match("one: 1\n", rv)
            assert.match("two: 2\n", rv)
            assert.match("three: 3\n", rv)
        end)

        it("sort#templates", function ()
            local env = Environment:new()

            local X = {}
            X.__index = X
            setmetatable(X, X)

            function X:new(x)
                local obj = setmetatable({}, self)
                obj.value = x
                obj.type = "obj"
                return obj
            end

            function X:__eq(other)
                if other.type ~= self.type then
                    return nil
                end
                return self.value == other.value
            end

            function X:__lt(other)
                if other.type ~= self.type then
                    return nil
                end
                return self.value < other.value
            end

            function X:__tostring()
                return tostring(self.value)
            end

            local values = { X:new(4), X:new(23), X:new(42), X:new(-1) }

            local rv = env:render_str("{{ values|sort|join(',') }}", { values = values })

            assert.Equal("-1,4,23,42", rv)
        end)

        it("path_loader#templates", function ()
            local env = Environment:new()
            local loader = minijinja.path_loader("spec/templates")
            env:set_loader(loader)

            local rv = env:render_template("base.txt", { woot = "woot" })
            assert.Equal("I am from foo! woot!", rv)

            assert.Error(function ()
                env:render_template("missing.txt")
            end)
            assert.Error(function ()
                env:render_template("../environment_spec.lua")
            end)
        end)

        it("fromjson#templates", function ()
            local env = Environment:new()

            local te = [[{"3":1,"2":{"b":1,"c":2,"a":3},"1":3}]]

            -- The filter should preserve key order
            local expr = "{% for k, v in te | fromjson | items %}{{ k }}: {{ v }} {% endfor %}"
            local rv = env:render_str(expr, { te = te })

            assert.Equal([[3: 1 2: {"b": 1, "c": 2, "a": 3} 1: 3 ]], rv)
        end)
    end)

    describe("callbacks#Environment", function ()
        it("loader#callbacks", function ()
            local env = Environment:new()

            local called = {}
            local function loader(name)
                table.insert(called, name)
                return "Hello from " .. name
            end

            env:set_loader(loader)

            assert.Equal("Hello from index.html", env:render_template("index.html"))
            assert.Equal("Hello from index.html", env:render_template("index.html"))
            assert.Equal("Hello from other.html", env:render_template("other.html"))

            assert.Equal("Hello from index.html", env:render_template("index.html"))
            assert.Same({ "index.html", "other.html" }, called)

            env:clear_templates()

            assert.Equal("Hello from index.html", env:render_template("index.html"))
            assert.Same({ "index.html", "other.html", "index.html" }, called)
        end)

        it("reload#callbacks", function ()
            local env = Environment:new()

            local called = {}
            local function loader(name)
                table.insert(called, name)
                return "Hello from " .. name
            end

            env:set_loader(loader)
            env.reload_before_render = true

            assert.Equal("Hello from index.html", env:render_template("index.html"))
            assert.Equal("Hello from index.html", env:render_template("index.html"))
            assert.Equal("Hello from other.html", env:render_template("other.html"))
            assert.Same({ "index.html", "index.html", "other.html" }, called)
        end)

        it("path-join#callbacks", function ()
            local env = Environment:new()

            local function path_join(name, parent)
                local dir = parent:match("^(.*)/") or ""
                return dir .. "/" .. name
            end

            env:set_path_join_callback(path_join)

            env:add_template("foo/bar.txt", "{% include 'baz.txt' %}")
            env:add_template("foo/baz.txt", "I am baz!")

            local rv = env:render_template("foo/bar.txt")
            assert.Equal("I am baz!", rv)
        end)

        it("uknown-method#callbacks", function ()
            local env = Environment:new()

            local function bar()
                return "bar"
            end

            local function unknown_method(state, value, method, args)
                assert.Equal("state", minijinja.type(state))
                assert.Same({}, value)
                assert.Equal("bar", method)
                assert.Same({ ["end"] = { 1, 2, 3 } }, args)

                return state:apply_filter("bar", value)
            end

            env:set_unknown_method_callback(unknown_method)
            env:add_filter("bar", bar)

            local rv = env:eval("foo.bar(1,2,3)", { foo = {} })
            assert.Equal("bar", rv)
        end)

        it("pycompat#callbacks", function ()
            local env = Environment:new()
            local source = "{'x': 42}.get('x')"

            env:set_pycompat()
            assert.Equal(42, env:eval(source))

            env:set_pycompat(false)
            assert.Error(function ()
                env:eval(source)
            end)
        end)

        it("autoescape#callbacks", function ()
            local env = Environment:new()
            local rv

            local function auto_escape(name)
                if name == "foo.html" then
                    return "html"
                end
                return "none"
            end

            env:set_auto_escape_callback(auto_escape)
            env:add_template("foo.html", "Hello {{ foo }}")
            env:add_template("invalid.html", "Hello {{ foo }}")

            rv = env:render_template("foo.html", { foo = "<x>" })
            assert.Equal("Hello &lt;x&gt;", rv)

            rv = env:render_template("invalid.html", { foo = "<x>" })
            assert.Equal("Hello <x>", rv)
        end)

        it("formatter#callbacks", function ()
            local env = Environment:new()

            local function formatter(state, value)
                local val
                local t = type(value)
                if t == "number" then
                    val = value * 2
                elseif t == "boolean" then
                    val = value and "TRUE" or "FALSE"
                elseif t == "function" then
                    val = "beep boop"
                elseif t == "userdata" then
                    val = "~~~~~~"
                elseif t == "string" then
                    val = value:upper()
                elseif t == "table" then
                    val = "<table>"
                end

                return tostring(val)
            end

            env:set_formatter(formatter)

            assert.Equal("20", env:render_str("{{ foo }}", { foo = 10 }))
            assert.Equal("TRUE", env:render_str("{{ foo }}", { foo = true }))
            assert.Equal("FALSE", env:render_str("{{ foo }}", { foo = false }))
            assert.Equal("FOO", env:render_str("{{ foo }}", { foo = "foo" }))
            assert.Equal("<table>", env:render_str("{{ foo }}", { foo = {} }))
            assert.Equal("beep boop", env:render_str("{{ foo }}", { foo = function () end }))
            assert.Equal("~~~~~~", env:render_str("{{ foo }}", { foo = minijinja.Environment }))
        end)
    end)
end)
