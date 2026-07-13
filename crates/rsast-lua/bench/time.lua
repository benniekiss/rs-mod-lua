local luamark = require("luamark")
local rsjson = require("rsjson")
local rsast = require("rsast")
local lpeg = require("lpeg")

local P, S, R, C, Ct = lpeg.P, lpeg.S, lpeg.R, lpeg.C, lpeg.Ct

function rsast_parse(pairs)
    return pairs:fold_flat({}, function (acc, pair)
        local rule = pair:as_rule()

        if rule == "record" then
            table.insert(acc, {})
        end

        if rule == "field" then
            local last = acc[#acc]
            table.insert(last, pair:as_str())
        end

        return acc
    end)
end

function load_rsast()
    local grammar = [[
    field = { (ASCII_DIGIT | "." | "-")+ }
    record = { #tag = field ~ ("," ~ field)* }
    file = { SOI ~ (record ~ ("\r\n" | "\n"))* ~ EOI }
    ]]

    return rsast.Ast.new(grammar)
end

function load_lpeg()
    -- field = { (ASCII_DIGIT | "." | "-")+ }
    local field = C((R("09") + S(".-")) ^ 1)

    -- record = { field ~ ("," ~ field)* }
    local record = Ct(field * (P(",") * field) ^ 0)

    -- newline = "\r\n" | "\n"
    local newline = P("\r\n") + P("\n")

    -- file = { SOI ~ (record ~ newline)* ~ EOI }
    local file = Ct((record * newline) ^ 0) * -1

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
            rsast_parser:parse("file", input, rsast_parse)
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
print("Parsing (Time): " .. iters .. " iters")
print(("-"):rep(sep))
print(luamark.render(parsing))
print()

local rsast_output = rsast_parser:parse("file", input, rsast_parse)

local lpeg_output = lpeg_parser:match(input)

local config = rsjson.EncodeConfig.new()
    :set_indent(4)

print(("-"):rep(sep))
print("rsast tree")
print(rsjson.encode(rsast_output, config))
print(("-"):rep(sep))
print("lpeg tree")
print(rsjson.encode(lpeg_output, config))
