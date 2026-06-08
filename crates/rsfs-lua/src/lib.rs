// SPDX-License-Identifier: MIT

mod fs;
mod lfs;

#[cfg_attr(feature = "module", mlua::lua_module(name = "rsfs"))]
pub fn rsfs_lua(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let table = fs::fs_lua(lua)?;

    table.set("lfs", lfs::lfs_lua(lua)?)?;

    Ok(table)
}
