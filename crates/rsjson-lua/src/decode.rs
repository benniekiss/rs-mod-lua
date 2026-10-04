// SPDX-License-Identifier: MIT

use rs_mod_lua_core::{config::DecodeConfig, de::LuaJsonDeserializer};
use serde::de::DeserializeSeed;

pub(crate) fn decode(
    lua: &mlua::Lua,
    json: &[u8],
    config: Option<DecodeConfig>,
) -> mlua::Result<mlua::Value> {
    let mut de = serde_json::Deserializer::from_slice(json);
    LuaJsonDeserializer::new(lua, &config.unwrap_or_default())
        .deserialize(&mut de)
        .map_err(mlua::Error::external)
}

#[cfg(test)]
mod test {
    use mlua::LuaSerdeExt;

    use super::*;

    #[test]
    fn it_json_to_str() {
        let lua = mlua::Lua::new();

        let res = decode(&lua, br#""one two three""#, None)
            .unwrap()
            .to_string()
            .unwrap();

        assert_eq!(res, "one two three");
    }

    #[test]
    fn it_json_to_int() {
        let lua = mlua::Lua::new();

        let res = decode(&lua, b"99", None).unwrap().as_integer().unwrap();

        assert_eq!(res, 99);
    }

    #[test]
    fn it_json_to_float() {
        let lua = mlua::Lua::new();

        let res = decode(&lua, b"9.9", None).unwrap().as_number().unwrap();

        assert_eq!(res, 9.9);
    }

    #[test]
    fn it_json_cast_u64_to_f64() {
        let lua = mlua::Lua::new();
        let mut config = DecodeConfig::default();
        config.cast_u64_to_f64 = true;

        let v = u64::MAX;

        let res = decode(&lua, v.to_string().as_bytes(), Some(config))
            .unwrap()
            .as_number()
            .unwrap();

        assert_eq!(res, v as f64);
    }

    #[test]
    fn it_json_err_cast_u64_to_f64() {
        let lua = mlua::Lua::new();
        let mut config = DecodeConfig::default();
        config.cast_u64_to_f64 = false;

        let v = u64::MAX;

        let res = decode(&lua, v.to_string().as_bytes(), Some(config));

        assert!(res.is_err());
    }

    #[test]
    fn it_json_to_bool() {
        let lua = mlua::Lua::new();

        let res = decode(&lua, b"true", None).unwrap().as_boolean().unwrap();

        assert!(res);

        let res = decode(&lua, b"false", None).unwrap().as_boolean().unwrap();

        assert!(!res);
    }

    #[test]
    fn it_json_to_null() {
        let lua = mlua::Lua::new();

        let res = decode(&lua, b"null", None).unwrap();

        assert!(res.is_null());
    }

    #[test]
    fn it_json_to_nil() {
        let lua = mlua::Lua::new();

        let config = DecodeConfig::default().lua_set_null(false);

        let res = decode(&lua, b"null", Some(config)).unwrap();

        assert!(res.is_nil());
    }

    #[test]
    fn it_json_to_array() {
        let lua = mlua::Lua::new();

        let te = lua.create_sequence_from(vec![1, 2, 3]).unwrap();
        let res = decode(&lua, b"[1,2,3]", None).unwrap();

        assert_eq!(
            lua.from_value::<Vec<i64>>(mlua::Value::Table(te)).unwrap(),
            lua.from_value::<Vec<i64>>(res).unwrap()
        );
    }

    #[test]
    fn it_json_array_decoding_null_as_nil_indices() {
        let lua = mlua::Lua::new();
        let config = DecodeConfig::default().lua_set_null(false);

        let res = decode(&lua, b"[null,1,null,2]", Some(config))
            .unwrap()
            .as_table()
            .unwrap()
            .to_owned();

        assert!(res.raw_get::<mlua::Value>(1).unwrap().is_nil());
        assert_eq!(res.raw_get::<i64>(2).unwrap(), 1);
        assert!(res.raw_get::<mlua::Value>(3).unwrap().is_nil());
        assert_eq!(res.raw_get::<i64>(4).unwrap(), 2);
    }

    #[test]
    fn it_json_array_mt() {
        let lua = mlua::Lua::new();
        let config = DecodeConfig::default().lua_set_array_metatable(true);

        let res = decode(&lua, b"[1,2,3]", Some(config))
            .unwrap()
            .as_table()
            .unwrap()
            .to_owned();

        assert_eq!(res.metatable().unwrap(), lua.array_metatable());
    }

    #[test]
    fn it_json_no_array_mt() {
        let lua = mlua::Lua::new();
        let config = DecodeConfig::default().lua_set_array_metatable(false);

        let res = decode(&lua, b"[1,2,3]", Some(config))
            .unwrap()
            .as_table()
            .unwrap()
            .to_owned();

        assert!(res.metatable().is_none());
    }

    #[test]
    fn it_json_to_table() {
        let lua = mlua::Lua::new();

        let res = decode(&lua, br#"{"a":1,"b":2,"c":3}"#, None)
            .unwrap()
            .as_table()
            .unwrap()
            .to_owned();

        assert_eq!(res.get::<i64>("a").unwrap(), 1);
        assert_eq!(res.get::<i64>("b").unwrap(), 2);
        assert_eq!(res.get::<i64>("c").unwrap(), 3);
    }

    #[test]
    fn it_json_array_of_objects() {
        let lua = mlua::Lua::new();

        let res = decode(&lua, br#"[{"a":1},{"b":2}]"#, None)
            .unwrap()
            .as_table()
            .unwrap()
            .to_owned();

        let first = res.get::<mlua::Table>(1).unwrap();
        let second = res.get::<mlua::Table>(2).unwrap();

        assert_eq!(first.get::<i64>("a").unwrap(), 1);
        assert_eq!(second.get::<i64>("b").unwrap(), 2);
    }

    #[test]
    fn it_json_object_of_arrays() {
        let lua = mlua::Lua::new();

        let res = decode(&lua, br#"{"a":[1,2,3],"b":[4,5,6]}"#, None)
            .unwrap()
            .as_table()
            .unwrap()
            .to_owned();

        let a = res.get::<mlua::Table>("a").unwrap();
        let b = res.get::<mlua::Table>("b").unwrap();

        assert_eq!(a.get::<i64>(1).unwrap(), 1);
        assert_eq!(a.get::<i64>(2).unwrap(), 2);
        assert_eq!(a.get::<i64>(3).unwrap(), 3);

        assert_eq!(b.get::<i64>(1).unwrap(), 4);
        assert_eq!(b.get::<i64>(2).unwrap(), 5);
        assert_eq!(b.get::<i64>(3).unwrap(), 6);
    }

    #[test]
    fn it_json_array_of_arrays() {
        let lua = mlua::Lua::new();

        let res = decode(&lua, br#"[[[1,2,[3,4,5]], [6,7,8]]]"#, None)
            .unwrap()
            .as_table()
            .unwrap()
            .to_owned();

        let first = res.get::<mlua::Table>(1).unwrap();
        let second = first.get::<mlua::Table>(1).unwrap();
        let third = second.get::<mlua::Table>(3).unwrap();
        let fourth = first.get::<mlua::Table>(2).unwrap();

        assert_eq!(second.get::<i64>(1).unwrap(), 1);
        assert_eq!(second.get::<i64>(2).unwrap(), 2);
        assert_eq!(third.get::<i64>(1).unwrap(), 3);
        assert_eq!(third.get::<i64>(2).unwrap(), 4);
        assert_eq!(third.get::<i64>(3).unwrap(), 5);
        assert_eq!(fourth.get::<i64>(1).unwrap(), 6);
        assert_eq!(fourth.get::<i64>(2).unwrap(), 7);
        assert_eq!(fourth.get::<i64>(3).unwrap(), 8);
    }

    #[test]
    fn it_json_object_of_objects() {
        let lua = mlua::Lua::new();

        let res = decode(&lua, br#"{"a":{"b":{"c":42}}}"#, None)
            .unwrap()
            .as_table()
            .unwrap()
            .to_owned();

        let a = res.get::<mlua::Table>("a").unwrap();
        let b = a.get::<mlua::Table>("b").unwrap();

        assert_eq!(b.get::<i64>("c").unwrap(), 42);
    }

    #[test]
    fn it_json_empty_array() {
        let lua = mlua::Lua::new();

        let res = decode(&lua, b"[]", None)
            .unwrap()
            .as_table()
            .unwrap()
            .to_owned();

        assert!(res.is_empty());
    }

    #[test]
    fn it_json_empty_object() {
        let lua = mlua::Lua::new();

        let res = decode(&lua, b"{}", None)
            .unwrap()
            .as_table()
            .unwrap()
            .to_owned();

        assert!(res.is_empty());
    }
}
