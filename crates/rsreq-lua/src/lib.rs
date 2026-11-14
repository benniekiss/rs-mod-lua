// SPDX-License-Identifier: MIT

#[cfg_attr(feature = "module", mlua::lua_module(name = "rsreq"))]
pub fn rsreq_lua(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let table = lua.create_table()?;

    Ok(table)
}
