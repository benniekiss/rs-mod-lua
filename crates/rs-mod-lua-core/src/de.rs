// SPDX-License-Identifier: MIT

use std::fmt;

use mlua::LuaSerdeExt;
use serde::de::{self, DeserializeSeed, MapAccess, SeqAccess, Visitor};

use crate::config::DecodeConfig;

pub struct LuaJsonDeserializer<'lua> {
    lua: &'lua mlua::Lua,
    config: &'lua DecodeConfig,
}

impl<'lua> LuaJsonDeserializer<'lua> {
    pub fn new(lua: &'lua mlua::Lua, config: &'lua DecodeConfig) -> Self {
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
        let mut values = Vec::with_capacity(hint);

        while let Some(value) =
            seq.next_element_seed(LuaJsonDeserializer::new(self.lua, self.config))?
        {
            values.push(value);
        }

        let table = self
            .lua
            .create_sequence_from(values)
            .map_err(de::Error::custom)?;

        if self.config.set_array_metatable {
            table
                .set_metatable(Some(self.lua.array_metatable()))
                .map_err(de::Error::custom)?;
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
                let mut entries = Vec::with_capacity(hint.saturating_add(1));
                entries.push((k, v));

                while let Some((k, v)) = map.next_entry_seed(
                    LuaJsonDeserializer::new(self.lua, self.config),
                    LuaJsonDeserializer::new(self.lua, self.config),
                )? {
                    entries.push((k, v));
                }

                self.lua
                    .create_table_from(entries)
                    .map(mlua::Value::Table)
                    .map_err(de::Error::custom)
            },

            None => Ok(mlua::Value::Table(
                self.lua.create_table().map_err(de::Error::custom)?,
            )),
        }
    }
}
