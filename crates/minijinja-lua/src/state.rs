// SPDX-License-Identifier: MIT

use std::{
    fmt,
    ops::{Deref, DerefMut},
};

use minijinja::Value as JinjaValue;
use mlua::LuaSerdeExt;

use crate::convert::{
    LuaAutoEscape,
    LuaUndefinedBehavior,
    lua_args_to_minijinja,
    lua_to_minijinja,
    minijinja_to_lua,
};

pub(crate) trait LuaState<'template, 'env> {
    fn state(&self) -> &minijinja::State<'template, 'env>;
}

/// A [`mlua::UserData`] wrapper around a [`minijinja::State`]. This is passed to
/// filters and other callbacks in the Jinja environment. It can only be
/// initialized within an [`mlua::Lua::scope`] callback, as it is not `'static`
#[derive(Debug)]
pub struct LuaStateRef<'scope, 'template, 'env>(&'scope minijinja::State<'template, 'env>);

impl<'scope, 'template, 'env> From<&'scope minijinja::State<'template, 'env>>
    for LuaStateRef<'scope, 'template, 'env>
{
    fn from(value: &'scope minijinja::State<'template, 'env>) -> Self {
        LuaStateRef(value)
    }
}

impl<'scope, 'template, 'env> From<LuaStateRef<'scope, 'template, 'env>>
    for &'scope minijinja::State<'template, 'env>
{
    fn from(value: LuaStateRef<'scope, 'template, 'env>) -> Self {
        value.0
    }
}

impl<'scope, 'template, 'env> Deref for LuaStateRef<'scope, 'template, 'env> {
    type Target = minijinja::State<'template, 'env>;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'scope, 'template, 'env> LuaState<'template, 'env> for LuaStateRef<'scope, 'template, 'env> {
    fn state(&self) -> &minijinja::State<'template, 'env> {
        self.0
    }
}

impl<'scope, 'template, 'env> fmt::Display for LuaStateRef<'scope, 'template, 'env> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "State")
    }
}

impl<'scope, 'template, 'env> mlua::UserData for LuaStateRef<'scope, 'template, 'env> {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field("__name", "state");
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        add_common_methods(methods);
    }
}

/// A [`mlua::UserData`] wrapper around a mutable [`minijinja::State`]. This is passed to
/// the callback provided to [`LuaEnvironment.render_captured`](crate::environment::LuaEnvironment).
/// It can only be initialized within an [`mlua::Lua::scope`] callback, as it is not `'static`
#[derive(Debug)]
pub struct LuaStateMut<'scope, 'template, 'env>(&'scope mut minijinja::State<'template, 'env>);

impl<'scope, 'template, 'env> LuaStateMut<'scope, 'template, 'env> {
    fn state_mut(&mut self) -> &mut minijinja::State<'template, 'env> {
        self.0
    }
}

impl<'scope, 'template, 'env> From<&'scope mut minijinja::State<'template, 'env>>
    for LuaStateMut<'scope, 'template, 'env>
{
    fn from(value: &'scope mut minijinja::State<'template, 'env>) -> Self {
        LuaStateMut(value)
    }
}

impl<'scope, 'template, 'env> From<LuaStateMut<'scope, 'template, 'env>>
    for &'scope mut minijinja::State<'template, 'env>
{
    fn from(value: LuaStateMut<'scope, 'template, 'env>) -> Self {
        value.0
    }
}

impl<'scope, 'template, 'env> Deref for LuaStateMut<'scope, 'template, 'env> {
    type Target = minijinja::State<'template, 'env>;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'scope, 'template, 'env> DerefMut for LuaStateMut<'scope, 'template, 'env> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

impl<'scope, 'template, 'env> LuaState<'template, 'env> for LuaStateMut<'scope, 'template, 'env> {
    fn state(&self) -> &minijinja::State<'template, 'env> {
        self.0
    }
}

impl<'scope, 'template, 'env> fmt::Display for LuaStateMut<'scope, 'template, 'env> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "State")
    }
}

impl<'scope, 'template, 'env> mlua::UserData for LuaStateMut<'scope, 'template, 'env> {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field("__name", "state");
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        add_common_methods(methods);

        // Render the named block
        methods.add_method_mut(
            "render_block",
            |_, this, block: mlua::BorrowedStr| -> mlua::Result<String> {
                this.state_mut()
                    .render_block(&block)
                    .map_err(mlua::Error::external)
            },
        );
    }
}

/// A helper to add methods shared between [`LuaStateRef`] and [`LuaStateMut`].
fn add_common_methods<'template, 'env, S, M>(methods: &mut M)
where
    S: LuaState<'template, 'env>,
    M: mlua::UserDataMethods<S>,
    'env: 'template,
{
    // The name of the current template
    methods.add_method("name", |_, this, ()| -> mlua::Result<String> {
        Ok(this.state().name().to_string())
    });

    // The current auto escape flag
    methods.add_method(
        "auto_escape",
        |_, this, ()| -> mlua::Result<LuaAutoEscape> { Ok(this.state().auto_escape().into()) },
    );

    // The current undefined behavior
    methods.add_method(
        "undefined_behavior",
        |_, this, ()| -> mlua::Result<LuaUndefinedBehavior> {
            Ok(this.state().undefined_behavior().into())
        },
    );

    // The name of the current block
    methods.add_method(
        "current_block",
        |_, this, ()| -> mlua::Result<Option<String>> {
            Ok(this.state().current_block().map(|s| s.to_string()))
        },
    );

    // Lookup a value by key in the current context
    methods.add_method(
        "lookup",
        |lua, this, name: mlua::BorrowedStr| -> mlua::Result<mlua::MultiValue> {
            // Since the context may contain dynamic objects, convert the returned value
            // through the custom layer before returning.
            Ok(this
                .state()
                .lookup(&name)
                .and_then(|v| minijinja_to_lua(lua, &v))
                .unwrap_or_default())
        },
    );

    // Call the named macro with the provided args.
    methods.add_method(
        "call_macro",
        |lua,
         this,
         (name, mut args): (mlua::BorrowedStr, mlua::MultiValue)|
         -> mlua::Result<String> {
            let args: Vec<JinjaValue> = lua_args_to_minijinja(lua, &mut args, true);

            this.state()
                .call_macro(&name, &args)
                .map_err(mlua::Error::external)
        },
    );

    // A list of exported variables
    methods.add_method("exports", |_, this, ()| -> mlua::Result<Vec<String>> {
        Ok(this
            .state()
            .exports()
            .into_iter()
            .map(|i| i.to_string())
            .collect())
    });

    // A list of all known variables
    methods.add_method(
        "known_variables",
        |_, this, ()| -> mlua::Result<Vec<String>> {
            Ok(this
                .state()
                .known_variables()
                .into_iter()
                .map(|i| i.to_string())
                .collect())
        },
    );

    // Apply the named filter with the provided args
    methods.add_method(
        "apply_filter",
        |lua,
         this,
         (filter, mut args): (mlua::BorrowedStr, mlua::MultiValue)|
         -> mlua::Result<mlua::MultiValue> {
            let args: Vec<JinjaValue> = lua_args_to_minijinja(lua, &mut args, true);

            // Since the context may contain dynamic objects, convert the returned value
            // through the custom layer before returning.
            this.state()
                .apply_filter(&filter, &args)
                .map(|v| minijinja_to_lua(lua, &v).unwrap_or_default())
                .map_err(mlua::Error::external)
        },
    );

    // Perform the named test with the provided args
    methods.add_method(
        "perform_test",
        |lua,
         this,
         (test, mut args): (mlua::BorrowedStr, mlua::MultiValue)|
         -> mlua::Result<bool> {
            let args: Vec<JinjaValue> = lua_args_to_minijinja(lua, &mut args, true);

            this.state()
                .perform_test(&test, &args)
                .map_err(mlua::Error::external)
        },
    );

    // Format a value to a string
    methods.add_method(
        "format",
        |lua, this, val: mlua::Value| -> mlua::Result<String> {
            let val = lua_to_minijinja(lua, &val).unwrap_or_default();

            this.state().format(val).map_err(mlua::Error::external)
        },
    );

    // A tuple of the current and remaining fuel usage
    methods.add_method(
        "fuel_levels",
        |lua, this, ()| -> mlua::Result<mlua::Value> { lua.to_value(&this.state().fuel_levels()) },
    );

    // Get a temp value.
    // See: https://docs.rs/minijinja/latest/minijinja/struct.State.html#method.get_temp
    methods.add_method(
        "get_temp",
        |lua, this, name: mlua::BorrowedStr| -> mlua::Result<mlua::MultiValue> {
            // Since the context may contain dynamic objects, convert the returned value
            // through the custom layer before returning.
            Ok(this
                .state()
                .get_temp(&name)
                .and_then(|v| minijinja_to_lua(lua, &v))
                .unwrap_or_default())
        },
    );

    // Set a temp value and return the old value
    methods.add_method(
        "set_temp",
        |lua,
         this,
         (name, val): (mlua::BorrowedStr, mlua::Value)|
         -> mlua::Result<mlua::MultiValue> {
            if let Some(val) = lua_to_minijinja(lua, &val) {
                Ok(this
                    .state()
                    .set_temp(&name, val)
                    .and_then(|v| minijinja_to_lua(lua, &v))
                    .unwrap_or_default())
            } else {
                Err(mlua::Error::FromLuaConversionError {
                    from: val.type_name(),
                    to: "minijinja::Value".to_string(),
                    message: None,
                })
            }
        },
    );

    // Get a temp value or call `func` to add the value
    methods.add_method(
        "get_or_set_temp",
        |lua,
         this,
         (name, func): (mlua::BorrowedStr, mlua::Function)|
         -> mlua::Result<mlua::MultiValue> {
            let val = match this.state().get_temp(&name) {
                Some(v) => v,
                None => {
                    let val = func.call::<mlua::Value>(mlua::Value::Nil)?;

                    if let Some(val) = lua_to_minijinja(lua, &val) {
                        this.state().set_temp(&name, val.clone());
                        val
                    } else {
                        return Err(mlua::Error::FromLuaConversionError {
                            from: val.type_name(),
                            to: "minijinja::Value".to_string(),
                            message: None,
                        });
                    }
                },
            };

            Ok(minijinja_to_lua(lua, &val).unwrap_or_default())
        },
    );
}
