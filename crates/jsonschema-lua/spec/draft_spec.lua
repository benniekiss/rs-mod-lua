local jsonschema = require("jsonschema")

describe("draft#jsonschema", function ()
    local Draft = jsonschema.Draft

    it("tostring#draft", function ()
        assert.Equal("Draft201909", tostring(Draft.DRAFT201909))
        assert.Equal("Draft202012", tostring(Draft.DRAFT202012))
        assert.Equal("Draft4", tostring(Draft.DRAFT4))
        assert.Equal("Draft6", tostring(Draft.DRAFT6))
        assert.Equal("Draft7", tostring(Draft.DRAFT7))
    end)

    it("from_schema_uri#draft", function ()
        assert.Equal(
            Draft.DRAFT202012,
            Draft.from_schema_uri("https://json-schema.org/draft/2020-12/schema")
        )
        assert.Equal(
            Draft.DRAFT201909,
            Draft.from_schema_uri("https://json-schema.org/draft/2019-09/schema")
        )
        assert.Equal(Draft.DRAFT7, Draft.from_schema_uri("https://json-schema.org/draft-07/schema"))
        assert.Equal(Draft.DRAFT6, Draft.from_schema_uri("https://json-schema.org/draft-06/schema"))
        assert.Equal(Draft.DRAFT4, Draft.from_schema_uri("https://json-schema.org/draft-04/schema"))
        assert.Equal(Draft.UNKNOWN, Draft.from_schema_uri("unknown_schema"))
    end)

    describe("detect#draft", function ()
        it("empty schema#draft", function ()
            assert.Equal(Draft.DRAFT202012, Draft.DRAFT202012:detect({}))
            assert.Equal(Draft.DRAFT201909, Draft.DRAFT201909:detect({}))
            assert.Equal(Draft.DRAFT7, Draft.DRAFT7:detect({}))
            assert.Equal(Draft.DRAFT6, Draft.DRAFT6:detect({}))
            assert.Equal(Draft.DRAFT4, Draft.DRAFT4:detect({}))
            assert.Equal(Draft.UNKNOWN, Draft.UNKNOWN:detect({}))
        end)

        it("schema#detect", function ()
            assert.Equal(
                Draft.DRAFT202012,
                Draft.DRAFT4:detect({
                    ["$schema"] = "https://json-schema.org/draft/2020-12/schema",
                })
            )
        end)
    end)

    describe("is_known_keyword#draft", function ()
        it("DRAFT202012#is_known_keyword", function ()
            assert.True(Draft.DRAFT202012:is_known_keyword("$dynamicAnchor"))
            assert.False(Draft.DRAFT202012:is_known_keyword("foo"))
        end)

        it("DRAFT201909#is_known_keyword", function ()
            assert.True(Draft.DRAFT201909:is_known_keyword("$anchor"))
            assert.False(Draft.DRAFT201909:is_known_keyword("$dynamicAnchor"))
        end)

        it("DRAFT7#is_known_keyword", function ()
            assert.True(Draft.DRAFT7:is_known_keyword("if"))
            assert.False(Draft.DRAFT7:is_known_keyword("$anchor"))
        end)

        it("DRAFT6#is_known_keyword", function ()
            assert.True(Draft.DRAFT6:is_known_keyword("propertyNames"))
            assert.False(Draft.DRAFT6:is_known_keyword("readOnly"))
        end)

        it("DRAFT4#is_known_keyword", function ()
            assert.True(Draft.DRAFT4:is_known_keyword("type"))
            assert.False(Draft.DRAFT4:is_known_keyword("propertyNames"))
        end)

        it("UNKNOWN#is_known_keyword", function ()
            assert.True(Draft.UNKNOWN:is_known_keyword("$dynamicAnchor"))
            assert.True(Draft.UNKNOWN:is_known_keyword("propertyNames"))
        end)
    end)
end)
