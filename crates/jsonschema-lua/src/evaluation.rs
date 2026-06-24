use std::ops::Deref;

use mlua::LuaSerdeExt;

pub(crate) struct LuaAnnotationEntry {
    schema_location: String,
    absolute_keyword_location: Option<jsonschema::Uri<String>>,
    instance_location: jsonschema::paths::Location,
    annotations: jsonschema::output::Annotations,
}

impl From<jsonschema::AnnotationEntry<'_>> for LuaAnnotationEntry {
    fn from(value: jsonschema::AnnotationEntry) -> Self {
        Self {
            schema_location: value.schema_location.to_string(),
            absolute_keyword_location: value.absolute_keyword_location.cloned(),
            instance_location: value.instance_location.clone(),
            annotations: value.annotations.clone(),
        }
    }
}

impl mlua::IntoLua for LuaAnnotationEntry {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        let table = lua.create_table()?;

        table.set("schema_location", self.schema_location)?;
        table.set(
            "absolute_keyword_location",
            self.absolute_keyword_location
                .map(|v| lua.to_value(&v).ok()),
        )?;
        table.set("instance_location", lua.to_value(&self.instance_location)?)?;
        table.set("annotations", lua.to_value(&self.annotations)?)?;

        Ok(mlua::Value::Table(table))
    }
}

pub(crate) struct LuaErrorEntry {
    schema_location: String,
    absolute_keyword_location: Option<jsonschema::Uri<String>>,
    instance_location: jsonschema::paths::Location,
    error: jsonschema::output::ErrorDescription,
}

impl From<jsonschema::ErrorEntry<'_>> for LuaErrorEntry {
    fn from(value: jsonschema::ErrorEntry) -> Self {
        Self {
            schema_location: value.schema_location.to_string(),
            absolute_keyword_location: value.absolute_keyword_location.cloned(),
            instance_location: value.instance_location.clone(),
            error: value.error.clone(),
        }
    }
}

impl mlua::IntoLua for LuaErrorEntry {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        let table = lua.create_table()?;

        table.set("schema_location", self.schema_location)?;
        table.set(
            "absolute_keyword_location",
            self.absolute_keyword_location
                .map(|v| lua.to_value(&v).ok()),
        )?;
        table.set("instance_location", lua.to_value(&self.instance_location)?)?;
        table.set("error", self.error.into_inner())?;

        Ok(mlua::Value::Table(table))
    }
}

#[derive(mlua::UserData)]
pub(crate) struct LuaEvaluation(jsonschema::Evaluation);

impl From<jsonschema::Evaluation> for LuaEvaluation {
    fn from(value: jsonschema::Evaluation) -> Self {
        Self(value)
    }
}

impl From<LuaEvaluation> for jsonschema::Evaluation {
    fn from(value: LuaEvaluation) -> Self {
        value.0
    }
}

impl AsRef<jsonschema::Evaluation> for LuaEvaluation {
    fn as_ref(&self) -> &jsonschema::Evaluation {
        &self.0
    }
}

impl Deref for LuaEvaluation {
    type Target = jsonschema::Evaluation;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[mlua::userdata_impl]
impl LuaEvaluation {
    #[lua(name = "flag")]
    pub(crate) fn lua_flag(&self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        lua.to_value(&self.0.flag())
    }

    #[lua(name = "list")]
    pub(crate) fn lua_list(&self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        lua.to_value(&self.0.list())
    }

    #[lua(name = "hierarchical")]
    pub(crate) fn lua_hierarchical(&self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        lua.to_value(&self.0.hierarchical())
    }

    #[lua(name = "annotations", infallible)]
    pub(crate) fn lua_annotations(&self) -> Vec<LuaAnnotationEntry> {
        self.0
            .iter_annotations()
            .map(LuaAnnotationEntry::from)
            .collect::<Vec<_>>()
    }

    #[lua(name = "errors", infallible)]
    pub(crate) fn lua_errors(&self) -> Vec<LuaErrorEntry> {
        self.0
            .iter_errors()
            .map(LuaErrorEntry::from)
            .collect::<Vec<_>>()
    }
}
