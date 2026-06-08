// SPDX-License-Identifier: MIT

use std::{
    fmt,
    sync::atomic::{AtomicPtr, Ordering},
};

use minijinja::Value as JinjaValue;
use mlua::LuaSerdeExt;

use crate::convert::{
    auto_escape_to_lua,
    lua_args_to_minijinja,
    lua_to_minijinja,
    minijinja_to_lua,
    undefined_behavior_to_lua,
};

thread_local! {
    static CURRENT_LUA: AtomicPtr<mlua::Lua> = const { AtomicPtr::new(std::ptr::null_mut()) };
}

trait LuaState<'env, 'render> {
    fn state(&self) -> &minijinja::State<'env, 'render>;
}

/// A [`mlua::UserData`] wrapper around a [`minijinja::State`]. This is passed to
/// filters and other callbacks in the Jinja environment. It can only be
/// initialized within an [`mlua::Lua::scope`] callback, as it is not `'static`
#[derive(Debug)]
pub struct LuaStateRef<'scope, 'env, 'render> {
    state: &'scope minijinja::State<'env, 'render>,
}

impl<'scope, 'env, 'render> LuaStateRef<'scope, 'env, 'render> {
    /// Get a new state
    pub(crate) fn new(state: &'scope minijinja::State<'env, 'render>) -> Self {
        Self { state }
    }
}

impl<'scope, 'env, 'render> LuaState<'env, 'render> for LuaStateRef<'scope, 'env, 'render> {
    fn state(&self) -> &minijinja::State<'env, 'render> {
        self.state
    }
}

impl<'scope, 'env, 'render> fmt::Display for LuaStateRef<'scope, 'env, 'render> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "State")
    }
}

impl<'scope, 'env, 'render> mlua::UserData for LuaStateRef<'scope, 'env, 'render> {
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
pub struct LuaStateMut<'scope, 'env, 'render> {
    state: &'scope mut minijinja::State<'env, 'render>,
}

impl<'scope, 'env, 'render> LuaStateMut<'scope, 'env, 'render> {
    /// Get a new state
    pub(crate) fn new(state: &'scope mut minijinja::State<'env, 'render>) -> Self {
        Self { state }
    }

    fn state_mut(&mut self) -> &mut minijinja::State<'env, 'render> {
        self.state
    }
}

impl<'scope, 'env, 'render> LuaState<'env, 'render> for LuaStateMut<'scope, 'env, 'render> {
    fn state(&self) -> &minijinja::State<'env, 'render> {
        self.state
    }
}

impl<'scope, 'env, 'render> fmt::Display for LuaStateMut<'scope, 'env, 'render> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "State")
    }
}

impl<'scope, 'env, 'render> mlua::UserData for LuaStateMut<'scope, 'env, 'render> {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field("__name", "state");
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        add_common_methods(methods);

        // Render the named block
        methods.add_method_mut(
            "render_block",
            |_, this, block: String| -> mlua::Result<String> {
                this.state_mut()
                    .render_block(&block)
                    .map_err(mlua::Error::external)
            },
        );
    }
}

/// A helper to add methods shared between [`LuaStateRef`] and [`LuaStateMut`].
fn add_common_methods<'env, 'render, S, M>(methods: &mut M)
where
    S: LuaState<'env, 'render>,
    M: mlua::UserDataMethods<S>,
    'render: 'env,
{
    // The name of the current template
    methods.add_method("name", |_, this, _: mlua::Value| -> mlua::Result<String> {
        Ok(this.state().name().to_string())
    });

    // The current auto escape flag
    methods.add_method(
        "auto_escape",
        |_, this, _: mlua::Value| -> mlua::Result<Option<String>> {
            Ok(auto_escape_to_lua(this.state().auto_escape()))
        },
    );

    // The current undefined behavior
    methods.add_method(
        "undefined_behavior",
        |_, this, _: mlua::Value| -> mlua::Result<Option<String>> {
            Ok(undefined_behavior_to_lua(this.state().undefined_behavior()))
        },
    );

    // The name of the current block
    methods.add_method(
        "current_block",
        |_, this, _: mlua::Value| -> mlua::Result<Option<String>> {
            Ok(this.state().current_block().map(|s| s.to_string()))
        },
    );

    // Lookup a value by key in the current context
    methods.add_method(
        "lookup",
        |lua, this, name: String| -> mlua::Result<Option<mlua::Value>> {
            // Since the context may contain dynamic objects, convert the returned value
            // through the custom layer before returning.
            Ok(this
                .state()
                .lookup(&name)
                .and_then(|v| minijinja_to_lua(lua, &v)))
        },
    );

    // Call the named macro with the provided args.
    methods.add_method(
        "call_macro",
        |lua, this, (name, args): (String, mlua::Variadic<mlua::Value>)| -> mlua::Result<String> {
            let args: Vec<JinjaValue> = lua_args_to_minijinja(lua, args, true);

            this.state()
                .call_macro(&name, &args)
                .map_err(mlua::Error::external)
        },
    );

    // A list of exported variables
    methods.add_method(
        "exports",
        |_, this, _: mlua::Value| -> mlua::Result<Vec<String>> {
            Ok(this
                .state()
                .exports()
                .into_iter()
                .map(|i| i.to_string())
                .collect())
        },
    );

    // A list of all known variables
    methods.add_method(
        "known_variables",
        |_, this, _: mlua::Value| -> mlua::Result<Vec<String>> {
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
         (filter, args): (String, mlua::Variadic<mlua::Value>)|
         -> mlua::Result<Option<mlua::Value>> {
            let args: Vec<JinjaValue> = lua_args_to_minijinja(lua, args, true);

            // Since the context may contain dynamic objects, convert the returned value
            // through the custom layer before returning.
            this.state()
                .apply_filter(&filter, &args)
                .map(|v| minijinja_to_lua(lua, &v))
                .map_err(mlua::Error::external)
        },
    );

    // Perform the named test with the provided args
    methods.add_method(
        "perform_test",
        |lua, this, (test, args): (String, mlua::Variadic<mlua::Value>)| -> mlua::Result<bool> {
            let args: Vec<JinjaValue> = lua_args_to_minijinja(lua, args, true);

            this.state()
                .perform_test(&test, &args)
                .map_err(mlua::Error::external)
        },
    );

    // Format a value to a string
    methods.add_method(
        "format",
        |lua, this, val: mlua::Value| -> mlua::Result<String> {
            let val = lua_to_minijinja(lua, &val).unwrap_or(JinjaValue::UNDEFINED);

            this.state().format(val).map_err(mlua::Error::external)
        },
    );

    // A tuple of the current and remaining fuel usage
    methods.add_method(
        "fuel_levels",
        |lua, this, _: mlua::Value| -> mlua::Result<mlua::Value> {
            lua.to_value(&this.state().fuel_levels())
        },
    );

    // Get a temp value.
    // See: https://docs.rs/minijinja/latest/minijinja/struct.State.html#method.get_temp
    methods.add_method(
        "get_temp",
        |lua, this, name: String| -> mlua::Result<Option<mlua::Value>> {
            // Since the context may contain dynamic objects, convert the returned value
            // through the custom layer before returning.
            Ok(this
                .state()
                .get_temp(&name)
                .and_then(|v| minijinja_to_lua(lua, &v)))
        },
    );

    // Set a temp value and return the old value
    methods.add_method(
        "set_temp",
        |lua, this, (name, val): (String, mlua::Value)| -> mlua::Result<Option<mlua::Value>> {
            if let Some(val) = lua_to_minijinja(lua, &val) {
                Ok(this
                    .state()
                    .set_temp(&name, val)
                    .and_then(|v| minijinja_to_lua(lua, &v)))
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
        |lua, this, (name, func): (String, mlua::Function)| -> mlua::Result<Option<mlua::Value>> {
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

            Ok(minijinja_to_lua(lua, &val))
        },
    );
}

/// Allow access to a [`mlua::Lua`] instance across a `Send + Sync` boundary in module mode.
///
/// This code mirrors the [`minijinja-py`](https://github.com/mitsuhiko/minijinja/blob/29ac0b2936eacf83ebf781c52f4f4ffc3add4c52/minijinja-py/src/state.rs) implementation.
pub(crate) fn with_lua<R, F: FnOnce(&mlua::Lua) -> Result<R, mlua::Error>>(
    f: F,
) -> Result<R, mlua::Error> {
    CURRENT_LUA.with(|handle| {
        let ptr = unsafe { (handle.load(Ordering::Relaxed) as *const mlua::Lua).as_ref() };

        match ptr {
            Some(lua) => f(lua),
            None => Err(mlua::Error::runtime(
                "mlua::Lua state accessed outside of a render context.",
            )),
        }
    })
}

/// Invokes a function with the state stashed away.
///
/// This code mirrors the [`minijinja-py`](https://github.com/mitsuhiko/minijinja/blob/29ac0b2936eacf83ebf781c52f4f4ffc3add4c52/minijinja-py/src/state.rs) implementation.
pub(crate) fn bind_lua<R, F: FnOnce() -> R>(lua: &mlua::Lua, f: F) -> R {
    let old_handle = CURRENT_LUA
        .with(|handle| handle.swap(lua as *const mlua::Lua as *mut mlua::Lua, Ordering::Relaxed));

    let rv = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));

    CURRENT_LUA.with(|handle| handle.store(old_handle, Ordering::Relaxed));
    match rv {
        Ok(rv) => rv,
        Err(payload) => std::panic::resume_unwind(payload),
    }
}
