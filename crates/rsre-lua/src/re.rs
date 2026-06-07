// SPDX-License-Identifier: MIT

use std::collections::BTreeMap;

use fancy_regex as regex;

#[derive(Clone)]
pub(crate) struct LuaMatch {
    start: usize,
    end: usize,
    text: String,
}

impl From<regex::Match<'_>> for LuaMatch {
    fn from(m: regex::Match<'_>) -> Self {
        Self {
            start: m.start() + 1, // since lua is 1-indexed
            end: m.end() + 1,     // since lua is 1-indexed
            text: m.as_str().to_string(),
        }
    }
}

impl mlua::UserData for LuaMatch {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method(
            "start",
            |_: &mlua::Lua, this: &LuaMatch, _: ()| -> mlua::Result<usize> { Ok(this.start) },
        );

        methods.add_method(
            "stop",
            |_: &mlua::Lua, this: &LuaMatch, _: ()| -> mlua::Result<usize> { Ok(this.end) },
        );

        methods.add_method(
            "range",
            |_: &mlua::Lua, this: &LuaMatch, _: ()| -> mlua::Result<(usize, usize)> {
                Ok((this.start, this.end))
            },
        );

        methods.add_method(
            "as_str",
            |_: &mlua::Lua, this: &LuaMatch, _: ()| -> mlua::Result<String> {
                Ok(this.text.clone())
            },
        );

        methods.add_meta_method("__tostring", |_, this, _: ()| -> mlua::Result<String> {
            Ok(this.text.clone())
        });
    }
}

#[derive(Clone)]
pub(crate) struct LuaCaptures {
    matches: Vec<Option<LuaMatch>>,
    names: BTreeMap<String, usize>,
}

impl LuaCaptures {
    pub(crate) fn set_names(&mut self, names: Vec<Option<String>>) {
        for (i, name) in names.into_iter().enumerate() {
            if let Some(n) = name {
                self.names.insert(n, i);
            }
        }
    }

    pub(crate) fn get(&self, index: usize) -> Option<LuaMatch> {
        self.matches.get(index)?.clone()
    }

    pub(crate) fn name(&self, name: &str) -> Option<LuaMatch> {
        if let Some(index) = self.names.get(name) {
            self.get(*index)
        } else {
            None
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.matches.len()
    }
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

impl mlua::UserData for LuaCaptures {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method(
            "get",
            |_, this, index: usize| -> mlua::Result<Option<LuaMatch>> {
                Ok(this.get(index - 1)) // since lua is 1-indexed
            },
        );
        methods.add_method(
            "name",
            |_, this, name: String| -> mlua::Result<Option<LuaMatch>> { Ok(this.name(&name)) },
        );
        methods.add_method("len", |_, this, _: ()| -> mlua::Result<usize> {
            Ok(this.len())
        });
    }
}

pub(crate) struct LuaRegex {
    re: regex::Regex,
}

impl LuaRegex {
    pub(crate) fn new(patt: &str) -> Result<Self, regex::Error> {
        regex::Regex::new(patt).map(|re| Self { re })
    }
}

impl mlua::UserData for LuaRegex {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function(
            "new",
            |_: &mlua::Lua, patt: String| -> mlua::Result<LuaRegex> {
                LuaRegex::new(&patt).map_err(mlua::Error::external)
            },
        );

        methods.add_method("match", |_, this, hay: String| -> mlua::Result<bool> {
            this.re.is_match(&hay).map_err(mlua::Error::external)
        });

        methods.add_method(
            "find",
            |_, this, (hay, start): (String, Option<usize>)| -> mlua::Result<Option<LuaMatch>> {
                let start = start.unwrap_or(1);
                this.re
                    .find_from_pos(&hay, start - 1)
                    .map(|r| r.map(LuaMatch::from))
                    .map_err(mlua::Error::external)
            },
        );

        methods.add_method(
            "captures",
            |_, this, (hay, start): (String, Option<usize>)| -> mlua::Result<Option<LuaCaptures>> {
                let start = start.unwrap_or(1);
                let mut captures = this
                    .re
                    .captures_from_pos(&hay, start - 1)
                    .map(|r| r.map(LuaCaptures::from))
                    .map_err(mlua::Error::external)?;

                if let Some(ref mut c) = captures {
                    c.set_names(
                        this.re
                            .capture_names()
                            .map(|v| v.map(|s| s.to_string()))
                            .collect::<Vec<Option<String>>>(),
                    );
                }

                Ok(captures)
            },
        );

        methods.add_method(
            "replace",
            |_,
             this,
             (text, rep, limit): (String, String, Option<usize>)|
             -> mlua::Result<String> {
                let limit = limit.unwrap_or(0);
                this.re
                    .try_replacen(&text, limit, rep)
                    .map(|s| s.to_string())
                    .map_err(mlua::Error::external)
            },
        );

        methods.add_method(
            "split",
            |_, this, (target, limit): (String, Option<usize>)| -> mlua::Result<Vec<String>> {
                match limit {
                    Some(l) => this
                        .re
                        .splitn(&target, l)
                        .map(|r| r.map(|s| s.to_string()))
                        .collect::<Result<Vec<String>, _>>()
                        .map_err(mlua::Error::external),
                    None => this
                        .re
                        .split(&target)
                        .map(|r| r.map(|s| s.to_string()))
                        .collect::<Result<Vec<String>, _>>()
                        .map_err(mlua::Error::external),
                }
            },
        );

        methods.add_method("as_str", |_, this, _: ()| -> mlua::Result<String> {
            Ok(this.re.as_str().to_string())
        });

        methods.add_meta_method("__tostring", |_, this, _: ()| -> mlua::Result<String> {
            Ok(this.re.as_str().to_string())
        });
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
