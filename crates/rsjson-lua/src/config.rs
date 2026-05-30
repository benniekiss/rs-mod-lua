// SPDX-License-Identifier: MIT

use mlua::prelude::{FromLua, LuaUserData, LuaValue};
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Serialize, Deserialize, FromLua, Clone)]
pub(crate) struct EncodeConfig {
    pub(crate) indent: Option<usize>,
    pub(crate) prefix: String,
    pub(crate) sort_keys: bool,
    pub(crate) empty_table_as_array: bool,
    pub(crate) detect_mixed_tables: bool,
    pub(crate) error_unsupported: bool,
    pub(crate) error_cycles: bool,
}

impl EncodeConfig {
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

impl Default for EncodeConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl LuaUserData for EncodeConfig {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_lua, _val: LuaValue| {
            let config = EncodeConfig::new();
            Ok(config)
        });
    }

    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("indent", |_, this: &EncodeConfig| Ok(this.indent));
        fields.add_field_method_set(
            "indent",
            |_, this: &mut EncodeConfig, val: Option<usize>| {
                this.indent = val;
                Ok(())
            },
        );

        fields.add_field_method_get("prefix", |_, this: &EncodeConfig| Ok(this.prefix.clone()));
        fields.add_field_method_set("prefix", |_, this: &mut EncodeConfig, val: String| {
            this.prefix = val;
            Ok(())
        });

        fields.add_field_method_get("sort_keys", |_, this: &EncodeConfig| Ok(this.sort_keys));
        fields.add_field_method_set("sort_keys", |_, this: &mut EncodeConfig, val: bool| {
            this.sort_keys = val;
            Ok(())
        });

        fields.add_field_method_get("empty_table_as_array", |_, this: &EncodeConfig| {
            Ok(this.empty_table_as_array)
        });
        fields.add_field_method_set(
            "empty_table_as_array",
            |_, this: &mut EncodeConfig, val: bool| {
                this.empty_table_as_array = val;
                Ok(())
            },
        );

        fields.add_field_method_get("detect_mixed_tables", |_, this: &EncodeConfig| {
            Ok(this.detect_mixed_tables)
        });
        fields.add_field_method_set(
            "detect_mixed_tables",
            |_, this: &mut EncodeConfig, val: bool| {
                this.detect_mixed_tables = val;
                Ok(())
            },
        );

        fields.add_field_method_get("error_unsupported", |_, this: &EncodeConfig| {
            Ok(this.error_unsupported)
        });
        fields.add_field_method_set(
            "error_unsupported",
            |_, this: &mut EncodeConfig, val: bool| {
                this.error_unsupported = val;
                Ok(())
            },
        );

        fields.add_field_method_get("error_cycles", |_, this: &EncodeConfig| {
            Ok(this.error_cycles)
        });
        fields.add_field_method_set("error_cycles", |_, this: &mut EncodeConfig, val: bool| {
            this.error_cycles = val;
            Ok(())
        });
    }
}

#[non_exhaustive]
#[derive(FromLua, Clone)]
pub(crate) struct DecodeConfig {
    pub(crate) null: bool,
    pub(crate) cast_u64_to_f64: bool,
    pub(crate) set_array_mt: bool,
}

impl DecodeConfig {
    pub(crate) fn new() -> Self {
        Self {
            null: true,
            cast_u64_to_f64: true,
            set_array_mt: true,
        }
    }
}

impl Default for DecodeConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl LuaUserData for DecodeConfig {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |_lua, _val: LuaValue| Ok(DecodeConfig::new()));
    }

    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("null", |_, this| Ok(this.null));
        fields.add_field_method_set("null", |_, this: &mut DecodeConfig, val: bool| {
            this.null = val;
            Ok(())
        });
        fields.add_field_method_get("cast_u64_to_f64", |_, this| Ok(this.cast_u64_to_f64));
        fields.add_field_method_set(
            "cast_u64_to_f64",
            |_, this: &mut DecodeConfig, val: bool| {
                this.cast_u64_to_f64 = val;
                Ok(())
            },
        );
        fields.add_field_method_get("set_array_mt", |_, this| Ok(this.set_array_mt));
        fields.add_field_method_set("set_array_mt", |_, this: &mut DecodeConfig, val: bool| {
            this.set_array_mt = val;
            Ok(())
        });
    }
}
