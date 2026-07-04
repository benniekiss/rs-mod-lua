local jsonschema = require("jsonschema")

describe("evaluation#jsonschema", function ()
    local schema = {
        ["$schema"] = "https://json-schema.org/draft/2020-12/schema",
        ["properties"] = {
            ["foo"] = {
                ["type"] = "array",
                ["maxItems"] = 3,
                ["contains"] = {
                    ["type"] = "integer",
                },
            },
        },
        ["additionalProperties"] = false,
    }

    describe("valid", function ()
        assert:set_parameter("TableFormatLevel", 15)

        local instance = {
            ["foo"] = {
                "1",
                "2",
                3,
            },
        }

        it("flag#evaluation", function ()
            local eval = jsonschema.evaluate(schema, instance)
            assert.True(eval:flag().valid)
        end)

        it("list#evaluation", function ()
            local eval = jsonschema.evaluate(schema, instance)
            local ex = {
                ['details'] = {
                    [1] = {
                        ['evaluationPath'] = "",
                        ['instanceLocation'] = "",
                        ['schemaLocation'] = "",
                        ['valid'] = true,
                    },
                    [2] = {
                        ['evaluationPath'] = "/additionalProperties",
                        ['instanceLocation'] = "",
                        ['schemaLocation'] = "/additionalProperties",
                        ['valid'] = true,
                    },
                    [3] = {
                        ['evaluationPath'] = "/properties/foo",
                        ['instanceLocation'] = "/foo",
                        ['schemaLocation'] = "/properties/foo",
                        ['valid'] = true,
                    },
                    [4] = {
                        ['evaluationPath'] = "/properties/foo/type",
                        ['instanceLocation'] = "/foo",
                        ['schemaLocation'] = "/properties/foo/type",
                        ['valid'] = true,
                    },
                    [5] = {
                        ['evaluationPath'] = "/properties/foo/maxItems",
                        ['instanceLocation'] = "/foo",
                        ['schemaLocation'] = "/properties/foo/maxItems",
                        ['valid'] = true,
                    },
                    [6] = {
                        ['annotations'] = {
                            [1] = 2,
                        },
                        ['evaluationPath'] = "/properties/foo/contains",
                        ['instanceLocation'] = "/foo",
                        ['schemaLocation'] = "/properties/foo/contains",
                        ['valid'] = true,
                    },
                    [7] = {
                        ['evaluationPath'] = "/properties/foo/contains",
                        ['instanceLocation'] = "/foo/2",
                        ['schemaLocation'] = "/properties/foo/contains",
                        ['valid'] = true,
                    },
                    [8] = {
                        ['evaluationPath'] = "/properties/foo/contains/type",
                        ['instanceLocation'] = "/foo/2",
                        ['schemaLocation'] = "/properties/foo/contains/type",
                        ['valid'] = true,
                    },
                },
                ['valid'] = true,
            }
            assert.Same(ex, eval:list())
        end)

        it("hierarchical#evaluation", function ()
            local eval = jsonschema.evaluate(schema, instance)
            local ex = {
                ['details'] = {
                    [1] = {
                        ['details'] = {
                            [1] = {
                                ['details'] = {
                                    [1] = {
                                        ['evaluationPath'] = "/properties/foo/type",
                                        ['instanceLocation'] = "/foo",
                                        ['schemaLocation'] = "/properties/foo/type",
                                        ['valid'] = true,
                                    },
                                    [2] = {
                                        ['evaluationPath'] = "/properties/foo/maxItems",
                                        ['instanceLocation'] = "/foo",
                                        ['schemaLocation'] = "/properties/foo/maxItems",
                                        ['valid'] = true,
                                    },
                                    [3] = {
                                        ['annotations'] = {
                                            [1] = 2,
                                        },
                                        ['details'] = {
                                            [1] = {
                                                ['details'] = {
                                                    [1] = {
                                                        ['evaluationPath'] = "/properties/foo/contains/type",
                                                        ['instanceLocation'] = "/foo/2",
                                                        ['schemaLocation'] = "/properties/foo/contains/type",
                                                        ['valid'] = true,
                                                    },
                                                },
                                                ['evaluationPath'] = "/properties/foo/contains",
                                                ['instanceLocation'] = "/foo/2",
                                                ['schemaLocation'] = "/properties/foo/contains",
                                                ['valid'] = true,
                                            },
                                        },
                                        ['evaluationPath'] = "/properties/foo/contains",
                                        ['instanceLocation'] = "/foo",
                                        ['schemaLocation'] = "/properties/foo/contains",
                                        ['valid'] = true,
                                    },
                                },
                                ['evaluationPath'] = "/properties/foo",
                                ['instanceLocation'] = "/foo",
                                ['schemaLocation'] = "/properties/foo",
                                ['valid'] = true,
                            },
                        },
                        ['evaluationPath'] = "/additionalProperties",
                        ['instanceLocation'] = "",
                        ['schemaLocation'] = "/additionalProperties",
                        ['valid'] = true,
                    },
                },
                ['evaluationPath'] = "",
                ['instanceLocation'] = "",
                ['schemaLocation'] = "",
                ['valid'] = true,
            }
            assert.Same(ex, eval:hierarchical())
        end)

        it("annotations#evaluation", function ()
            local eval = jsonschema.evaluate(schema, instance)
            local ex = {
                [1] = {
                    ['annotations'] = {
                        [1] = 2,
                    },
                    ['instance_location'] = "/foo",
                    ['schema_location'] = "/properties/foo/contains",
                },
            }
            assert.Same(ex, eval:annotations())
        end)

        it("errors#evaluation", function ()
            local eval = jsonschema.evaluate(schema, instance)
            local ex = {}
            assert.Same(ex, eval:errors())
        end)
    end)

    describe("invalid", function ()
        assert:set_parameter("TableFormatLevel", 10)

        local instance = {
            ["foo"] = {
                "1",
                "2",
                "3",
                "4",
            },
        }

        it("flag#evaluation", function ()
            local eval = jsonschema.evaluate(schema, instance)
            assert.False(eval:flag().valid)
        end)

        it("list#evaluation", function ()
            local eval = jsonschema.evaluate(schema, instance)
            local ex = {
                ["details"] = {
                    [1] = {
                        ["evaluationPath"] = "",
                        ["instanceLocation"] = "",
                        ["schemaLocation"] = "",
                        ["valid"] = false,
                    },
                    [2] = {
                        ["evaluationPath"] = "/additionalProperties",
                        ['instanceLocation'] = "",
                        ['schemaLocation'] = "/additionalProperties",
                        ['valid'] = false,
                    },
                    [3] = {
                        ['evaluationPath'] = "/properties/foo",
                        ['instanceLocation'] = "/foo",
                        ['schemaLocation'] = "/properties/foo",
                        ['valid'] = false,
                    },
                    [4] = {
                        ['evaluationPath'] = "/properties/foo/type",
                        ['instanceLocation'] = "/foo",
                        ['schemaLocation'] = "/properties/foo/type",
                        ['valid'] = true,
                    },
                    [5] = {
                        ['errors'] = {
                            ['maxItems'] = '["1","2","3","4"] has more than 3 items',
                        },
                        ['evaluationPath'] = "/properties/foo/maxItems",
                        ['instanceLocation'] = "/foo",
                        ['schemaLocation'] = "/properties/foo/maxItems",
                        ['valid'] = false,
                    },
                    [6] = {
                        ['errors'] = {
                            ['contains'] = 'None of ["1","2","3","4"] are valid under the given schema',
                        },
                        ['evaluationPath'] = "/properties/foo/contains",
                        ['instanceLocation'] = "/foo",
                        ['schemaLocation'] = "/properties/foo/contains",
                        ['valid'] = false,
                    },
                },
                ['valid'] = false,
            }
            assert.Same(ex, eval:list())
        end)

        it("hierarchical#evaluation", function ()
            local eval = jsonschema.evaluate(schema, instance)
            local ex = {
                ['details'] = {
                    [1] = {
                        ['details'] = {
                            [1] = {
                                ['details'] = {
                                    [1] = {
                                        ['evaluationPath'] = "/properties/foo/type",
                                        ['instanceLocation'] = "/foo",
                                        ['schemaLocation'] = "/properties/foo/type",
                                        ['valid'] = true,
                                    },
                                    [2] = {
                                        ['errors'] = {
                                            ['maxItems'] = '["1","2","3","4"] has more than 3 items',
                                        },
                                        ['evaluationPath'] = "/properties/foo/maxItems",
                                        ['instanceLocation'] = "/foo",
                                        ['schemaLocation'] = "/properties/foo/maxItems",
                                        ['valid'] = false,
                                    },
                                    [3] = {
                                        ['errors'] = {
                                            ['contains'] = 'None of ["1","2","3","4"] are valid under the given schema',
                                        },
                                        ['evaluationPath'] = "/properties/foo/contains",
                                        ['instanceLocation'] = "/foo",
                                        ['schemaLocation'] = "/properties/foo/contains",
                                        ['valid'] = false,
                                    },
                                },
                                ['evaluationPath'] = "/properties/foo",
                                ['instanceLocation'] = "/foo",
                                ['schemaLocation'] = "/properties/foo",
                                ['valid'] = false,
                            },
                        },
                        ['evaluationPath'] = "/additionalProperties",
                        ['instanceLocation'] = "",
                        ['schemaLocation'] = "/additionalProperties",
                        ['valid'] = false,
                    },
                },
                ['evaluationPath'] = "",
                ['instanceLocation'] = "",
                ['schemaLocation'] = "",
                ['valid'] = false,
            }
            assert.Same(ex, eval:hierarchical())
        end)

        it("annotations#evaluation", function ()
            local eval = jsonschema.evaluate(schema, instance)
            local ex = {}
            assert.Same(ex, eval:annotations())
        end)

        it("errors#evaluation", function ()
            local eval = jsonschema.evaluate(schema, instance)
            local ex = {
                [1] = {
                    ['error'] = {
                        ['keyword'] = "maxItems",
                        ['message'] = '["1","2","3","4"] has more than 3 items',
                    },
                    ['instance_location'] = "/foo",
                    ['schema_location'] = "/properties/foo/maxItems",
                },
                [2] = {
                    ['error'] = {
                        ['keyword'] = "contains",
                        ['message'] = 'None of ["1","2","3","4"] are valid under the given schema',
                    },
                    ['instance_location'] = "/foo",
                    ['schema_location'] = "/properties/foo/contains",
                },
            }
            assert.Same(ex, eval:errors())
        end)
    end)
end)
