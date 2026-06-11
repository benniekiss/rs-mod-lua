local rsre = require("rsre")

describe("api", function ()
    describe("regex#api", function ()
        it("new#regex", function ()
            assert.Not.Error(function ()
                rsre.Regex.new("123")
            end)
        end)

        it("match#regex", function ()
            local re = rsre.Regex.new([[\d{3}]])
            local m = re:match("123")

            assert.True(m)
        end)

        it("find#regex", function ()
            local pattern = [[\d{3}]]
            local hay = "abc123def"

            local re = rsre.Regex.new(pattern)
            local m = re:find(hay)

            local start = m.start
            local stop = m.stop
            local text = m.text

            assert.Equal(4, start)
            assert.Equal(7, stop)
            assert.Equal("123", text)
            assert.Equal(text, hay:sub(start, stop - 1)) -- stop is an exclusive index, but `:sub` is inclusive
        end)

        it("find_from_pos#regex", function ()
            local pattern = [[\d{3}]]
            local hay = "abc123456def"

            local re = rsre.Regex.new(pattern)
            local m = re:find(hay, 6)

            local start = m.start
            local stop = m.stop
            local text = m.text

            assert.Equal(6, start)
            assert.Equal(9, stop)
            assert.Equal("345", text)
            assert.Equal(text, hay:sub(start, stop - 1)) -- stop is an exclusive index, but `:sub` is inclusive
        end)

        describe("captures#regex", function ()
            it("indices#captures", function ()
                local pattern = [[(\d{3})(\d{3})]]
                local hay = "abc123456def"

                local re = rsre.Regex.new(pattern)
                local c = re:captures(hay)

                assert.Equal(3, c:len())

                assert.Equal(4, c:get(1).start)
                assert.Equal(10, c:get(1).stop)
                assert.Equal("123456", c:get(1).text)

                assert.Equal(4, c:get(2).start)
                assert.Equal(7, c:get(2).stop)
                assert.Equal("123", c:get(2).text)

                assert.Equal(7, c:get(3).start)
                assert.Equal(10, c:get(3).stop)
                assert.Equal("456", c:get(3).text)
            end)

            it("names#captures", function ()
                local pattern = [[(?<one>\d{3})(?<two>\d{3})]]
                local hay = "abc123456def"

                local re = rsre.Regex.new(pattern)
                local c = re:captures(hay)

                assert.Equal(3, c:len())

                assert.Equal(4, c:get(1).start)
                assert.Equal(10, c:get(1).stop)
                assert.Equal("123456", c:get(1).text)

                assert.Equal(4, c:get(2).start)
                assert.Equal(7, c:get(2).stop)
                assert.Equal("123", c:get(2).text)

                assert.Equal(4, c:name("one").start)
                assert.Equal(7, c:name("one").stop)
                assert.Equal("123", c:name("one").text)

                assert.Equal(7, c:get(3).start)
                assert.Equal(10, c:get(3).stop)
                assert.Equal("456", c:get(3).text)

                assert.Equal(7, c:name("two").start)
                assert.Equal(10, c:name("two").stop)
                assert.Equal("456", c:name("two").text)
            end)
        end)

        describe("captures_from_pos#regex", function ()
            it("indices#captures_from_pos", function ()
                local pattern = [[(\d{3})(\d{3})]]
                local hay = "abc12345678def"

                local re = rsre.Regex.new(pattern)
                local c = re:captures(hay, 5)

                assert.Equal(3, c:len())

                assert.Equal(5, c:get(1).start)
                assert.Equal(11, c:get(1).stop)
                assert.Equal("234567", c:get(1).text)

                assert.Equal(5, c:get(2).start)
                assert.Equal(8, c:get(2).stop)
                assert.Equal("234", c:get(2).text)

                assert.Equal(8, c:get(3).start)
                assert.Equal(11, c:get(3).stop)
                assert.Equal("567", c:get(3).text)
            end)

            it("names#captures_from_pos", function ()
                local pattern = [[(?<one>\d{3})(?<two>\d{3})]]
                local hay = "abc12345678def"

                local re = rsre.Regex.new(pattern)
                local c = re:captures(hay, 5)

                assert.Equal(3, c:len())

                assert.Equal(5, c:get(1).start)
                assert.Equal(11, c:get(1).stop)
                assert.Equal("234567", c:get(1).text)

                assert.Equal(5, c:get(2).start)
                assert.Equal(8, c:get(2).stop)
                assert.Equal("234", c:get(2).text)

                assert.Equal(5, c:name("one").start)
                assert.Equal(8, c:name("one").stop)
                assert.Equal("234", c:name("one").text)

                assert.Equal(8, c:get(3).start)
                assert.Equal(11, c:get(3).stop)
                assert.Equal("567", c:get(3).text)

                assert.Equal(8, c:name("two").start)
                assert.Equal(11, c:name("two").stop)
                assert.Equal("567", c:name("two").text)
            end)
        end)

        it("replace#regex", function ()
            local re = rsre.Regex.new("foo")
            local rep = "bar"

            local te = "foofoobarbarfoobarbaz"
            local ex = "barfoobarbarfoobarbaz"

            assert.Equal(ex, re:replace(te, rep, 1))
        end)

        it("replace_all#regex", function ()
            local re = rsre.Regex.new("foo")
            local rep = "bar"

            local te = "foofoobarbarfoobarbaz"
            local ex = "barbarbarbarbarbarbaz"

            assert.Equal(ex, re:replace(te, rep))
        end)

        it("replacen#regex", function ()
            local re = rsre.Regex.new("foo")
            local rep = "bar"

            local te = "foofoobarbarfoobarbaz"
            local ex = "barbarbarbarfoobarbaz"

            assert.Equal(ex, re:replace(te, rep, 2))
        end)

        it("split#regex", function ()
            local re = rsre.Regex.new("X")
            local te = "fooXbarX baz XbuzzX"
            local ex = { "foo", "bar", " baz ", "buzz", "" }

            assert.Same(ex, re:split(te))
        end)

        it("splitn#regex", function ()
            local re = rsre.Regex.new("X")
            local te = "fooXbarX baz XbuzzX"
            local ex = { "foo", "bar", " baz XbuzzX" }

            assert.Same(ex, re:split(te, 3))
        end)
    end)
end)
