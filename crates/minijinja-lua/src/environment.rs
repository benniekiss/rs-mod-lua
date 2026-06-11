// SPDX-License-Identifier: MIT

use std::{
    borrow::Cow,
    fmt,
    sync::atomic::{AtomicBool, Ordering},
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
#[derive(mlua::UserData, Debug)]
pub struct LuaEnvironment {
    #[lua(skip)]
    env: Environment<'static>,
    #[lua(skip)]
    reload_before_render: AtomicBool,
    #[cfg(feature = "minijinja-contrib")]
    #[lua(skip)]
    pycompat: AtomicBool,
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

#[mlua::userdata_impl]
impl LuaEnvironment {
    /// Get a new environment
    #[lua(infallible)]
    pub(crate) fn new() -> Self {
        let mut env = Environment::new();

        #[cfg(feature = "minijinja-contrib")]
        minijinja_contrib::add_to_environment(&mut env);

        #[cfg(feature = "json")]
        crate::contrib::json::add_to_environment(&mut env);

        #[cfg(feature = "datetime")]
        crate::contrib::datetime::add_to_environment(&mut env);

        Self {
            env,
            reload_before_render: AtomicBool::new(false),
            pycompat: AtomicBool::new(false),
        }
    }

    /// Get a new empty environment
    #[lua(infallible)]
    pub(crate) fn empty() -> Self {
        let env = Environment::empty();

        Self {
            env,
            reload_before_render: AtomicBool::new(false),
            pycompat: AtomicBool::new(false),
        }
    }

    #[lua(getter, infallible)]
    pub(crate) fn reload_before_render(&self) -> bool {
        self.reload_before_render.load(Ordering::Relaxed)
    }

    #[lua(setter, name = "reload_before_render", infallible)]
    pub(crate) fn set_reload_before_render(&self, val: bool) {
        self.reload_before_render.store(val, Ordering::Relaxed);
    }

    #[cfg(feature = "minijinja-contrib")]
    #[lua(getter, infallible)]
    pub(crate) fn pycompat(&self) -> bool {
        self.pycompat.load(Ordering::Relaxed)
    }

    #[cfg(feature = "minijinja-contrib")]
    #[lua(setter, name = "pycompat", infallible)]
    pub(crate) fn set_pycompat(&mut self, val: bool) {
        self.pycompat.store(val, Ordering::Relaxed);

        match val {
            true => self
                .env
                .set_unknown_method_callback(minijinja_contrib::pycompat::unknown_method_callback),
            false => self.env.set_unknown_method_callback(|_, _, _, _| {
                Err(JinjaError::from(JinjaErrorKind::UnknownMethod))
            }),
        }
    }

    #[lua(getter, infallible)]
    pub(crate) fn keep_trailing_newline(&self) -> bool {
        self.env.keep_trailing_newline()
    }

    #[lua(setter, name = "keep_trailing_newline", infallible)]
    pub(crate) fn set_keep_trailing_newline(&mut self, val: bool) {
        self.env.set_keep_trailing_newline(val)
    }

    #[lua(getter, infallible)]
    pub(crate) fn trim_blocks(&self) -> bool {
        self.env.trim_blocks()
    }

    #[lua(setter, name = "trim_blocks", infallible)]
    pub(crate) fn set_trim_blocks(&mut self, val: bool) {
        self.env.set_trim_blocks(val)
    }

    #[lua(getter, infallible)]
    pub(crate) fn lstrip_blocks(&self) -> bool {
        self.env.lstrip_blocks()
    }

    #[lua(setter, name = "lstrip_blocks", infallible)]
    pub(crate) fn set_lstrip_blocks(&mut self, val: bool) {
        self.env.set_lstrip_blocks(val)
    }

    #[lua(getter, infallible)]
    pub(crate) fn debug(&self) -> bool {
        self.env.debug()
    }

    #[lua(setter, name = "debug", infallible)]
    pub(crate) fn set_debug(&mut self, val: bool) {
        self.env.set_debug(val)
    }

    #[lua(getter, infallible)]
    pub(crate) fn fuel(&self) -> Option<u64> {
        self.env.fuel()
    }

    #[lua(setter, name = "fuel", infallible)]
    pub(crate) fn set_fuel(&mut self, val: Option<u64>) {
        self.env.set_fuel(val)
    }

    #[lua(getter, infallible)]
    pub(crate) fn recursion_limit(&self) -> usize {
        self.env.recursion_limit()
    }

    #[lua(setter, name = "recursion_limit", infallible)]
    pub(crate) fn set_recursion_limit(&mut self, val: usize) {
        self.env.set_recursion_limit(val)
    }

    #[lua(getter, infallible)]
    pub(crate) fn undefined_behavior(&self) -> Option<String> {
        let ub = self.env.undefined_behavior();

        undefined_behavior_to_lua(ub)
    }

    #[lua(setter, name = "undefined_behavior")]
    pub(crate) fn set_undefined_behavior(&mut self, val: String) -> mlua::Result<()> {
        let ub = lua_to_undefined_behavior(&val)?;

        self.env.set_undefined_behavior(ub);

        Ok(())
    }

    pub(crate) fn add_template(
        &mut self,
        lua: &mlua::Lua,
        name: String,
        source: String,
    ) -> mlua::Result<()> {
        bind_lua(lua, || {
            self.env
                .add_template_owned(name, source)
                .map_err(mlua::Error::external)
        })
    }

    #[lua(infallible)]
    pub(crate) fn remove_template(&mut self, lua: &mlua::Lua, name: String) {
        bind_lua(lua, || self.env.remove_template(&name))
    }

    #[lua(infallible)]
    pub(crate) fn clear_templates(&mut self, lua: &mlua::Lua) {
        bind_lua(lua, || self.env.clear_templates())
    }

    pub(crate) fn undeclared_variables(
        &mut self,
        lua: &mlua::Lua,
        name: String,
        nested: Option<bool>,
    ) -> mlua::Result<mlua::Value> {
        bind_lua(lua, || {
            if self.reload_before_render() {
                self.env.clear_templates();
            }

            let nested = nested.unwrap_or(false);

            let vars = self
                .env
                .get_template(&name)
                .map_err(mlua::Error::external)?
                .undeclared_variables(nested);

            lua.to_value(&vars)
        })
    }

    pub(crate) fn set_loader(
        &mut self,
        lua: &mlua::Lua,
        callback: mlua::Function,
    ) -> mlua::Result<()> {
        let key = lua.create_registry_value(callback)?;
        let func = LuaFunctionObject::new(key);

        self.env.set_loader(move |name| {
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
    }

    pub(crate) fn set_path_join_callback(
        &mut self,
        lua: &mlua::Lua,
        callback: mlua::Function,
    ) -> mlua::Result<()> {
        let key = lua.create_registry_value(callback)?;
        let func = LuaFunctionObject::new(key);

        self.env.set_path_join_callback(move |name, parent| {
            func.with_func(args!(name, parent), None)
                .ok()
                .flatten()
                .and_then(|v| v.as_str().map(|s| Cow::Owned(s.to_string())))
                .unwrap_or(Cow::Borrowed(name))
        });

        Ok(())
    }

    pub(crate) fn set_unknown_method_callback(
        &mut self,
        lua: &mlua::Lua,
        callback: mlua::Function,
    ) -> mlua::Result<()> {
        let key = lua.create_registry_value(callback)?;
        let mut func = LuaFunctionObject::new(key);
        func.set_pass_state(true);

        self.env
            .set_unknown_method_callback(move |state, value, method, args| {
                func.with_func(args!(value, method, ..args), Some(state))
                    .map(|v| v.unwrap_or(JinjaValue::UNDEFINED))
            });

        Ok(())
    }

    pub(crate) fn set_auto_escape_callback(
        &mut self,
        lua: &mlua::Lua,
        callback: mlua::Function,
    ) -> mlua::Result<()> {
        let key = lua.create_registry_value(callback)?;
        let func = LuaFunctionObject::new(key);

        self.env.set_auto_escape_callback(move |name| {
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
    }

    pub(crate) fn set_formatter(
        &mut self,
        lua: &mlua::Lua,
        callback: mlua::Function,
    ) -> mlua::Result<()> {
        let key = lua.create_registry_value(callback)?;
        let mut func = LuaFunctionObject::new(key);
        func.set_pass_state(true);

        self.env.set_formatter(move |out, state, value| {
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
    }

    pub(crate) fn set_syntax(&mut self, syntax: mlua::Table) -> mlua::Result<()> {
        let config = lua_to_syntax_config(syntax).map_err(mlua::Error::external)?;
        self.env.set_syntax(config);

        Ok(())
    }

    pub(crate) fn render_template(
        &mut self,
        lua: &mlua::Lua,
        name: String,
        ctx: Option<mlua::Table>,
    ) -> mlua::Result<String> {
        if self.reload_before_render() {
            self.env.clear_templates();
        }

        let ctx = ctx.unwrap_or(lua.create_table()?);

        let context = lua_to_minijinja(lua, &mlua::Value::Table(ctx));

        bind_lua(lua, || {
            self.env
                .get_template(&name)
                .map_err(mlua::Error::external)?
                .render(context)
                .map_err(mlua::Error::external)
        })
    }

    pub(crate) fn render_str(
        &self,
        lua: &mlua::Lua,
        source: String,
        ctx: Option<mlua::Table>,
        name: Option<String>,
    ) -> mlua::Result<String> {
        let ctx = ctx.unwrap_or(lua.create_table()?);

        let name = name.unwrap_or("<string>".to_string());
        let context = lua_to_minijinja(lua, &mlua::Value::Table(ctx));

        bind_lua(lua, || {
            self.env
                .render_named_str(&name, &source, context)
                .map_err(mlua::Error::external)
        })
    }

    pub(crate) fn render_captured(
        &mut self,
        lua: &mlua::Lua,
        name: String,
        ctx: Option<mlua::Table>,
        callback: mlua::Function,
    ) -> mlua::Result<mlua::MultiValue> {
        if self.reload_before_render() {
            self.env.clear_templates();
        }

        let key = lua.create_registry_value(callback)?;
        let mut func = LuaFunctionObject::new(key);
        func.set_pass_state(true);

        let ctx = ctx.unwrap_or(lua.create_table()?);

        let context = lua_to_minijinja(lua, &mlua::Value::Table(ctx));

        bind_lua(lua, || {
            let mut captured = self
                .env
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
    }

    pub(crate) fn eval(
        &self,
        lua: &mlua::Lua,
        source: String,
        ctx: Option<mlua::Table>,
    ) -> mlua::Result<mlua::Value> {
        let ctx = ctx.unwrap_or(lua.create_table()?);

        let context = lua_to_minijinja(lua, &mlua::Value::Table(ctx));

        bind_lua(lua, || {
            let expr = self
                .env
                .compile_expression(&source)
                .map_err(mlua::Error::external)?
                .eval(&context)
                .map_err(mlua::Error::external)?;

            minijinja_to_lua(lua, &expr).ok_or_else(|| {
                mlua::Error::DeserializeError("could not convert output to lua".to_string())
            })
        })
    }

    pub(crate) fn add_filter(
        &mut self,
        lua: &mlua::Lua,
        name: String,
        filter: mlua::Function,
        pass_state: Option<bool>,
    ) -> mlua::Result<()> {
        let key = lua.create_registry_value(filter)?;
        let mut func = LuaFunctionObject::new(key);
        func.set_pass_state(pass_state.unwrap_or(true));

        self.env
            .add_filter(name, move |state: &State, args: JinjaRest<JinjaValue>| {
                func.with_func(&args, Some(state))
            });

        Ok(())
    }

    #[lua(infallible)]
    pub(crate) fn remove_filter(&mut self, name: String) {
        self.env.remove_filter(&name)
    }

    pub(crate) fn add_test(
        &mut self,
        lua: &mlua::Lua,
        name: String,
        test: mlua::Function,
        pass_state: Option<bool>,
    ) -> mlua::Result<()> {
        let key = lua.create_registry_value(test)?;
        let mut func = LuaFunctionObject::new(key);
        func.set_pass_state(pass_state.unwrap_or(true));

        self.env
            .add_test(name, move |state: &State, args: JinjaRest<JinjaValue>| {
                func.with_func(&args, Some(state))
            });

        Ok(())
    }

    #[lua(infallible)]
    pub(crate) fn remove_test(&mut self, name: String) {
        self.env.remove_test(&name)
    }

    pub(crate) fn add_global(
        &mut self,
        lua: &mlua::Lua,
        name: String,
        val: mlua::Value,
        pass_state: Option<bool>,
    ) -> mlua::Result<()> {
        match val {
            mlua::Value::Function(f) => {
                let key = lua.create_registry_value(f)?;
                let mut func = LuaFunctionObject::new(key);
                func.set_pass_state(pass_state.unwrap_or(true));

                self.env
                    .add_function(name, move |state: &State, args: JinjaRest<JinjaValue>| {
                        func.with_func(&args, Some(state))
                    })
            },
            _ => self.env.add_global(name, lua_to_minijinja(lua, &val)),
        };

        Ok(())
    }

    #[lua(infallible)]
    pub(crate) fn remove_global(&mut self, name: String) {
        self.env.remove_global(&name)
    }

    pub(crate) fn globals(&self, lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
        let table = lua.create_table()?;

        for (name, value) in self.env.globals() {
            let val = minijinja_to_lua(lua, &value);
            table.set(name, val)?;
        }

        Ok(table)
    }
}
