# Benchmarks

Benchmarks are in the `bench` folder, and they can be run with the following
commands, where `{iters}` is the number of iterations to run (defaults to
`100`). They use the [`luamark`](https://github.com/jeffzi/luamark) module.

Do note that these benchmarks are currently heavily biased towards `lpeg`, as `rsast`
internally produces an AST with much more information available. An `lpeg` grammar
that produces a similar AST is still more performant, but only within a 10x factor, not 100x.

```shell
# time benchmarks
lx --lua-version 5.4 lua --package=rsast-lua --test crates/rsast-lua/bench/time.lua {iters}

# memory benchmarks
lx --lua-version 5.4 lua --package=rsast-lua --test crates/rsast-lua/bench/mem.lua {iters}
```

Here is a comparison with `lpeg`. Performance is dramatically inferior to `lpeg`:

Time:

```shell
$ lx --lua-version 5.5 lua --test --package=rsast-lua crates/rsast-lua/bench/time.lua

--------------------
Parsing (Time): 100 iters
--------------------
Name   Rank      Relative         Median      Ops  
-----  ----  ----------------  ------------  ------
lpeg      1  █             1x  200us ± 50us    5k/s
rsast     2  ████████ ↓115.5x          23ms  43.3/s
```

Validation:

```shell
--------------------
Validation (Time): 100 iters
--------------------
Name   Rank    Relative     Median    Ops  
-----  ----  -------------  ------  -------
lpeg      1  █          1x   100us    10k/s
rsast     2  ████████ ↓13x     1ms  769.2/s
```

Memory:

```shell
$ lx --lua-version 5.5 lua --test --package=rsast-lua crates/rsast-lua/bench/mem.lua

--------------------
Parsing (Mem): 100 iters
--------------------
Name   Rank      Relative         Median  
-----  ----  ----------------  ------------
lpeg      1  █             1x   18kB ± 501B
rsast     2  ████████ ↓13.33x  239kB ± 12kB
```
