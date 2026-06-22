use crate::path::LuaPath;

pub(crate) fn dirs_lua(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let table = lua.create_table()?;

    table.set(
        "audio_dir",
        lua.create_function(|_, ()| Ok(dirs::audio_dir().map(LuaPath::from)))?,
    )?;
    table.set(
        "cache_dir",
        lua.create_function(|_, ()| Ok(dirs::cache_dir().map(LuaPath::from)))?,
    )?;
    table.set(
        "config_dir",
        lua.create_function(|_, ()| Ok(dirs::config_dir().map(LuaPath::from)))?,
    )?;
    table.set(
        "config_local_dir",
        lua.create_function(|_, ()| Ok(dirs::config_local_dir().map(LuaPath::from)))?,
    )?;
    table.set(
        "data_dir",
        lua.create_function(|_, ()| Ok(dirs::data_dir().map(LuaPath::from)))?,
    )?;
    table.set(
        "data_local_dir",
        lua.create_function(|_, ()| Ok(dirs::data_local_dir().map(LuaPath::from)))?,
    )?;
    table.set(
        "desktop_dir",
        lua.create_function(|_, ()| Ok(dirs::desktop_dir().map(LuaPath::from)))?,
    )?;
    table.set(
        "document_dir",
        lua.create_function(|_, ()| Ok(dirs::document_dir().map(LuaPath::from)))?,
    )?;
    table.set(
        "download_dir",
        lua.create_function(|_, ()| Ok(dirs::download_dir().map(LuaPath::from)))?,
    )?;
    table.set(
        "executable_dir",
        lua.create_function(|_, ()| Ok(dirs::executable_dir().map(LuaPath::from)))?,
    )?;
    table.set(
        "font_dir",
        lua.create_function(|_, ()| Ok(dirs::font_dir().map(LuaPath::from)))?,
    )?;
    table.set(
        "home_dir",
        lua.create_function(|_, ()| Ok(dirs::home_dir().map(LuaPath::from)))?,
    )?;
    table.set(
        "picture_dir",
        lua.create_function(|_, ()| Ok(dirs::picture_dir().map(LuaPath::from)))?,
    )?;
    table.set(
        "preference_dir",
        lua.create_function(|_, ()| Ok(dirs::preference_dir().map(LuaPath::from)))?,
    )?;
    table.set(
        "public_dir",
        lua.create_function(|_, ()| Ok(dirs::public_dir().map(LuaPath::from)))?,
    )?;
    table.set(
        "runtime_dir",
        lua.create_function(|_, ()| Ok(dirs::runtime_dir().map(LuaPath::from)))?,
    )?;
    table.set(
        "state_dir",
        lua.create_function(|_, ()| Ok(dirs::state_dir().map(LuaPath::from)))?,
    )?;
    table.set(
        "template_dir",
        lua.create_function(|_, ()| Ok(dirs::template_dir().map(LuaPath::from)))?,
    )?;
    table.set(
        "video_dir",
        lua.create_function(|_, ()| Ok(dirs::video_dir().map(LuaPath::from)))?,
    )?;

    Ok(table)
}
