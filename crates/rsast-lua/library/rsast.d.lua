-- SPDX-License-Identifier: MIT

---@meta rsre

local rsre = {}

---@class (exact) rsre.Match: userdata
---
---@field start integer Get the 1-indexed byte start position of the match (inclusive).
---@field stop  integer Get the 1-indexed byte stop position of the match (exclusive).
---@field text  string  Get the matched substring.
rsre.Match = {}

--- Get the start and stop positions of the match.
---
---@see rsre.Match.start
---@see rsre.Match.stop
---
---@return integer, integer
function rsre.Match:range() end

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

---@class (exact) rsre.Regex: userdata
---@field new fun(pattern: string): rsre.Regex
rsre.Regex = {}

--- Compile a new regex.
---
---@param pattern string
---
---@return rsre.Regex
function rsre.Regex.new(pattern) end

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
