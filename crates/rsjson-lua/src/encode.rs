// SPDX-License-Identifier: MIT

use mlua::prelude::{Lua, LuaError, LuaString, LuaValue};
use serde::Serialize;
use serde_json::ser::{PrettyFormatter, Serializer};

use crate::config::EncodeConfig;

/// Serialize an `mlua::Value` to a JSON string.
pub(crate) fn encode(
    lua: &Lua,
    value: &LuaValue,
    config: Option<EncodeConfig>,
) -> Result<LuaString, LuaError> {
    let config = config.unwrap_or_default();

    let obj = value
        .to_serializable()
        .sort_keys(config.sort_keys)
        .encode_empty_tables_as_array(config.empty_table_as_array)
        .detect_mixed_tables(config.detect_mixed_tables)
        .deny_unsupported_types(config.error_unsupported)
        .deny_recursive_tables(config.error_cycles);

    let mut writer: Vec<u8> = Vec::new();

    match config.indent {
        Some(n) => {
            let prefix = config.prefix.repeat(n);
            let formatter = PrettyFormatter::with_indent(prefix.as_bytes());
            let mut ser = Serializer::with_formatter(&mut writer, formatter);
            obj.serialize(&mut ser).map_err(LuaError::external)?;
        },
        None => {
            let mut ser = Serializer::new(&mut writer);
            obj.serialize(&mut ser).map_err(LuaError::external)?;
        },
    }

    lua.create_string(writer)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_str_to_json() {
        let lua = Lua::new();

        let te = lua.create_string("one two three").unwrap();
        let res = encode(&lua, &LuaValue::String(te), None).unwrap();

        assert_eq!(res, r#""one two three""#);
    }

    #[test]
    fn it_int_to_json() {
        let lua = Lua::new();

        let res = encode(&lua, &LuaValue::Integer(99), None).unwrap();

        assert_eq!(res, "99");
    }

    #[test]
    fn it_float_to_json() {
        let lua = Lua::new();

        let res = encode(&lua, &LuaValue::Number(9.9), None).unwrap();

        assert_eq!(res, "9.9");
    }

    #[test]
    fn it_bool_to_json() {
        let lua = Lua::new();

        let res = encode(&lua, &LuaValue::Boolean(true), None).unwrap();

        assert_eq!(res, "true");

        let res = encode(&lua, &LuaValue::Boolean(false), None).unwrap();

        assert_eq!(res, "false");
    }

    #[test]
    fn it_nil_to_json() {
        let lua = Lua::new();

        let res = encode(&lua, &LuaValue::Nil, None).unwrap();

        assert_eq!(res, "null");
    }

    #[test]
    fn it_array_to_json() {
        let lua = Lua::new();

        let te = lua.create_sequence_from(vec![1, 2, 3]).unwrap();
        let res = encode(&lua, &LuaValue::Table(te), None).unwrap();

        assert_eq!(res, "[1,2,3]");
    }

    #[test]
    fn it_table_to_json() {
        let lua = Lua::new();

        let mut config = EncodeConfig::new();
        config.sort_keys = true;

        let te = lua.create_table().unwrap();
        te.set("a", 1).unwrap();
        te.set("b", 2).unwrap();
        te.set("c", 3).unwrap();

        let res = encode(&lua, &LuaValue::Table(te), Some(config)).unwrap();

        assert_eq!(res, r#"{"a":1,"b":2,"c":3}"#);
    }
}
