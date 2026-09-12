// SPDX-License-Identifier: MIT

use std::ops::Deref;

#[derive(mlua::UserData, mlua::FromLua, Clone)]
pub struct DecodeConfig {
    #[lua(skip)]
    options: mlua::serde::SerializeOptions,
    #[lua(skip)]
    pub cast_u64_to_f64: bool,
}

impl From<mlua::serde::SerializeOptions> for DecodeConfig {
    fn from(value: mlua::serde::SerializeOptions) -> Self {
        DecodeConfig {
            options: value,
            cast_u64_to_f64: false,
        }
    }
}

impl From<DecodeConfig> for mlua::serde::SerializeOptions {
    fn from(value: DecodeConfig) -> Self {
        value.options
    }
}

impl AsRef<mlua::serde::SerializeOptions> for DecodeConfig {
    fn as_ref(&self) -> &mlua::serde::SerializeOptions {
        &self.options
    }
}

impl Deref for DecodeConfig {
    type Target = mlua::serde::SerializeOptions;

    fn deref(&self) -> &Self::Target {
        &self.options
    }
}

impl Default for DecodeConfig {
    fn default() -> Self {
        mlua::serde::SerializeOptions::new()
            .detect_serde_json_arbitrary_precision(true)
            .into()
    }
}

#[mlua::userdata_impl]
impl DecodeConfig {
    #[lua(name = "new", infallible)]
    pub fn lua_new() -> Self {
        Self::default()
    }

    #[lua(name = "null", getter, infallible)]
    pub fn lua_null(&self) -> bool {
        self.options.serialize_unit_to_null && self.options.serialize_none_to_null
    }

    #[must_use]
    #[lua(name = "set_null", infallible)]
    pub fn lua_set_null(&mut self, enable: bool) -> Self {
        self.options = self
            .options
            .serialize_unit_to_null(enable)
            .serialize_none_to_null(enable);
        self.clone()
    }

    #[lua(name = "cast_u64_to_f64", getter, infallible)]
    pub fn lua_cast_u64_to_f64(&self) -> bool {
        self.cast_u64_to_f64
    }

    #[must_use]
    #[lua(name = "set_cast_u64_to_f64", infallible)]
    pub fn lua_set_cast_u64_to_f64(&mut self, enable: bool) -> Self {
        self.cast_u64_to_f64 = enable;
        self.clone()
    }

    #[lua(name = "array_metatable", getter, infallible)]
    pub fn lua_array_metatable(&self) -> bool {
        self.options.set_array_metatable
    }

    #[must_use]
    #[lua(name = "set_array_metatable", infallible)]
    pub fn lua_set_array_metatable(&mut self, enable: bool) -> Self {
        self.options = self.options.set_array_metatable(enable);
        self.clone()
    }
}
