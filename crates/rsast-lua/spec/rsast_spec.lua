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
            local res, foo = ast:parse("record", data, function (pairs)
                return pairs:peek(function (pair)
                    return pair:pairs(function (ps)
                        return ps:peek(function (p)
                            return p:as_str(), "foo"
                        end)
                    end)
                end)
            end)

            local ex = "65279"

            assert.Equal(ex, res)
            assert.Equal("foo", foo)
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
            local res, foo = ast:parse("record", data, function (pairs)
                return pairs:next(function (pair)
                    return pair:pairs(function (ps)
                        return ps:next(function (p)
                            return p:as_str(), "foo"
                        end)
                    end)
                end)
            end)

            local ex = "65279"

            assert.Equal(ex, res)
            assert.Equal("foo", foo)
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
            local res, foo = ast:parse("record", data, function (pairs)
                return pairs:next_back(function (pair)
                    return pair:pairs(function (ps)
                        return ps:next_back(function (p)
                            return p:as_str(), "foo"
                        end)
                    end)
                end)
            end)

            local ex = "1463895090"

            assert.Equal(ex, res)
            assert.Equal("foo", foo)
        end)

        it("tokens#pairs", function ()
            local res = ast:parse("field", data, function (pairs) return pairs:tokens() end)

            local ex = {
                [1] = {
                    ["pos"] = 0,
                    ["rule"] = "field",
                    ["type"] = "start",
                },
                [2] = {
                    ["pos"] = 5,
                    ["rule"] = "field",
                    ["type"] = "end",
                },
            }

            assert.Same(ex, res)
        end)

        it("tokens_callback#pairs", function ()
            local res_first, res_last, res_none = ast:parse("field", data, function (pairs)
                return pairs:tokens(function (tokens)
                    local last = tokens:next_back()
                    local first = tokens:next()
                    local none = tokens:next()

                    return first, last, none
                end)
            end)

            local ex_first = { ["pos"] = 0, ["rule"] = "field", ["type"] = "start" }
            local ex_last = { ["pos"] = 5, ["rule"] = "field", ["type"] = "end" }

            assert.Same(ex_first, res_first)
            assert.Same(ex_last, res_last)
            assert.Nil(res_none)
        end)

        it("dump#pairs", function ()
            local res = ast:parse("record", data, function (pairs) return pairs:dump() end)

            local ex = {
                [1] = {
                    ["inner"] = {
                        ["pairs"] = {
                            [1] = {
                                ["inner"] = "65279",
                                ["pos"] = {
                                    [1] = 0,
                                    [2] = 5,
                                },
                                ["rule"] = '"field"',
                            },
                            [2] = {
                                ["inner"] = "1179403647",
                                ["pos"] = {
                                    [1] = 6,
                                    [2] = 16,
                                },
                                ["rule"] = '"field"',
                            },
                            [3] = {
                                ["inner"] = "1463895090",
                                ["pos"] = {
                                    [1] = 17,
                                    [2] = 27,
                                },
                                ["rule"] = '"field"',
                            },
                        },
                        ["pos"] = {
                            [1] = 0,
                            [2] = 27,
                        },
                    },
                    ["pos"] = {
                        [1] = 0,
                        [2] = 27,
                    },
                    ["rule"] = '"record"',
                },
            }
            assert.Same(ex, res)
        end)

        it("dump_flat#pairs", function ()
            local res = ast:parse("record", data, function (pairs) return pairs:dump_flat() end)

            local ex = {
                [1] = {
                    ["inner"] = {
                        ["pairs"] = {
                            [1] = {
                                ["inner"] = "65279",
                                ["pos"] = {
                                    [1] = 0,
                                    [2] = 5,
                                },
                                ["rule"] = '"field"',
                            },
                            [2] = {
                                ["inner"] = "1179403647",
                                ["pos"] = {
                                    [1] = 6,
                                    [2] = 16,
                                },
                                ["rule"] = '"field"',
                            },
                            [3] = {
                                ["inner"] = "1463895090",
                                ["pos"] = {
                                    [1] = 17,
                                    [2] = 27,
                                },
                                ["rule"] = '"field"',
                            },
                        },
                        ["pos"] = {
                            [1] = 0,
                            [2] = 27,
                        },
                    },
                    ["pos"] = {
                        [1] = 0,
                        [2] = 27,
                    },
                    ["rule"] = '"record"',
                },
                [2] = {
                    ["inner"] = "65279",
                    ["pos"] = {
                        [1] = 0,
                        [2] = 5,
                    },
                    ["rule"] = '"field"',
                },
                [3] = {
                    ["inner"] = "1179403647",
                    ["pos"] = {
                        [1] = 6,
                        [2] = 16,
                    },
                    ["rule"] = '"field"',
                },
                [4] = {
                    ["inner"] = "1463895090",
                    ["pos"] = {
                        [1] = 17,
                        [2] = 27,
                    },
                    ["rule"] = '"field"',
                },
            }

            assert.Same(ex, res)
        end)

        it("fold#pairs", function ()
            local res = ast:parse("file", data, function (pairs)
                local acc = {}
                return pairs:next(function (pair)
                    return pair:pairs(function (ps)
                        return ps:fold(acc, function (val, p)
                            table.insert(val, p:as_rule())
                            return val
                        end)
                    end)
                end)
            end)

            local ex = {
                "record",
                "record",
                "record",
                "record",
                "record",
                "EOI",
            }

            assert.Same(ex, res)
        end)

        it("fold_break#pairs", function ()
            local res = ast:parse("file", data, function (pairs)
                local acc = {}
                return pairs:next(function (pair)
                    return pair:pairs(function (ps)
                        return ps:fold(acc, function (val, p)
                            table.insert(val, p:as_rule())

                            return val, #val < 3
                        end)
                    end)
                end)
            end)

            local ex = {
                "record",
                "record",
                "record",
            }

            assert.Same(ex, res)
        end)

        it("fold_error#pairs", function ()
            local message = "FOLD ERROR"

            local should_error = function ()
                ast:parse("file", data, function (pairs)
                    local acc = {}
                    return pairs:next(function (pair)
                        return pair:pairs(function (ps)
                            return ps:fold(acc, function (_, _)
                                error(message)
                            end)
                        end)
                    end)
                end)
            end

            assert.matches_error(should_error, message)
        end)

        it("fold_flat#pairs", function ()
            local res = ast:parse("file", data, function (pairs)
                local acc = {}
                return pairs:fold_flat(acc, function (val, pair)
                    table.insert(val, pair:as_rule())
                    return val
                end)
            end)

            local ex = {
                "file",
                "record",
                "field",
                "field",
                "field",
                "record",
                "field",
                "field",
                "field",
                "record",
                "field",
                "field",
                "record",
                "field",
                "field",
                "record",
                "field",
                "EOI",
            }

            assert.Same(ex, res)
        end)

        it("fold_flat_break#pairs", function ()
            local res = ast:parse("file", data, function (pairs)
                local acc = {}
                return pairs:fold_flat(acc, function (val, pair)
                    table.insert(val, pair:as_rule())
                    return val, #val < 3
                end)
            end)

            local ex = {
                "file",
                "record",
                "field",
            }

            assert.Same(ex, res)
        end)

        it("fold_flat_error#pairs", function ()
            local message = "FOLD ERROR"

            local should_error = function ()
                ast:parse("file", data, function (pairs)
                    local acc = {}
                    return pairs:next(function (pair)
                        return pair:pairs(function (ps)
                            return ps:fold_flat(acc, function (_, _)
                                error(message)
                            end)
                        end)
                    end)
                end)
            end

            assert.matches_error(should_error, message)
        end)

        it("rfold#pairs", function ()
            local res = ast:parse("file", data, function (pairs)
                local acc = {}
                return pairs:next(function (pair)
                    return pair:pairs(function (ps)
                        return ps:rfold(acc, function (val, p)
                            table.insert(val, p:as_rule())
                            return val
                        end)
                    end)
                end)
            end)

            local ex = {
                "EOI",
                "record",
                "record",
                "record",
                "record",
                "record",
            }

            assert.Same(ex, res)
        end)

        it("rfold_break#pairs", function ()
            local res = ast:parse("file", data, function (pairs)
                local acc = {}
                return pairs:next(function (pair)
                    return pair:pairs(function (ps)
                        return ps:rfold(acc, function (val, p)
                            table.insert(val, p:as_rule())

                            return val, #val < 3
                        end)
                    end)
                end)
            end)

            local ex = {
                "EOI",
                "record",
                "record",
            }

            assert.Same(ex, res)
        end)

        it("rfold_error#pairs", function ()
            local message = "FOLD ERROR"

            local should_error = function ()
                ast:parse("file", data, function (pairs)
                    local acc = {}
                    return pairs:next(function (pair)
                        return pair:pairs(function (ps)
                            return ps:rfold(acc, function (_, _)
                                error(message)
                            end)
                        end)
                    end)
                end)
            end

            assert.matches_error(should_error, message)
        end)

        it("rfold_flat#pairs", function ()
            local res = ast:parse("file", data, function (pairs)
                local acc = {}
                return pairs:rfold_flat(acc, function (val, pair)
                    table.insert(val, pair:as_rule())
                    return val
                end)
            end)

            local ex = {
                "EOI",
                "field",
                "record",
                "field",
                "field",
                "record",
                "field",
                "field",
                "record",
                "field",
                "field",
                "field",
                "record",
                "field",
                "field",
                "field",
                "record",
                "file",
            }

            assert.Same(ex, res)
        end)

        it("rfold_flat_break#pairs", function ()
            local res = ast:parse("file", data, function (pairs)
                local acc = {}
                return pairs:rfold_flat(acc, function (val, pair)
                    table.insert(val, pair:as_rule())
                    return val, #val < 3
                end)
            end)

            local ex = {
                "EOI",
                "field",
                "record",
            }

            assert.Same(ex, res)
        end)

        it("rfold_flat_error#pairs", function ()
            local message = "FOLD ERROR"

            local should_error = function ()
                ast:parse("file", data, function (pairs)
                    local acc = {}
                    return pairs:next(function (pair)
                        return pair:pairs(function (ps)
                            return ps:rfold_flat(acc, function (_, _)
                                error(message)
                            end)
                        end)
                    end)
                end)
            end

            assert.matches_error(should_error, message)
        end)
    end)

    describe("pair#rsast", function ()
        local ast = rsast.Ast.new(grammar)
        ---@cast ast - nil

        it("start#pair", function ()
            local res = ast:parse("record", data, function (pairs)
                return pairs:next(function (pair)
                    return pair:pairs(function (ps)
                        ps:next()
                        return ps:next(function (p) return p:start() end)
                    end)
                end)
            end)

            local ex = 6

            assert.Equal(ex, res)
        end)

        it("stop#pair", function ()
            local res = ast:parse("record", data, function (pairs)
                return pairs:next(function (pair)
                    return pair:pairs(function (ps)
                        ps:next()
                        return ps:next(function (p) return p:stop() end)
                    end)
                end)
            end)

            local ex = 16

            assert.Equal(ex, res)
        end)

        it("as_rule#pair", function ()
            local res = ast:parse("record", data, function (pairs)
                return pairs:next(function (pair)
                    return pair:pairs(function (ps)
                        ps:next()
                        return ps:next(function (p) return p:as_rule() end)
                    end)
                end)
            end)

            local ex = "field"

            assert.Equal(ex, res)
        end)

        it("as_str#pair", function ()
            local res = ast:parse("record", data, function (pairs)
                return pairs:next(function (pair) return pair:as_str() end)
            end)

            local ex = "65279,1179403647,1463895090"

            assert.Equal(ex, res)
        end)

        it("as_node_tag#pair", function ()
            local res = ast:parse("record", data, function (pairs)
                return pairs:fold_flat({}, function (val, pair)
                    table.insert(val, tostring(pair:as_node_tag()))
                    return val
                end)
            end)

            local ex = "tag"

            assert.Same(ex, res[2])
        end)

        it("get_input#pair", function ()
            local res = ast:parse("record", data, function (pairs)
                return pairs:next(function (pair) return pair:get_input() end)
            end)

            assert.Equal(data, res)
        end)

        it("line_col#pair", function ()
            local res_line, res_col = ast:parse("record", data, function (pairs)
                return pairs:next(function (pair)
                    return pair:pairs(function (ps)
                        ps:next()
                        return ps:next(function (p) return p:line_col() end)
                    end)
                end)
            end)

            local ex_line = 1
            local ex_col = 7

            assert.Equal(ex_line, res_line)
            assert.Equal(ex_col, res_col)
        end)

        it("dump#pair", function ()
            local res = ast:parse("field", data, function (pairs)
                return pairs:next(function (pair) return pair:dump() end)
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

        it("tokens#pairs", function ()
            local res = ast:parse(
                "field",
                data,
                function (pairs) return pairs:next(function (pair) return pair:tokens() end) end
            )

            local ex = {
                [1] = {
                    ["pos"] = 0,
                    ["rule"] = "field",
                    ["type"] = "start",
                },
                [2] = {
                    ["pos"] = 5,
                    ["rule"] = "field",
                    ["type"] = "end",
                },
            }

            assert.Same(ex, res)
        end)

        it("tokens_callback#pairs", function ()
            local callback = function (pairs)
                return pairs:next(function (pair)
                    return pair:tokens(function (tokens)
                        local peek = tokens:peek()
                        local last = tokens:next_back()
                        local first = tokens:next()
                        local none = tokens:next()

                        return peek, first, last, none
                    end)
                end)
            end

            local res_peek, res_first, res_last, res_none = ast:parse(
                "field",
                data,
                callback
            )

            local ex_first = { ["pos"] = 0, ["rule"] = "field", ["type"] = "start" }
            local ex_last = { ["pos"] = 5, ["rule"] = "field", ["type"] = "end" }

            assert.Same(res_peek, res_first)
            assert.Same(ex_first, res_first)
            assert.Same(ex_last, res_last)
            assert.Nil(res_none)
        end)

        it("lines#pairs", function ()
            local res = ast:parse(
                "file",
                data,
                function (pairs) return pairs:next(function (pair) return pair:lines() end) end
            )

            local ex = {
                "65279,1179403647,1463895090\n",
                "3.1415927,2.7182817,1.618034\n",
                "-40,-273.15\n",
                "13,42\n",
                "65537\n",
            }

            assert.Same(ex, res)
        end)

        it("lines_callback#pairs", function ()
            local res_peek, res_first, res_sec = ast:parse("file", data, function (pairs)
                return pairs:next(function (pair)
                    return pair:lines(function (lines)
                        local peek = { lines:peek() }
                        local first = { lines:next() }
                        local sec = { lines:next() }

                        return peek, first, sec
                    end)
                end)
            end)

            local ex_first = { "65279,1179403647,1463895090\n", 0, 28 }
            local ex_sec = { "3.1415927,2.7182817,1.618034\n", 28, 57 }

            assert.Same(res_peek, res_first)
            assert.Same(ex_first, res_first)
            assert.Same(ex_sec, res_sec)
        end)

        it("pairs#pair", function ()
            local res = ast:parse(
                "record",
                data,
                function (pairs) return pairs:next(function (pair) return pair:pairs() end) end
            )

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
                    [2] = {
                        ["inner"] = "1179403647",
                        ["pos"] = {
                            [1] = 6,
                            [2] = 16,
                        },
                        ["rule"] = '"field"',
                    },
                    [3] = {
                        ["inner"] = "1463895090",
                        ["pos"] = {
                            [1] = 17,
                            [2] = 27,
                        },
                        ["rule"] = '"field"',
                    },
                },
                ["pos"] = {
                    [1] = 0,
                    [2] = 27,
                },
            }

            assert.Same(ex, res)
        end)

        it("pairs_callback#pair", function ()
            local res = ast:parse("record", data, function (pairs)
                return pairs:next(function (pair)
                    return pair:pairs(function (ps) return ps:as_str() end)
                end)
            end)

            local ex = "65279,1179403647,1463895090"

            assert.Equal(ex, res)
        end)
    end)
end)
