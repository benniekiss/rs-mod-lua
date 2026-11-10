use std::{
    fmt,
    sync::{Arc, Mutex},
};

use crate::re;

#[derive(Debug, Default)]
pub(crate) enum LuaBytesMode {
    #[default]
    Unicode,
    Ascii,
    UnicodeBytes,
}

impl fmt::Display for LuaBytesMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Unicode => "unicode",
            Self::Ascii => "ascii",
            Self::UnicodeBytes => "unicode_bytes",
        };

        write!(f, "{s}")
    }
}

impl From<fancy_regex::BytesMode> for LuaBytesMode {
    fn from(value: fancy_regex::BytesMode) -> Self {
        LuaBytesMode::from(&value)
    }
}

impl From<&fancy_regex::BytesMode> for LuaBytesMode {
    fn from(value: &fancy_regex::BytesMode) -> Self {
        match value {
            fancy_regex::BytesMode::Unicode => Self::Unicode,
            fancy_regex::BytesMode::Ascii => Self::Ascii,
            fancy_regex::BytesMode::UnicodeBytes => Self::UnicodeBytes,
        }
    }
}

impl From<LuaBytesMode> for fancy_regex::BytesMode {
    fn from(value: LuaBytesMode) -> Self {
        match value {
            LuaBytesMode::Unicode => Self::Unicode,
            LuaBytesMode::Ascii => Self::Ascii,
            LuaBytesMode::UnicodeBytes => Self::UnicodeBytes,
        }
    }
}

impl mlua::FromLua for LuaBytesMode {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        value
            .to_string()
            .and_then(|v| match v.to_lowercase().as_str() {
                "unicode" => Ok(Self::Unicode),
                "ascii" => Ok(Self::Ascii),
                "unicode_bytes" => Ok(Self::UnicodeBytes),
                _ => Err(mlua::Error::FromLuaConversionError {
                    from: value.type_name(),
                    to: "LuaBytesMode".to_string(),
                    message: Some("Not a valid BytesMode".to_string()),
                }),
            })
    }
}

impl mlua::IntoLua for LuaBytesMode {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        self.to_string().into_lua(lua)
    }
}

pub(crate) fn bytesmode_lua(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let table = lua.create_table()?;

    table.set("UNICODE", LuaBytesMode::Unicode)?;
    table.set("ASCII", LuaBytesMode::Ascii)?;
    table.set("UNICODE_BYTES", LuaBytesMode::UnicodeBytes)?;

    Ok(table)
}

#[derive(mlua::UserData, mlua::FromLua, Clone)]
pub struct LuaRegexOptions(Arc<Mutex<fancy_regex::RegexOptionsBuilder>>);

impl From<fancy_regex::RegexOptionsBuilder> for LuaRegexOptions {
    fn from(value: fancy_regex::RegexOptionsBuilder) -> Self {
        Self(Arc::new(Mutex::new(value)))
    }
}

#[mlua::userdata_impl]
impl LuaRegexOptions {
    #[lua(name = "new", infallible)]
    fn lua_new() -> Self {
        fancy_regex::RegexOptionsBuilder::new().into()
    }

    #[lua(name = "build")]
    pub(crate) fn lua_build(&self, pattern: String) -> mlua::Result<re::LuaRegex> {
        self.0
            .lock()
            .map_err(mlua::Error::runtime)?
            .build(pattern)
            .map_err(mlua::Error::external)
            .map(re::LuaRegex::from)
    }

    #[lua(name = "case_insensitive")]
    fn lua_case_insensitive(&self, yes: bool) -> mlua::Result<Self> {
        self.0
            .lock()
            .map_err(mlua::Error::runtime)?
            .case_insensitive(yes);

        Ok(self.clone())
    }

    #[lua(name = "multi_line")]
    fn lua_multi_line(&self, yes: bool) -> mlua::Result<Self> {
        self.0.lock().map_err(mlua::Error::runtime)?.multi_line(yes);

        Ok(self.clone())
    }

    #[lua(name = "ignore_whitespace")]
    fn lua_ignore_whitespace(&self, yes: bool) -> mlua::Result<Self> {
        self.0
            .lock()
            .map_err(mlua::Error::runtime)?
            .ignore_whitespace(yes);

        Ok(self.clone())
    }

    #[lua(name = "dot_matches_new_line")]
    fn lua_dot_matches_new_line(&self, yes: bool) -> mlua::Result<Self> {
        self.0
            .lock()
            .map_err(mlua::Error::runtime)?
            .dot_matches_new_line(yes);

        Ok(self.clone())
    }

    #[lua(name = "crlf")]
    fn lua_crlf(&self, yes: bool) -> mlua::Result<Self> {
        self.0.lock().map_err(mlua::Error::runtime)?.crlf(yes);

        Ok(self.clone())
    }

    #[lua(name = "verbose_mode")]
    fn lua_verbose_mode(&self, yes: bool) -> mlua::Result<Self> {
        self.0
            .lock()
            .map_err(mlua::Error::runtime)?
            .verbose_mode(yes);

        Ok(self.clone())
    }

    #[lua(name = "unicode_mode")]
    fn lua_unicode_mode(&self, yes: bool) -> mlua::Result<Self> {
        self.0
            .lock()
            .map_err(mlua::Error::runtime)?
            .unicode_mode(yes);

        Ok(self.clone())
    }

    #[lua(name = "backtrack_limit")]
    fn lua_backtrack_limit(&self, limit: usize) -> mlua::Result<Self> {
        self.0
            .lock()
            .map_err(mlua::Error::runtime)?
            .backtrack_limit(limit);

        Ok(self.clone())
    }

    #[lua(name = "delegate_size_limit")]
    fn lua_delegate_size_limit(&self, limit: usize) -> mlua::Result<Self> {
        self.0
            .lock()
            .map_err(mlua::Error::runtime)?
            .delegate_size_limit(limit);

        Ok(self.clone())
    }

    #[lua(name = "delegate_dfa_size_limit")]
    fn lua_delegate_dfa_size_limit(&self, limit: usize) -> mlua::Result<Self> {
        self.0
            .lock()
            .map_err(mlua::Error::runtime)?
            .delegate_dfa_size_limit(limit);

        Ok(self.clone())
    }

    #[lua(name = "delegate_dfa_skip_cache_capacity_check")]
    fn lua_delegate_dfa_skip_cache_capacity_check(&self, yes: bool) -> mlua::Result<Self> {
        self.0
            .lock()
            .map_err(mlua::Error::runtime)?
            .delegate_dfa_skip_cache_capacity_check(yes);

        Ok(self.clone())
    }

    #[lua(name = "max_prog_size")]
    fn lua_max_prog_size(&self, limit: usize) -> mlua::Result<Self> {
        self.0
            .lock()
            .map_err(mlua::Error::runtime)?
            .max_prog_size(limit);

        Ok(self.clone())
    }

    #[lua(name = "find_not_empty")]
    fn lua_find_not_empty(&self, yes: bool) -> mlua::Result<Self> {
        self.0
            .lock()
            .map_err(mlua::Error::runtime)?
            .find_not_empty(yes);

        Ok(self.clone())
    }

    #[lua(name = "optimize")]
    fn lua_optimize(&self, yes: bool) -> mlua::Result<Self> {
        self.0.lock().map_err(mlua::Error::runtime)?.optimize(yes);

        Ok(self.clone())
    }

    #[lua(name = "ignore_numbered_groups_when_named_groups_exist")]
    fn lua_ignore_numbered_groups_when_named_groups_exist(&self, yes: bool) -> mlua::Result<Self> {
        self.0
            .lock()
            .map_err(mlua::Error::runtime)?
            .ignore_numbered_groups_when_named_groups_exist(yes);

        Ok(self.clone())
    }

    #[lua(name = "seek")]
    fn lua_seek(&self, yes: bool) -> mlua::Result<Self> {
        self.0.lock().map_err(mlua::Error::runtime)?.seek(yes);

        Ok(self.clone())
    }

    // Currently disabled since `seek_filter` takes a `fn` pointer,
    // not a capture.
    //
    // #[lua(name = "seek_filter")]
    // fn lua_seek_filter(&self, filter: mlua::Function) -> mlua::Result<Self> {
    //     self.0
    //         .lock()
    //         .map_err(mlua::Error::runtime)?
    //         .seek_filter(|s| filter.call::<bool>(s).unwrap_or(false));
    //
    //     Ok(self.clone())
    // }

    #[lua(name = "oniguruma_mode")]
    fn lua_oniguruma_mode(&self, yes: bool) -> mlua::Result<Self> {
        self.0
            .lock()
            .map_err(mlua::Error::runtime)?
            .oniguruma_mode(yes);

        Ok(self.clone())
    }

    #[lua(name = "bytes_mode")]
    fn lua_bytes_mode(&self, mode: LuaBytesMode) -> mlua::Result<Self> {
        self.0
            .lock()
            .map_err(mlua::Error::runtime)?
            .bytes_mode(mode.into());

        Ok(self.clone())
    }

    #[lua(name = "disallow_empty_match_at_eof_after_newline")]
    fn lua_disallow_empty_match_at_eof_after_newline(&self, yes: bool) -> mlua::Result<Self> {
        self.0
            .lock()
            .map_err(mlua::Error::runtime)?
            .disallow_empty_match_at_eof_after_newline(yes);

        Ok(self.clone())
    }

    // Disabled since it is unused with the current API
    //
    // #[lua(name = "allow_input_assertion_overrides")]
    // fn lua_allow_input_assertion_overrides(&self, yes: bool) -> mlua::Result<Self> {
    //     self.0
    //         .lock()
    //         .map_err(mlua::Error::runtime)?
    //         .allow_input_assertion_overrides(yes);
    //
    //     Ok(self.clone())
    // }

    #[lua(name = "build_delegate_prefilter")]
    fn lua_build_delegate_prefilter(&self, yes: bool) -> mlua::Result<Self> {
        self.0
            .lock()
            .map_err(mlua::Error::runtime)?
            .build_delegate_prefilter(yes);

        Ok(self.clone())
    }

    #[lua(name = "start_bytes")]
    fn lua_start_bytes(&self, pattern: &str) -> mlua::Result<Option<Vec<u8>>> {
        self.0
            .lock()
            .map_err(mlua::Error::runtime)?
            .start_bytes(pattern)
            .map_err(mlua::Error::external)
            .map(|o| o.map(|s| s.iter().collect::<Vec<_>>()))
    }

    #[lua(name = "required_bytes")]
    fn lua_required_bytes(&self, pattern: &str) -> mlua::Result<Vec<u8>> {
        self.0
            .lock()
            .map_err(mlua::Error::runtime)?
            .required_bytes(pattern)
            .map_err(mlua::Error::external)
            .map(|s| s.iter().collect::<Vec<_>>())
    }
}
