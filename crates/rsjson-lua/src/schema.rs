use mlua::LuaSerdeExt;

pub struct LuaEvaluation(jsonschema::Evaluation);

impl LuaEvaluation {
    pub fn new(evaluation: jsonschema::Evaluation) -> Self {
        Self(evaluation)
    }
}

impl mlua::UserData for LuaEvaluation {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method(
            "flag",
            |lua, this, _: mlua::Value| -> mlua::Result<mlua::Value> {
                lua.to_value(&this.0.flag())
            },
        );

        methods.add_method(
            "list",
            |lua, this, _: mlua::Value| -> mlua::Result<mlua::Value> {
                lua.to_value(&this.0.list())
            },
        );

        methods.add_method(
            "hierarchical",
            |lua, this, _: mlua::Value| -> mlua::Result<mlua::Value> {
                lua.to_value(&this.0.hierarchical())
            },
        );
    }
}

pub struct LuaValidator(jsonschema::Validator);

impl LuaValidator {
    pub fn new(validator: jsonschema::Validator) -> Self {
        Self(validator)
    }
}

impl mlua::UserData for LuaValidator {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("validate", |_, this, json: String| -> mlua::Result<()> {
            let json_val: serde_json::Value =
                serde_json::from_str(&json).map_err(mlua::Error::external)?;

            this.0
                .validate(&json_val)
                .map_err(|err| mlua::Error::external(err.to_owned()))
        });

        methods.add_method("is_valid", |_, this, json: String| -> mlua::Result<bool> {
            let json_val: serde_json::Value =
                serde_json::from_str(&json).map_err(mlua::Error::external)?;

            Ok(this.0.is_valid(&json_val))
        });

        methods.add_method(
            "evaluate",
            |_, this, json: String| -> mlua::Result<LuaEvaluation> {
                let json_val: serde_json::Value =
                    serde_json::from_str(&json).map_err(mlua::Error::external)?;

                let evaluation = this.0.evaluate(&json_val);

                Ok(LuaEvaluation::new(evaluation))
            },
        );

        methods.add_method("draft", |_, this, _: mlua::Value| -> mlua::Result<String> {
            let draft = match this.0.draft() {
                jsonschema::Draft::Draft201909 => "Draft201909",
                jsonschema::Draft::Draft202012 => "Draft202012",
                jsonschema::Draft::Draft4 => "Draft4",
                jsonschema::Draft::Draft6 => "Draft6",
                jsonschema::Draft::Draft7 => "Draft7",
                _ => "unknown",
            };

            Ok(draft.to_string())
        });
    }
}

pub fn jsonschema_lua(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let table = lua.create_table()?;

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

    // table.set("validator_map_for", lua.create_function(|| {}))?;
    // table.set("bundle", lua.create_function(|| {}))?;
    // table.set("dereference", lua.create_function(|| {}))?;

    Ok(table)
}

// pub fn jsonschema_meta_lua(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
//     let table = lua.create_table()?;

//     table.set("is_valid", lua.create_function(|| {}))?;
//     table.set("validate", lua.create_function(|| {}))?;
//     table.set("validator_for", lua.create_function(|| {}))?;

//     Ok(table)
// }
