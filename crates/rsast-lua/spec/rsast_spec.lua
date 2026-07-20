local rsast = require("rsast")

grammar = [[
field = { (ASCII_DIGIT | "." | "-")+ }
record = { #tag = field ~ ("," ~ field)* }
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

    describe("ast#rsast", function ()
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

        it("validate#ast", function ()
            local ast = rsast.Ast.new(grammar)
            ---@cast ast - nil

            local res = ast:validate("field", data)

            assert.True(res)
        end)

        it("validate_invalid#ast", function ()
            local ast = rsast.Ast.new(grammar)
            ---@cast ast - nil

            local res, err = ast:validate("file", "123foobar")

            local ex_error = [[
 --> 1:1
  |
1 | 123foobar
  | ^---
  |
  = expected "EOI"]]
            assert.False(res)
            assert.Equal(ex_error, err)
        end)

        it("parse#ast", function ()
            local ast = rsast.Ast.new(grammar)
            ---@cast ast - nil

            local ex = "65279"

            local res, foo = ast:parse("field", data, function (pair)
                return pair:as_str(), "foo"
            end)

            assert.Same(ex, res)
            assert.Equal("foo", foo)
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

    describe("pairs#rsast", function ()
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
                ---@diagnostic disable-next-line: need-check-nil
                return pairs:peek()
                    :pairs()
                    :peek()
                    :dump()
            end)

            local ex = { ["node_tag"] = "tag", ["start"] = 0, ["stop"] = 5, ["rule"] = "field" }

            assert.Same(ex, res)
        end)

        it("next#pairs", function ()
            local res = ast:parse("record", data, function (pairs)
                ---@diagnostic disable-next-line: need-check-nil
                return pairs:next()
                    :pairs()
                    :next()
                    :dump()
            end)

            local ex = { ["node_tag"] = "tag", ["start"] = 0, ["stop"] = 5, ["rule"] = "field" }

            assert.Same(ex, res)
        end)

        it("next_back#pairs", function ()
            local res = ast:parse("record", data, function (pairs)
                ---@diagnostic disable-next-line: need-check-nil
                return pairs:next_back()
                    :pairs()
                    :next_back()
                    :dump()
            end)

            local ex = { ["start"] = 17, ["stop"] = 27, ["rule"] = "field" }

            assert.Same(ex, res)
        end)

        it("iter#pairs", function ()
            local res = ast:parse("record", data, function (pairs)
                local rules = {}

                local record = pairs:next()
                ---@cast record - nil

                table.insert(rules, record:start())

                for p in record:pairs():iter() do
                    table.insert(rules, p:start())
                end

                return rules
            end)

            local ex = {
                0,
                0,
                6,
                17,
            }

            assert.Same(ex, res)
        end)

        it("reviter#pairs", function ()
            local res = ast:parse("record", data, function (pairs)
                local rules = {}

                local record = pairs:next()
                ---@cast record - nil

                table.insert(rules, record:start())

                for p in record:pairs():reviter() do
                    table.insert(rules, p:start())
                end

                return rules
            end)

            local ex = {
                0,
                17,
                6,
                0,
            }

            assert.Same(ex, res)
        end)

        it("flatten#pairs", function ()
            local res = ast:parse("record", data, function (pairs)
                local flat = pairs:flatten()

                local rules = {}
                for p in flat:iter() do
                    table.insert(rules, p:start())
                end
                return rules
            end)

            local ex = {
                0,
                0,
                6,
                17,
            }

            assert.Same(ex, res)
        end)

        it("dump#pairs", function ()
            local res = ast:parse("record", data, function (pairs) return pairs:dump() end)

            local ex = {
                ['pairs'] = {
                    [1] = {
                        ['pairs'] = {
                            ['pairs'] = {
                                [1] = {
                                    ["node_tag"] = "tag",
                                    ['rule'] = "field",
                                    ['start'] = 0,
                                    ['stop'] = 5,
                                },
                                [2] = {
                                    ['rule'] = "field",
                                    ['start'] = 6,
                                    ['stop'] = 16,
                                },
                                [3] = {
                                    ['rule'] = "field",
                                    ['start'] = 17,
                                    ['stop'] = 27,
                                },
                            },
                            ['start'] = 0,
                            ['stop'] = 27,
                        },
                        ['rule'] = "record",
                        ['start'] = 0,
                        ['stop'] = 27,
                    },
                },
                ['start'] = 0,
                ['stop'] = 27,
            }

            assert.Same(ex, res)
        end)
    end)

    describe("pair#rsast", function ()
        local ast = rsast.Ast.new(grammar)
        ---@cast ast - nil

        it("start#pair", function ()
            local res = ast:parse("record", data, function (pairs)
                ---@diagnostic disable-next-line: need-check-nil
                local p = pairs:next():pairs()
                p:next()
                ---@diagnostic disable-next-line: need-check-nil
                return p:next():start()
            end)

            local ex = 6

            assert.Equal(ex, res)
        end)

        it("stop#pair", function ()
            local res = ast:parse("record", data, function (pairs)
                ---@diagnostic disable-next-line: need-check-nil
                local p = pairs:next():pairs()
                p:next()
                ---@diagnostic disable-next-line: need-check-nil
                return p:next():stop()
            end)

            local ex = 16

            assert.Equal(ex, res)
        end)

        it("as_rule#pair", function ()
            local res = ast:parse("record", data, function (pairs)
                ---@diagnostic disable-next-line: need-check-nil
                local p = pairs:next():pairs()
                p:next()
                ---@diagnostic disable-next-line: need-check-nil
                return p:next():as_rule()
            end)

            local ex = "field"

            assert.Equal(ex, res)
        end)

        it("as_str#pair", function ()
            local res = ast:parse(
                "record",
                data,
                ---@diagnostic disable-next-line: need-check-nil
                function (pairs) return pairs:next():as_str() end
            )

            local ex = "65279,1179403647,1463895090"

            assert.Equal(ex, res)
        end)

        it("as_node_tag#pair", function ()
            local res = ast:parse("record", data, function (pairs)
                ---@diagnostic disable-next-line: need-check-nil
                return pairs:next()
                    :pairs()
                    :next()
                    :as_node_tag()
            end)

            local ex = "tag"

            assert.Same(ex, res)
        end)

        it("get_input#pair", function ()
            local res = ast:parse("record", data, function (pairs)
                ---@diagnostic disable-next-line: need-check-nil
                return pairs:next():get_input()
            end)

            assert.Equal(data, res)
        end)

        it("line_col#pair", function ()
            local res_line, res_col = ast:parse("record", data, function (pairs)
                ---@diagnostic disable-next-line: need-check-nil
                local p = pairs:next():pairs()
                p:next()
                ---@diagnostic disable-next-line: need-check-nil
                return p:next():line_col()
            end)

            local ex_line = 1
            local ex_col = 7

            assert.Equal(ex_line, res_line)
            assert.Equal(ex_col, res_col)
        end)

        it("dump#pair", function ()
            local res = ast:parse("field", data, function (pairs)
                ---@diagnostic disable-next-line: need-check-nil
                return pairs:next():dump()
            end)

            local ex = { ["rule"] = "field", ["start"] = 0, ["stop"] = 5 }

            assert.Same(ex, res)
        end)

        it("pairs#pair", function ()
            local res = ast:parse(
                "record",
                data,
                ---@diagnostic disable-next-line: need-check-nil
                function (pairs) return pairs:next():pairs():dump() end
            )

            local ex = {
                ["pairs"] = {
                    [1] = {
                        ["node_tag"] = "tag",
                        ["rule"] = "field",
                        ["start"] = 0,
                        ["stop"] = 5,
                    },
                    [2] = {
                        ["rule"] = "field",
                        ["start"] = 6,
                        ["stop"] = 16,
                    },
                    [3] = {
                        ["rule"] = "field",
                        ["start"] = 17,
                        ["stop"] = 27,
                    },
                },
                ["start"] = 0,
                ["stop"] = 27,
            }

            assert.Same(ex, res)
        end)
    end)
end)
