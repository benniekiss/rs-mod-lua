// SPDX-License-Identifier: MIT

mod ast;

#[cfg_attr(feature = "module", mlua::lua_module(name = "rsast"))]
pub fn rsast_lua(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let table = lua.create_table()?;

    Ok(table)
}
