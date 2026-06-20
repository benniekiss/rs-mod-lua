// SPDX-License-Identifier: MIT

pub mod contrib;
mod convert;
mod environment;
mod lua;
mod state;

use mlua::LuaSerdeExt;

use crate::convert::LuaSyntaxConfig;
pub use crate::{
    environment::LuaEnvironment,
    state::{LuaStateMut, LuaStateRef},
};

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
pub fn minijinja_lua(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let table = lua.create_table()?;

    table.set("type", mlua::Function::wrap(contrib::minijinja_types))?;
    table.set("path_loader", contrib::minijinja_path_loader(lua)?)?;

    table.set("None", lua.null())?;
    table.set("Environment", lua.create_proxy::<LuaEnvironment>()?)?;
    table.set("SyntaxConfig", lua.create_proxy::<LuaSyntaxConfig>()?)?;

    Ok(table)
}
