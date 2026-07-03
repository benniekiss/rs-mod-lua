use core::fmt;

use crate::{
    lua::{lua_to_json, with_lua},
    uri::LuaUri,
};

#[derive(Debug)]
pub(crate) struct LuaRetrieveError(String);

impl fmt::Display for LuaRetrieveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for LuaRetrieveError {}

impl From<mlua::Error> for LuaRetrieveError {
    fn from(e: mlua::Error) -> Self {
        LuaRetrieveError(e.to_string())
    }
}

pub(crate) struct LuaRetriever {
    key: mlua::RegistryKey,
    options: Option<rsjson_lua::config::EncodeConfig>,
}

impl LuaRetriever {
    pub(crate) fn new(
        key: mlua::RegistryKey,
        options: Option<rsjson_lua::config::EncodeConfig>,
    ) -> Self {
        Self { key, options }
    }
}

impl jsonschema::Retrieve for LuaRetriever {
    fn retrieve(
        &self,
        uri: &jsonschema::Uri<String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
        with_lua(|lua| {
            let func = lua.registry_value::<mlua::Function>(&self.key)?;
            let uri = lua.create_userdata::<LuaUri>(uri.into())?;

            let value = func.call(uri)?;

            lua_to_json(lua, value, self.options.clone())
        })
        .map_err(|err| {
            Box::new(LuaRetrieveError::from(err)) as Box<dyn std::error::Error + Send + Sync>
        })
    }
}
