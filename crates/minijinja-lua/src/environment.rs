// SPDX-License-Identifier: MIT

use std::{
    borrow::Cow,
    fmt,
    sync::{
        RwLock,
        RwLockReadGuard,
        RwLockWriteGuard,
        atomic::{AtomicBool, Ordering},
    },
};

use minijinja::{
    Environment,
    Error as JinjaError,
    ErrorKind as JinjaErrorKind,
    State,
    args,
    value::{Rest as JinjaRest, Value as JinjaValue},
};
use mlua::LuaSerdeExt;

use crate::{
    convert::{
        LuaFunctionObject,
        LuaObject,
        lua_to_auto_escape,
        lua_to_minijinja,
        lua_to_syntax_config,
        lua_to_undefined_behavior,
        minijinja_to_lua,
        undefined_behavior_to_lua,
    },
    state::bind_lua,
};

/// A wrapper around a [`minijinja::Environment`]. This wrapper can be serialized into
/// an [`mlua::UserData`] object for use within mlua::Lua.
#[derive(Debug)]
pub struct LuaEnvironment {
    env: RwLock<Environment<'static>>,
    reload_before_render: AtomicBool,
}

impl LuaEnvironment {
    /// Get a new environment
    pub(crate) fn new() -> Self {
        let mut env = Environment::new();

        #[cfg(feature = "minijinja-contrib")]
        minijinja_contrib::add_to_environment(&mut env);

        #[cfg(feature = "json")]
        crate::contrib::json::add_to_environment(&mut env);

        #[cfg(feature = "datetime")]
        crate::contrib::datetime::add_to_environment(&mut env);

        Self {
            env: RwLock::new(env),
            reload_before_render: AtomicBool::new(false),
        }
    }

    /// Get a new empty environment
    pub(crate) fn empty() -> Self {
        let env = Environment::empty();

        Self {
            env: RwLock::new(env),
            reload_before_render: AtomicBool::new(false),
        }
    }

    pub(crate) fn reload_before_render(&self) -> bool {
        self.reload_before_render.load(Ordering::Relaxed)
    }

    pub(crate) fn set_reload_before_render(&self, enable: bool) {
        self.reload_before_render.store(enable, Ordering::Relaxed);
    }

    /// Get a read-only lock on the underlying `minijinja::Environment`
    pub(crate) fn read_env(
        &self,
    ) -> Result<RwLockReadGuard<'_, Environment<'static>>, mlua::Error> {
        self.env
            .read()
            .map_err(|_| mlua::Error::runtime("environment lock poisoned"))
    }

    /// Get a write lock on the underlying [`minijinja::Environment`]
    pub(crate) fn write_env(
        &self,
    ) -> Result<RwLockWriteGuard<'_, Environment<'static>>, mlua::Error> {
        self.env
            .write()
            .map_err(|_| mlua::Error::runtime("environment lock poisoned"))
    }
}

impl Default for LuaEnvironment {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for LuaEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Environment")
    }
}

impl mlua::UserData for LuaEnvironment {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get(
            "reload_before_render",
            |_, this: &LuaEnvironment| -> Result<bool, mlua::Error> {
                Ok(this.reload_before_render())
            },
        );

        fields.add_field_method_set(
            "reload_before_render",
            |_, this: &mut LuaEnvironment, val: bool| -> Result<(), mlua::Error> {
                this.set_reload_before_render(val);

                Ok(())
            },
        );

        fields.add_field_method_get(
            "keep_trailing_newline",
            |_, this: &LuaEnvironment| -> Result<bool, mlua::Error> {
                Ok(this.read_env()?.keep_trailing_newline())
            },
        );

        fields.add_field_method_set(
            "keep_trailing_newline",
            |_, this: &mut LuaEnvironment, val: bool| -> Result<(), mlua::Error> {
                this.write_env()?.set_keep_trailing_newline(val);

                Ok(())
            },
        );

        fields.add_field_method_get(
            "trim_blocks",
            |_, this: &LuaEnvironment| -> Result<bool, mlua::Error> {
                Ok(this.read_env()?.trim_blocks())
            },
        );

        fields.add_field_method_set(
            "trim_blocks",
            |_, this: &mut LuaEnvironment, val: bool| -> Result<(), mlua::Error> {
                this.write_env()?.set_trim_blocks(val);

                Ok(())
            },
        );

        fields.add_field_method_get(
            "lstrip_blocks",
            |_, this: &LuaEnvironment| -> Result<bool, mlua::Error> {
                Ok(this.read_env()?.lstrip_blocks())
            },
        );

        fields.add_field_method_set(
            "lstrip_blocks",
            |_, this: &mut LuaEnvironment, val: bool| -> Result<(), mlua::Error> {
                this.write_env()?.set_lstrip_blocks(val);

                Ok(())
            },
        );

        fields.add_field_method_get(
            "debug",
            |_, this: &LuaEnvironment| -> Result<bool, mlua::Error> {
                Ok(this.read_env()?.debug())
            },
        );

        fields.add_field_method_set(
            "debug",
            |_, this: &mut LuaEnvironment, val: bool| -> Result<(), mlua::Error> {
                this.write_env()?.set_debug(val);

                Ok(())
            },
        );

        fields.add_field_method_get(
            "fuel",
            |_, this: &LuaEnvironment| -> Result<Option<u64>, mlua::Error> {
                Ok(this.read_env()?.fuel())
            },
        );

        fields.add_field_method_set(
            "fuel",
            |_, this: &mut LuaEnvironment, val: Option<u64>| -> Result<(), mlua::Error> {
                this.write_env()?.set_fuel(val);

                Ok(())
            },
        );

        fields.add_field_method_get(
            "recursion_limit",
            |_, this: &LuaEnvironment| -> Result<usize, mlua::Error> {
                Ok(this.read_env()?.recursion_limit())
            },
        );

        fields.add_field_method_set(
            "recursion_limit",
            |_, this: &mut LuaEnvironment, val: usize| -> Result<(), mlua::Error> {
                this.write_env()?.set_recursion_limit(val);

                Ok(())
            },
        );

        fields.add_field_method_get(
            "undefined_behavior",
            |_, this: &LuaEnvironment| -> Result<Option<String>, mlua::Error> {
                let ub = this.read_env()?.undefined_behavior();

                Ok(undefined_behavior_to_lua(ub))
            },
        );

        fields.add_field_method_set(
            "undefined_behavior",
            |_, this: &mut LuaEnvironment, val: String| -> Result<(), mlua::Error> {
                let behavior = lua_to_undefined_behavior(&val)?;

                this.write_env()?.set_undefined_behavior(behavior);

                Ok(())
            },
        );
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function(
            "new",
            |_, _: mlua::MultiValue| -> Result<LuaEnvironment, _> { Ok(LuaEnvironment::new()) },
        );

        methods.add_function(
            "empty",
            |_, _: mlua::MultiValue| -> Result<LuaEnvironment, _> { Ok(LuaEnvironment::empty()) },
        );

        methods.add_method(
            "add_template",
            |lua: &mlua::Lua,
             this: &LuaEnvironment,
             (name, source): (String, String)|
             -> Result<(), mlua::Error> {
                bind_lua(lua, || {
                    this.write_env()?
                        .add_template_owned(name, source)
                        .map_err(mlua::Error::external)
                })
            },
        );

        methods.add_method(
            "remove_template",
            |lua: &mlua::Lua, this: &LuaEnvironment, name: String| -> Result<(), mlua::Error> {
                bind_lua(lua, || {
                    this.write_env()?.remove_template(&name);
                    Ok(())
                })
            },
        );

        methods.add_method(
            "clear_templates",
            |lua: &mlua::Lua, this: &LuaEnvironment, _: mlua::Value| -> Result<(), mlua::Error> {
                bind_lua(lua, || {
                    this.write_env()?.clear_templates();

                    Ok(())
                })
            },
        );

        methods.add_method(
            "undeclared_variables",
            |lua: &mlua::Lua,
             this: &LuaEnvironment,
             (name, nested): (String, Option<bool>)|
             -> Result<mlua::Value, mlua::Error> {
                bind_lua(lua, || {
                    if this.reload_before_render() {
                        this.write_env()?.clear_templates();
                    }

                    let nested = nested.unwrap_or(false);

                    let vars = this
                        .read_env()?
                        .get_template(&name)
                        .map_err(mlua::Error::external)?
                        .undeclared_variables(nested);

                    lua.to_value(&vars)
                })
            },
        );

        methods.add_method(
            "set_loader",
            |lua: &mlua::Lua,
             this: &LuaEnvironment,
             callback: mlua::Function|
             -> Result<(), mlua::Error> {
                let key = lua.create_registry_value(callback)?;
                let func = LuaFunctionObject::new(key);

                this.write_env()?.set_loader(move |name| {
                    let source = func.with_func(args!(name), None)?;
                    Ok(source.and_then(|v| {
                        // If the lua function returns nil, i.e., no path found
                        // it is mapped as `minijinja::value::ValueKind::Undefined`, however
                        // we need to return a `None` to indicate no path was found.
                        if v.is_undefined() {
                            None
                        } else {
                            Some(v.to_string())
                        }
                    }))
                });

                Ok(())
            },
        );

        methods.add_method(
            "set_path_join_callback",
            |lua: &mlua::Lua,
             this: &LuaEnvironment,
             callback: mlua::Function|
             -> Result<(), mlua::Error> {
                let key = lua.create_registry_value(callback)?;
                let func = LuaFunctionObject::new(key);

                this.write_env()?
                    .set_path_join_callback(move |name, parent| {
                        func.with_func(args!(name, parent), None)
                            .ok()
                            .flatten()
                            .and_then(|v| v.as_str().map(|s| Cow::Owned(s.to_string())))
                            .unwrap_or(Cow::Borrowed(name))
                    });
                Ok(())
            },
        );

        methods.add_method(
            "set_unknown_method_callback",
            |lua: &mlua::Lua,
             this: &LuaEnvironment,
             callback: mlua::Function|
             -> Result<(), mlua::Error> {
                let key = lua.create_registry_value(callback)?;
                let mut func = LuaFunctionObject::new(key);
                func.set_pass_state(true);

                this.write_env()?
                    .set_unknown_method_callback(move |state, value, method, args| {
                        func.with_func(args!(value, method, ..args), Some(state))
                            .map(|v| v.unwrap_or(JinjaValue::UNDEFINED))
                    });

                Ok(())
            },
        );

        methods.add_method(
            "set_pycompat",
            |_, this: &LuaEnvironment, enable: Option<bool>| {
                match enable {
                    Some(true) | None => {
                        this.write_env()?.set_unknown_method_callback(
                            minijinja_contrib::pycompat::unknown_method_callback,
                        );
                    },
                    Some(false) => {
                        this.write_env()?.set_unknown_method_callback(|_, _, _, _| {
                            Err(JinjaError::from(JinjaErrorKind::UnknownMethod))
                        });
                    },
                }

                Ok(())
            },
        );

        methods.add_method(
            "set_auto_escape_callback",
            |lua: &mlua::Lua,
             this: &LuaEnvironment,
             callback: mlua::Function|
             -> Result<(), mlua::Error> {
                let key = lua.create_registry_value(callback)?;
                let func = LuaFunctionObject::new(key);

                this.write_env()?.set_auto_escape_callback(move |name| {
                    func.with_func(args!(name), None)
                        .ok()
                        .flatten()
                        .and_then(|v| {
                            let s = v.as_str()?.to_string();
                            lua_to_auto_escape(&s).ok()
                        })
                        .unwrap_or(minijinja::AutoEscape::None)
                });
                Ok(())
            },
        );

        methods.add_method(
            "set_formatter",
            |lua: &mlua::Lua,
             this: &LuaEnvironment,
             callback: mlua::Function|
             -> Result<(), mlua::Error> {
                let key = lua.create_registry_value(callback)?;
                let mut func = LuaFunctionObject::new(key);
                func.set_pass_state(true);

                this.write_env()?.set_formatter(move |out, state, value| {
                    let Some(val) = func.with_func(args!(value), Some(state)).ok().flatten() else {
                        return Ok(());
                    };

                    let Some(s) = val.as_str() else {
                        return Err(JinjaError::new(
                            JinjaErrorKind::WriteFailure,
                            "formatter must return a string",
                        ));
                    };

                    out.write_str(s)
                        .map_err(|_| JinjaError::new(JinjaErrorKind::WriteFailure, "write failed"))
                });

                Ok(())
            },
        );

        methods.add_method(
            "set_syntax",
            |_, this: &LuaEnvironment, syntax: mlua::Table| -> Result<(), mlua::Error> {
                let config = lua_to_syntax_config(syntax).map_err(mlua::Error::external)?;
                this.write_env()?.set_syntax(config);

                Ok(())
            },
        );

        methods.add_method(
            "render_template",
            |lua: &mlua::Lua,
             this: &LuaEnvironment,
             (name, ctx): (String, Option<mlua::Table>)|
             -> Result<String, mlua::Error> {
                if this.reload_before_render() {
                    this.write_env()?.clear_templates();
                }

                let ctx = ctx.unwrap_or(lua.create_table()?);

                let context = lua_to_minijinja(lua, &mlua::Value::Table(ctx));

                bind_lua(lua, || {
                    this.read_env()?
                        .get_template(&name)
                        .map_err(mlua::Error::external)?
                        .render(context)
                        .map_err(mlua::Error::external)
                })
            },
        );

        methods.add_method(
            "render_str",
            |lua: &mlua::Lua,
             this: &LuaEnvironment,
             (source, ctx, name): (String, Option<mlua::Table>, Option<String>)|
             -> Result<String, mlua::Error> {
                let ctx = ctx.unwrap_or(lua.create_table()?);

                let name = name.unwrap_or("<string>".to_string());
                let context = lua_to_minijinja(lua, &mlua::Value::Table(ctx));

                bind_lua(lua, || {
                    this.read_env()?
                        .render_named_str(&name, &source, context)
                        .map_err(mlua::Error::external)
                })
            },
        );

        methods.add_method(
            "render_captured",
            |lua: &mlua::Lua,
             this: &LuaEnvironment,
             (name, ctx, callback): (String, Option<mlua::Table>, mlua::Function)|
             -> mlua::Result<mlua::MultiValue> {
                if this.reload_before_render() {
                    this.write_env()?.clear_templates();
                }

                let key = lua.create_registry_value(callback)?;
                let mut func = LuaFunctionObject::new(key);
                func.set_pass_state(true);

                let ctx = ctx.unwrap_or(lua.create_table()?);

                let context = lua_to_minijinja(lua, &mlua::Value::Table(ctx));

                bind_lua(lua, || {
                    let env = this.read_env()?;

                    let mut captured = env
                        .get_template(&name)
                        .map_err(mlua::Error::external)?
                        .render_captured(context)
                        .map_err(mlua::Error::external)?;

                    let expr = captured
                        .with_state_mut(|state| func.with_func_mut(&[], Some(state)))
                        .map_err(mlua::Error::external)?
                        .and_then(|v| minijinja_to_lua(lua, &v))
                        .unwrap_or_else(|| mlua::Value::Nil);

                    let rendered = captured.into_output();

                    let mut mv = mlua::MultiValue::new();

                    mv.push_back(mlua::Value::String(lua.create_string(rendered)?));
                    mv.push_back(expr);

                    Ok(mv)
                })
            },
        );

        methods.add_method(
            "eval",
            |lua: &mlua::Lua,
             this: &LuaEnvironment,
             (source, ctx): (String, Option<mlua::Table>)|
             -> Result<mlua::Value, mlua::Error> {
                let ctx = ctx.unwrap_or(lua.create_table()?);

                let context = lua_to_minijinja(lua, &mlua::Value::Table(ctx));

                bind_lua(lua, || {
                    let expr = this
                        .read_env()?
                        .compile_expression(&source)
                        .map_err(mlua::Error::external)?
                        .eval(&context)
                        .map_err(mlua::Error::external)?;

                    minijinja_to_lua(lua, &expr).ok_or_else(|| mlua::Error::ToLuaConversionError {
                        from: "".to_string(),
                        to: "",
                        message: None,
                    })
                })
            },
        );

        methods.add_method(
            "add_filter",
            |lua: &mlua::Lua,
             this: &LuaEnvironment,
             (name, filter, pass_state): (String, mlua::Function, Option<bool>)|
             -> Result<(), mlua::Error> {
                let key = lua.create_registry_value(filter)?;
                let mut func = LuaFunctionObject::new(key);
                func.set_pass_state(pass_state.unwrap_or(true));

                this.write_env()?.add_filter(
                    name,
                    move |state: &State, args: JinjaRest<JinjaValue>| {
                        func.with_func(&args, Some(state))
                    },
                );

                Ok(())
            },
        );

        methods.add_method(
            "remove_filter",
            |_, this: &LuaEnvironment, name: String| -> Result<(), mlua::Error> {
                this.write_env()?.remove_filter(&name);

                Ok(())
            },
        );

        methods.add_method(
            "add_test",
            |lua: &mlua::Lua,
             this: &LuaEnvironment,
             (name, test, pass_state): (String, mlua::Function, Option<bool>)|
             -> Result<(), mlua::Error> {
                let key = lua.create_registry_value(test)?;
                let mut func = LuaFunctionObject::new(key);
                func.set_pass_state(pass_state.unwrap_or(true));

                this.write_env()?.add_test(
                    name,
                    move |state: &State, args: JinjaRest<JinjaValue>| {
                        func.with_func(&args, Some(state))
                    },
                );

                Ok(())
            },
        );

        methods.add_method(
            "remove_test",
            |_, this: &LuaEnvironment, name: String| -> Result<(), mlua::Error> {
                this.write_env()?.remove_test(&name);

                Ok(())
            },
        );

        methods.add_method(
            "add_global",
            |lua: &mlua::Lua,
             this: &LuaEnvironment,
             (name, val, pass_state): (String, mlua::Value, Option<bool>)|
             -> Result<(), mlua::Error> {
                match val {
                    mlua::Value::Function(f) => {
                        let key = lua.create_registry_value(f)?;
                        let mut func = LuaFunctionObject::new(key);
                        func.set_pass_state(pass_state.unwrap_or(true));

                        this.write_env()?.add_function(
                            name,
                            move |state: &State, args: JinjaRest<JinjaValue>| {
                                func.with_func(&args, Some(state))
                            },
                        )
                    },
                    _ => this
                        .write_env()?
                        .add_global(name, lua_to_minijinja(lua, &val)),
                };

                Ok(())
            },
        );

        methods.add_method(
            "remove_global",
            |_, this: &LuaEnvironment, name: String| -> Result<(), mlua::Error> {
                this.write_env()?.remove_global(&name);

                Ok(())
            },
        );

        methods.add_method(
            "globals",
            |lua: &mlua::Lua,
             this: &LuaEnvironment,
             _val: mlua::Value|
             -> Result<mlua::Table, mlua::Error> {
                let table = lua.create_table()?;

                for (name, value) in this.read_env()?.globals() {
                    let val = minijinja_to_lua(lua, &value);
                    table.set(name, val)?;
                }

                Ok(table)
            },
        );
    }
}
