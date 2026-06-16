// SPDX-License-Identifier: MIT

use std::{collections::BTreeMap, ops::Deref};

#[derive(mlua::UserData, Clone)]
pub(crate) struct LuaMatch {
    #[lua(get)]
    start: usize,
    #[lua(get)]
    stop: usize,
    #[lua(get)]
    text: String,
}

impl From<fancy_regex::Match<'_>> for LuaMatch {
    fn from(m: fancy_regex::Match<'_>) -> Self {
        Self {
            start: m.start().saturating_add(1), // since lua is 1-indexed
            stop: m.end().saturating_add(1),    // since lua is 1-indexed
            text: m.as_str().to_string(),
        }
    }
}

#[mlua::userdata_impl]
impl LuaMatch {
    #[lua(meta, name = "__tostring", infallible)]
    pub(crate) fn lua_tostring(&self) -> String {
        self.text.clone()
    }

    #[lua(name = "range", infallible)]
    pub(crate) fn lua_range(&self) -> (usize, usize) {
        (self.start, self.stop)
    }
}

#[derive(mlua::UserData, Clone)]
pub(crate) struct LuaCaptures {
    #[lua(skip)]
    matches: Vec<Option<LuaMatch>>,
    #[lua(skip)]
    names: BTreeMap<String, usize>,
}

impl From<fancy_regex::Captures<'_>> for LuaCaptures {
    fn from(captures: fancy_regex::Captures) -> Self {
        Self {
            matches: captures
                .iter()
                .map(|v| v.map(LuaMatch::from))
                .collect::<Vec<Option<LuaMatch>>>(),
            names: BTreeMap::new(),
        }
    }
}

#[mlua::userdata_impl]
impl LuaCaptures {
    #[lua(skip)]
    pub(crate) fn get(&self, index: usize) -> Option<LuaMatch> {
        self.matches.get(index)?.clone()
    }

    #[lua(skip)]
    pub(crate) fn set_names(&mut self, names: Vec<Option<String>>) {
        for (i, name) in names.into_iter().enumerate() {
            if let Some(n) = name {
                self.names.insert(n, i);
            }
        }
    }

    #[lua(name = "get", infallible)]
    pub(crate) fn lua_get(&self, index: usize) -> Option<LuaMatch> {
        self.matches.get(index.saturating_sub(1))?.clone()
    }

    #[lua(name = "name", infallible)]
    pub(crate) fn lua_name(&self, name: &str) -> Option<LuaMatch> {
        if let Some(index) = self.names.get(name) {
            self.get(*index)
        } else {
            None
        }
    }

    #[lua(name = "len", infallible)]
    pub(crate) fn lua_len(&self) -> usize {
        self.matches.len()
    }
}

#[derive(mlua::UserData, Clone)]
pub(crate) struct LuaRegex(fancy_regex::Regex);

impl From<fancy_regex::Regex> for LuaRegex {
    fn from(value: fancy_regex::Regex) -> Self {
        LuaRegex(value)
    }
}

impl From<LuaRegex> for fancy_regex::Regex {
    fn from(value: LuaRegex) -> Self {
        value.0
    }
}

impl Deref for LuaRegex {
    type Target = fancy_regex::Regex;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[mlua::userdata_impl]
impl LuaRegex {
    #[lua(meta, name = "__tostring", infallible)]
    pub(crate) fn lua_tostring(&self) -> String {
        self.0.as_str().to_string()
    }

    #[lua(name = "new")]
    pub(crate) fn lua_new(patt: &str) -> mlua::Result<Self> {
        fancy_regex::Regex::new(patt)
            .map(|re| re.into())
            .map_err(mlua::Error::external)
    }

    #[lua(name = "match")]
    pub(crate) fn lua_is_match(&self, hay: &str) -> mlua::Result<bool> {
        self.0.is_match(hay).map_err(mlua::Error::external)
    }

    #[lua(name = "find")]
    pub(crate) fn lua_find_from_pos(
        &self,
        hay: &str,
        start: Option<usize>,
    ) -> mlua::Result<Option<LuaMatch>> {
        let start = start.unwrap_or(1).saturating_sub(1);
        self.0
            .find_from_pos(hay, start)
            .map(|r| r.map(LuaMatch::from))
            .map_err(mlua::Error::external)
    }

    #[lua(name = "captures")]
    pub(crate) fn lua_captures_from_pos(
        &self,
        hay: &str,
        start: Option<usize>,
    ) -> mlua::Result<Option<LuaCaptures>> {
        let start = start.unwrap_or(1);
        let mut captures = self
            .0
            .captures_from_pos(hay, start - 1)
            .map(|r| r.map(LuaCaptures::from))
            .map_err(mlua::Error::external)?;

        if let Some(ref mut c) = captures {
            c.set_names(
                self.0
                    .capture_names()
                    .map(|v| v.map(|s| s.to_string()))
                    .collect::<Vec<Option<String>>>(),
            );
        }

        Ok(captures)
    }

    #[lua(name = "replace")]
    pub(crate) fn lua_try_replacen(
        &self,
        text: &str,
        rep: &str,
        limit: Option<usize>,
    ) -> mlua::Result<String> {
        let limit = limit.unwrap_or(0);
        self.0
            .try_replacen(text, limit, rep)
            .map(|s| s.to_string())
            .map_err(mlua::Error::external)
    }

    #[lua(name = "split")]
    pub(crate) fn lua_splitn(
        &self,
        target: &str,
        limit: Option<usize>,
    ) -> mlua::Result<Vec<String>> {
        match limit {
            Some(l) => self
                .0
                .splitn(target, l)
                .map(|r| r.map(|s| s.to_string()))
                .collect::<Result<Vec<String>, _>>()
                .map_err(mlua::Error::external),
            None => self
                .0
                .split(target)
                .map(|r| r.map(|s| s.to_string()))
                .collect::<Result<Vec<String>, _>>()
                .map_err(mlua::Error::external),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_match_from() {
        let re = fancy_regex::Regex::new(r"\d{3}").unwrap();

        let m = re.find("abc123def").map(|r| r.map(LuaMatch::from));

        assert!(m.is_ok())
    }

    #[test]
    fn test_captures_from() {
        let re = fancy_regex::Regex::new(r"\d{3}").unwrap();

        let c = re.captures("abc123def").map(|r| r.map(LuaCaptures::from));

        assert!(c.is_ok())
    }

    #[test]
    fn test_captures_get() {
        let re = fancy_regex::Regex::new(r".*(?<digits>\d{3}).*").unwrap();

        let c = re
            .captures("abc123def")
            .map(|r| r.map(LuaCaptures::from))
            .unwrap()
            .unwrap();

        // Note that `get()` is 1-indexed
        assert_eq!(c.lua_get(1).unwrap().text, "abc123def");
        assert_eq!(c.lua_get(2).unwrap().text, "123");
    }

    #[test]
    fn test_captures_name() {
        let re = fancy_regex::Regex::new(r".*(?<digits>\d{3}).*").unwrap();

        let mut c = re
            .captures("abc123def")
            .map(|r| r.map(LuaCaptures::from))
            .unwrap()
            .unwrap();

        assert!(c.lua_name("digits").is_none());

        c.set_names(
            re.capture_names()
                .map(|v| v.map(|s| s.to_string()))
                .collect::<Vec<Option<String>>>(),
        );

        assert_eq!(c.lua_name("digits").unwrap().text, "123");
    }

    #[test]
    fn test_captures_len() {
        let re = fancy_regex::Regex::new(r".*(?<digits>\d{3}).*").unwrap();

        let c = re
            .captures("abc123def")
            .map(|r| r.map(LuaCaptures::from))
            .unwrap()
            .unwrap();

        assert_eq!(c.lua_len(), 2);
    }
}
