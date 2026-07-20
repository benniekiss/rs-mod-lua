# Benchmarks

Benchmarks are in the `bench` folder, and they can be run with the following
commands, where `{rep}` is the number of times to repeat the input text (defaults to
`100`). They use the [`luamark`](https://github.com/jeffzi/luamark) module.

Before running the benchmarks, make sure to uncomment the `test_dependencies` table
in `lux.toml`, then run `lx --lua-version 5.4 sync`.

```shell
# time benchmarks
lx --lua-version 5.4 lua --package=rsast-lua --test crates/rsast-lua/bench/time.lua {rep}

# memory benchmarks
lx --lua-version 5.4 lua --package=rsast-lua --test crates/rsast-lua/bench/mem.lua {rep}
```

Here is a comparison with `lpeg`. Performance is dramatically inferior to `lpeg`:

Time:

```shell
$ lx --lua-version 5.4 lua --package=rsast-lua --test crates/rsast-lua/bench/time.lua 10000

----------------------------------------
Parsing (Time): 810000 bytes
----------------------------------------
Name   Rank      Relative         Median       Ops  
-----  ----  ----------------  -------------  ------
lpeg      1  █             1x    13ms ± 50us  76.9/s
rsast     2  ████████ ↓11.92x  155ms ± 575us   6.5/s
----------------------------------------
```

Memory:

```shell
$ lx --lua-version 5.4 lua --package=rsast-lua --test crates/rsast-lua/bench/mem.lua 10000

----------------------------------------
Parsing (Time): 810000 bytes
----------------------------------------
Name   Rank     Relative        Median  
-----  ----  ---------------  ----------
lpeg     ≈1  ████         1x         7MB
rsast    ≈1  ████████ ↓1.68x  12MB ± 6MB
----------------------------------------
```
