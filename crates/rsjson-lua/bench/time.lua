local luamark = require("luamark")

local rsjson = require("rsjson")
local dkjson = require("dkjson").use_lpeg()
local cjson = require("cjson").new()
local rapidjson = require("rapidjson")

local t = {
    one = 1,
    two = "2",
    three = { 4, 5, 6 },
    ["four"] = {
        [1] = "one",
        [2] = "two",
        [3] = "three",
    },
    nested = {
        one = 1,
        two = "2",
        three = { 4, 5, 6 },
        ["four"] = {
            [1] = "one",
            [2] = "two",
            [3] = "three",
        },
        nested = {
            one = 1,
            two = "2",
            three = { 4, 5, 6 },
            ["four"] = {
                [1] = "one",
                [2] = "two",
                [3] = "three",
            },
            nested = {
                one = 1,
                two = "2",
                three = { 4, 5, 6 },
                ["four"] = {
                    [1] = "one",
                    [2] = "two",
                    [3] = "three",
                },
            },
        },
    },
    nested2 = {
        one = 1,
        two = "2",
        three = { 4, 5, 6 },
        ["four"] = {
            [1] = "one",
            [2] = "two",
            [3] = "three",
        },
        nested = {
            one = 1,
            two = "2",
            three = { 4, 5, 6 },
            ["four"] = {
                [1] = "one",
                [2] = "two",
                [3] = "three",
            },
            nested = {
                one = 1,
                two = "2",
                three = { 4, 5, 6 },
                ["four"] = {
                    [1] = "one",
                    [2] = "two",
                    [3] = "three",
                },
                nested = {
                    one = 1,
                    two = "2",
                    three = { 4, 5, 6 },
                    ["four"] = {
                        [1] = "one",
                        [2] = "two",
                        [3] = "three",
                    },
                },
            },
        },
        nested2 = {
            one = 1,
            two = "2",
            three = { 4, 5, 6 },
            ["four"] = {
                [1] = "one",
                [2] = "two",
                [3] = "three",
            },
            nested = {
                one = 1,
                two = "2",
                three = { 4, 5, 6 },
                ["four"] = {
                    [1] = "one",
                    [2] = "two",
                    [3] = "three",
                },
                nested = {
                    one = 1,
                    two = "2",
                    three = { 4, 5, 6 },
                    ["four"] = {
                        [1] = "one",
                        [2] = "two",
                        [3] = "three",
                    },
                    nested = {
                        one = 1,
                        two = "2",
                        three = { 4, 5, 6 },
                        ["four"] = {
                            [1] = "one",
                            [2] = "two",
                            [3] = "three",
                        },
                    },
                },
            },
        },
    },
}

local encoding_test = {
    one = t,
    two = t,
    three = {
        one = t,
        two = t,
        three = {
            one = t,
            two = t,
            three = {
                one = t,
                two = t,
                three = t,
            },
        },
    },
}

local decoding_test = [[{"three":[4,5,6],"nested":{"three":[4,5,6],"one":1,"nested":{"three":[4,5,6],"one":1,"nested":{"one":1,"three":[4,5,6],"two":"2","four":["one","two","three"]},"two":"2","four":["one","two","three"]},"two":"2","four":["one","two","three"]},"one":1,"nested2":{"three":[4,5,6],"nested":{"three":[4,5,6],"one":1,"nested":{"three":[4,5,6],"one":1,"nested":{"one":1,"three":[4,5,6],"two":"2","four":["one","two","three"]},"two":"2","four":["one","two","three"]},"two":"2","four":["one","two","three"]},"one":1,"nested2":{"three":[4,5,6],"one":1,"nested":{"three":[4,5,6],"one":1,"nested":{"three":[4,5,6],"one":1,"nested":{"one":1,"three":[4,5,6],"two":"2","four":["one","two","three"]},"two":"2","four":["one","two","three"]},"two":"2","four":["one","two","three"]},"two":"2","four":["one","two","three"]},"two":"2","four":["one","two","three"]},"two":"2","four":["one","two","three"]}]]

local iters = arg[1] or 100

local encoding = luamark.compare_time({
    rsjson = function (ctx, p)
        local config = ctx.rsjson
        for _ = 1, iters do
            rsjson.encode(encoding_test, config)
        end
    end,
    dkjson = function (ctx, p)
        local config = ctx.dkjson

        for _ = 1, iters do
            dkjson.encode(encoding_test, config)
        end
    end,
    cjson = function (ctx, p)
        for _ = 1, iters do
            cjson.encode(encoding_test)
        end
    end,
    rapidjson = function (ctx, p)
        local config = ctx.rapidjson

        for _ = 1, iters do
            rapidjson.encode(encoding_test, config)
        end
    end,
},
    {
        params = { pretty = { false, true } },
        setup = function (p)
            local pretty = p.pretty

            if pretty then
                local rsjson_config = rsjson.EncodeConfig:new()
                    :set_indent(4)
                local dkjson_config = { indent = pretty }
                local rapidjson_config = { pretty = pretty }
                return {
                    rsjson = rsjson_config,
                    dkjson = dkjson_config,
                    rapidjson = rapidjson_config,
                    cjson = {},
                }
            else
                return {}
            end
        end,
    })

local decoding = luamark.compare_time({
    rsjson = function ()
        for _ = 1, iters do
            rsjson.decode(decoding_test)
        end
    end,
    dkjson = function ()
        for _ = 1, iters do
            dkjson.decode(decoding_test)
        end
    end,
    cjson = function ()
        for _ = 1, iters do
            cjson.decode(decoding_test)
        end
    end,
    rapidjson = function ()
        for _ = 1, iters do
            rapidjson.decode(decoding_test)
        end
    end,
})

local sep = 20
print(("-"):rep(sep))
print("Encoding (Time): " .. iters .. " iters")
print(("-"):rep(sep))
print(luamark.render(encoding))
print()
print(("-"):rep(sep))
print("Decoding (Time): " .. iters .. " iters")
print(("-"):rep(sep))
print(luamark.render(decoding))
