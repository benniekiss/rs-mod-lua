local luamark = require("luamark")
local rsjson = require("rsjson")
local rsast = require("rsast")
local lpeg = require("lpeg")

local P, S, R = lpeg.P, lpeg.S, lpeg.R

function load_rsast()
    local grammar = [[
    field = _{ (ASCII_DIGIT | "." | "-")+ }
    record = _{ field ~ ("," ~ field)* }
    file = _{ SOI ~ (record ~ ("\r\n" | "\n"))* ~ EOI }
    ]]

    return rsast.Ast.new(grammar)
end

function load_lpeg()
    -- field = { (ASCII_DIGIT | "." | "-")+ }
    local field = P((R("09") + S(".-")) ^ 1)

    -- record = { field ~ ("," ~ field)* }
    local record = P(field * (P(",") * field) ^ 0)

    -- newline = "\r\n" | "\n"
    local newline = P("\r\n") + P("\n")

    -- file = { SOI ~ (record ~ newline)* ~ EOI }
    local file = P((record * newline) ^ 0) * -1

    return file
end

local rsast_parser = load_rsast()
local lpeg_parser = load_lpeg()

local input = [[
65279,1179403647,1463895090
3.1415927,2.7182817,1.618034
-40,-273.15
13,42
65537
]]

local iters = arg[1] or 100

local parsing = luamark.compare_time({
    rsast = function (ctx)
        for i = 1, iters do
            rsast_parser:validate("file", input)
        end
    end,
    lpeg = function (ctx)
        for i = 1, iters do
            lpeg_parser:match(input)
        end
    end,
})

local sep = 20
print(("-"):rep(sep))
print("Validation (Time): " .. iters .. " iters")
print(("-"):rep(sep))
print(luamark.render(parsing))
print()

local rsast_output = rsast_parser:validate("file", input)

local lpeg_output = lpeg_parser:match(input)

local config = rsjson.EncodeConfig.new()
    :set_indent(4)

print(("-"):rep(sep))
print("rsast valid")
print(rsjson.encode(rsast_output, config))
print(("-"):rep(sep))
print("lpeg valid")
print(rsjson.encode(lpeg_output, config))
