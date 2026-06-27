use std::ops::Deref;

use rsjson_lua::config::EncodeConfig;

use crate::{
    draft::LuaDraft,
    evaluation::LuaEvaluation,
    lua::{bind_lua, lua_to_json},
};

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
        lua_to_json(lua, json, options).map(|val| bind_lua(lua, || self.0.is_valid(&val)))
    }

    #[lua(name = "validate")]
    pub(crate) fn lua_validate(
        &self,
        lua: &mlua::Lua,
        json: mlua::Value,
        options: Option<EncodeConfig>,
    ) -> mlua::Result<(bool, Option<String>)> {
        lua_to_json(lua, json, options).map(|val| {
            bind_lua(lua, || match self.0.validate(&val) {
                Ok(_) => (true, None),
                Err(err) => (false, Some(err.to_string())),
            })
        })
    }

    #[lua(name = "evaluate")]
    pub(crate) fn lua_evaluate(
        &self,
        lua: &mlua::Lua,
        json: mlua::Value,
        options: Option<EncodeConfig>,
    ) -> mlua::Result<LuaEvaluation> {
        lua_to_json(lua, json, options).map(|val| bind_lua(lua, || self.0.evaluate(&val).into()))
    }

    #[lua(name = "errors")]
    pub(crate) fn lua_errors(
        &self,
        lua: &mlua::Lua,
        json: mlua::Value,
        options: Option<EncodeConfig>,
    ) -> mlua::Result<Vec<String>> {
        lua_to_json(lua, json, options).map(|val| {
            bind_lua(lua, || {
                self.0
                    .iter_errors(&val)
                    .into_errors()
                    .into_iter()
                    .map(|err| err.to_string())
                    .collect::<Vec<_>>()
            })
        })
    }

    #[lua(name = "draft", infallible)]
    pub(crate) fn lua_draft(&self) -> LuaDraft {
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
