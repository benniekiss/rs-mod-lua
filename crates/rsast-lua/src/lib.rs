// SPDX-License-Identifier: MIT

use std::num::NonZeroUsize;

use crate::vm::LuaPestVm;

mod lines;
mod pairs;
mod tokens;
mod vm;

#[cfg_attr(feature = "module", mlua::lua_module(name = "rsast"))]
pub fn rsast_lua(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let table = lua.create_table()?;

    table.set(
        "set_call_limit",
        lua.create_function(|_, limit: Option<usize>| {
            let limit = limit.and_then(NonZeroUsize::new);
            pest::set_call_limit(limit);
            Ok(())
        })?,
    )?;

    table.set(
        "set_error_detail",
        lua.create_function(|_, enable: bool| {
            pest::set_error_detail(enable);
            Ok(())
        })?,
    )?;

    table.set("Ast", lua.create_proxy::<LuaPestVm>()?)?;

    Ok(table)
}
