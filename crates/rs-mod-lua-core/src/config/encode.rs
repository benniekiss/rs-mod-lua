// SPDX-License-Identifier: MIT

use std::ops::Deref;

#[derive(mlua::UserData, mlua::FromLua, Clone)]
pub struct EncodeConfig {
    #[lua(skip)]
    options: mlua::serde::DeserializeOptions,
    #[lua(skip)]
    pub indent: Option<usize>,
    #[lua(skip)]
    pub prefix: String,
}

impl From<mlua::serde::DeserializeOptions> for EncodeConfig {
    fn from(value: mlua::serde::DeserializeOptions) -> Self {
        EncodeConfig {
            options: value,
            indent: None,
            prefix: " ".to_string(),
        }
    }
}

impl From<EncodeConfig> for mlua::serde::DeserializeOptions {
    fn from(value: EncodeConfig) -> Self {
        value.options
    }
}

impl AsRef<mlua::serde::DeserializeOptions> for EncodeConfig {
    fn as_ref(&self) -> &mlua::serde::DeserializeOptions {
        &self.options
    }
}

impl Deref for EncodeConfig {
    type Target = mlua::serde::DeserializeOptions;

    fn deref(&self) -> &Self::Target {
        &self.options
    }
}

impl Default for EncodeConfig {
    fn default() -> Self {
        mlua::serde::DeserializeOptions::new().into()
    }
}

#[mlua::userdata_impl]
impl EncodeConfig {
    #[lua(name = "new", infallible)]
    pub fn lua_new() -> Self {
        Self::default()
    }

    #[lua(name = "indent", getter, infallible)]
    pub fn lua_indent(&self) -> Option<usize> {
        self.indent
    }

    #[must_use]
    #[lua(name = "set_indent", infallible)]
    pub fn lua_set_indent(&mut self, indent: Option<usize>) -> Self {
        self.indent = indent;
        self.clone()
    }

    #[lua(name = "prefix", getter, infallible)]
    pub fn lua_prefix(&self) -> String {
        self.prefix.clone()
    }

    #[must_use]
    #[lua(name = "set_prefix", infallible)]
    pub fn lua_set_prefix(&mut self, prefix: &str) -> Self {
        self.prefix = prefix.to_string();
        self.clone()
    }

    #[lua(name = "deny_unsupported_types", getter, infallible)]
    pub fn lua_deny_unsupported_types(&self) -> bool {
        self.options.deny_unsupported_types
    }

    #[must_use]
    #[lua(name = "set_deny_unsupported_types", infallible)]
    pub fn lua_set_deny_unsupported_types(&mut self, enable: bool) -> Self {
        self.options = self.options.deny_unsupported_types(enable);
        self.clone()
    }

    #[lua(name = "deny_recursive_tables", getter, infallible)]
    pub fn lua_deny_recursive_tables(&self) -> bool {
        self.options.deny_recursive_tables
    }

    #[must_use]
    #[lua(name = "set_deny_recursive_tables", infallible)]
    pub fn lua_set_deny_recursive_tables(&mut self, enable: bool) -> Self {
        self.options = self.options.deny_recursive_tables(enable);
        self.clone()
    }

    #[lua(name = "sort_keys", getter, infallible)]
    pub fn lua_sort_keys(&self) -> bool {
        self.options.sort_keys
    }

    #[must_use]
    #[lua(name = "set_sort_keys", infallible)]
    pub fn lua_set_sort_keys(&mut self, enable: bool) -> Self {
        self.options = self.options.sort_keys(enable);
        self.clone()
    }

    #[lua(name = "encode_empty_tables_as_array", getter, infallible)]
    pub fn lua_encode_empty_tables_as_array(&self) -> bool {
        self.options.encode_empty_tables_as_array
    }

    #[must_use]
    #[lua(name = "set_encode_empty_tables_as_array", infallible)]
    pub fn lua_set_encode_empty_tables_as_array(&mut self, enable: bool) -> Self {
        self.options = self.options.encode_empty_tables_as_array(enable);
        self.clone()
    }

    #[lua(name = "detect_mixed_tables", getter, infallible)]
    pub fn lua_detect_mixed_tables(&self) -> bool {
        self.options.detect_mixed_tables
    }

    #[must_use]
    #[lua(name = "set_detect_mixed_tables", infallible)]
    pub fn lua_set_detect_mixed_tables(&mut self, enable: bool) -> Self {
        self.options = self.options.detect_mixed_tables(enable);
        self.clone()
    }
}
