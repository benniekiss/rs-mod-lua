-- SPDX-License-Identifier: MIT

---@meta rsre

local rsre = {}

--- Controls how the regex engine handles input encoding.
---
--- Variants:
---
--- - *unicode*:
---
---   - Input is expected to be valid UTF-8
---   - . matches any Unicode scalar value (except \n unless dot_matches_new_line is set)
---   - \w, \d, \s match Unicode characters
---   - Unicode properties like \p{Letter} are available
---   - Word boundaries (\b) are Unicode-aware
---
--- - *ascii*:
---
---   - Input can be arbitrary bytes (no UTF-8 requirement)
---   - . matches any single byte (except \n unless dot_matches_new_line is set)
---   - \w matches [a-zA-Z0-9_] only (ASCII)
---   - \d matches [0-9] only (ASCII)
---   - \s matches ASCII whitespace only
---   - Unicode properties are not available
---   - Word boundaries (\b) use ASCII-only word characters
---
---   Use this mode when matching raw binary data or filenames that may
---   contain non-UTF-8 bytes and you don’t need Unicode character classes.
---
--- - *unicode_bytes*:
---
---   - Input can be arbitrary bytes (no UTF-8 requirement)
---   - . matches Unicode scalar values (sequences of valid UTF-8 bytes)
---   - \w, \d, \s match Unicode characters
---   - Unicode properties like \p{Letter} are available
---
---   Use this mode when the input may contain non-UTF-8 bytes but you
---   still want Unicode-aware character classes. Note that . will not
---   match individual non-UTF-8 bytes — it only matches valid UTF-8
---   codepoint sequences.
---
---@enum rsre.BytesMode
rsre.BytesMode = {
    UNICODE = "unicode",
    ASCII = "ascii",
    UNICODE_BYTES = "unicode_bytes",
}

---@class (exact) rsre.Match: table
---
---@field start integer The 1-indexed byte start position of the match (inclusive).
---@field stop  integer The 1-indexed byte stop position of the match (exclusive).
---@field text  string  Get the matched substring.
rsre.Match = {}

---@class (exact) rsre.Captures: userdata
rsre.Captures = {}

--- Get the match at the provided index.
---
---@param index integer
---
---@return rsre.Match
function rsre.Captures:get(index) end

--- Get match with the provided name.
---
---@param name string
---
---@return rsre.Match
function rsre.Captures:name(name) end

--- Get the total number of matches.
---
---@return integer
function rsre.Captures:len() end

---@class rsre.Options: userdata
---
rsre.Options = {}

--- Create a new Options
---
---@return rsre.Options
function rsre.Options.new() end

--- Compile a regex with these options
---
---@param pattern string The pattern to compile
---
---@return rsre.Regex
function rsre.Options:build(pattern) end

--- Set case-insensitivity matching.`
---
--- Equivalent to the `i` flag in a pattern.
---
---@param yes boolean
---
---@return self
function rsre.Options:case_insensitive(yes) end

--- Set multiline matching.
---
--- Equivalent to the `m` flag in a pattern.
---
---@param yes boolean
---
---@return self
function rsre.Options:multi_line(yes) end

--- Set whether to ignore whitespace.
---
---@param yes boolean
---
---@return self
function rsre.Options:ignore_whitespace(yes) end

--- Set whether dot (`.`) matches a new line character.
---
--- Equivalent to the `s` flag in a pattern.
---
---@param yes boolean
---
---@return self
function rsre.Options:dot_matches_new_line(yes) end

--- Set `\r\n` as a single line ending in multiline mode.
---
--- Equivalent to the `R` flag in a pattern.
---
---@param yes boolean
---
---@return self
function rsre.Options:crlf(yes) end

--- Set verbose mode.
---
--- Equivalent to the `x` flag in a pattern.
---
---@param yes boolean
---
---@return self
function rsre.Options:verbose_mode(yes) end

--- Set unicode mode.
---
--- Equivalent to the `u` flag in a pattern.
---
---@param yes boolean
---
---@return self
function rsre.Options:unicode_mode(yes) end

--- Set the limit for how many times backtracking should be
--- attempted for regexes where backtracking is used. If this
--- limit is exceeded, execution errors.
---
--- This can prevent a regex with catastrophic backtracking
--- from running too long.
---
--- Default is `1_000_000`.
---
---@param limit integer
---
---@return self
function rsre.Options:backtrack_limit(limit) end

--- Set the approximate size limit of the compiled regular expression.
---
---@param limit integer
---
---@return self
function rsre.Options:delegate_size_limit(limit) end

--- Set the approximate capacity, in bytes, of the cache used
--- by the lazy DFA of delegated regexes.
---
---@param limit integer
---
---@return self
function rsre.Options:delegate_dfa_size_limit(limit) end

--- Set whether the reverse lazy DFA built for a variable
--- length lookbehind may exceed the limit set by
--- [`delegate_dfa_size_limit`](lua-rsre.RegexOptions.delegate_dfa_size_limit)
--- when it needs a larger minimum cache capacity.
---
--- When `true` (the default), the cache capacity is rounded up
--- to the minimum that the DFA requires. When `false`, the limit
--- is strictly honored and a pattern whose DFA needs more capacity
--- fails to compile
---
---@param yes boolean
---
---@return self
function rsre.Options:delegate_dfa_skip_cache_capacity_check(yes) end

--- Set the maximum number of VM instructions the compiled
--- program may contain.
---
---@param limit integer
---
---@return self
function rsre.Options:max_prog_size(limit) end

--- Require that matches are not empty, i.e. match at least one
--- character.
---
--- When enabled and the pattern will only ever produce an empty
--- match, the regex will fail to compile.
---
---@param yes boolean
---
---@return self
function rsre.Options:find_not_empty(yes) end

--- Set whether to perform expression-tree optimizations.
---
---@param yes boolean
---
---@return self
function rsre.Options:optimize(yes) end

--- Set whether to treat unnamed capture groups as non-capturing
--- when named groups exist.
---
--- This prevents accessing capture groups by number from within the
--- pattern (backrefs, subroutine calls) when named groups are
--- present.
---
---@param yes boolean
---
---@return self
function rsre.Options:ignore_numbered_groups_when_named_groups_exist(yes) end

--- **EXPERIMENTAL API**
---
--- Set whether to enable the seek pre-filter optimization for hard
--- (backtracking) patterns.
---
---@see https://docs.rs/fancy-regex/latest/fancy_regex/struct.RegexOptionsBuilder.html#method.seek
---
---@param yes boolean
---
---@return self
function rsre.Options:seek(yes) end

--- Set whether to better match the [Oniguruma](https://github.com/kkos/oniguruma)
--- default parsing behavior.
---
---@see https://docs.rs/fancy-regex/latest/fancy_regex/struct.RegexOptionsBuilder.html#method.oniguruma_mode
---
---@param yes boolean
---
---@return self
function rsre.Options:oniguruma_mode(yes) end

--- Set the input encoding mode for the regex.
---
---@param mode rsre.BytesMode
---
---@return self
function rsre.Options:bytes_mode(mode) end

--- Set whether to ignore trailing newline for the purposes of
--- anchors, ^ and $, and lookarounds, unless \z is specifically
--- used to anchor to the end of the string.
---
---@param yes boolean
---
---@return self
function rsre.Options:disallow_empty_match_at_eof_after_newline(yes) end

--- Set whether to build a prefilter for the regex.
---
--- If the regex only does an anchored search, the prefilter would not
--- be used, and it would waste time and memory to build it.
---
---@param yes boolean
---
---@return self
function rsre.Options:build_delegate_prefilter(yes) end

--- Get a list of bytes that a match of `pattern` can start with.
---
--- Returns a list of possible matching bytes, or `nil` if
--- the byte set could not be built.
---
--- Use `string.char` or `utf8.char` to get the string representation.
---
---@param pattern string
---
---@return integer[]?
function rsre.Options:start_bytes(pattern) end

--- Get a list of bytes that a match of `pattern` must contain.
---
--- Use `string.char` or `utf8.char` to get the string representation.
---
---@param pattern string
---
---@return integer[]
function rsre.Options:required_bytes(pattern) end

---@class (exact) rsre.Regex: userdata
rsre.Regex = {}

--- Compile a new regex.
---
---@param pattern string
---
---@return rsre.Regex
function rsre.Regex.new(pattern) end

--- Compile a new regex with the provided options.
---
--- Equivalent to calling `options:build(patter)`
---
---@param pattern string
---@param options rsre.Options
---
---@return rsre.Regex
function rsre.Regex.new_with_options(pattern, options) end

--- Test if the regex matches the provided string.
---
---@param hay string
---
---@return boolean
function rsre.Regex:match(hay) end

--- Find the first, leftmost match after `pos`.
---
---@param hay  string
---@param pos? integer
---
---@return rsre.Match
function rsre.Regex:find(hay, pos) end

--- Find the first, leftmost captures after `pos`.
---
---@param hay  string
---@param pos? integer
---
---@return rsre.Captures
function rsre.Regex:captures(hay, pos) end

--- Replace matches in `text` with `rep`, optionally up to `limit` times.
---
---@param text   string
---@param rep    string
---@param limit? integer
---
---@return string
function rsre.Regex:replace(text, rep, limit) end

--- Split a string at every match, optionally up to `limit` times.
---
---@param text   string
---@param limit? integer
---
---@return string[]
function rsre.Regex:split(text, limit) end

--- The original regex as a string.
---
---@return string
function rsre.Regex:__tostring() end

--- Escapes special characters in text with '\'. Returns a string which,
--- when interpreted as a regex, matches exactly `text`.
---
---@param text string The string to escape
---
---@return string
function rsre.escape(text) end

return rsre
