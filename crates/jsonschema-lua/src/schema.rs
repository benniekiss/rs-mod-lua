use mlua::LuaSerdeExt;
use rsjson_lua::config::EncodeConfig;

use crate::lua::*;

fn jsonschema_meta_lua(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let table = lua.create_table()?;

    table.set(
        "is_valid",
        lua.create_function(
            |lua, (schema, options): (mlua::Value, Option<EncodeConfig>)| -> mlua::Result<bool> {
                let schema_val = lua.from_value_with(schema, *options.unwrap_or_default())?;

                Ok(jsonschema::meta::is_valid(&schema_val))
            },
        )?,
    )?;

    table.set(
        "validate",
        lua.create_function(
            |lua, (schema, options): (mlua::Value, Option<EncodeConfig>)| -> mlua::Result<()> {
                let schema_val = lua.from_value_with(schema, *options.unwrap_or_default())?;

                jsonschema::meta::validate(&schema_val)
                    .map_err(|err| mlua::Error::external(err.to_owned()))
            },
        )?,
    )?;

    table.set(
        "validator_for",
        lua.create_function(
            |lua,
             (schema, options): (mlua::Value, Option<EncodeConfig>)|
             -> mlua::Result<LuaValidator> {
                let schema_val = lua.from_value_with(schema, *options.unwrap_or_default())?;

                jsonschema::meta::validator_for(&schema_val)
                    .map(|v| v.into())
                    .map_err(mlua::Error::external)
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
                let schema_val = lua.from_value_with(schema, *options.unwrap_or_default())?;

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
                let schema_val = lua.from_value_with(schema, *options.unwrap_or_default())?;

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
                   (schema, options): (mlua::Value, Option<EncodeConfig>)|
                   -> mlua::Result<mlua::Value> {
                let schema_val = lua.from_value_with(schema, *options.unwrap_or_default())?;

                jsonschema::async_bundle(&schema_val)
                    .await
                    .map_err(mlua::Error::external)
                    .and_then(|bundle| lua.to_value(&bundle))
            },
        )?,
    )?;

    table.set(
        "dereference",
        lua.create_async_function(
            async |lua,
                   (schema, options): (mlua::Value, Option<EncodeConfig>)|
                   -> mlua::Result<mlua::Value> {
                let schema_val = lua.from_value_with(schema, *options.unwrap_or_default())?;

                jsonschema::async_dereference(&schema_val)
                    .await
                    .map_err(mlua::Error::external)
                    .and_then(|reference| lua.to_value(&reference))
            },
        )?,
    )?;

    Ok(table)
}

pub(crate) fn jsonschema_lua(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let table = lua.create_table()?;

    table.set("meta", jsonschema_meta_lua(lua)?)?;

    #[cfg(feature = "async")]
    table.set("async", jsonschema_async_lua(lua)?)?;

    table.set(
        "is_valid",
        lua.create_function(
            |lua,
             (schema, json, options): (mlua::Value, mlua::Value, Option<EncodeConfig>)|
             -> mlua::Result<bool> {
                let options = options.unwrap_or_default();
                let schema_val = lua.from_value_with(schema, *options)?;
                let json_val = lua.from_value_with(json, *options)?;

                Ok(jsonschema::is_valid(&schema_val, &json_val))
            },
        )?,
    )?;

    table.set(
        "validate",
        lua.create_function(
            |lua,
             (schema, json, options): (mlua::Value, mlua::Value, Option<EncodeConfig>)|
             -> mlua::Result<()> {
                let options = options.unwrap_or_default();
                let schema_val = lua.from_value_with(schema, *options)?;
                let json_val = lua.from_value_with(json, *options)?;

                jsonschema::validate(&schema_val, &json_val)
                    .map_err(|err| mlua::Error::external(err.to_owned()))
            },
        )?,
    )?;

    table.set(
        "evaluate",
        lua.create_function(
            |lua,
             (schema, json, options): (mlua::Value, mlua::Value, Option<EncodeConfig>)|
             -> mlua::Result<LuaEvaluation> {
                let options = options.unwrap_or_default();
                let schema_val = lua.from_value_with(schema, *options)?;
                let json_val = lua.from_value_with(json, *options)?;

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
                let schema_val = lua.from_value_with(schema, *options.unwrap_or_default())?;

                jsonschema::validator_for(&schema_val)
                    .map(|v| v.into())
                    .map_err(mlua::Error::external)
            },
        )?,
    )?;

    table.set(
        "validator_map_for",
        lua.create_function(
            |lua,
             (schema, options): (mlua::Value, Option<EncodeConfig>)|
             -> mlua::Result<LuaValidatorMap> {
                let schema_val = lua.from_value_with(schema, *options.unwrap_or_default())?;

                jsonschema::validator_map_for(&schema_val)
                    .map(|v| v.into())
                    .map_err(mlua::Error::external)
            },
        )?,
    )?;

    table.set(
        "bundle",
        lua.create_function(
            |lua,
             (schema, options): (mlua::Value, Option<EncodeConfig>)|
             -> mlua::Result<mlua::Value> {
                let schema_val = lua.from_value_with(schema, *options.unwrap_or_default())?;

                jsonschema::bundle(&schema_val)
                    .map_err(mlua::Error::external)
                    .and_then(|bundle| lua.to_value(&bundle))
            },
        )?,
    )?;

    table.set(
        "dereference",
        lua.create_function(
            |lua,
             (schema, options): (mlua::Value, Option<EncodeConfig>)|
             -> mlua::Result<mlua::Value> {
                let schema_val = lua.from_value_with(schema, *options.unwrap_or_default())?;

                jsonschema::dereference(&schema_val)
                    .map_err(mlua::Error::external)
                    .and_then(|reference| lua.to_value(&reference))
            },
        )?,
    )?;

    Ok(table)
}
