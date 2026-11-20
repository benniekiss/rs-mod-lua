use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

pub(crate) struct LuaHeaders(HeaderMap);

impl From<HeaderMap> for LuaHeaders {
    fn from(value: HeaderMap) -> Self {
        Self(value)
    }
}

impl From<LuaHeaders> for HeaderMap {
    fn from(value: LuaHeaders) -> Self {
        value.0
    }
}

impl mlua::FromLua for LuaHeaders {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        mlua::Table::from_lua(value, lua).and_then(|t| {
            let mut headers = HeaderMap::with_capacity(t.raw_len());

            t.pairs::<mlua::LuaString, mlua::LuaString>().try_for_each(
                |p| -> mlua::Result<()> {
                    let (k, v) = p?;
                    headers
                        .try_append(
                            HeaderName::from_bytes(&k.as_bytes()).map_err(mlua::Error::external)?,
                            HeaderValue::from_bytes(&v.as_bytes())
                                .map_err(mlua::Error::external)?,
                        )
                        .map_err(mlua::Error::external)?;

                    Ok(())
                },
            )?;

            Ok(headers.into())
        })
    }
}
