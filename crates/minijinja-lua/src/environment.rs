// SPDX-License-Identifier: MIT

use std::{borrow::Cow, fmt, ops::Deref};

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
pub struct LuaEnvironment(Environment<'static>);

impl From<Environment<'static>> for LuaEnvironment {
    fn from(value: Environment<'static>) -> Self {
        LuaEnvironment(value)
    }
}

impl From<LuaEnvironment> for Environment<'static> {
    fn from(value: LuaEnvironment) -> Self {
        value.0
    }
}

impl Deref for LuaEnvironment {
    type Target = Environment<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
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
    #[lua(name = "new", infallible)]
    pub(crate) fn lua_new() -> Self {
        let mut env = Environment::new();

        #[cfg(feature = "minijinja-contrib")]
        minijinja_contrib::add_to_environment(&mut env);

        #[cfg(feature = "json")]
        crate::contrib::json::add_to_environment(&mut env);

        #[cfg(feature = "datetime")]
        crate::contrib::datetime::add_to_environment(&mut env);

        env.into()
    }

    /// Get a new empty environment
    #[lua(name = "empty", infallible)]
    pub(crate) fn lua_empty() -> Self {
        Environment::empty().into()
    }

    #[lua(name = "keep_trailing_newline", getter, infallible)]
    pub(crate) fn lua_keep_trailing_newline(&self) -> bool {
        self.0.keep_trailing_newline()
    }

    #[lua(name = "keep_trailing_newline", setter, infallible)]
    pub(crate) fn lua_set_keep_trailing_newline(&mut self, val: bool) {
        self.0.set_keep_trailing_newline(val)
    }

    #[lua(name = "trim_blocks", getter, infallible)]
    pub(crate) fn lua_trim_blocks(&self) -> bool {
        self.0.trim_blocks()
    }

    #[lua(name = "trim_blocks", setter, infallible)]
    pub(crate) fn lua_set_trim_blocks(&mut self, val: bool) {
        self.0.set_trim_blocks(val)
    }

    #[lua(name = "lstrip_blocks", getter, infallible)]
    pub(crate) fn lua_lstrip_blocks(&self) -> bool {
        self.0.lstrip_blocks()
    }

    #[lua(name = "lstrip_blocks", setter, infallible)]
    pub(crate) fn lua_set_lstrip_blocks(&mut self, val: bool) {
        self.0.set_lstrip_blocks(val)
    }

    #[lua(name = "debug", getter, infallible)]
    pub(crate) fn lua_debug(&self) -> bool {
        self.0.debug()
    }

    #[lua(name = "debug", setter, infallible)]
    pub(crate) fn lua_set_debug(&mut self, val: bool) {
        self.0.set_debug(val)
    }

    #[lua(name = "fuel", getter, infallible)]
    pub(crate) fn lua_fuel(&self) -> Option<u64> {
        self.0.fuel()
    }

    #[lua(name = "fuel", setter, infallible)]
    pub(crate) fn lua_set_fuel(&mut self, val: Option<u64>) {
        self.0.set_fuel(val)
    }

    #[lua(name = "recursion_limit", getter, infallible)]
    pub(crate) fn lua_recursion_limit(&self) -> usize {
        self.0.recursion_limit()
    }

    #[lua(name = "recursion_limit", setter, infallible)]
    pub(crate) fn lua_set_recursion_limit(&mut self, val: usize) {
        self.0.set_recursion_limit(val)
    }

    #[lua(name = "undefined_behavior", getter, infallible)]
    pub(crate) fn lua_undefined_behavior(&self) -> Option<String> {
        let ub = self.0.undefined_behavior();

        undefined_behavior_to_lua(ub)
    }

    #[lua(name = "undefined_behavior", setter)]
    pub(crate) fn lua_set_undefined_behavior(&mut self, val: String) -> mlua::Result<()> {
        let ub = lua_to_undefined_behavior(&val)?;

        self.0.set_undefined_behavior(ub);

        Ok(())
    }

    #[lua(name = "add_template", infallible)]
    pub(crate) fn lua_add_template(
        &mut self,
        lua: &mlua::Lua,
        name: String,
        source: String,
    ) -> mlua::Result<()> {
        bind_lua(lua, || {
            self.0
                .add_template_owned(name, source)
                .map_err(mlua::Error::external)
        })
    }

    #[lua(name = "remove_template", infallible)]
    pub(crate) fn lua_remove_template(&mut self, lua: &mlua::Lua, name: String) {
        bind_lua(lua, || self.0.remove_template(&name))
    }

    #[lua(name = "clear_templates", infallible)]
    pub(crate) fn lua_clear_templates(&mut self, lua: &mlua::Lua) {
        bind_lua(lua, || self.0.clear_templates())
    }

    #[lua(name = "undeclared_variables")]
    pub(crate) fn lua_undeclared_variables(
        &mut self,
        lua: &mlua::Lua,
        name: String,
        nested: Option<bool>,
    ) -> mlua::Result<mlua::Value> {
        bind_lua(lua, || {
            let nested = nested.unwrap_or(false);

            let vars = self
                .0
                .get_template(&name)
                .map_err(mlua::Error::external)?
                .undeclared_variables(nested);

            lua.to_value(&vars)
        })
    }

    #[lua(name = "set_loader")]
    pub(crate) fn lua_set_loader(
        &mut self,
        lua: &mlua::Lua,
        callback: mlua::Function,
    ) -> mlua::Result<()> {
        let key = lua.create_registry_value(callback)?;
        let func = LuaFunctionObject::new(key);

        self.0.set_loader(move |name| {
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

    #[lua(name = "set_path_join_callback")]
    pub(crate) fn lua_set_path_join_callback(
        &mut self,
        lua: &mlua::Lua,
        callback: mlua::Function,
    ) -> mlua::Result<()> {
        let key = lua.create_registry_value(callback)?;
        let func = LuaFunctionObject::new(key);

        self.0.set_path_join_callback(move |name, parent| {
            func.with_func(args!(name, parent), None)
                .ok()
                .flatten()
                .and_then(|v| v.as_str().map(|s| Cow::Owned(s.to_string())))
                .unwrap_or(Cow::Borrowed(name))
        });

        Ok(())
    }

    #[lua(name = "set_unknown_method_callback")]
    pub(crate) fn lua_set_unknown_method_callback(
        &mut self,
        lua: &mlua::Lua,
        callback: mlua::Function,
    ) -> mlua::Result<()> {
        let key = lua.create_registry_value(callback)?;
        let mut func = LuaFunctionObject::new(key);
        func.set_pass_state(true);

        self.0
            .set_unknown_method_callback(move |state, value, method, args| {
                func.with_func(args!(value, method, ..args), Some(state))
                    .map(|v| v.unwrap_or_default())
            });

        Ok(())
    }

    #[cfg(feature = "minijinja-contrib")]
    #[lua(name = "set_pycompat", infallible)]
    pub(crate) fn lua_set_pycompat(&mut self, enable: Option<bool>) {
        match enable {
            Some(true) | None => self
                .0
                .set_unknown_method_callback(minijinja_contrib::pycompat::unknown_method_callback),
            Some(false) => self.0.set_unknown_method_callback(|_, _, _, _| {
                Err(JinjaError::from(JinjaErrorKind::UnknownMethod))
            }),
        }
    }

    #[lua(name = "set_auto_escape_callback")]
    pub(crate) fn lua_set_auto_escape_callback(
        &mut self,
        lua: &mlua::Lua,
        callback: mlua::Function,
    ) -> mlua::Result<()> {
        let key = lua.create_registry_value(callback)?;
        let func = LuaFunctionObject::new(key);

        self.0.set_auto_escape_callback(move |name| {
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

    #[lua(name = "set_formatter")]
    pub(crate) fn lua_set_formatter(
        &mut self,
        lua: &mlua::Lua,
        callback: mlua::Function,
    ) -> mlua::Result<()> {
        let key = lua.create_registry_value(callback)?;
        let mut func = LuaFunctionObject::new(key);
        func.set_pass_state(true);

        self.0.set_formatter(move |out, state, value| {
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

    #[lua(name = "set_syntax")]
    pub(crate) fn lua_set_syntax(&mut self, syntax: mlua::Table) -> mlua::Result<()> {
        let config = lua_to_syntax_config(syntax).map_err(mlua::Error::external)?;
        self.0.set_syntax(config);

        Ok(())
    }

    #[lua(name = "render_template")]
    pub(crate) fn lua_render_template(
        &mut self,
        lua: &mlua::Lua,
        name: String,
        ctx: Option<mlua::Table>,
    ) -> mlua::Result<String> {
        let ctx = ctx.unwrap_or(lua.create_table()?);

        let context = lua_to_minijinja(lua, &mlua::Value::Table(ctx));

        bind_lua(lua, || {
            self.0
                .get_template(&name)
                .map_err(mlua::Error::external)?
                .render(context)
                .map_err(mlua::Error::external)
        })
    }

    #[lua(name = "render_str")]
    pub(crate) fn lua_render_str(
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
            self.0
                .render_named_str(&name, &source, context)
                .map_err(mlua::Error::external)
        })
    }

    #[lua(name = "render_captured")]
    pub(crate) fn lua_render_captured(
        &mut self,
        lua: &mlua::Lua,
        name: String,
        ctx: Option<mlua::Table>,
        callback: mlua::Function,
    ) -> mlua::Result<mlua::MultiValue> {
        let key = lua.create_registry_value(callback)?;
        let mut func = LuaFunctionObject::new(key);
        func.set_pass_state(true);

        let ctx = ctx.unwrap_or(lua.create_table()?);

        let context = lua_to_minijinja(lua, &mlua::Value::Table(ctx));

        bind_lua(lua, || {
            let mut captured = self
                .0
                .get_template(&name)
                .map_err(mlua::Error::external)?
                .render_captured(context)
                .map_err(mlua::Error::external)?;

            let mut mv = captured
                .with_state_mut(|state| func.with_func_mut(&[], Some(state)))
                .map_err(mlua::Error::external)?
                .and_then(|v| minijinja_to_lua(lua, &v))
                .unwrap_or_default();

            let rendered = captured.into_output();

            mv.push_front(mlua::Value::String(lua.create_string(rendered)?));

            Ok(mv)
        })
    }

    #[lua(name = "eval")]
    pub(crate) fn lua_eval(
        &self,
        lua: &mlua::Lua,
        source: String,
        ctx: Option<mlua::Table>,
    ) -> mlua::Result<mlua::MultiValue> {
        let ctx = ctx.unwrap_or(lua.create_table()?);

        let context = lua_to_minijinja(lua, &mlua::Value::Table(ctx));

        bind_lua(lua, || {
            let expr = self
                .0
                .compile_expression(&source)
                .map_err(mlua::Error::external)?
                .eval(&context)
                .map_err(mlua::Error::external)?;

            minijinja_to_lua(lua, &expr).ok_or_else(|| {
                mlua::Error::DeserializeError("could not convert output to lua".to_string())
            })
        })
    }

    #[lua(name = "add_filter")]
    pub(crate) fn lua_add_filter(
        &mut self,
        lua: &mlua::Lua,
        name: String,
        filter: mlua::Function,
        pass_state: Option<bool>,
    ) -> mlua::Result<()> {
        let key = lua.create_registry_value(filter)?;
        let mut func = LuaFunctionObject::new(key);
        func.set_pass_state(pass_state.unwrap_or(true));

        self.0
            .add_filter(name, move |state: &State, args: JinjaRest<JinjaValue>| {
                func.with_func(&args, Some(state))
            });

        Ok(())
    }

    #[lua(name = "remove_filter", infallible)]
    pub(crate) fn lua_remove_filter(&mut self, name: String) {
        self.0.remove_filter(&name)
    }

    #[lua(name = "add_test")]
    pub(crate) fn lua_add_test(
        &mut self,
        lua: &mlua::Lua,
        name: String,
        test: mlua::Function,
        pass_state: Option<bool>,
    ) -> mlua::Result<()> {
        let key = lua.create_registry_value(test)?;
        let mut func = LuaFunctionObject::new(key);
        func.set_pass_state(pass_state.unwrap_or(true));

        self.0
            .add_test(name, move |state: &State, args: JinjaRest<JinjaValue>| {
                func.with_func(&args, Some(state))
            });

        Ok(())
    }

    #[lua(name = "remove_test", infallible)]
    pub(crate) fn lua_remove_test(&mut self, name: String) {
        self.0.remove_test(&name)
    }

    #[lua(name = "add_global")]
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

                self.0
                    .add_function(name, move |state: &State, args: JinjaRest<JinjaValue>| {
                        func.with_func(&args, Some(state))
                    })
            },
            _ => self.0.add_global(name, lua_to_minijinja(lua, &val)),
        };

        Ok(())
    }

    #[lua(name = "remove_global", infallible)]
    pub(crate) fn lua_remove_global(&mut self, name: String) {
        self.0.remove_global(&name)
    }

    #[lua(name = "globals")]
    pub(crate) fn lua_globals(&self, lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
        let table = lua.create_table()?;

        for (name, value) in self.0.globals() {
            minijinja_to_lua(lua, &value)
                .and_then(|mut v| table.set(name, v.pop_front().unwrap_or_default()).ok());
        }

        Ok(table)
    }
}
