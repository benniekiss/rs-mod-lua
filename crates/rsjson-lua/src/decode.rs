// SPDX-License-Identifier: MIT

use std::fmt;

use mlua::LuaSerdeExt;
use serde::de::{self, DeserializeSeed, MapAccess, SeqAccess, Visitor};

use crate::config::DecodeConfig;

pub(crate) struct LuaJsonDeserializer<'lua> {
    lua: &'lua mlua::Lua,
    config: &'lua DecodeConfig,
}

impl<'lua> LuaJsonDeserializer<'lua> {
    pub(crate) fn new(lua: &'lua mlua::Lua, config: &'lua DecodeConfig) -> Self {
        Self { lua, config }
    }
}

impl<'de, 'lua> DeserializeSeed<'de> for LuaJsonDeserializer<'lua> {
    type Value = mlua::Value;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(LuaJsonVisitor::new(self.lua, self.config))
    }
}

pub(crate) struct LuaJsonVisitor<'lua> {
    lua: &'lua mlua::Lua,
    config: &'lua DecodeConfig,
}

impl<'lua> LuaJsonVisitor<'lua> {
    const SERDE_JSON_NUMBER: &'static str = "$serde_json::private::Number";

    fn new(lua: &'lua mlua::Lua, config: &'lua DecodeConfig) -> Self {
        Self { lua, config }
    }
}

impl<'de, 'lua> Visitor<'de> for LuaJsonVisitor<'lua> {
    type Value = mlua::Value;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "any JSON value")
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if self.config.serialize_unit_to_null {
            Ok(mlua::Value::NULL)
        } else {
            Ok(mlua::Value::Nil)
        }
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if self.config.serialize_none_to_null {
            Ok(mlua::Value::NULL)
        } else {
            Ok(mlua::Value::Nil)
        }
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(mlua::Value::Boolean(v))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(mlua::Value::Integer(v))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match i64::try_from(v) {
            Ok(i) => Ok(mlua::Value::Integer(i)),
            Err(_) if self.config.cast_u64_to_f64 => Ok(mlua::Value::Number(v as f64)),
            Err(err) => Err(de::Error::custom(err.to_string())),
        }
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(mlua::Value::Number(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.lua
            .create_string(v)
            .map(mlua::Value::String)
            .map_err(de::Error::custom)
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_str(&v)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let hint = seq.size_hint().unwrap_or(0);
        let table = self
            .lua
            .create_table_with_capacity(hint, 0)
            .map_err(de::Error::custom)?;

        if self.config.set_array_metatable {
            table
                .set_metatable(Some(self.lua.array_metatable()))
                .map_err(de::Error::custom)?;
        }

        while let Some(v) =
            seq.next_element_seed(LuaJsonDeserializer::new(self.lua, self.config))?
        {
            table.raw_push(v).map_err(de::Error::custom)?;
        }

        Ok(mlua::Value::Table(table))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        match map.next_entry_seed(
            LuaJsonDeserializer::new(self.lua, self.config),
            LuaJsonDeserializer::new(self.lua, self.config),
        )? {
            // Check for the arbitrary_precision sentinel (`Self::SERDE_JSON_NUMBER`)
            Some((mlua::Value::String(k), mlua::Value::String(v)))
                if self.config.detect_serde_json_arbitrary_precision
                    && k == Self::SERDE_JSON_NUMBER =>
            {
                // The value is the raw number string, e.g. "1.23456789012345678901234567890"
                v.to_str()
                    .and_then(|s| {
                        s.parse::<i64>()
                            .map(mlua::Value::Integer)
                            .or_else(|_| s.parse::<f64>().map(mlua::Value::Number))
                            .map_err(mlua::Error::external)
                    })
                    // If the value cannot be cast to i64 or f64, preserve it as a string
                    .or(Ok(mlua::Value::String(v)))
            },

            Some((k, v)) => {
                let hint = map.size_hint().unwrap_or(0);
                let table = self
                    .lua
                    .create_table_with_capacity(0, hint)
                    .map_err(de::Error::custom)?;

                table.raw_set(k, v).map_err(de::Error::custom)?;

                while let Some((k, v)) = map.next_entry_seed(
                    LuaJsonDeserializer::new(self.lua, self.config),
                    LuaJsonDeserializer::new(self.lua, self.config),
                )? {
                    table.raw_set(k, v).map_err(de::Error::custom)?;
                }

                Ok(mlua::Value::Table(table))
            },

            None => Ok(mlua::Value::Table(
                self.lua.create_table().map_err(de::Error::custom)?,
            )),
        }
    }
}

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

        let mut config = DecodeConfig::default();
        config.lua_set_null(false);

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
    fn it_json_array_mt() {
        let lua = mlua::Lua::new();
        let mut config = DecodeConfig::default();
        config.lua_set_array_metatable(true);

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
        let mut config = DecodeConfig::default();
        config.lua_set_array_metatable(false);

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
