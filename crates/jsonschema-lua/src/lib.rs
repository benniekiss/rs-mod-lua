// SPDX-License-Identifier: MIT

mod lua;
mod schema;

#[cfg_attr(feature = "module", mlua::lua_module(name = "jsonschema"))]
pub fn jsonschema_lua(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let table = schema::jsonschema_lua(lua)?;

    Ok(table)
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup_lua() -> mlua::Lua {
        let lua = mlua::Lua::new();

        let table = jsonschema_lua(&lua).unwrap();
        lua.globals().set("jsonschema", table).unwrap();

        lua
    }

    #[test]
    fn it_rsjson_table() {
        let lua = setup_lua();

        let table: mlua::Table = lua.globals().get("rsjson").unwrap();

        let encode_func: mlua::Value = table.get("encode").unwrap();
        let decode_func: mlua::Value = table.get("decode").unwrap();
        let null_val: mlua::Value = table.get("null").unwrap();
        let enc_conf: mlua::Value = table.get("EncodeConfig").unwrap();
        let dec_conf: mlua::Value = table.get("DecodeConfig").unwrap();

        assert!(encode_func.is_function());
        assert!(decode_func.is_function());
        assert!(null_val.is_null());
        assert!(enc_conf.is_userdata());
        assert!(dec_conf.is_userdata());
    }
}
