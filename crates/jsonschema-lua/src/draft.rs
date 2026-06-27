use rsjson_lua::config::EncodeConfig;
use serde::{Deserialize, Serialize};

use crate::lua::lua_to_json;

#[derive(Debug, Clone, Serialize, Deserialize, mlua::UserData, mlua::FromLua)]
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

#[mlua::userdata_impl]
impl LuaDraft {
    const DRAFT201909: &'static str = "DRAFT201909";
    const DRAFT202012: &'static str = "DRAFT202012";
    const DRAFT4: &'static str = "DRAFT4";
    const DRAFT6: &'static str = "DRAFT6";
    const DRAFT7: &'static str = "DRAFT7";

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
