use mlua::LuaSerdeExt;
use rsjson_lua::config::{DecodeConfig, EncodeConfig};

use crate::{
    evaluation::*,
    lua::lua_to_json,
    validator::{LuaValidator, LuaValidatorMap},
};

fn jsonschema_meta_lua(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let table = lua.create_table()?;

    table.set(
        "is_valid",
        lua.create_function(
            |lua, (schema, options): (mlua::Value, Option<EncodeConfig>)| -> mlua::Result<bool> {
                lua_to_json(lua, schema, options).map(|val| jsonschema::meta::is_valid(&val))
            },
        )?,
    )?;

    table.set(
        "validate",
        lua.create_function(
            |lua,
             (schema, options): (mlua::Value, Option<EncodeConfig>)|
             -> mlua::Result<(bool, Option<String>)> {
                lua_to_json(lua, schema, options).map(|val| {
                    match jsonschema::meta::validate(&val) {
                        Ok(_) => (true, None),
                        Err(err) => (false, Some(err.to_string())),
                    }
                })
            },
        )?,
    )?;

    table.set(
        "validator_for",
        lua.create_function(
            |lua,
             (schema, options): (mlua::Value, Option<EncodeConfig>)|
             -> mlua::Result<LuaValidator> {
                lua_to_json(lua, schema, options)
                    .and_then(|val| {
                        jsonschema::meta::validator_for(&val).map_err(mlua::Error::external)
                    })
                    .map(|v| v.into())
            },
        )?,
    )?;

    Ok(table)
}

#[cfg(feature = "async")]
fn jsonschema_async_lua(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let table = lua.create_table()?;

    table.set(
        "validator_for",
        lua.create_async_function(
            async |lua,
                   (schema, options): (mlua::Value, Option<EncodeConfig>)|
                   -> mlua::Result<LuaValidator> {
                let schema_val = lua_to_json(&lua, schema, options)?;

                jsonschema::async_validator_for(&schema_val)
                    .await
                    .map(|v| v.into())
                    .map_err(mlua::Error::external)
            },
        )?,
    )?;

    table.set(
        "validator_map_for",
        lua.create_async_function(
            async |lua,
                   (schema, options): (mlua::Value, Option<EncodeConfig>)|
                   -> mlua::Result<LuaValidatorMap> {
                let schema_val = lua_to_json(&lua, schema, options)?;

                jsonschema::async_validator_map_for(&schema_val)
                    .await
                    .map(|v| v.into())
                    .map_err(mlua::Error::external)
            },
        )?,
    )?;

    table.set(
        "bundle",
        lua.create_async_function(
            async |lua,
                   (schema, encode, decode): (
                mlua::Value,
                Option<EncodeConfig>,
                Option<DecodeConfig>,
            )|
                   -> mlua::Result<mlua::Value> {
                let schema_val = lua_to_json(&lua, schema, encode)?;

                jsonschema::async_bundle(&schema_val)
                    .await
                    .map_err(mlua::Error::external)
                    .and_then(|bundle| lua.to_value_with(&bundle, *decode.unwrap_or_default()))
            },
        )?,
    )?;

    table.set(
        "dereference",
        lua.create_async_function(
            async |lua,
                   (schema, encode, decode): (
                mlua::Value,
                Option<EncodeConfig>,
                Option<DecodeConfig>,
            )|
                   -> mlua::Result<mlua::Value> {
                let schema_val = lua_to_json(&lua, schema, encode)?;

                jsonschema::async_dereference(&schema_val)
                    .await
                    .map_err(mlua::Error::external)
                    .and_then(|reference| {
                        lua.to_value_with(&reference, *decode.unwrap_or_default())
                    })
            },
        )?,
    )?;

    Ok(table)
}

pub(crate) fn jsonschema_lua(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let table = lua.create_table()?;

    table.set("EncodeConfig", lua.create_proxy::<EncodeConfig>()?)?;
    table.set("DecodeConfig", lua.create_proxy::<DecodeConfig>()?)?;

    table.set("meta", jsonschema_meta_lua(lua)?)?;

    #[cfg(feature = "async")]
    table.set("async", jsonschema_async_lua(lua)?)?;

    table.set(
        "is_valid",
        lua.create_function(
            |lua,
             (schema, json, options): (mlua::Value, mlua::Value, Option<EncodeConfig>)|
             -> mlua::Result<bool> {
                let schema_val = lua_to_json(lua, schema, options.clone())?;
                let json_val = lua_to_json(lua, json, options.clone())?;

                Ok(jsonschema::is_valid(&schema_val, &json_val))
            },
        )?,
    )?;

    table.set(
        "validate",
        lua.create_function(
            |lua,
             (schema, json, options): (mlua::Value, mlua::Value, Option<EncodeConfig>)|
             -> mlua::Result<(bool, Option<String>)> {
                let schema_val = lua_to_json(lua, schema, options.clone())?;
                let json_val = lua_to_json(lua, json, options.clone())?;

                let res = match jsonschema::validate(&schema_val, &json_val) {
                    Ok(_) => (true, None),
                    Err(err) => (false, Some(err.to_string())),
                };

                Ok(res)
            },
        )?,
    )?;

    table.set(
        "evaluate",
        lua.create_function(
            |lua,
             (schema, json, options): (mlua::Value, mlua::Value, Option<EncodeConfig>)|
             -> mlua::Result<LuaEvaluation> {
                let schema_val = lua_to_json(lua, schema, options.clone())?;
                let json_val = lua_to_json(lua, json, options.clone())?;

                Ok(jsonschema::evaluate(&schema_val, &json_val).into())
            },
        )?,
    )?;

    table.set(
        "validator_for",
        lua.create_function(
            |lua,
             (schema, options): (mlua::Value, Option<EncodeConfig>)|
             -> mlua::Result<LuaValidator> {
                lua_to_json(lua, schema, options)
                    .and_then(|val| jsonschema::validator_for(&val).map_err(mlua::Error::external))
                    .map(|v| v.into())
            },
        )?,
    )?;

    table.set(
        "validator_map_for",
        lua.create_function(
            |lua,
             (schema, options): (mlua::Value, Option<EncodeConfig>)|
             -> mlua::Result<LuaValidatorMap> {
                lua_to_json(lua, schema, options)
                    .and_then(|val| {
                        jsonschema::validator_map_for(&val).map_err(mlua::Error::external)
                    })
                    .map(|m| m.into())
            },
        )?,
    )?;

    table.set(
        "bundle",
        lua.create_function(
            |lua,
             (schema, encode, decode): (
                mlua::Value,
                Option<EncodeConfig>,
                Option<DecodeConfig>,
            )|
             -> mlua::Result<mlua::Value> {
                lua_to_json(lua, schema, encode)
                    .and_then(|val| jsonschema::bundle(&val).map_err(mlua::Error::external))
                    .and_then(|bundle| lua.to_value_with(&bundle, *decode.unwrap_or_default()))
            },
        )?,
    )?;

    table.set(
        "dereference",
        lua.create_function(
            |lua,
             (schema, encode, decode): (
                mlua::Value,
                Option<EncodeConfig>,
                Option<DecodeConfig>,
            )|
             -> mlua::Result<mlua::Value> {
                lua_to_json(lua, schema, encode)
                    .and_then(|val| jsonschema::dereference(&val).map_err(mlua::Error::external))
                    .and_then(|der| lua.to_value_with(&der, *decode.unwrap_or_default()))
            },
        )?,
    )?;

    Ok(table)
}
