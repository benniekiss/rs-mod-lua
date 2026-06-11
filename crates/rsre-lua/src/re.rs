// SPDX-License-Identifier: MIT

use std::collections::BTreeMap;

use fancy_regex as regex;

#[derive(mlua::UserData, Clone)]
pub(crate) struct LuaMatch {
    #[lua(get)]
    start: usize,
    #[lua(get)]
    stop: usize,
    #[lua(get)]
    text: String,
}

impl From<regex::Match<'_>> for LuaMatch {
    fn from(m: regex::Match<'_>) -> Self {
        Self {
            start: m.start().saturating_add(1), // since lua is 1-indexed
            stop: m.end().saturating_add(1),    // since lua is 1-indexed
            text: m.as_str().to_string(),
        }
    }
}

#[mlua::userdata_impl]
impl LuaMatch {
    #[lua(infallible)]
    pub(crate) fn range(&self) -> (usize, usize) {
        (self.start, self.stop)
    }

    #[lua(meta, name = "__tostring", infallible)]
    pub(crate) fn tostring(&self) -> String {
        self.text.clone()
    }
}

#[derive(mlua::UserData, Clone)]
pub(crate) struct LuaCaptures {
    #[lua(skip)]
    matches: Vec<Option<LuaMatch>>,
    #[lua(skip)]
    names: BTreeMap<String, usize>,
}

impl From<regex::Captures<'_>> for LuaCaptures {
    fn from(captures: regex::Captures) -> Self {
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
    pub(crate) fn set_names(&mut self, names: Vec<Option<String>>) {
        for (i, name) in names.into_iter().enumerate() {
            if let Some(n) = name {
                self.names.insert(n, i);
            }
        }
    }

    #[lua(infallible)]
    pub(crate) fn get(&self, index: usize) -> Option<LuaMatch> {
        self.matches.get(index.saturating_sub(1))?.clone()
    }

    #[lua(infallible)]
    pub(crate) fn name(&self, name: &str) -> Option<LuaMatch> {
        if let Some(index) = self.names.get(name) {
            self.get(index.saturating_add(1))
        } else {
            None
        }
    }

    #[lua(infallible)]
    pub(crate) fn len(&self) -> usize {
        self.matches.len()
    }
}

#[derive(mlua::UserData)]
pub(crate) struct LuaRegex {
    #[lua(skip)]
    re: regex::Regex,
}

#[mlua::userdata_impl]
impl LuaRegex {
    pub(crate) fn new(patt: &str) -> mlua::Result<Self> {
        regex::Regex::new(patt)
            .map(|re| Self { re })
            .map_err(mlua::Error::external)
    }

    #[lua(meta, name = "__tostring", infallible)]
    pub(crate) fn tostring(&self) -> String {
        self.re.as_str().to_string()
    }

    #[lua(name = "match")]
    pub(crate) fn is_match(&self, hay: String) -> mlua::Result<bool> {
        self.re.is_match(&hay).map_err(mlua::Error::external)
    }

    #[lua(name = "find")]
    pub(crate) fn find_from_pos(
        &self,
        hay: String,
        start: Option<usize>,
    ) -> mlua::Result<Option<LuaMatch>> {
        let start = start.unwrap_or(1).saturating_sub(1);
        self.re
            .find_from_pos(&hay, start)
            .map(|r| r.map(LuaMatch::from))
            .map_err(mlua::Error::external)
    }

    #[lua(name = "captures")]
    pub(crate) fn captures_from_pos(
        &self,
        hay: String,
        start: Option<usize>,
    ) -> mlua::Result<Option<LuaCaptures>> {
        let start = start.unwrap_or(1);
        let mut captures = self
            .re
            .captures_from_pos(&hay, start - 1)
            .map(|r| r.map(LuaCaptures::from))
            .map_err(mlua::Error::external)?;

        if let Some(ref mut c) = captures {
            c.set_names(
                self.re
                    .capture_names()
                    .map(|v| v.map(|s| s.to_string()))
                    .collect::<Vec<Option<String>>>(),
            );
        }

        Ok(captures)
    }

    #[lua(name = "replace")]
    pub(crate) fn try_replacen(
        &self,
        text: String,
        rep: String,
        limit: Option<usize>,
    ) -> mlua::Result<String> {
        let limit = limit.unwrap_or(0);
        self.re
            .try_replacen(&text, limit, rep)
            .map(|s| s.to_string())
            .map_err(mlua::Error::external)
    }

    #[lua(name = "split")]
    pub(crate) fn splitn(&self, target: String, limit: Option<usize>) -> mlua::Result<Vec<String>> {
        match limit {
            Some(l) => self
                .re
                .splitn(&target, l)
                .map(|r| r.map(|s| s.to_string()))
                .collect::<Result<Vec<String>, _>>()
                .map_err(mlua::Error::external),
            None => self
                .re
                .split(&target)
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
        let re = regex::Regex::new(r"\d{3}").unwrap();

        let m = re.find("abc123def").map(|r| r.map(LuaMatch::from));

        assert!(m.is_ok())
    }

    #[test]
    fn test_captures_from() {
        let re = regex::Regex::new(r"\d{3}").unwrap();

        let c = re.captures("abc123def").map(|r| r.map(LuaCaptures::from));

        assert!(c.is_ok())
    }

    #[test]
    fn test_captures_get() {
        let re = regex::Regex::new(r".*(?<digits>\d{3}).*").unwrap();

        let c = re
            .captures("abc123def")
            .map(|r| r.map(LuaCaptures::from))
            .unwrap()
            .unwrap();

        assert_eq!(c.get(0).unwrap().text, "abc123def");
        assert_eq!(c.get(1).unwrap().text, "123");
    }

    #[test]
    fn test_captures_name() {
        let re = regex::Regex::new(r".*(?<digits>\d{3}).*").unwrap();

        let mut c = re
            .captures("abc123def")
            .map(|r| r.map(LuaCaptures::from))
            .unwrap()
            .unwrap();

        assert!(c.name("digits").is_none());

        c.set_names(
            re.capture_names()
                .map(|v| v.map(|s| s.to_string()))
                .collect::<Vec<Option<String>>>(),
        );

        assert_eq!(c.name("digits").unwrap().text, "123");
    }

    #[test]
    fn test_captures_len() {
        let re = regex::Regex::new(r".*(?<digits>\d{3}).*").unwrap();

        let c = re
            .captures("abc123def")
            .map(|r| r.map(LuaCaptures::from))
            .unwrap()
            .unwrap();

        assert_eq!(c.len(), 2);
    }
}
