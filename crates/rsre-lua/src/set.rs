use std::sync::Arc;

use mlua::IntoLua;

use crate::{
    options::LuaRegexOptions,
    re::{LuaCaptures, LuaMatch},
};

#[derive(Clone)]
pub(crate) struct LuaSetMatch {
    pattern: usize,
    captures: LuaCaptures,
    m: LuaMatch,
}

impl From<fancy_regex::RegexSetMatch<'_, [u8]>> for LuaSetMatch {
    fn from(value: fancy_regex::RegexSetMatch<'_, [u8]>) -> Self {
        Self {
            pattern: value.pattern().saturating_add(1),
            captures: value.captures().into(),
            m: value.get().into(),
        }
    }
}

impl mlua::IntoLua for LuaSetMatch {
    fn into_lua(
        self,
        lua: &mlua::prelude::Lua,
    ) -> mlua::prelude::LuaResult<mlua::prelude::LuaValue> {
        lua.create_table_with_capacity(0, 3).and_then(|t| {
            t.raw_set("pattern", self.pattern)?;
            t.raw_set("captures", self.captures)?;
            t.raw_set("match", self.m)?;

            Ok(mlua::Value::Table(t))
        })
    }
}

#[derive(mlua::UserData)]
pub(crate) struct LuaRegexSet {
    #[lua(skip)]
    inner: fancy_regex::RegexSet,
    #[lua(skip)]
    re: Vec<Arc<fancy_regex::Regex>>,
}

#[mlua::userdata_impl]
impl LuaRegexSet {
    #[lua(name = "new")]
    fn lua_new(patterns: Vec<String>) -> mlua::Result<Self> {
        fancy_regex::RegexSet::new(patterns)
            .map_err(mlua::Error::external)
            .map(Self)
    }

    #[lua(name = "new_with_options")]
    fn lua_new_with_options(patterns: Vec<String>, options: LuaRegexOptions) -> mlua::Result<Self> {
        let inner = options.lock()?;

        inner.build_delegate_prefilter(false);

        patterns
            .into_iter()
            .map(|p| inner.build(p).map(Arc::new).map_err(mlua::Error::external))
            .collect::<mlua::Result<Vec<_>>>()
            .map(|re| {
                let opts = fancy_regex::RegexSetOptions::new()
                    .delegate_size_limit(inner.delegate_size_limit())
                    .delegate_dfa_size_limit(inner.delegate_dfa_size_limit());

                Self {
                    inner: fancy_regex::RegexSet::new_with_options(re, opts),
                    re,
                }
            })
    }

    #[lua(name = "len", infallible)]
    fn lua_len(&self) -> usize {
        self.0.len()
    }

    #[lua(name = "is_empty", infallible)]
    fn lua_is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[lua(name = "find")]
    fn lua_find(
        &self,
        lua: &mlua::Lua,
        hay: &[u8],
        start: Option<usize>,
    ) -> mlua::Result<mlua::MultiValue> {
        let start = start.unwrap_or(1).saturating_sub(1);
        let m = self
            .0
            .find_input(fancy_regex::RegexInput::new(hay).from_pos(start))
            .map_err(mlua::Error::external)?;

        let mut mv = Vec::with_capacity(2);

        if let Some(m) = m {
            let s = m.start();
            let v = m
                .into_iter()
                .filter_map(|r| r.map(LuaSetMatch::from).ok())
                .collect::<Vec<_>>();

            mv.push(s.into_lua(lua)?);
            mv.push(v.into_lua(lua)?);
        } else {
            mv.push(mlua::Value::Nil);
            mv.push(mlua::Value::Table(lua.create_table()?));
        };

        Ok(mlua::MultiValue::from_vec(mv))
    }
}
