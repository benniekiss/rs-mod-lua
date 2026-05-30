// SPDX-License-Identifier: MIT

mod config;
mod decode;
mod encode;

use mlua::prelude::{Lua, LuaError, LuaResult, LuaSerdeExt, LuaString, LuaTable, LuaValue};

use crate::config::{DecodeConfig, EncodeConfig};

#[cfg_attr(feature = "module", mlua::lua_module(name = "rsjson_lua"))]
pub fn rsjson_lua(lua: &Lua) -> LuaResult<LuaTable> {
    let table = lua.create_table()?;

    table.set("null", lua.null())?;

    table.set("EncodeConfig", lua.create_proxy::<EncodeConfig>()?)?;

    table.set("DecodeConfig", lua.create_proxy::<DecodeConfig>()?)?;

    table.set(
        "encode",
        lua.create_function(|lua, (value, config): (LuaValue, Option<EncodeConfig>)| {
            encode::encode(lua, &value, config).map_err(LuaError::external)
        })?,
    )?;

    table.set(
        "decode",
        lua.create_function(|lua, (json, config): (LuaString, Option<DecodeConfig>)| {
            decode::decode(lua, &json.as_bytes(), config).map_err(LuaError::external)
        })?,
    )?;

    Ok(table)
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup_lua() -> Lua {
        let lua = Lua::new();

        let table = rsjson_lua(&lua).unwrap();
        lua.globals().set("rsjson", table).unwrap();

        lua
    }

    #[test]
    fn it_rsjson_table() {
        let lua = setup_lua();

        let table: LuaTable = lua.globals().get("rsjson").unwrap();

        let encode_func: LuaValue = table.get("encode").unwrap();
        let decode_func: LuaValue = table.get("decode").unwrap();
        let null_val: LuaValue = table.get("null").unwrap();
        let enc_conf: LuaValue = table.get("EncodeConfig").unwrap();
        let dec_conf: LuaValue = table.get("DecodeConfig").unwrap();

        assert!(encode_func.is_function());
        assert!(decode_func.is_function());
        assert!(null_val.is_null());
        assert!(enc_conf.is_userdata());
        assert!(dec_conf.is_userdata());
    }
}
