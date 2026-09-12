use std::cell::RefCell;

use mlua::{IntoLua, LuaSerdeExt};
use rs_mod__mod__mod__mod__mod__mod__mod__mod__mod_lua_core::config::{DecodeConfig, EncodeConfig};

pub(crate) fn lua_to_json(
    lua: &mlua::Lua,
    value: mlua::Value,
    options: Option<EncodeConfig>,
) -> mlua::Result<serde_json::Value> {
    match value.as_string() {
        Some(s) => serde_json::from_str(&s.to_string_lossy()).map_err(mlua::Error::external),
        None => lua.from_value_with(value, *options.unwrap_or_default()),
    }
}

pub(crate) fn json_to_lua(
    lua: &mlua::Lua,
    value: serde_json::Value,
    options: Option<DecodeConfig>,
    as_str: bool,
) -> mlua::Result<mlua::Value> {
    match as_str {
        true => serde_json::to_string(&value)
            .map_err(mlua::Error::external)
            .and_then(|s| s.into_lua(lua)),
        false => lua.to_value_with(&value, *options.unwrap_or_default()),
    }
}
