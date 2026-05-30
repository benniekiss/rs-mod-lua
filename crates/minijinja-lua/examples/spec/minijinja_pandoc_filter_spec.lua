local filter = require("minijinja_pandoc_filter")

describe("Examples", function()
    describe("minijinja_pandoc_filter#examples", function()
        it("simple#examples", function()
            local source = [[
---
minijinja:
    context:
        foo: "BOO"
---

Test: {{ foo }}
]]
            local doc = pandoc.read(source, "markdown")

            local te = doc:walk(filter)
            local ex = pandoc.read(
                [=[[ Para [Str "Test:",Space,Str "BOO"]]]=],
                "native"
            )

            assert.Equal(ex, te)
        end)

        it("nested context#examples", function()
            local source = [[
---
minijinja:
    context:
        foo:
            bar: "BOO"
---

Test: {{ foo.bar }}
]]
            local doc = pandoc.read(source, "markdown")

            local te = doc:walk(filter)
            local ex = pandoc.read(
                [=[[ Para [Str "Test:",Space,Str "BOO"]]]=],
                "native"
            )

            assert.Equal(ex, te)
        end)
    end)
end)
