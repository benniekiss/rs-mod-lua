# Usage

```lua
rsast = require("rsast")

grammar = [[
field = { (ASCII_DIGIT | "." | "-")+ }
record = { #tag = field ~ ("," ~ field)* }
file = { SOI ~ (record ~ ("\r\n" | "\n"))* ~ EOI }
]]

local ast = rsast.Ast.new(grammar)

data = [[
65279,1179403647,1463895090
3.1415927,2.7182817,1.618034
-40,-273.15
13,42
65537
]]

-- Parse the input into a basic Ast.
--
-- see `rsast.Tree` for more information
local tree = ast:parse("file", data)

-- Collect the rule names of the first three nodes
local rules = ast:parse("file", data, function(ps)
    return ps:fold_flat({}, function(val, p)
        table.insert(val, p:as_rule())
        return val, #val < 3
    end)
end)
```

The API is documented in the [`library/rsast.d.lua`](../library/rsast.d.lua)
file, which should work with LuaLS or EmmyluaLS.

For more information, review the
[`pest`](https://docs.rs/pest/latest/pest/index.html)
documentation.
