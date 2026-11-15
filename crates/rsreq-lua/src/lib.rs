// SPDX-License-Identifier: MIT

mod client;

#[cfg_attr(feature = "module", mlua::lua_module(name = "rsreq"))]
pub fn rsreq_lua(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let table = lua.create_table()?;

    table.set("Client", lua.create_proxy::<client::LuaClient>()?)?;

    Ok(table)
}
