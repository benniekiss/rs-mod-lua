use std::ops::Deref;

use mlua::LuaSerdeExt;
use serde::{Deserialize, Serialize};

#[derive(mlua::UserData, Serialize, Deserialize)]
#[serde(transparent)]
pub(crate) struct LuaUri(jsonschema::Uri<String>);

impl From<jsonschema::Uri<String>> for LuaUri {
    fn from(value: jsonschema::Uri<String>) -> Self {
        Self(value)
    }
}

impl From<&jsonschema::Uri<String>> for LuaUri {
    fn from(value: &jsonschema::Uri<String>) -> Self {
        Self(value.clone())
    }
}

impl From<jsonschema::Uri<&str>> for LuaUri {
    fn from(value: jsonschema::Uri<&str>) -> Self {
        Self(value.to_owned())
    }
}

impl From<LuaUri> for jsonschema::Uri<String> {
    fn from(value: LuaUri) -> Self {
        value.0
    }
}

impl AsRef<jsonschema::Uri<String>> for LuaUri {
    fn as_ref(&self) -> &jsonschema::Uri<String> {
        &self.0
    }
}

impl Deref for LuaUri {
    type Target = jsonschema::Uri<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl mlua::FromLua for LuaUri {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        lua.from_value(value)
    }
}

#[mlua::userdata_impl]
impl LuaUri {
    #[lua(name = "__tostring", meta, infallible)]
    pub(crate) fn lua_tostring(&self) -> String {
        self.0.as_str().to_string()
    }

    #[lua(name = "parse")]
    pub(crate) fn lua_parse(uri: &str) -> mlua::Result<Self> {
        jsonschema::Uri::parse(uri)
            .map(|s| s.into())
            .map_err(mlua::Error::external)
    }

    #[lua(name = "schema", getter, infallible)]
    pub(crate) fn lua_scheme(&self) -> String {
        self.0.scheme().to_string()
    }

    #[lua(name = "authority", getter, infallible)]
    pub(crate) fn lua_authority(&self) -> Option<String> {
        self.0.authority().map(|a| a.to_string())
    }

    #[lua(name = "path", getter, infallible)]
    pub(crate) fn lua_path(&self) -> String {
        self.0.path().to_string()
    }

    #[lua(name = "query", getter, infallible)]
    pub(crate) fn lua_query(&self) -> Option<String> {
        self.0.query().map(|q| q.to_string())
    }

    #[lua(name = "fragment", getter, infallible)]
    pub(crate) fn lua_fragment(&self) -> Option<String> {
        self.0.fragment().map(|q| q.to_string())
    }

    #[lua(name = "normalize", infallible)]
    pub(crate) fn lua_normalize(&self) -> LuaUri {
        self.0.normalize().into()
    }

    #[lua(name = "has_authority", infallible)]
    pub(crate) fn lua_has_authority(&self) -> bool {
        self.0.has_authority()
    }

    #[lua(name = "has_query", infallible)]
    pub(crate) fn lua_has_query(&self) -> bool {
        self.0.has_query()
    }

    #[lua(name = "has_fragment", infallible)]
    pub(crate) fn lua_has_fragment(&self) -> bool {
        self.0.has_fragment()
    }

    #[lua(name = "strip_fragment", infallible)]
    pub(crate) fn lua_strip_fragment(&self) -> LuaUri {
        self.0.strip_fragment().into()
    }

    #[lua(name = "set_fragment", infallible)]
    pub(crate) fn lua_with_fragment(&mut self, fragment: Option<String>) {
        let opt =
            fragment.and_then(|f| jsonschema::uri::EncodedString::new(&f).map(|s| s.to_owned()));
        self.0.set_fragment(opt.as_deref());
    }
}
