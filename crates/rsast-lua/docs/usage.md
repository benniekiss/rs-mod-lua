# Usage

```lua
rsast = require("rsast")

local grammar = [[
field = { (ASCII_DIGIT | "." | "-")+ }
record = { #tag = field ~ ("," ~ field)* }
file = { SOI ~ (record ~ ("\r\n" | "\n"))* ~ EOI }
]]

local input = [[
65279,1179403647,1463895090
3.1415927,2.7182817,1.618034
-40,-273.15
13,42
65537
]]

local ast = rsast.Ast.new(grammar)

-- Parse the input into a basic Ast.
--
-- see `rsast.Tree` for more information
local tree = ast:parse("file", input, function(ps) return ps:dump() end)

-- Collect the rule names of the nodes
local rules = ast:parse("file", data, function(ps)
    local flat = ps:flatten()

    local rules = {}
    for p in flat:iter() do
        table.insert(rules, p:as_rule())
    end

    return rules
end)
```

The API is documented in the [`library/rsast.d.lua`](../library/rsast.d.lua)
file, which should work with LuaLS or EmmyluaLS.

For more information, review the
[`pest`](https://docs.rs/pest/latest/pest/index.html)
documentation.
