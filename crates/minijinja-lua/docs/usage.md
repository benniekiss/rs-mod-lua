# Usage

```lua
mj = require("minijinja")

env = mj.Environment:new()

--- A sample filter
---
---@param state     minijinja.State
---@param val       string
---@param args      { case: "upper" | "lower" }
---
---@return string
local function lua_filter(state, val, args)
    if args.case == "upper" then
        return val:upper()
    end

    if args.case == "lower" then
        return val:lower()
    end
end

env:add_filter("lua_filter", lua_filter)

env:add_template("my_temp", "Test: {{ foo | lua_filter(case='upper') }}")

env:render_template("my_temp", {
    foo = "foo"
})
-- output: "Test: FOO"
```

The API is documented in the [`library/minijinja.d.lua`](../library/minijinja.d.lua)
file, which should work with LuaLS or EmmyluaLS.

This crate provides a few additional filters:

- [`fromjson`](https://docs.rs/minijinja-lua/latest/minijinja_lua/contrib/json/fn.fromjson.html)
- [`datefmt`](https://docs.rs/minijinja-lua/latest/minijinja_lua/contrib/datetime/fn.datefmt.html)
- [`timefmt`](https://docs.rs/minijinja-lua/latest/minijinja_lua/contrib/datetime/fn.timefmt.html)

For more information, review the
[`minijinja`](https://docs.rs/minijinja/latest/minijinja/index.html)
documentation:

- [syntax](https://docs.rs/minijinja/latest/minijinja/syntax/index.html)
- [filters](https://docs.rs/minijinja/latest/minijinja/filters/index.html)
- [tests](https://docs.rs/minijinja/latest/minijinja/tests/index.html)
- [functions](https://docs.rs/minijinja/latest/minijinja/functions/index.html)
