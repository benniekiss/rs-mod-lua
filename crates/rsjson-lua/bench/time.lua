local luamark = require("luamark")

local rsjson = require("rsjson").serde
local dkjson = require("dkjson").use_lpeg()
local cjson = require("cjson").new()
local rapidjson = require("rapidjson")

local encoding_test = {
    one = 1,
    two = "2",
    three = {4,5,6},
    ["four"] = {
        [1] = "one",
        [2] = "two",
        [3] = "three",
    },
    nested = {
        one = 1,
        two = "2",
        three = {4,5,6},
        ["four"] = {
            [1] = "one",
            [2] = "two",
            [3] = "three",
         },
         nested = {
            one = 1,
            two = "2",
            three = {4,5,6},
            ["four"] = {
                  [1] = "one",
                  [2] = "two",
                  [3] = "three",
            },
            nested = {
               one = 1,
               two = "2",
               three = {4,5,6},
               ["four"] = {
                     [1] = "one",
                     [2] = "two",
                     [3] = "three",
               },
            },
         },
    },
}

local decoding_test = [[{"two":"2","one":1,"nested":{"four":["one","two","three"],"three":[4,5,6],"two":"2","one":1},"four":["one","two","three"],"three":[4,5,6]}]]

local iters = arg[1] or 100

local encoding = luamark.compare_time({
   rsjson = function(ctx, p)
      for _ = 1, iters do rsjson.encode(encoding_test, ctx.rsjson) end
   end,
   dkjson = function(ctx, p)
      for _ = 1, iters do
         dkjson.encode(encoding_test, ctx.dkjson)
      end
   end,
   cjson = function(ctx, p)
      for _ = 1, iters do cjson.encode(encoding_test) end
   end,
   rapidjson = function(ctx, p)
      for _ = 1, iters do rapidjson.encode(encoding_test) end
   end
   },
   {
      params = { pretty = { false, true } },
      setup = function(p)
         local pretty = p.pretty

         local rsjson_config = rsjson.EncodeConfig:new()
         if pretty then
            rsjson_config.indent = 4
         end

         return {
            rsjson = rsjson_config,
            dkjson = { indent = pretty },
            cjson = {},
            rapidjson = { pretty = pretty },
         }
      end
   }
)

local decoding = luamark.compare_time({
   rsjson = function()
      for _ = 1, iters do rsjson.decode(decoding_test) end
   end,
   dkjson = function()
      for _ = 1, iters do dkjson.decode(decoding_test) end
   end,
   cjson = function()
      for _ = 1, iters do cjson.decode(decoding_test) end
   end,
   rapidjson = function()
      for _ = 1, iters do rapidjson.decode(decoding_test) end
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
