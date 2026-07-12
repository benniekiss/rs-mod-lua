local rsast = require("rsast")

grammar = [[
field = { (ASCII_DIGIT | "." | "-")+ }
record = { field ~ ("," ~ field)* }
file = { SOI ~ (record ~ ("\r\n" | "\n"))* ~ EOI }
]]

data = [[
65279,1179403647,1463895090
3.1415927,2.7182817,1.618034
-40,-273.15
13,42
65537
]]

describe("rsast", function ()
    assert:set_parameter("TableFormatLevel", 15)

    describe("ast", function ()
        it("new#ast", function ()
            local ast, errors = rsast.Ast.new(grammar)
            assert.Not.Nil(ast)
            assert.Nil(errors)
        end)

        it("new_error#ast", function ()
            local ast, errors = rsast.Ast.new("invalid grammar")
            assert.Nil(ast)
            assert.Same({
                [[
 --> 1:9
  |
1 | invalid grammar
  |         ^---
  |
  = expected assignment_operator]],
            }, errors)
        end)

        it("parse#ast", function ()
            local ast = rsast.Ast.new(grammar)
            ---@cast ast - nil

            local ex = {
                ["pairs"] = {
                    [1] = {
                        ["inner"] = "65279",
                        ["pos"] = {
                            [1] = 0,
                            [2] = 5,
                        },
                        ["rule"] = '"field"',
                    },
                },
                ["pos"] = {
                    [1] = 0,
                    [2] = 5,
                },
            }

            local res = ast:parse("field", data)

            assert.Same(ex, res)
        end)

        it("parse_callback#ast", function ()
            local ast = rsast.Ast.new(grammar)
            ---@cast ast - nil

            local ex = "65279"

            local res = ast:parse("field", data, function (pair) return pair:as_str() end)

            assert.Same(ex, res)
        end)

        it("parse_error#ast", function ()
            local ast = rsast.Ast.new(grammar)
            ---@cast ast - nil

            assert.matches_error(function ()
                ast:parse("file", "invalid data", function () end)
            end, [[
runtime error:  --> 1:1
  |
1 | invalid data
  | ^---
  |
  = expected "file"]], nil, true)
        end)
    end)

    describe("pairs", function ()
        local ast = rsast.Ast.new(grammar)
        ---@cast ast - nil

        it("as_str#pairs", function ()
            local res = ast:parse("record", data, function (pairs) return pairs:as_str() end)

            local ex = "65279,1179403647,1463895090"

            assert.Equal(ex, res)
        end)

        it("get_input#pairs", function ()
            local res = ast:parse("record", data, function (pairs) return pairs:get_input() end)

            assert.Equal(data, res)
        end)

        it("concat#pairs", function ()
            local res = ast:parse("record", data, function (pairs) return pairs:concat() end)

            local ex = "65279,1179403647,1463895090"

            assert.Equal(ex, res)
        end)

        it("is_empty#pairs", function ()
            local res = ast:parse("record", data, function (pairs) return pairs:is_empty() end)

            assert.False(res)

            res = ast:parse("record", data, function (pairs)
                pairs:next()
                return pairs:is_empty()
            end)

            assert.True(res)
        end)

        it("peek#pairs", function ()
            local res = ast:parse("record", data, function (pairs)
                return pairs:peek(function (pair)
                    return pair:pairs(function (ps) return ps:peek() end)
                end)
            end)

            local ex = {
                ["inner"] = "65279",
                ["pos"] = {
                    [1] = 0,
                    [2] = 5,
                },
                ["rule"] = '"field"',
            }

            assert.Same(ex, res)
        end)

        it("peek_callback#pairs", function ()
            local res = ast:parse("record", data, function (pairs)
                return pairs:peek(function (pair)
                    return pair:pairs(
                        function (ps) return ps:peek(function (p) return p:as_str() end) end
                    )
                end)
            end)

            local ex = "65279"

            assert.Equal(ex, res)
        end)

        it("next#pairs", function ()
            local res = ast:parse("record", data, function (pairs)
                return pairs:next(function (pair)
                    return pair:pairs(
                        function (ps) return ps:next() end
                    )
                end)
            end)

            local ex = {
                ["inner"] = "65279",
                ["pos"] = {
                    [1] = 0,
                    [2] = 5,
                },
                ["rule"] = '"field"',
            }

            assert.Same(ex, res)
        end)

        it("next_callback#pairs", function ()
            local res = ast:parse("record", data, function (pairs)
                return pairs:next(function (pair)
                    return pair:pairs(
                        function (ps) return ps:next(function (p) return p:as_str() end) end
                    )
                end)
            end)

            local ex = "65279"

            assert.Equal(ex, res)
        end)

        it("next_back#pairs", function ()
            local res = ast:parse("record", data, function (pairs)
                return pairs:next_back(function (pair)
                    return pair:pairs(
                        function (ps) return ps:next_back() end
                    )
                end)
            end)

            local ex = {
                ["inner"] = "1463895090",
                ["pos"] = {
                    [1] = 17,
                    [2] = 27,
                },
                ["rule"] = '"field"',
            }

            assert.Same(ex, res)
        end)

        it("next_back_callback#pairs", function ()
            local res = ast:parse("record", data, function (pairs)
                return pairs:next_back(function (pair)
                    return pair:pairs(
                        function (ps) return ps:next_back(function (p) return p:as_str() end) end
                    )
                end)
            end)

            local ex = "1463895090"

            assert.Equal(ex, res)
        end)

        it("tokens#pairs", function () end)
        it("tokens_flat#pairs", function () end)

        it("dump#pairs", function () end)
        it("dump_flat#pairs", function () end)

        describe("for_each#pairs", function () end)
        describe("for_each_flat#pairs", function () end)
    end)
end)
