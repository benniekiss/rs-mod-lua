use std::ops::Deref;

#[derive(Clone)]
pub(crate) struct LuaJsonSchemaDraft(jsonschema::Draft);

impl Deref for LuaJsonSchemaDraft {
    type Target = jsonschema::Draft;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<jsonschema::Draft> for LuaJsonSchemaDraft {
    fn from(value: jsonschema::Draft) -> Self {
        LuaJsonSchemaDraft(value)
    }
}

impl From<LuaJsonSchemaDraft> for jsonschema::Draft {
    fn from(value: LuaJsonSchemaDraft) -> Self {
        value.0
    }
}

impl TryFrom<&str> for LuaJsonSchemaDraft {
    type Error = mlua::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "draft201909" => Ok(jsonschema::Draft::Draft201909.into()),
            "draft202012" => Ok(jsonschema::Draft::Draft202012.into()),
            "draft4" => Ok(jsonschema::Draft::Draft4.into()),
            "draft6" => Ok(jsonschema::Draft::Draft6.into()),
            "draft7" => Ok(jsonschema::Draft::Draft7.into()),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: "string",
                to: "JsonSchemaDraft".to_string(),
                message: Some("unknown Json Draft version".to_string()),
            }),
        }
    }
}

impl TryFrom<String> for LuaJsonSchemaDraft {
    type Error = mlua::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

impl mlua::FromLua for LuaJsonSchemaDraft {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        value.to_string()?.try_into()
    }
}

impl mlua::IntoLua for LuaJsonSchemaDraft {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        match self.deref() {
            jsonschema::Draft::Draft201909 => "Draft201909".into_lua(lua),
            jsonschema::Draft::Draft202012 => "Draft202012".into_lua(lua),
            jsonschema::Draft::Draft4 => "Draft4".into_lua(lua),
            jsonschema::Draft::Draft6 => "Draft6".into_lua(lua),
            jsonschema::Draft::Draft7 => "Draft7".into_lua(lua),
            _ => Err(mlua::Error::runtime("unknown Json Draft version")),
        }
    }
}
