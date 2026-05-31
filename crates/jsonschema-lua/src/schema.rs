use mlua::LuaSerdeExt;

use crate::lua::*;

fn jsonschema_meta_lua(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let table = lua.create_table()?;

    table.set(
        "is_valid",
        lua.create_function(|_, schema: String| -> mlua::Result<bool> {
            let schema_val: serde_json::Value =
                serde_json::from_str(&schema).map_err(mlua::Error::external)?;

            Ok(jsonschema::meta::is_valid(&schema_val))
        })?,
    )?;

    table.set(
        "validate",
        lua.create_function(|_, schema: String| -> mlua::Result<()> {
            let schema_val: serde_json::Value =
                serde_json::from_str(&schema).map_err(mlua::Error::external)?;

            jsonschema::meta::validate(&schema_val)
                .map_err(|err| mlua::Error::external(err.to_owned()))
        })?,
    )?;

    table.set(
        "validator_for",
        lua.create_function(|_, schema: String| -> mlua::Result<LuaValidator> {
            let schema_val: serde_json::Value =
                serde_json::from_str(&schema).map_err(mlua::Error::external)?;

            let validator =
                jsonschema::meta::validator_for(&schema_val).map_err(mlua::Error::external)?;

            Ok(LuaValidator::new(validator.as_ref().clone()))
        })?,
    )?;

    Ok(table)
}

#[cfg(feature = "async")]
fn jsonschema_async_lua(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let table = lua.create_table()?;

    table.set(
        "validator_for",
        lua.create_async_function(async |_, schema: String| -> mlua::Result<LuaValidator> {
            let schema_val: serde_json::Value =
                serde_json::from_str(&schema).map_err(mlua::Error::external)?;

            let validator = jsonschema::async_validator_for(&schema_val)
                .await
                .map_err(mlua::Error::external)?;

            Ok(LuaValidator::new(validator))
        })?,
    )?;

    table.set(
        "validator_map_for",
        lua.create_async_function(async |_, schema: String| -> mlua::Result<LuaValidatorMap> {
            let schema_val: serde_json::Value =
                serde_json::from_str(&schema).map_err(mlua::Error::external)?;

            let validator_map = jsonschema::async_validator_map_for(&schema_val)
                .await
                .map_err(mlua::Error::external)?;

            Ok(LuaValidatorMap::new(validator_map))
        })?,
    )?;

    table.set(
        "bundle",
        lua.create_async_function(async |lua, schema: String| -> mlua::Result<mlua::Value> {
            let schema_val: serde_json::Value =
                serde_json::from_str(&schema).map_err(mlua::Error::external)?;

            let bundle = jsonschema::async_bundle(&schema_val)
                .await
                .map_err(mlua::Error::external)?;

            lua.to_value(&bundle)
        })?,
    )?;

    table.set(
        "dereference",
        lua.create_async_function(async |lua, schema: String| -> mlua::Result<mlua::Value> {
            let schema_val: serde_json::Value =
                serde_json::from_str(&schema).map_err(mlua::Error::external)?;

            let reference = jsonschema::async_dereference(&schema_val)
                .await
                .map_err(mlua::Error::external)?;

            lua.to_value(&reference)
        })?,
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
            |_, (schema, json): (String, String)| -> mlua::Result<bool> {
                let schema_val: serde_json::Value =
                    serde_json::from_str(&schema).map_err(mlua::Error::external)?;

                let json_val: serde_json::Value =
                    serde_json::from_str(&json).map_err(mlua::Error::external)?;

                Ok(jsonschema::is_valid(&schema_val, &json_val))
            },
        )?,
    )?;

    table.set(
        "validate",
        lua.create_function(|_, (schema, json): (String, String)| -> mlua::Result<()> {
            let schema_val: serde_json::Value =
                serde_json::from_str(&schema).map_err(mlua::Error::external)?;

            let json_val: serde_json::Value =
                serde_json::from_str(&json).map_err(mlua::Error::external)?;

            jsonschema::validate(&schema_val, &json_val)
                .map_err(|err| mlua::Error::external(err.to_owned()))
        })?,
    )?;

    table.set(
        "evaluate",
        lua.create_function(
            |_, (schema, json): (String, String)| -> mlua::Result<LuaEvaluation> {
                let schema_val: serde_json::Value =
                    serde_json::from_str(&schema).map_err(mlua::Error::external)?;

                let json_val: serde_json::Value =
                    serde_json::from_str(&json).map_err(mlua::Error::external)?;

                let evaluation: jsonschema::Evaluation =
                    jsonschema::evaluate(&schema_val, &json_val);

                Ok(LuaEvaluation::new(evaluation))
            },
        )?,
    )?;

    table.set(
        "validator_for",
        lua.create_function(|_, schema: String| -> mlua::Result<LuaValidator> {
            let schema_val: serde_json::Value =
                serde_json::from_str(&schema).map_err(mlua::Error::external)?;

            let validator =
                jsonschema::validator_for(&schema_val).map_err(mlua::Error::external)?;

            Ok(LuaValidator::new(validator))
        })?,
    )?;

    table.set(
        "validator_map_for",
        lua.create_function(|_, schema: String| -> mlua::Result<LuaValidatorMap> {
            let schema_val: serde_json::Value =
                serde_json::from_str(&schema).map_err(mlua::Error::external)?;

            let validator_map =
                jsonschema::validator_map_for(&schema_val).map_err(mlua::Error::external)?;

            Ok(LuaValidatorMap::new(validator_map))
        })?,
    )?;

    table.set(
        "bundle",
        lua.create_function(|lua, schema: String| -> mlua::Result<mlua::Value> {
            let schema_val: serde_json::Value =
                serde_json::from_str(&schema).map_err(mlua::Error::external)?;

            let bundle = jsonschema::bundle(&schema_val).map_err(mlua::Error::external)?;

            lua.to_value(&bundle)
        })?,
    )?;

    table.set(
        "dereference",
        lua.create_function(|lua, schema: String| -> mlua::Result<mlua::Value> {
            let schema_val: serde_json::Value =
                serde_json::from_str(&schema).map_err(mlua::Error::external)?;

            let reference = jsonschema::dereference(&schema_val).map_err(mlua::Error::external)?;

            lua.to_value(&reference)
        })?,
    )?;

    Ok(table)
}
