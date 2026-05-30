# Usage

```lua
rsre = require("rsre")

-- compile a pattern to a regex
--
-- you can use lua's long strings to avoid needing to escape meta characters
re = rsre.Regex.new([[(?<digits>\d{3})]])

haystack = "abc123def"

-- check for a match
is_match = re:match(haystack)
print('is_match = ' .. tostring(is_match))

-- find the first, left-most match
match = re:find(haystack)
print('\n\nFIND:')
print('start = ' .. match:start())
print('stop  = ' .. match:stop())
print('text  = ' .. match:as_str())

-- get all capture groups from the first, left-most match
captures = re:captures(haystack)

-- the first capture group is the entire match
match = captures:get(1)
print('\n\nCAPTURES:')
print('group 1:')
print('start = ' .. match:start())
print('stop  = ' .. match:stop())
print('text  = ' .. match:as_str())

-- the next two groups are the same
print('\ngroup 2:')
index = captures:get(2)
print('start = ' .. index:start())
print('stop  = ' .. index:stop())
print('text  = ' .. index:as_str())

print('\ngroup `digits`:')
name = captures:name("digits")
print('start = ' .. name:start())
print('stop  = ' .. name:stop())
print('text  = ' .. name:as_str())
```

The API is documented in the [`library/rsre.d.lua`](../library/rsre.d.lua)
file, which should work with LuaLS or EmmyluaLS.

For more information, review the
[`fancy-regex`](https://docs.rs/fancy-regex/latest/fancy_regex)
documentation.
