// SPDX-License-Identifier: MIT

use std::{
    fmt,
    sync::atomic::{AtomicPtr, Ordering},
};

use minijinja::Value as JinjaValue;
use mlua::{
    LuaSerdeExt,
    prelude::{Lua, LuaError, LuaFunction, LuaUserData, LuaValue, LuaVariadic},
};

use crate::convert::{
    auto_escape_to_lua,
    lua_args_to_minijinja,
    lua_to_minijinja,
    minijinja_to_lua,
    undefined_behavior_to_lua,
};

thread_local! {
    static CURRENT_LUA: AtomicPtr<Lua> = const { AtomicPtr::new(std::ptr::null_mut()) };
}

/// A [`mlua::UserData`] wrapper around a [`minijinja::State`]. This is passed to
/// filters and other callbacks in the Jinja environment. It can only be
/// initialized within an [`mlua::Lua::scope`] callback, as it is not `'static`
#[derive(Debug)]
pub struct LuaState<'scope> {
    state: &'scope minijinja::State<'scope, 'scope>,
}

impl<'scope> LuaState<'scope> {
    /// Get a new state
    pub(crate) fn new(state: &'scope minijinja::State<'scope, 'scope>) -> Self {
        Self { state }
    }
}

impl<'scope> fmt::Display for LuaState<'scope> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "State")
    }
}

impl<'scope> LuaUserData for LuaState<'scope> {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field("__name", "state");
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        // The name of the current template
        methods.add_method(
            "name",
            |_, this: &LuaState<'scope>, _: LuaValue| -> Result<String, _> {
                Ok(this.state.name().to_string())
            },
        );

        // The current auto escape flag
        methods.add_method(
            "auto_escape",
            |_, this: &LuaState<'scope>, _: LuaValue| -> Result<Option<String>, _> {
                Ok(auto_escape_to_lua(this.state.auto_escape()))
            },
        );

        // The current undefined behavior
        methods.add_method(
            "undefined_behavior",
            |_, this: &LuaState<'scope>, _: LuaValue| -> Result<Option<String>, _> {
                Ok(undefined_behavior_to_lua(this.state.undefined_behavior()))
            },
        );

        // The name of the current block
        methods.add_method(
            "current_block",
            |_, this: &LuaState<'scope>, _: LuaValue| -> Result<Option<&str>, _> {
                Ok(this.state.current_block())
            },
        );

        // Lookup a value by key in the current context
        methods.add_method(
            "lookup",
            |lua: &Lua, this: &LuaState<'scope>, name: String| -> Result<Option<LuaValue>, _> {
                // Since the context may contain dynamic objects, convert the returned value
                // through the custom layer before returning.
                Ok(this
                    .state
                    .lookup(&name)
                    .and_then(|v| minijinja_to_lua(lua, &v)))
            },
        );

        // Call the named macro with the provided args.
        methods.add_method(
            "call_macro",
            |lua: &Lua,
             this: &LuaState<'scope>,
             (name, args): (String, LuaVariadic<LuaValue>)|
             -> Result<String, LuaError> {
                let args: Vec<JinjaValue> = lua_args_to_minijinja(lua, args, true);

                this.state
                    .call_macro(&name, &args)
                    .map_err(LuaError::external)
            },
        );

        // A list of exported variables
        methods.add_method(
            "exports",
            |_, this: &LuaState<'scope>, _: LuaValue| -> Result<Vec<&str>, _> {
                Ok(this.state.exports())
            },
        );

        // A list of all known variables
        methods.add_method(
            "known_variables",
            |_,
             this: &LuaState<'scope>,
             _: LuaValue|
             -> Result<Vec<std::borrow::Cow<'_, str>>, _> {
                Ok(this.state.known_variables())
            },
        );

        // Apply the named filter with the provided args
        methods.add_method(
            "apply_filter",
            |lua: &Lua,
             this: &LuaState<'scope>,
             (filter, args): (String, LuaVariadic<LuaValue>)|
             -> Result<Option<LuaValue>, LuaError> {
                let args: Vec<JinjaValue> = lua_args_to_minijinja(lua, args, true);

                // Since the context may contain dynamic objects, convert the returned value
                // through the custom layer before returning.
                this.state
                    .apply_filter(&filter, &args)
                    .map(|v| minijinja_to_lua(lua, &v))
                    .map_err(LuaError::external)
            },
        );

        // Perform the named test with the provided args
        methods.add_method(
            "perform_test",
            |lua: &Lua,
             this: &LuaState<'scope>,
             (test, args): (String, LuaVariadic<LuaValue>)|
             -> Result<bool, LuaError> {
                let args: Vec<JinjaValue> = lua_args_to_minijinja(lua, args, true);

                this.state
                    .perform_test(&test, &args)
                    .map_err(LuaError::external)
            },
        );

        // Format a value to a string
        methods.add_method(
            "format",
            |lua: &Lua, this: &LuaState<'scope>, val: LuaValue| -> Result<String, LuaError> {
                let val = lua_to_minijinja(lua, &val).unwrap_or(JinjaValue::UNDEFINED);

                this.state.format(val).map_err(LuaError::external)
            },
        );

        // A tuple of the current and remaining fuel usage
        methods.add_method(
            "fuel_levels",
            |lua: &Lua, this: &LuaState<'scope>, _: LuaValue| -> Result<LuaValue, _> {
                lua.to_value(&this.state.fuel_levels())
            },
        );

        // Get a temp value.
        // See: https://docs.rs/minijinja/latest/minijinja/struct.State.html#method.get_temp
        methods.add_method(
            "get_temp",
            |lua: &Lua,
             this: &LuaState<'scope>,
             name: String|
             -> Result<Option<LuaValue>, LuaError> {
                // Since the context may contain dynamic objects, convert the returned value
                // through the custom layer before returning.
                Ok(this
                    .state
                    .get_temp(&name)
                    .and_then(|v| minijinja_to_lua(lua, &v)))
            },
        );

        // Set a temp value and return the old value
        methods.add_method(
            "set_temp",
            |lua: &Lua,
             this: &LuaState<'scope>,
             (name, val): (String, LuaValue)|
             -> Result<Option<LuaValue>, LuaError> {
                if let Some(val) = lua_to_minijinja(lua, &val) {
                    Ok(this
                        .state
                        .set_temp(&name, val)
                        .and_then(|v| minijinja_to_lua(lua, &v)))
                } else {
                    Err(LuaError::ToLuaConversionError {
                        from: val.type_name().to_string(),
                        to: "minijinja::Value",
                        message: None,
                    })
                }
            },
        );

        // Get a temp value or call `func` to add the value
        methods.add_method(
            "get_or_set_temp",
            |lua: &Lua,
             this: &LuaState<'scope>,
             (name, func): (String, LuaFunction)|
             -> Result<Option<LuaValue>, LuaError> {
                let val = match this.state.get_temp(&name) {
                    Some(v) => v,
                    None => {
                        let val = func.call::<LuaValue>(LuaValue::Nil)?;

                        if let Some(val) = lua_to_minijinja(lua, &val) {
                            this.state.set_temp(&name, val.clone());
                            val
                        } else {
                            return Err(LuaError::ToLuaConversionError {
                                from: val.type_name().to_string(),
                                to: "minijinja::Value",
                                message: None,
                            });
                        }
                    },
                };

                Ok(minijinja_to_lua(lua, &val))
            },
        );
    }
}

/// Allow access to a [`mlua::Lua`] instance across a `Send + Sync` boundary in module mode.
///
/// This code mirrors the [`minijinja-py`](https://github.com/mitsuhiko/minijinja/blob/29ac0b2936eacf83ebf781c52f4f4ffc3add4c52/minijinja-py/src/state.rs) implementation.
pub(crate) fn with_lua<R, F: FnOnce(&Lua) -> Result<R, LuaError>>(f: F) -> Result<R, LuaError> {
    CURRENT_LUA.with(|handle| {
        let ptr = unsafe { (handle.load(Ordering::Relaxed) as *const Lua).as_ref() };

        match ptr {
            Some(lua) => f(lua),
            None => Err(LuaError::runtime(
                "mlua::Lua state accessed outside of a render context.",
            )),
        }
    })
}

/// Invokes a function with the state stashed away.
///
/// This code mirrors the [`minijinja-py`](https://github.com/mitsuhiko/minijinja/blob/29ac0b2936eacf83ebf781c52f4f4ffc3add4c52/minijinja-py/src/state.rs) implementation.
pub(crate) fn bind_lua<R, F: FnOnce() -> R>(lua: &Lua, f: F) -> R {
    let old_handle =
        CURRENT_LUA.with(|handle| handle.swap(lua as *const Lua as *mut Lua, Ordering::Relaxed));

    let rv = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));

    CURRENT_LUA.with(|handle| handle.store(old_handle, Ordering::Relaxed));
    match rv {
        Ok(rv) => rv,
        Err(payload) => std::panic::resume_unwind(payload),
    }
}
