// SPDX-License-Identifier: MIT

mod re;

#[cfg_attr(feature = "module", mlua::lua_module(name = "rsre"))]
pub fn rsre_lua(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let table = lua.create_table()?;

    table.set("Regex", lua.create_proxy::<re::LuaRegex>()?)?;
    table.set(
        "escape",
        lua.create_function(|_, text: mlua::BorrowedStr| {
            Ok(fancy_regex::escape(&text).to_string())
        })?,
    )?;

    Ok(table)
}
