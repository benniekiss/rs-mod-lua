// SPDX-License-Identifier: MIT

mod draft;
mod evaluation;
mod lua;
mod options;
mod retriever;
mod schema;
mod uri;
mod validator;

#[cfg_attr(feature = "module", mlua::lua_module(name = "jsonschema"))]
pub fn jsonschema_lua(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let table = schema::jsonschema_lua(lua)?;

    Ok(table)
}
