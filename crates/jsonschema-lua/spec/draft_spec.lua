local jsonschema = require("jsonschema")

describe("draft#jsonschema", function ()
    local drafts = jsonschema.drafts

    it("tostring#draft", function ()
        assert.Equal("Draft201909", tostring(drafts.DRAFT201909))
        assert.Equal("Draft202012", tostring(drafts.DRAFT202012))
        assert.Equal("Draft4", tostring(drafts.DRAFT4))
        assert.Equal("Draft6", tostring(drafts.DRAFT6))
        assert.Equal("Draft7", tostring(drafts.DRAFT7))
    end)

    it("from_schema_uri#draft", function ()
        assert.Equal(
            drafts.DRAFT202012,
            drafts.from_schema_uri("https://json-schema.org/draft/2020-12/schema")
        )
        assert.Equal(
            drafts.DRAFT201909,
            drafts.from_schema_uri("https://json-schema.org/draft/2019-09/schema")
        )
        assert.Equal(
            drafts.DRAFT7,
            drafts.from_schema_uri("https://json-schema.org/draft-07/schema")
        )
        assert.Equal(
            drafts.DRAFT6,
            drafts.from_schema_uri("https://json-schema.org/draft-06/schema")
        )
        assert.Equal(
            drafts.DRAFT4,
            drafts.from_schema_uri("https://json-schema.org/draft-04/schema")
        )
        assert.Equal(drafts.UNKNOWN, drafts.from_schema_uri("unknown_schema"))
    end)

    describe("detect#draft", function ()
        it("empty schema#draft", function ()
            assert.Equal(drafts.DRAFT202012, drafts.DRAFT202012:detect({}))
            assert.Equal(drafts.DRAFT201909, drafts.DRAFT201909:detect({}))
            assert.Equal(drafts.DRAFT7, drafts.DRAFT7:detect({}))
            assert.Equal(drafts.DRAFT6, drafts.DRAFT6:detect({}))
            assert.Equal(drafts.DRAFT4, drafts.DRAFT4:detect({}))
            assert.Equal(drafts.UNKNOWN, drafts.UNKNOWN:detect({}))
        end)

        it("schema#detect", function ()
            assert.Equal(
                drafts.DRAFT202012,
                drafts.DRAFT4:detect({
                    ["$schema"] = "https://json-schema.org/draft/2020-12/schema",
                })
            )
        end)
    end)

    describe("is_known_keyword#draft", function ()
        it("DRAFT202012#is_known_keyword", function ()
            assert.True(drafts.DRAFT202012:is_known_keyword("$dynamicAnchor"))
            assert.False(drafts.DRAFT202012:is_known_keyword("foo"))
        end)

        it("DRAFT201909#is_known_keyword", function ()
            assert.True(drafts.DRAFT201909:is_known_keyword("$anchor"))
            assert.False(drafts.DRAFT201909:is_known_keyword("$dynamicAnchor"))
        end)

        it("DRAFT7#is_known_keyword", function ()
            assert.True(drafts.DRAFT7:is_known_keyword("if"))
            assert.False(drafts.DRAFT7:is_known_keyword("$anchor"))
        end)

        it("DRAFT6#is_known_keyword", function ()
            assert.True(drafts.DRAFT6:is_known_keyword("propertyNames"))
            assert.False(drafts.DRAFT6:is_known_keyword("readOnly"))
        end)

        it("DRAFT4#is_known_keyword", function ()
            assert.True(drafts.DRAFT4:is_known_keyword("type"))
            assert.False(drafts.DRAFT4:is_known_keyword("propertyNames"))
        end)

        it("UNKNOWN#is_known_keyword", function ()
            assert.True(drafts.UNKNOWN:is_known_keyword("$dynamicAnchor"))
            assert.True(drafts.UNKNOWN:is_known_keyword("propertyNames"))
        end)
    end)
end)
