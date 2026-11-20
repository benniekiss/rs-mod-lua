use mlua::LuaSerdeExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all(serialize = "lowercase"))]
pub(crate) enum LuaVersion {
    #[serde(alias = "HTTP09", alias = "http09", alias = "HTTP/0.9")]
    Http09,
    #[serde(alias = "HTTP10", alias = "http10", alias = "HTTP/1.0")]
    Http10,
    #[serde(alias = "HTTP11", alias = "http11", alias = "HTTP/1.1")]
    Http11,
    #[serde(alias = "HTTP20", alias = "http20", alias = "HTTP/2.0")]
    Http20,
    #[serde(alias = "HTTP30", alias = "http30", alias = "HTTP/3.0")]
    Http30,
}

impl From<reqwest::Version> for LuaVersion {
    fn from(value: reqwest::Version) -> Self {
        match value {
            reqwest::Version::HTTP_09 => Self::Http09,
            reqwest::Version::HTTP_10 => Self::Http10,
            reqwest::Version::HTTP_11 => Self::Http11,
            reqwest::Version::HTTP_2 => Self::Http20,
            reqwest::Version::HTTP_3 => Self::Http30,
            _ => unreachable!("HTTP version not implemented"),
        }
    }
}

impl From<LuaVersion> for reqwest::Version {
    fn from(value: LuaVersion) -> Self {
        match value {
            LuaVersion::Http09 => Self::HTTP_09,
            LuaVersion::Http10 => Self::HTTP_10,
            LuaVersion::Http11 => Self::HTTP_11,
            LuaVersion::Http20 => Self::HTTP_2,
            LuaVersion::Http30 => Self::HTTP_3,
        }
    }
}

impl mlua::FromLua for LuaVersion {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        lua.from_value(value)
    }
}

impl mlua::IntoLua for LuaVersion {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        lua.to_value(&self)
    }
}
