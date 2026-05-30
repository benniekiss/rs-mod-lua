// SPDX-License-Identifier: MIT

mod re;

#[cfg_attr(feature = "module", mlua::lua_module(name = "rsre"))]
pub fn rsre_lua(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let table = lua.create_table()?;

    table.set("Regex", lua.create_proxy::<re::LuaRegex>()?)?;

    Ok(table)
}
