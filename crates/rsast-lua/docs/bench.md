# Benchmarks

Benchmarks are in the `bench` folder, and they can be run with the following
commands, where `{iters}` is the number of iterations to run (defaults to
`100`). They use the [`luamark`](https://github.com/jeffzi/luamark) module

```shell
# time benchmarks
lx --lua-version 5.4 lua --package=rsast-lua --test crates/rsast-lua/bench/time.lua {iters}

# memory benchmarks
lx --lua-version 5.4 lua --package=rsast-lua --test crates/rsast-lua/bench/mem.lua {iters}
```

Here is a comparison with `lpeg`. Performance is dramatically inferior to `lpeg`:

Time:

```shell
$ lx --lua-version 5.4 lua --test --package=rsast-lua crates/rsast-lua/bench/time.lua

--------------------
Parsing (Time): 100 iters
--------------------
Name   Rank    Relative        Median     Ops
-----  ----  -------------  ------------  ----
lpeg      1  █          1x           1ms  1k/s
rsast     2  ████████ ↓25x  25ms ± 500us  40/s

--------------------
rsast tree
[
    [
        "65279",
        "1179403647",
        "1463895090"
    ],
    [
        "3.1415927",
        "2.7182817",
        "1.618034"
    ],
    [
        "-40",
        "-273.15"
    ],
    [
        "13",
        "42"
    ],
    [
        "65537"
    ]
]
--------------------
lpeg tree
[
    [
        "65279",
        "1179403647",
        "1463895090"
    ],
    [
        "3.1415927",
        "2.7182817",
        "1.618034"
    ],
    [
        "-40",
        "-273.15"
    ],
    [
        "13",
        "42"
    ],
    [
        "65537"
    ]
]
```

Memory:

```shell
$ lx --lua-version 5.4 lua --test --package=rsast-lua crates/rsast-lua/bench/mem.lua

--------------------
Parsing (Mem): 100 iters
--------------------
Name   Rank      Relative         Median  
-----  ----  ----------------  ------------
lpeg      1  █             1x     5kB ± 2kB
rsast     2  ████████ ↓36.96x  182kB ± 22kB

--------------------
rsast tree
[
    [
        "65279",
        "1179403647",
        "1463895090"
    ],
    [
        "3.1415927",
        "2.7182817",
        "1.618034"
    ],
    [
        "-40",
        "-273.15"
    ],
    [
        "13",
        "42"
    ],
    [
        "65537"
    ]
]
--------------------
lpeg tree
[
    [
        "65279",
        "1179403647",
        "1463895090"
    ],
    [
        "3.1415927",
        "2.7182817",
        "1.618034"
    ],
    [
        "-40",
        "-273.15"
    ],
    [
        "13",
        "42"
    ],
    [
        "65537"
    ]
]
```
