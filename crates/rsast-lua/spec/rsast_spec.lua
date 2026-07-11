local rsast = require("rsast")

grammar = [[
WHITESPACE = _{ " " | "\t" | "\r" | "\n" }

object = {
    "{" ~ "}" |
    "{" ~ pair ~ ("," ~ pair)* ~ "}"
}
pair = { string ~ ":" ~ value }

array = {
    "[" ~ "]" |
    "[" ~ value ~ ("," ~ value)* ~ "]"
}

value = _{ object | array | string | number | boolean | null }

boolean = { "true" | "false" }

null = { "null" }

string = ${ "\"" ~ inner ~ "\"" }
inner = @{ char* }
char = {
    !("\"" | "\\") ~ ANY
    | "\\" ~ ("\"" | "\\" | "/" | "b" | "f" | "n" | "r" | "t")
    | "\\" ~ ("u" ~ ASCII_HEX_DIGIT{4})
}

number = @{
    "-"?
    ~ ("0" | ASCII_NONZERO_DIGIT ~ ASCII_DIGIT*)
    ~ ("." ~ ASCII_DIGIT*)?
    ~ (^"e" ~ ("+" | "-")? ~ ASCII_DIGIT+)?
}

json = _{ SOI ~ (object | array) ~ EOI }
]]

data = [[
{
    "nesting": { "inner object": {} },
    "an array": [1.5, true, null, 1e-6],
    "string with escaped double quotes" : "\"quick brown foxes\""
}
]]

describe("rsast", function ()
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

            assert.Not.Error(function ()
                ast:parse("json", data, function () end)
            end)
        end)

        it("parse_error#ast", function ()
            local ast = rsast.Ast.new(grammar)
            ---@cast ast - nil

            assert.matches_error(function ()
                ast:parse("json", "invalid data", function () end)
            end, [[
runtime error:  --> 1:1
  |
1 | invalid data
  | ^---
  |
  = expected "array" or "object"]], nil, true)
        end)
    end)

    describe("pairs", function () end)

    describe("tokens", function () end)
end)
