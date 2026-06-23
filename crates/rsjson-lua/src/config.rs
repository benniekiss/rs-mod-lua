// SPDX-License-Identifier: MIT

use std::ops::Deref;

#[derive(mlua::UserData, mlua::FromLua, Clone)]
pub struct EncodeConfig {
    #[lua(skip)]
    options: mlua::DeserializeOptions,
    #[lua(skip)]
    pub(crate) indent: Option<usize>,
    #[lua(skip)]
    pub(crate) prefix: String,
}

impl From<mlua::DeserializeOptions> for EncodeConfig {
    fn from(value: mlua::DeserializeOptions) -> Self {
        EncodeConfig {
            options: value,
            indent: None,
            prefix: " ".to_string(),
        }
    }
}

impl From<EncodeConfig> for mlua::DeserializeOptions {
    fn from(value: EncodeConfig) -> Self {
        value.options
    }
}

impl AsRef<mlua::DeserializeOptions> for EncodeConfig {
    fn as_ref(&self) -> &mlua::DeserializeOptions {
        &self.options
    }
}

impl Deref for EncodeConfig {
    type Target = mlua::DeserializeOptions;

    fn deref(&self) -> &Self::Target {
        &self.options
    }
}

impl Default for EncodeConfig {
    fn default() -> Self {
        mlua::DeserializeOptions::new().into()
    }
}

#[mlua::userdata_impl]
impl EncodeConfig {
    #[lua(name = "new", infallible)]
    pub(crate) fn lua_new() -> Self {
        Self::default()
    }

    #[lua(name = "indent", getter, infallible)]
    pub(crate) fn lua_indent(&self) -> Option<usize> {
        self.indent
    }

    #[lua(name = "set_indent", infallible)]
    pub(crate) fn lua_set_indent(&mut self, indent: Option<usize>) -> Self {
        self.indent = indent;
        self.clone()
    }

    #[lua(name = "prefix", getter, infallible)]
    pub(crate) fn lua_prefix(&self) -> String {
        self.prefix.clone()
    }

    #[lua(name = "set_prefix", infallible)]
    pub(crate) fn lua_set_prefix(&mut self, prefix: &str) -> Self {
        self.prefix = prefix.to_string();
        self.clone()
    }

    #[lua(name = "deny_unsupported_types", getter, infallible)]
    pub(crate) fn lua_deny_unsupported_types(&self) -> bool {
        self.options.deny_unsupported_types
    }

    #[lua(name = "set_deny_unsupported_types", infallible)]
    pub(crate) fn lua_set_deny_unsupported_types(&mut self, enable: bool) -> Self {
        self.options = self.options.deny_unsupported_types(enable);
        self.clone()
    }

    #[lua(name = "deny_recursive_tables", getter, infallible)]
    pub(crate) fn lua_deny_recursive_tables(&self) -> bool {
        self.options.deny_recursive_tables
    }

    #[lua(name = "set_deny_recursive_tables", infallible)]
    pub(crate) fn lua_set_deny_recursive_tables(&mut self, enable: bool) -> Self {
        self.options = self.options.deny_recursive_tables(enable);
        self.clone()
    }

    #[lua(name = "sort_keys", getter, infallible)]
    pub(crate) fn lua_sort_keys(&self) -> bool {
        self.options.sort_keys
    }

    #[lua(name = "set_sort_keys", infallible)]
    pub(crate) fn lua_set_sort_keys(&mut self, enable: bool) -> Self {
        self.options = self.options.sort_keys(enable);
        self.clone()
    }

    #[lua(name = "encode_empty_tables_as_array", getter, infallible)]
    pub(crate) fn lua_encode_empty_tables_as_array(&self) -> bool {
        self.options.encode_empty_tables_as_array
    }

    #[lua(name = "set_encode_empty_tables_as_array", infallible)]
    pub(crate) fn lua_set_encode_empty_tables_as_array(&mut self, enable: bool) -> Self {
        self.options = self.options.encode_empty_tables_as_array(enable);
        self.clone()
    }

    #[lua(name = "detect_mixed_tables", getter, infallible)]
    pub(crate) fn lua_detect_mixed_tables(&self) -> bool {
        self.options.detect_mixed_tables
    }

    #[lua(name = "set_detect_mixed_tables", infallible)]
    pub(crate) fn lua_set_detect_mixed_tables(&mut self, enable: bool) -> Self {
        self.options = self.options.detect_mixed_tables(enable);
        self.clone()
    }
}

#[derive(mlua::UserData, mlua::FromLua, Clone)]
pub struct DecodeConfig {
    #[lua(skip)]
    options: mlua::SerializeOptions,
    #[lua(skip)]
    pub(crate) cast_u64_to_f64: bool,
}

impl From<mlua::SerializeOptions> for DecodeConfig {
    fn from(value: mlua::SerializeOptions) -> Self {
        DecodeConfig {
            options: value,
            cast_u64_to_f64: false,
        }
    }
}

impl From<DecodeConfig> for mlua::SerializeOptions {
    fn from(value: DecodeConfig) -> Self {
        value.options
    }
}

impl AsRef<mlua::SerializeOptions> for DecodeConfig {
    fn as_ref(&self) -> &mlua::SerializeOptions {
        &self.options
    }
}

impl Deref for DecodeConfig {
    type Target = mlua::SerializeOptions;

    fn deref(&self) -> &Self::Target {
        &self.options
    }
}

impl Default for DecodeConfig {
    fn default() -> Self {
        mlua::SerializeOptions::new()
            .detect_serde_json_arbitrary_precision(true)
            .into()
    }
}

#[mlua::userdata_impl]
impl DecodeConfig {
    #[lua(name = "new", infallible)]
    pub(crate) fn lua_new() -> Self {
        Self::default()
    }

    #[lua(name = "null", getter, infallible)]
    pub(crate) fn lua_null(&self) -> bool {
        self.options.serialize_unit_to_null && self.options.serialize_none_to_null
    }

    #[lua(name = "set_null", infallible)]
    pub(crate) fn lua_set_null(&mut self, enable: bool) -> Self {
        self.options = self
            .options
            .serialize_unit_to_null(enable)
            .serialize_none_to_null(enable);
        self.clone()
    }

    #[lua(name = "cast_u64_to_f64", getter, infallible)]
    pub(crate) fn lua_cast_u64_to_f64(&self) -> bool {
        self.cast_u64_to_f64
    }

    #[lua(name = "set_cast_u64_to_f64", infallible)]
    pub(crate) fn lua_set_cast_u64_to_f64(&mut self, enable: bool) -> Self {
        self.cast_u64_to_f64 = enable;
        self.clone()
    }

    #[lua(name = "array_metatable", getter, infallible)]
    pub(crate) fn lua_array_metatable(&self) -> bool {
        self.options.set_array_metatable
    }

    #[lua(name = "set_array_metatable", infallible)]
    pub(crate) fn lua_set_array_metatable(&mut self, enable: bool) -> Self {
        self.options = self.options.set_array_metatable(enable);
        self.clone()
    }
}
