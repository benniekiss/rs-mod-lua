# Benchmarks

Benchmarks are in the `bench` folder, and they can be run with the following
commands, where `{iters}` is the number of iterations to run (defaults to
`100`). They use the [`luamark`](https://github.com/jeffzi/luamark) module

```shell
# time benchmarks
lx --lua-version 5.4 lua --package=rsjson-lua --test crates/rsjson-lua/bench/time.lua {iters}

# memory benchmarks
lx --lua-version 5.4 lua --package=rsjson-lua --test crates/rsjson-lua/bench/mem.lua {iters}
```

Here is a comparison between `cjson`, `rapidjson`, and `dkjson` (with the lpeg extension enabled):

Time:

```shell
$ lx --lua-version 5.4 lua --test --package=rsjson-lua crates/rsjson-lua/bench/time.lua 1000

--------------------
Encoding (Time): 1000 iters
--------------------
pretty=false
  Name     Rank     Relative         Median       Ops  
---------  ----  ---------------  ------------  -------
rapidjson     1  █            1x           4ms    250/s
cjson         2  █        ↓1.75x           7ms  142.9/s
rsjson        3  █        ↓3.25x          13ms   76.9/s
dkjson        4  ████████   ↓18x  72ms ± 500us   13.9/s

pretty=true # note: cjson does not support pretty printing
  Name     Rank      Relative      Median    Ops  
---------  ----  ----------------  ------  -------
rapidjson     1  █             1x     6ms  166.7/s
cjson         2  █         ↓1.17x     7ms  142.9/s
rsjson        3  █         ↓2.67x    16ms   62.5/s
dkjson        4  ████████ ↓13.67x    82ms   12.2/s

--------------------
Decoding (Time): 1000 iters
--------------------
  Name     Rank     Relative      Median   Ops  
---------  ----  ---------------  ------  ------
cjson        ≈1  █            1x    12ms  83.3/s
rapidjson    ≈1  █            1x    12ms  83.3/s
rsjson        3  ████     ↓2.75x    33ms  30.3/s
dkjson        4  ████████ ↓4.75x    57ms  17.5/s
```

Memory:

```shell
$ lx --lua-version 5.4 lua --test --package=rsjson-lua crates/rsjson-lua/bench/mem.lua 1000

--------------------
Encoding (Mem): 1000 iters
--------------------
  Name     Rank      Relative      Median
---------  ----  ----------------  ------
rapidjson     1  █             1x    905B
dkjson        2  ██       ↓11.19x    10kB
rsjson        3  ██████   ↓32.07x    28kB
cjson         4  ████████    ↓41x    36kB

--------------------
Decoding (Mem): 1000 iters
--------------------
  Name     Rank     Relative        Median  
---------  ----  ---------------  ----------
cjson        ≈1  ██████       1x  27kB ± 3kB
dkjson       ≈1  ███████  ↓1.07x        29kB
rapidjson    ≈3  ███████   ↓1.2x        33kB
rsjson       ≈3  ████████  ↓1.2x  33kB ± 5kB
```
