use std::ops::Deref;

use mlua::LuaSerdeExt;
use rsjson_lua::config::{DecodeConfig, EncodeConfig};

use crate::{
    draft::LuaDraft,
    lua::{bind_lua, lua_to_json, with_lua},
    retriever::LuaRetriever,
    validator::{LuaValidator, LuaValidatorMap},
};

pub(crate) struct LuaKeyword {
    is_valid: mlua::RegistryKey,
    validate: mlua::RegistryKey,
}

impl LuaKeyword {
    pub(crate) fn new(
        lua: &mlua::Lua,
        is_valid: mlua::Function,
        validate: mlua::Function,
    ) -> mlua::Result<Self> {
        let is_valid = lua.create_registry_value(is_valid)?;
        let validate = lua.create_registry_value(validate)?;

        Ok(Self { is_valid, validate })
    }
}

impl jsonschema::Keyword for LuaKeyword {
    fn is_valid(&self, instance: &serde_json::Value) -> bool {
        with_lua(|lua| {
            let func = lua.registry_value::<mlua::Function>(&self.is_valid)?;
            let val = lua.to_value(instance)?;
            func.call::<bool>(val)
        })
        .unwrap_or(false)
    }

    fn validate<'i>(
        &self,
        instance: &'i serde_json::Value,
    ) -> Result<(), jsonschema::ValidationError<'i>> {
        with_lua(|lua| {
            let func = lua.registry_value::<mlua::Function>(&self.validate)?;
            let val = lua.to_value(instance)?;
            func.call::<()>(val)
        })
        .map_err(|err| jsonschema::ValidationError::custom(err.to_string()))
    }
}

#[derive(mlua::UserData, mlua::FromLua, Clone)]
pub(crate) struct LuaEmailOptions(jsonschema::EmailOptions);

impl From<jsonschema::EmailOptions> for LuaEmailOptions {
    fn from(value: jsonschema::EmailOptions) -> Self {
        Self(value)
    }
}

impl From<LuaEmailOptions> for jsonschema::EmailOptions {
    fn from(value: LuaEmailOptions) -> Self {
        value.0
    }
}

impl AsRef<jsonschema::EmailOptions> for LuaEmailOptions {
    fn as_ref(&self) -> &jsonschema::EmailOptions {
        &self.0
    }
}

impl Deref for LuaEmailOptions {
    type Target = jsonschema::EmailOptions;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[mlua::userdata_impl]
impl LuaEmailOptions {
    #[lua(name = "new", infallible)]
    pub(crate) fn lua_new() -> Self {
        jsonschema::EmailOptions::default().into()
    }

    #[lua(name = "with_minimum_sub_domains", infallible)]
    pub(crate) fn lua_with_minimum_sub_domains(mut self, min: usize) -> Self {
        self.0 = self.0.with_minimum_sub_domains(min);
        self
    }

    #[lua(name = "with_no_minimum_sub_domains", infallible)]
    pub(crate) fn lua_with_no_minimum_sub_domains(mut self) -> Self {
        self.0 = self.0.with_no_minimum_sub_domains();
        self
    }

    #[lua(name = "with_required_tld", infallible)]
    pub(crate) fn lua_with_required_tld(mut self) -> Self {
        self.0 = self.0.with_required_tld();
        self
    }

    #[lua(name = "with_domain_literal", infallible)]
    pub(crate) fn lua_with_domain_literal(mut self) -> Self {
        self.0 = self.0.with_domain_literal();
        self
    }

    #[lua(name = "without_domain_literal", infallible)]
    pub(crate) fn lua_without_domain_literal(mut self) -> Self {
        self.0 = self.0.without_domain_literal();
        self
    }

    #[lua(name = "with_display_text", infallible)]
    pub(crate) fn lua_with_display_text(mut self) -> Self {
        self.0 = self.0.with_display_text();
        self
    }

    #[lua(name = "without_display_text", infallible)]
    pub(crate) fn lua_without_display_text(mut self) -> Self {
        self.0 = self.0.without_display_text();
        self
    }
}

#[derive(mlua::UserData)]
pub(crate) struct LuaFancyRegexPatternOptions(jsonschema::PatternOptions<jsonschema::FancyRegex>);

impl From<jsonschema::PatternOptions<jsonschema::FancyRegex>> for LuaFancyRegexPatternOptions {
    fn from(value: jsonschema::PatternOptions<jsonschema::FancyRegex>) -> Self {
        Self(value)
    }
}

impl From<LuaFancyRegexPatternOptions> for jsonschema::PatternOptions<jsonschema::FancyRegex> {
    fn from(value: LuaFancyRegexPatternOptions) -> Self {
        value.0
    }
}

impl AsRef<jsonschema::PatternOptions<jsonschema::FancyRegex>> for LuaFancyRegexPatternOptions {
    fn as_ref(&self) -> &jsonschema::PatternOptions<jsonschema::FancyRegex> {
        &self.0
    }
}

impl Deref for LuaFancyRegexPatternOptions {
    type Target = jsonschema::PatternOptions<jsonschema::FancyRegex>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl mlua::FromLua for LuaFancyRegexPatternOptions {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        match value.as_userdata() {
            Some(ud) => ud.take::<LuaFancyRegexPatternOptions>(),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "PatternOptions".to_string(),
                message: None,
            }),
        }
    }
}

#[mlua::userdata_impl]
impl LuaFancyRegexPatternOptions {
    #[lua(name = "new", infallible)]
    pub(crate) fn lua_new() -> Self {
        jsonschema::PatternOptions::fancy_regex().into()
    }

    #[lua(name = "backtrack_limit", infallible)]
    pub(crate) fn lua_backtrack_limit(mut self, limit: usize) -> Self {
        self.0 = self.0.backtrack_limit(limit);
        self
    }

    #[lua(name = "size_limit", infallible)]
    pub(crate) fn lua_size_limit(mut self, limit: usize) -> Self {
        self.0 = self.0.size_limit(limit);
        self
    }

    #[lua(name = "dfa_size_limit", infallible)]
    pub(crate) fn lua_dfa_size_limit(mut self, limit: usize) -> Self {
        self.0 = self.0.dfa_size_limit(limit);
        self
    }
}

#[derive(mlua::UserData, mlua::FromLua, Clone)]
pub(crate) struct LuaValidationOptions(jsonschema::ValidationOptions<'static>);

impl From<jsonschema::ValidationOptions<'static>> for LuaValidationOptions {
    fn from(value: jsonschema::ValidationOptions<'static>) -> Self {
        Self(value)
    }
}

impl From<LuaValidationOptions> for jsonschema::ValidationOptions<'static> {
    fn from(value: LuaValidationOptions) -> Self {
        value.0
    }
}

impl AsRef<jsonschema::ValidationOptions<'static>> for LuaValidationOptions {
    fn as_ref(&self) -> &jsonschema::ValidationOptions<'static> {
        &self.0
    }
}

impl Deref for LuaValidationOptions {
    type Target = jsonschema::ValidationOptions<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[mlua::userdata_impl]
impl LuaValidationOptions {
    #[lua(name = "build")]
    pub(crate) fn lua_build(
        &self,
        lua: &mlua::Lua,
        schema: mlua::Value,
        options: Option<EncodeConfig>,
    ) -> mlua::Result<LuaValidator> {
        lua_to_json(lua, schema, options)
            .and_then(|s| self.0.build(&s).map_err(mlua::Error::external))
            .map(|v| v.into())
    }

    #[lua(name = "build_map")]
    pub(crate) fn lua_build_map(
        &self,
        lua: &mlua::Lua,
        schema: mlua::Value,
        options: Option<EncodeConfig>,
    ) -> mlua::Result<LuaValidatorMap> {
        lua_to_json(lua, schema, options)
            .and_then(|s| self.0.build_map(&s).map_err(mlua::Error::external))
            .map(|v| v.into())
    }

    #[lua(name = "bundle")]
    pub(crate) fn lua_bundle(
        &self,
        lua: &mlua::Lua,
        schema: mlua::Value,
        encode: Option<EncodeConfig>,
        decode: Option<DecodeConfig>,
    ) -> mlua::Result<mlua::Value> {
        lua_to_json(lua, schema, encode)
            .and_then(|s| bind_lua(lua, || self.0.bundle(&s).map_err(mlua::Error::external)))
            .and_then(|bundle| lua.to_value_with(&bundle, *decode.unwrap_or_default()))
    }

    #[lua(name = "dereference")]
    pub(crate) fn lua_dereference(
        &self,
        lua: &mlua::Lua,
        schema: mlua::Value,
        encode: Option<EncodeConfig>,
        decode: Option<DecodeConfig>,
    ) -> mlua::Result<mlua::Value> {
        lua_to_json(lua, schema, encode)
            .and_then(|s| {
                bind_lua(lua, || {
                    self.0.dereference(&s).map_err(mlua::Error::external)
                })
            })
            .and_then(|bundle| lua.to_value_with(&bundle, *decode.unwrap_or_default()))
    }

    #[lua(name = "with_draft", infallible)]
    pub(crate) fn lua_with_draft(
        mut self,
        lua: &mlua::Lua,
        draft: mlua::String,
    ) -> mlua::Result<Self> {
        let draft: LuaDraft = lua.from_value(mlua::Value::String(draft))?;
        self.0 = self.0.with_draft(draft.into());
        Ok(self)
    }

    #[lua(name = "with_base_uri", infallible)]
    pub(crate) fn lua_with_base_uri(mut self, base_uri: String) -> Self {
        self.0 = self.0.with_base_uri(base_uri);
        self
    }

    #[lua(name = "with_email_options", infallible)]
    pub(crate) fn lua_with_email_options(mut self, options: LuaEmailOptions) -> Self {
        self.0 = self.0.with_email_options(options.into());
        self
    }

    #[lua(name = "with_pattern_options", infallible)]
    pub(crate) fn lua_with_pattern_options(mut self, options: LuaFancyRegexPatternOptions) -> Self {
        self.0 = self.0.with_pattern_options(options.into());
        self
    }

    #[lua(name = "with_retriever")]
    pub(crate) fn lua_with_retriever(
        mut self,
        lua: &mlua::Lua,
        func: mlua::Function,
        options: Option<rsjson_lua::config::EncodeConfig>,
    ) -> mlua::Result<Self> {
        let key = lua.create_registry_value(func)?;
        let retriever = LuaRetriever::new(key, options);

        self.0 = self.0.with_retriever(retriever);
        Ok(self)
    }

    #[lua(name = "should_validate_formats", infallible)]
    pub(crate) fn lua_should_validate_formats(mut self, validate: bool) -> Self {
        self.0 = self.0.should_validate_formats(validate);
        self
    }

    #[lua(name = "should_ignore_unknown_formats", infallible)]
    pub(crate) fn lua_should_ignore_unknown_formats(mut self, ignore: bool) -> Self {
        self.0 = self.0.should_ignore_unknown_formats(ignore);
        self
    }

    #[lua(name = "with_format", infallible)]
    pub(crate) fn lua_with_format(
        mut self,
        lua: &mlua::Lua,
        name: String,
        format: mlua::Function,
    ) -> mlua::Result<Self> {
        let key = lua.create_registry_value(format)?;

        self.0 = self.0.with_format(name, move |s| {
            with_lua(|lua| {
                let func = lua.registry_value::<mlua::Function>(&key)?;
                func.call::<bool>(s)
            })
            .unwrap_or(false)
        });
        Ok(self)
    }

    #[lua(name = "with_keyword")]
    pub(crate) fn lua_with_keyword(
        mut self,
        lua: &mlua::Lua,
        name: String,
        factory: mlua::Function,
    ) -> mlua::Result<Self> {
        let key = lua.create_registry_value(factory)?;

        self.0 = self.0.with_keyword(name, move |name, value, location| {
            with_lua(|lua| {
                let func = lua.registry_value::<mlua::Function>(&key)?;
                let name = lua.to_value(name)?;
                let value = lua.to_value(value)?;

                let (is_valid, validate) = func.call::<(mlua::Function, mlua::Function)>((
                    name,
                    value,
                    location.as_str(),
                ))?;

                LuaKeyword::new(lua, is_valid, validate)
            })
            .map(|kw| Box::new(kw) as Box<dyn jsonschema::Keyword>)
            .map_err(|err| jsonschema::ValidationError::custom(err.to_string()))
        });
        Ok(self)
    }
}
