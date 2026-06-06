// SPDX-License-Identifier: MIT

use crate::fs::lfs_lua;

mod fs;

#[cfg_attr(feature = "module", mlua::lua_module(name = "rsfs"))]
pub fn rsfs_lua(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let table = lua.create_table()?;

    table.set("lfs", lfs_lua(lua)?)?;

    Ok(table)
}
