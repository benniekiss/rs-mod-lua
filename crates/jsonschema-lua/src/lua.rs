use mlua::{IntoLua, LuaSerdeExt};
use rs_mod_lua_core::config::{DecodeConfig, EncodeConfig};

pub(crate) fn lua_to_json(
    lua: &mlua::Lua,
    value: mlua::Value,
    options: Option<EncodeConfig>,
) -> mlua::Result<serde_json::Value> {
    match value.as_string() {
        Some(s) => serde_json::from_str(&s.to_string_lossy()).map_err(mlua::Error::external),
        None => lua.from_value_with(value, *options.unwrap_or_default()),
    }
}

pub(crate) fn json_to_lua(
    lua: &mlua::Lua,
    value: serde_json::Value,
    options: Option<DecodeConfig>,
    as_str: bool,
) -> mlua::Result<mlua::Value> {
    match as_str {
        true => serde_json::to_string(&value)
            .map_err(mlua::Error::external)
            .and_then(|s| s.into_lua(lua)),
        false => lua.to_value_with(&value, *options.unwrap_or_default()),
    }
}

#[cfg(test)]
mod tests {
    use mlua::{Lua, Value};
    use serde_json::json;

    use super::{json_to_lua, lua_to_json};

    #[test]
    fn test_lua_to_json_strings_as_json() {
        let lua = Lua::new();
        for (input, expected) in [
            (r#""foo""#, json!("foo")),
            ("42", json!(42)),
            ("2.5", json!(2.5)),
            ("true", json!(true)),
            ("false", json!(false)),
            ("null", json!(null)),
            ("[]", json!([])),
            ("{}", json!({})),
            (
                r#"{"items":[1,null,{"name":"foo"}]}"#,
                json!({"items": [1, null, {"name": "foo"}]}),
            ),
        ] {
            let value = Value::String(lua.create_string(input).unwrap());
            assert_eq!(lua_to_json(&lua, value, None).unwrap(), expected, "{input}");
        }
    }

    #[test]
    fn test_lua_to_json_invalid_json_strings() {
        let lua = Lua::new();
        for input in ["foo", "", "{", "[1,]", "true false"] {
            let value = Value::String(lua.create_string(input).unwrap());
            let error = lua_to_json(&lua, value, None).unwrap_err();
            assert!(
                matches!(error, mlua::Error::ExternalError(_)),
                "{input}: {error}"
            );
        }
    }

    #[test]
    fn test_lua_to_json_native_values() {
        let lua = Lua::new();
        for (value, expected) in [
            (Value::Nil, json!(null)),
            (Value::NULL, json!(null)),
            (Value::Boolean(true), json!(true)),
            (Value::Boolean(false), json!(false)),
            (Value::Integer(42), json!(42)),
            (Value::Number(2.5), json!(2.5)),
        ] {
            assert_eq!(lua_to_json(&lua, value, None).unwrap(), expected);
        }
    }

    #[test]
    fn test_lua_to_json_nested_tables() {
        let lua = Lua::new();
        let value = lua
            .load(r#"return {items = {1, {name = "foo"}}, enabled = true}"#)
            .eval()
            .unwrap();
        assert_eq!(
            lua_to_json(&lua, value, None).unwrap(),
            json!({"items": [1, {"name": "foo"}], "enabled": true})
        )
    }

    #[test]
    fn test_lua_to_json_unsupported_values() {
        let lua = Lua::new();
        let value = Value::Function(lua.create_function(|_, ()| Ok(())).unwrap());
        assert!(lua_to_json(&lua, value, None).is_err());
    }

    #[test]
    fn test_json_to_lua_native_values() {
        let lua = Lua::new();
        for (value, expected) in [
            (json!(null), Value::NULL),
            (json!(true), Value::Boolean(true)),
            (json!(false), Value::Boolean(false)),
            (json!(42), Value::Integer(42)),
            (json!(2.5), Value::Number(2.5)),
            (
                json!("foo"),
                Value::String(lua.create_string("foo").unwrap()),
            ),
        ] {
            assert_eq!(json_to_lua(&lua, value, None, false).unwrap(), expected);
        }
    }

    #[test]
    fn test_json_to_lua_nested_tables() {
        let lua = Lua::new();
        let value = json_to_lua(
            &lua,
            json!({"items": [1, null, {"name": "foo"}]}),
            None,
            false,
        )
        .unwrap();
        let table = value.as_table().unwrap();
        let items = table.get::<mlua::Table>("items").unwrap();
        assert_eq!(items.raw_len(), 3);
        assert_eq!(items.get::<i64>(1).unwrap(), 1);
        assert_eq!(items.get::<Value>(2).unwrap(), Value::NULL);
        assert_eq!(
            items
                .get::<mlua::Table>(3)
                .unwrap()
                .get::<String>("name")
                .unwrap(),
            "foo"
        );
    }

    #[test]
    fn json_to_lua_strings() {
        let lua = Lua::new();
        for (value, expected) in [
            (json!(null), "null"),
            (json!(true), "true"),
            (json!(42), "42"),
            (json!(2.5), "2.5"),
            (json!("foo"), r#""foo""#),
            (json!("line\n\"quoted\""), r#""line\n\"quoted\"""#),
            (json!([]), "[]"),
            (json!({}), "{}"),
            (json!([1, 2]), "[1,2]"),
            (json!({"items": [1, null]}), r#"{"items":[1,null]}"#),
        ] {
            let result = json_to_lua(&lua, value, None, true).unwrap();
            assert_eq!(result, Value::String(lua.create_string(expected).unwrap()));
        }
    }
}
