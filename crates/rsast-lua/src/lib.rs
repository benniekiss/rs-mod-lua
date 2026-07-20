// SPDX-License-Identifier: MIT

use crate::vm::LuaPestVm;

mod pairs;
mod vm;

#[cfg_attr(feature = "module", mlua::lua_module(name = "rsast"))]
pub fn rsast_lua(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let table = lua.create_table()?;

    table.set("Ast", lua.create_proxy::<LuaPestVm>()?)?;

    Ok(table)
}
