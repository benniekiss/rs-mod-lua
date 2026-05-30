# Usage

```lua
rsjson = require("rsjson")

obj = { a = "a", b = "b", c = 3 }

-- compact printing
rsjson.encode(obj)

-- pretty printing
config = rsjson.EncodeConfig:new()
config.indent = 4

rsjson.encode(obj, config)

-- objects can be loaded from json
str = [[{
    "1": 1,
    "2": 2,
    "3": "three",
}]]

t = rsjson.decode(str)

for k,v in ipairs(t) do
    print(k .. " = " .. v .. "\n")
end
```

The API is documented in the [`lua/rsjson.lua`](../../lua/rsjson.lua) file.
