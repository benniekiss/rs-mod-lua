use mlua::LuaSerdeExt;

pub(crate) struct LuaAnnotationEntry {
    schema_location: String,
    absolute_keyword_location: Option<jsonschema::Uri<String>>,
    instance_location: jsonschema::paths::Location,
    annotations: jsonschema::output::Annotations,
}

impl LuaAnnotationEntry {
    pub(crate) fn from(annotation: jsonschema::AnnotationEntry) -> Self {
        Self {
            schema_location: annotation.schema_location.to_string(),
            absolute_keyword_location: annotation.absolute_keyword_location.cloned(),
            instance_location: annotation.instance_location.clone(),
            annotations: annotation.annotations.clone(),
        }
    }
}

impl mlua::IntoLua for LuaAnnotationEntry {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        let table = lua.create_table()?;

        table.set("schema_location", self.schema_location)?;
        table.set(
            "absolute_keyword_location",
            self.absolute_keyword_location
                .map(|v| lua.to_value(&v).ok()),
        )?;
        table.set("instance_location", lua.to_value(&self.instance_location)?)?;
        table.set("annotations", lua.to_value(&self.annotations)?)?;

        Ok(mlua::Value::Table(table))
    }
}

pub(crate) struct LuaErrorEntry {
    schema_location: String,
    absolute_keyword_location: Option<jsonschema::Uri<String>>,
    instance_location: jsonschema::paths::Location,
    error: jsonschema::output::ErrorDescription,
}

impl LuaErrorEntry {
    pub(crate) fn from(error: jsonschema::ErrorEntry) -> Self {
        Self {
            schema_location: error.schema_location.to_string(),
            absolute_keyword_location: error.absolute_keyword_location.cloned(),
            instance_location: error.instance_location.clone(),
            error: error.error.clone(),
        }
    }
}

impl mlua::IntoLua for LuaErrorEntry {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        let table = lua.create_table()?;

        table.set("schema_location", self.schema_location)?;
        table.set(
            "absolute_keyword_location",
            self.absolute_keyword_location
                .map(|v| lua.to_value(&v).ok()),
        )?;
        table.set("instance_location", lua.to_value(&self.instance_location)?)?;
        table.set("error", self.error.into_inner())?;

        Ok(mlua::Value::Table(table))
    }
}

pub(crate) struct LuaEvaluation(jsonschema::Evaluation);

impl LuaEvaluation {
    pub(crate) fn new(evaluation: jsonschema::Evaluation) -> Self {
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

        methods.add_method(
            "annotations",
            |_, this: &LuaEvaluation, _: mlua::Value| -> mlua::Result<Vec<LuaAnnotationEntry>> {
                let annt = this
                    .0
                    .iter_annotations()
                    .map(LuaAnnotationEntry::from)
                    .collect::<Vec<_>>();

                Ok(annt)
            },
        );

        methods.add_method(
            "errors",
            |_, this: &LuaEvaluation, _: mlua::Value| -> mlua::Result<Vec<LuaErrorEntry>> {
                let errs = this
                    .0
                    .iter_errors()
                    .map(LuaErrorEntry::from)
                    .collect::<Vec<_>>();

                Ok(errs)
            },
        );
    }
}

pub(crate) struct LuaValidator(jsonschema::Validator);

impl LuaValidator {
    pub(crate) fn new(validator: jsonschema::Validator) -> Self {
        Self(validator)
    }
}

impl mlua::UserData for LuaValidator {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("is_valid", |_, this, json: String| -> mlua::Result<bool> {
            let json_val: serde_json::Value =
                serde_json::from_str(&json).map_err(mlua::Error::external)?;

            Ok(this.0.is_valid(&json_val))
        });

        methods.add_method("validate", |_, this, json: String| -> mlua::Result<()> {
            let json_val: serde_json::Value =
                serde_json::from_str(&json).map_err(mlua::Error::external)?;

            this.0
                .validate(&json_val)
                .map_err(|err| mlua::Error::external(err.to_owned()))
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

        methods.add_method(
            "errors",
            |_, this, json: String| -> mlua::Result<Vec<mlua::Error>> {
                let json_val: serde_json::Value =
                    serde_json::from_str(&json).map_err(mlua::Error::external)?;

                let errs = this
                    .0
                    .iter_errors(&json_val)
                    .into_errors()
                    .into_iter()
                    .map(|err| mlua::Error::external(err.to_owned()))
                    .collect::<Vec<_>>();

                Ok(errs)
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

pub(crate) struct LuaValidatorMap(jsonschema::ValidatorMap);

impl LuaValidatorMap {
    pub(crate) fn new(map: jsonschema::ValidatorMap) -> Self {
        Self(map)
    }
}

impl mlua::UserData for LuaValidatorMap {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method(
            "get",
            |_, this, pointer: String| -> mlua::Result<Option<LuaValidator>> {
                Ok(this.0.get(&pointer).cloned().map(LuaValidator::new))
            },
        );

        methods.add_method(
            "contains_key",
            |_, this, pointer: String| -> mlua::Result<bool> { Ok(this.0.contains_key(&pointer)) },
        );

        methods.add_method(
            "keys",
            |_, this, _: mlua::Value| -> mlua::Result<Vec<String>> {
                Ok(this.0.keys().map(|s| s.to_string()).collect())
            },
        );
    }
}
