// SPDX-License-Identifier: MIT

mod config;
mod decode;
mod encode;

use config::{DecodeConfig, EncodeConfig};
use mlua::LuaSerdeExt;

#[cfg_attr(feature = "module", mlua::lua_module(name = "rsjson"))]
pub fn rsjson_lua(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let table = lua.create_table()?;

    table.set("null", lua.null())?;

    table.set("EncodeConfig", lua.create_proxy::<EncodeConfig>()?)?;

    table.set("DecodeConfig", lua.create_proxy::<DecodeConfig>()?)?;

    table.set(
        "encode",
        lua.create_function(
            |lua, (value, config): (mlua::Value, Option<EncodeConfig>)| {
                encode::encode(lua, &value, config)
            },
        )?,
    )?;

    table.set(
        "decode",
        lua.create_function(
            |lua, (json, config): (mlua::String, Option<DecodeConfig>)| {
                decode::decode(lua, &json.as_bytes(), config)
            },
        )?,
    )?;

    Ok(table)
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup_lua() -> mlua::Lua {
        let lua = mlua::Lua::new();

        let table = rsjson_lua(&lua).unwrap();
        lua.globals().set("rsjson", table).unwrap();

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
