// SPDX-License-Identifier: MIT

pub mod contrib;
mod convert;
mod environment;
mod state;

use mlua::prelude::{Lua, LuaFunction, LuaResult, LuaTable, LuaValue};

pub use crate::{environment::LuaEnvironment, state::LuaState};

/// Builds and returns the `minijinja` lua table.
///
/// The returned table has the following keys:
///
/// - [`Environment`](LuaEnvironment): `userdata` provides access to a [`minijinja::Environment`]
/// - [`None`](mlua::Value::NULL): a nil-like `lightuserdata` that can be used without the downsides
///   of `nil`. It maps to [`minijinja::value::ValueKind::None`]
/// - `path_loader()`: creates a callback function which can be passed to `Environment:set_loader()`
///   to load templates from the filesystem.
/// - `type()`: a function to return types for minijinja-lua objects.
#[cfg_attr(feature = "module", mlua::lua_module(name = "minijinja"))]
pub fn minijinja_lua(lua: &Lua) -> LuaResult<LuaTable> {
    let table = lua.create_table()?;

    table.set(
        "type",
        lua.create_function(|_, val: LuaValue| contrib::minijinja_types(&val))?,
    )?;

    let path_loader = contrib::minijinja_path_loader(lua)?;
    table.set(
        "path_loader",
        lua.create_function(move |_, val: LuaValue| -> Result<LuaFunction, _> {
            path_loader.call(val)
        })?,
    )?;

    table.set("None", LuaValue::NULL)?;
    table.set("Environment", lua.create_proxy::<LuaEnvironment>()?)?;

    Ok(table)
}
