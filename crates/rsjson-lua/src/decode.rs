// SPDX-License-Identifier: MIT

use std::fmt;

use mlua::{
    LuaSerdeExt,
    prelude::{Lua, LuaError, LuaValue},
};
use serde::de::{self, DeserializeSeed, MapAccess, SeqAccess, Visitor};

use crate::config::DecodeConfig;

pub(crate) struct LuaJsonDeserializer<'lua> {
    lua: &'lua Lua,
    config: &'lua DecodeConfig,
}

impl<'lua> LuaJsonDeserializer<'lua> {
    pub(crate) fn new(lua: &'lua Lua, config: &'lua DecodeConfig) -> Self {
        Self { lua, config }
    }
}

impl<'de, 'lua> DeserializeSeed<'de> for LuaJsonDeserializer<'lua> {
    type Value = LuaValue;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(LuaJsonVisitor::new(self.lua, self.config))
    }
}

pub(crate) struct LuaJsonVisitor<'lua> {
    lua: &'lua Lua,
    config: &'lua DecodeConfig,
}

impl<'lua> LuaJsonVisitor<'lua> {
    const SERDE_JSON_NUMBER: &'lua str = "$serde_json::private::Number";

    fn new(lua: &'lua Lua, config: &'lua DecodeConfig) -> Self {
        Self { lua, config }
    }
}

impl<'de, 'lua> Visitor<'de> for LuaJsonVisitor<'lua> {
    type Value = LuaValue;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "any JSON value")
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if self.config.null {
            Ok(LuaValue::NULL)
        } else {
            Ok(LuaValue::Nil)
        }
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(LuaValue::Boolean(v))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(LuaValue::Integer(v))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match i64::try_from(v) {
            Ok(i) => Ok(LuaValue::Integer(i)),
            Err(_) if self.config.cast_u64_to_f64 => Ok(LuaValue::Number(v as f64)),
            Err(err) => Err(de::Error::custom(err.to_string())),
        }
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(LuaValue::Number(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.lua
            .create_string(v.as_bytes()) // skip redundant UTF-8 check
            .map(LuaValue::String)
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

        if self.config.set_array_mt {
            table
                .set_metatable(Some(self.lua.array_metatable()))
                .map_err(de::Error::custom)?;
        }

        let mut i: i64 = 1;
        while let Some(v) = seq.next_element_seed(LuaJsonDeserializer {
            lua: self.lua,
            config: self.config,
        })? {
            table.raw_insert(i, v).map_err(de::Error::custom)?;
            i += 1;
        }

        Ok(LuaValue::Table(table))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        // Check for the arbitrary_precision sentinel (`Self::SERDE_JSON_NUMBER`)
        let first_key: Option<String> = map.next_key()?;

        match first_key.as_deref() {
            Some(Self::SERDE_JSON_NUMBER) => {
                // The value is the raw number string, e.g. "1.23456789012345678901234567890"
                let raw: String = map.next_value()?;

                if let Ok(i) = raw.parse::<i64>() {
                    return Ok(LuaValue::Integer(i));
                }

                if let Ok(f) = raw.parse::<f64>() {
                    return Ok(LuaValue::Number(f));
                }

                // If the value cannot be cast to i64 or f64, preserve it as a string
                self.lua
                    .create_string(raw.as_bytes())
                    .map(LuaValue::String)
                    .map_err(de::Error::custom)
            },

            Some(first) => {
                let hint = map.size_hint().unwrap_or(0);
                let table = self
                    .lua
                    .create_table_with_capacity(0, hint)
                    .map_err(de::Error::custom)?;

                let first_key = self
                    .lua
                    .create_string(first.as_bytes())
                    .map_err(de::Error::custom)?;

                let first_val: LuaValue = map.next_value_seed(LuaJsonDeserializer {
                    lua: self.lua,
                    config: self.config,
                })?;

                table
                    .raw_set(first_key, first_val)
                    .map_err(de::Error::custom)?;

                while let Some(k) = map.next_key::<String>()? {
                    let k = self
                        .lua
                        .create_string(k.as_bytes())
                        .map_err(de::Error::custom)?;

                    let v: LuaValue = map.next_value_seed(LuaJsonDeserializer {
                        lua: self.lua,
                        config: self.config,
                    })?;
                    table.raw_set(k, v).map_err(de::Error::custom)?;
                }

                Ok(LuaValue::Table(table))
            },

            None => Ok(LuaValue::Table(
                self.lua.create_table().map_err(de::Error::custom)?,
            )),
        }
    }
}

pub(crate) fn decode(
    lua: &Lua,
    json: &[u8],
    config: Option<DecodeConfig>,
) -> Result<LuaValue, LuaError> {
    let config = config.unwrap_or_default();

    let mut de = serde_json::Deserializer::from_slice(json);
    let seed = LuaJsonDeserializer::new(lua, &config);

    seed.deserialize(&mut de).map_err(LuaError::external)
}

#[cfg(test)]
mod test {
    use mlua::prelude::{LuaSerdeExt, LuaTable};

    use super::*;

    #[test]
    fn it_json_to_str() {
        let lua = Lua::new();

        let res = decode(&lua, br#""one two three""#, None)
            .unwrap()
            .to_string()
            .unwrap();

        assert_eq!(res, "one two three");
    }

    #[test]
    fn it_json_to_int() {
        let lua = Lua::new();

        let res = decode(&lua, b"99", None).unwrap().as_integer().unwrap();

        assert_eq!(res, 99);
    }

    #[test]
    fn it_json_to_float() {
        let lua = Lua::new();

        let res = decode(&lua, b"9.9", None).unwrap().as_number().unwrap();

        assert_eq!(res, 9.9);
    }

    #[test]
    fn it_json_cast_u64_to_f64() {
        let lua = Lua::new();
        let mut config = DecodeConfig::new();
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
        let lua = Lua::new();
        let mut config = DecodeConfig::new();
        config.cast_u64_to_f64 = false;

        let v = u64::MAX;

        let res = decode(&lua, v.to_string().as_bytes(), Some(config));

        assert!(res.is_err());
    }

    #[test]
    fn it_json_to_bool() {
        let lua = Lua::new();

        let res = decode(&lua, b"true", None).unwrap().as_boolean().unwrap();

        assert!(res);

        let res = decode(&lua, b"false", None).unwrap().as_boolean().unwrap();

        assert!(!res);
    }

    #[test]
    fn it_json_to_null() {
        let lua = Lua::new();

        let res = decode(&lua, b"null", None).unwrap();

        assert!(res.is_null());
    }

    #[test]
    fn it_json_to_nil() {
        let lua = Lua::new();

        let mut config = DecodeConfig::new();
        config.null = false;

        let res = decode(&lua, b"null", Some(config)).unwrap();

        assert!(res.is_nil());
    }

    #[test]
    fn it_json_to_array() {
        let lua = Lua::new();

        let te = lua.create_sequence_from(vec![1, 2, 3]).unwrap();
        let res = decode(&lua, b"[1,2,3]", None).unwrap();

        assert_eq!(
            lua.from_value::<Vec<i64>>(LuaValue::Table(te)).unwrap(),
            lua.from_value::<Vec<i64>>(res).unwrap()
        );
    }

    #[test]
    fn it_json_array_mt() {
        let lua = Lua::new();
        let mut config = DecodeConfig::new();
        config.set_array_mt = true;

        let res = decode(&lua, b"[1,2,3]", Some(config))
            .unwrap()
            .as_table()
            .unwrap()
            .to_owned();

        assert_eq!(res.metatable().unwrap(), lua.array_metatable());
    }

    #[test]
    fn it_json_no_array_mt() {
        let lua = Lua::new();
        let mut config = DecodeConfig::new();
        config.set_array_mt = false;

        let res = decode(&lua, b"[1,2,3]", Some(config))
            .unwrap()
            .as_table()
            .unwrap()
            .to_owned();

        assert!(res.metatable().is_none());
    }

    #[test]
    fn it_json_to_table() {
        let lua = Lua::new();

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
        let lua = Lua::new();

        let res = decode(&lua, br#"[{"a":1},{"b":2}]"#, None)
            .unwrap()
            .as_table()
            .unwrap()
            .to_owned();

        let first = res.get::<LuaTable>(1).unwrap();
        let second = res.get::<LuaTable>(2).unwrap();

        assert_eq!(first.get::<i64>("a").unwrap(), 1);
        assert_eq!(second.get::<i64>("b").unwrap(), 2);
    }

    #[test]
    fn it_json_object_of_arrays() {
        let lua = Lua::new();

        let res = decode(&lua, br#"{"a":[1,2,3],"b":[4,5,6]}"#, None)
            .unwrap()
            .as_table()
            .unwrap()
            .to_owned();

        let a = res.get::<LuaTable>("a").unwrap();
        let b = res.get::<LuaTable>("b").unwrap();

        assert_eq!(a.get::<i64>(1).unwrap(), 1);
        assert_eq!(a.get::<i64>(2).unwrap(), 2);
        assert_eq!(a.get::<i64>(3).unwrap(), 3);

        assert_eq!(b.get::<i64>(1).unwrap(), 4);
        assert_eq!(b.get::<i64>(2).unwrap(), 5);
        assert_eq!(b.get::<i64>(3).unwrap(), 6);
    }

    #[test]
    fn it_json_array_of_arrays() {
        let lua = Lua::new();

        let res = decode(&lua, br#"[[[1,2,[3,4,5]], [6,7,8]]]"#, None)
            .unwrap()
            .as_table()
            .unwrap()
            .to_owned();

        let first = res.get::<LuaTable>(1).unwrap();
        let second = first.get::<LuaTable>(1).unwrap();
        let third = second.get::<LuaTable>(3).unwrap();
        let fourth = first.get::<LuaTable>(2).unwrap();

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
        let lua = Lua::new();

        let res = decode(&lua, br#"{"a":{"b":{"c":42}}}"#, None)
            .unwrap()
            .as_table()
            .unwrap()
            .to_owned();

        let a = res.get::<LuaTable>("a").unwrap();
        let b = a.get::<LuaTable>("b").unwrap();

        assert_eq!(b.get::<i64>("c").unwrap(), 42);
    }

    #[test]
    fn it_json_empty_array() {
        let lua = Lua::new();

        let res = decode(&lua, b"[]", None)
            .unwrap()
            .as_table()
            .unwrap()
            .to_owned();

        assert!(res.is_empty());
    }

    #[test]
    fn it_json_empty_object() {
        let lua = Lua::new();

        let res = decode(&lua, b"{}", None)
            .unwrap()
            .as_table()
            .unwrap()
            .to_owned();

        assert!(res.is_empty());
    }
}
