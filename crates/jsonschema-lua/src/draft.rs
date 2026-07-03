use core::fmt;

use mlua::LuaSerdeExt;
use rsjson_lua::config::EncodeConfig;
use serde::{Deserialize, Serialize};

use crate::lua::lua_to_json;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, mlua::UserData)]
pub(crate) enum LuaDraft {
    #[serde(alias = "DRAFT201909", alias = "draft201909")]
    Draft201909,
    #[serde(alias = "DRAFT202012", alias = "draft202012")]
    Draft202012,
    #[serde(alias = "DRAFT4", alias = "draft4")]
    Draft4,
    #[serde(alias = "DRAFT6", alias = "draft6")]
    Draft6,
    #[serde(alias = "DRAFT7", alias = "draft7")]
    Draft7,
    Unknown,
}

impl fmt::Display for LuaDraft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repl = match self {
            LuaDraft::Draft201909 => "Draft201909",
            LuaDraft::Draft202012 => "Draft202012",
            LuaDraft::Draft4 => "Draft4",
            LuaDraft::Draft6 => "Draft6",
            LuaDraft::Draft7 => "Draft7",
            LuaDraft::Unknown => "Unknown",
        };

        write!(f, "{repl}")
    }
}

impl From<LuaDraft> for jsonschema::Draft {
    fn from(value: LuaDraft) -> Self {
        match value {
            LuaDraft::Draft201909 => jsonschema::Draft::Draft201909,
            LuaDraft::Draft202012 => jsonschema::Draft::Draft202012,
            LuaDraft::Draft4 => jsonschema::Draft::Draft4,
            LuaDraft::Draft6 => jsonschema::Draft::Draft6,
            LuaDraft::Draft7 => jsonschema::Draft::Draft7,
            LuaDraft::Unknown => jsonschema::Draft::Unknown,
        }
    }
}

impl From<&LuaDraft> for jsonschema::Draft {
    fn from(value: &LuaDraft) -> Self {
        value.to_owned().into()
    }
}

impl From<jsonschema::Draft> for LuaDraft {
    fn from(value: jsonschema::Draft) -> Self {
        match value {
            jsonschema::Draft::Draft201909 => LuaDraft::Draft201909,
            jsonschema::Draft::Draft202012 => LuaDraft::Draft202012,
            jsonschema::Draft::Draft4 => LuaDraft::Draft4,
            jsonschema::Draft::Draft6 => LuaDraft::Draft6,
            jsonschema::Draft::Draft7 => LuaDraft::Draft7,
            _ => LuaDraft::Unknown,
        }
    }
}

impl mlua::FromLua for LuaDraft {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::UserData(ud) if ud.is::<LuaDraft>() => {
                ud.borrow::<LuaDraft>().map(|v| v.clone())
            },
            _ => lua.from_value(value),
        }
    }
}

#[mlua::userdata_impl]
impl LuaDraft {
    #[lua(meta, name = "__tostring", infallible)]
    pub(crate) fn lua_tostring(&self) -> String {
        self.to_string()
    }

    #[lua(meta, name = "__eq", infallible)]
    pub(crate) fn lua_eq(&self, other: LuaDraft) -> bool {
        self == &other
    }

    #[lua(name = "from_schema_uri", infallible)]
    pub(crate) fn lua_from_schema_uri(uri: &str) -> Self {
        jsonschema::Draft::from_schema_uri(uri).into()
    }

    #[lua(name = "detect")]
    pub(crate) fn lua_detect(
        &self,
        lua: &mlua::Lua,
        schema: mlua::Value,
        options: Option<EncodeConfig>,
    ) -> mlua::Result<LuaDraft> {
        lua_to_json(lua, schema, options)
            .map(|v| jsonschema::Draft::detect(self.clone().into(), &v).into())
    }

    #[lua(name = "is_known_keyword", infallible)]
    pub(crate) fn lua_is_known_keyword(&self, keyword: &str) -> bool {
        jsonschema::Draft::is_known_keyword(&self.into(), keyword)
    }
}

pub(crate) fn draft_lua(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let table = lua.create_table()?;

    table.set("DRAFT201909", lua.create_userdata(LuaDraft::Draft201909)?)?;
    table.set("DRAFT202012", lua.create_userdata(LuaDraft::Draft202012)?)?;
    table.set("DRAFT4", lua.create_userdata(LuaDraft::Draft4)?)?;
    table.set("DRAFT6", lua.create_userdata(LuaDraft::Draft6)?)?;
    table.set("DRAFT7", lua.create_userdata(LuaDraft::Draft7)?)?;

    Ok(table)
}

#[cfg(test)]
mod tests {
    use std::assert_matches;

    use mlua::LuaSerdeExt;

    use super::*;

    #[test]
    fn test_lua_draft() {
        let lua = mlua::Lua::new();

        let draft_string = lua
            .to_value(&LuaDraft::Draft201909)
            .unwrap()
            .as_string()
            .cloned()
            .unwrap();

        assert!(draft_string == "Draft201909");

        let draft = lua.from_value(mlua::Value::String(draft_string)).unwrap();

        assert_matches!(draft, LuaDraft::Draft201909);
    }
}
