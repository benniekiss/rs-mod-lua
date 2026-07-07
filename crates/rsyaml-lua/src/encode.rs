// SPDX-License-Identifier: MIT

use crate::config::{EncodeConfig, YamlEncodeOptions};

/// Serialize an `mlua::Value` to a YAML string.
pub(crate) fn encode(
    value: &mlua::Value,
    config: Option<EncodeConfig>,
    options: Option<YamlEncodeOptions>,
) -> mlua::Result<String> {
    let obj = match config {
        Some(config) => value
            .to_serializable()
            .sort_keys(config.sort_keys)
            .encode_empty_tables_as_array(config.encode_empty_tables_as_array)
            .detect_mixed_tables(config.detect_mixed_tables)
            .deny_unsupported_types(config.deny_unsupported_types)
            .deny_recursive_tables(config.deny_recursive_tables),
        None => value.to_serializable(),
    };

    serde_saphyr::to_string_with_options(&obj, *options.unwrap_or_default())
        .map_err(mlua::Error::external)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_str_to_json() {
        let lua = mlua::Lua::new();

        let te = lua.create_string("one two three").unwrap();
        let res = encode(&mlua::Value::String(te), None, None).unwrap();

        assert_eq!(res, r#""one two three""#);
    }

    #[test]
    fn it_int_to_json() {
        let res = encode(&mlua::Value::Integer(99), None, None).unwrap();

        assert_eq!(res, "99");
    }

    #[test]
    fn it_float_to_json() {
        let res = encode(&mlua::Value::Number(9.9), None, None).unwrap();

        assert_eq!(res, "9.9");
    }

    #[test]
    fn it_bool_to_json() {
        let res = encode(&mlua::Value::Boolean(true), None, None).unwrap();

        assert_eq!(res, "true");

        let res = encode(&mlua::Value::Boolean(false), None, None).unwrap();

        assert_eq!(res, "false");
    }

    #[test]
    fn it_nil_to_json() {
        let res = encode(&mlua::Value::Nil, None, None).unwrap();

        assert_eq!(res, "null");
    }

    #[test]
    fn it_array_to_json() {
        let lua = mlua::Lua::new();

        let te = lua.create_sequence_from(vec![1, 2, 3]).unwrap();
        let res = encode(&mlua::Value::Table(te), None, None).unwrap();

        assert_eq!(res, "[1,2,3]");
    }

    #[test]
    fn it_table_to_json() {
        let lua = mlua::Lua::new();

        let mut config = EncodeConfig::default();
        config.lua_set_sort_keys(true);

        let te = lua.create_table().unwrap();
        te.set("a", 1).unwrap();
        te.set("b", 2).unwrap();
        te.set("c", 3).unwrap();

        let res = encode(&mlua::Value::Table(te), Some(config), None).unwrap();

        assert_eq!(res, r#"{"a":1,"b":2,"c":3}"#);
    }
}
