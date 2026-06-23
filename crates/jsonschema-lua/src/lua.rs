use std::ops::Deref;

use mlua::LuaSerdeExt;
use rsjson_lua::config::EncodeConfig;

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

#[derive(mlua::UserData)]
pub(crate) struct LuaValidator(jsonschema::Validator);

impl From<jsonschema::Validator> for LuaValidator {
    fn from(value: jsonschema::Validator) -> Self {
        Self(value)
    }
}

impl From<jsonschema::meta::MetaValidator<'_>> for LuaValidator {
    fn from(value: jsonschema::meta::MetaValidator) -> Self {
        Self(value.clone())
    }
}

impl From<LuaValidator> for jsonschema::Validator {
    fn from(value: LuaValidator) -> Self {
        value.0
    }
}

impl AsRef<jsonschema::Validator> for LuaValidator {
    fn as_ref(&self) -> &jsonschema::Validator {
        &self.0
    }
}

impl Deref for LuaValidator {
    type Target = jsonschema::Validator;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[mlua::userdata_impl]
impl LuaValidator {
    #[lua(name = "is_valid")]
    pub(crate) fn lua_is_valid(
        &self,
        lua: &mlua::Lua,
        json: mlua::Value,
        options: Option<EncodeConfig>,
    ) -> mlua::Result<bool> {
        lua.from_value_with(json, *options.unwrap_or_default())
            .map(|val| self.0.is_valid(&val))
    }

    #[lua(name = "validate")]
    pub(crate) fn lua_validate(
        &self,
        lua: &mlua::Lua,
        json: mlua::Value,
        options: Option<EncodeConfig>,
    ) -> mlua::Result<()> {
        lua.from_value_with(json, *options.unwrap_or_default())
            .and_then(|val| {
                self.0
                    .validate(&val)
                    .map_err(|err| mlua::Error::external(err.to_owned()))
            })
    }

    #[lua(name = "evaluate")]
    pub(crate) fn lua_evaluate(
        &self,
        lua: &mlua::Lua,
        json: mlua::Value,
        options: Option<EncodeConfig>,
    ) -> mlua::Result<LuaEvaluation> {
        lua.from_value_with(json, *options.unwrap_or_default())
            .map(|val| self.0.evaluate(&val).into())
    }

    #[lua(name = "errors")]
    pub(crate) fn lua_errors(
        &self,
        lua: &mlua::Lua,
        json: mlua::Value,
        options: Option<EncodeConfig>,
    ) -> mlua::Result<Vec<mlua::Error>> {
        lua.from_value_with(json, *options.unwrap_or_default())
            .map(|val| {
                self.0
                    .iter_errors(&val)
                    .into_errors()
                    .into_iter()
                    .map(|err| mlua::Error::external(err.to_owned()))
                    .collect::<Vec<_>>()
            })
    }

    #[lua(name = "draft", infallible)]
    pub(crate) fn lua_draft(&self) -> LuaJsonSchemaDraft {
        self.0.draft().into()
    }
}

#[derive(mlua::UserData)]
pub(crate) struct LuaValidatorMap(jsonschema::ValidatorMap);

impl From<jsonschema::ValidatorMap> for LuaValidatorMap {
    fn from(value: jsonschema::ValidatorMap) -> Self {
        Self(value)
    }
}

impl From<LuaValidatorMap> for jsonschema::ValidatorMap {
    fn from(value: LuaValidatorMap) -> Self {
        value.0
    }
}

impl AsRef<jsonschema::ValidatorMap> for LuaValidatorMap {
    fn as_ref(&self) -> &jsonschema::ValidatorMap {
        &self.0
    }
}

impl Deref for LuaValidatorMap {
    type Target = jsonschema::ValidatorMap;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[mlua::userdata_impl]
impl LuaValidatorMap {
    #[lua(name = "get", infallible)]
    pub(crate) fn lua_get(&self, pointer: &str) -> Option<LuaValidator> {
        self.0.get(pointer).cloned().map(|v| v.into())
    }

    #[lua(name = "contains_key", infallible)]
    pub(crate) fn lua_contains_key(&self, pointer: &str) -> bool {
        self.0.contains_key(pointer)
    }

    #[lua(name = "keys", infallible)]
    pub(crate) fn lua_keys(&self) -> Vec<String> {
        self.0.keys().map(|s| s.to_string()).collect()
    }
}
