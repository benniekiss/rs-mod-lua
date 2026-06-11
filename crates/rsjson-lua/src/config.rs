// SPDX-License-Identifier: MIT

#[derive(mlua::UserData, mlua::FromLua, Clone)]
pub(crate) struct EncodeConfig {
    pub(crate) indent: Option<usize>,
    pub(crate) prefix: String,
    pub(crate) sort_keys: bool,
    pub(crate) empty_table_as_array: bool,
    pub(crate) detect_mixed_tables: bool,
    pub(crate) error_unsupported: bool,
    pub(crate) error_cycles: bool,
}

impl Default for EncodeConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[mlua::userdata_impl]
impl EncodeConfig {
    #[lua(infallible)]
    pub(crate) fn new() -> Self {
        Self {
            indent: None,
            prefix: " ".to_string(),
            sort_keys: false,
            empty_table_as_array: false,
            detect_mixed_tables: false,
            error_unsupported: true,
            error_cycles: true,
        }
    }
}

#[derive(mlua::UserData, mlua::FromLua, Clone)]
pub(crate) struct DecodeConfig {
    pub(crate) null: bool,
    pub(crate) cast_u64_to_f64: bool,
    pub(crate) set_array_mt: bool,
}

impl Default for DecodeConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[mlua::userdata_impl]
impl DecodeConfig {
    #[lua(infallible)]
    pub(crate) fn new() -> Self {
        Self {
            null: true,
            cast_u64_to_f64: true,
            set_array_mt: true,
        }
    }
}
