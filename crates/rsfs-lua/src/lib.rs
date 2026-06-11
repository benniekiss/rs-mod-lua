// SPDX-License-Identifier: MIT

#![feature(
    trim_prefix_suffix,
    path_is_empty,
    path_trailing_sep,
    path_absolute_method,
    fs_set_times,
    const_path_separators,
    time_saturating_systemtime
)]

mod fs;
mod path;

use std::time::{Duration, SystemTime};

use crate::{
    fs::{LuaMetadata, LuaPermissions, LuaReadDir},
    path::LuaPath,
};

#[cfg_attr(feature = "module", mlua::lua_module(name = "rsfs"))]
pub fn rsfs_lua(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let table = lua.create_table()?;

    table.set("MAIN_SEPARATOR", std::path::MAIN_SEPARATOR_STR)?;

    table.set(
        "SEPARATORS_STR",
        lua.create_sequence_from(std::path::SEPARATORS_STR.iter().map(|s| s.to_string()))?,
    )?;

    table.set("Path", lua.create_proxy::<LuaPath>()?)?;

    table.set(
        "canonicalize",
        lua.create_function(|_, path: LuaPath| -> mlua::Result<LuaPath> {
            std::fs::canonicalize(path)
                .map(|p| p.into())
                .map_err(mlua::Error::external)
        })?,
    )?;

    table.set(
        "copy",
        lua.create_function(|_, (from, to): (LuaPath, LuaPath)| -> mlua::Result<u64> {
            std::fs::copy(from, to).map_err(mlua::Error::external)
        })?,
    )?;

    table.set(
        "create_dir",
        lua.create_function(|_, path: LuaPath| -> mlua::Result<()> {
            std::fs::create_dir(path).map_err(mlua::Error::external)
        })?,
    )?;

    table.set(
        "create_dir_all",
        lua.create_function(|_, path: LuaPath| -> mlua::Result<()> {
            std::fs::create_dir_all(path).map_err(mlua::Error::external)
        })?,
    )?;

    table.set(
        "exists",
        lua.create_function(|_, path: LuaPath| -> mlua::Result<bool> {
            std::fs::exists(path).map_err(mlua::Error::external)
        })?,
    )?;

    table.set(
        "hard_link",
        lua.create_function(
            |_, (original, link): (LuaPath, LuaPath)| -> mlua::Result<()> {
                std::fs::hard_link(original, link).map_err(mlua::Error::external)
            },
        )?,
    )?;

    table.set(
        "metadata",
        lua.create_function(|_, path: LuaPath| -> mlua::Result<LuaMetadata> {
            std::fs::metadata(path)
                .map(|m| m.into())
                .map_err(mlua::Error::external)
        })?,
    )?;

    table.set(
        "symlink_metadata",
        lua.create_function(|_, path: LuaPath| -> mlua::Result<LuaMetadata> {
            std::fs::symlink_metadata(path)
                .map(|m| m.into())
                .map_err(mlua::Error::external)
        })?,
    )?;

    table.set(
        "read",
        lua.create_function(|_, path: LuaPath| -> mlua::Result<Vec<u8>> {
            std::fs::read(path).map_err(mlua::Error::external)
        })?,
    )?;

    table.set(
        "read_dir",
        lua.create_function(|_, path: LuaPath| -> mlua::Result<LuaReadDir> {
            std::fs::read_dir(path)
                .map(|d| d.into())
                .map_err(mlua::Error::external)
        })?,
    )?;

    table.set(
        "read_link",
        lua.create_function(|_, path: LuaPath| -> mlua::Result<LuaPath> {
            std::fs::read_link(path)
                .map(|p| p.into())
                .map_err(mlua::Error::external)
        })?,
    )?;

    table.set(
        "remove_dir",
        lua.create_function(|_, path: LuaPath| -> mlua::Result<()> {
            std::fs::remove_dir(path).map_err(mlua::Error::external)
        })?,
    )?;

    table.set(
        "remove_dir_all",
        lua.create_function(|_, path: LuaPath| -> mlua::Result<()> {
            std::fs::remove_dir_all(path).map_err(mlua::Error::external)
        })?,
    )?;

    table.set(
        "remove_file",
        lua.create_function(|_, path: LuaPath| -> mlua::Result<()> {
            std::fs::remove_file(path).map_err(mlua::Error::external)
        })?,
    )?;

    table.set(
        "rename",
        lua.create_function(|_, (from, to): (LuaPath, LuaPath)| -> mlua::Result<()> {
            std::fs::rename(from, to).map_err(mlua::Error::external)
        })?,
    )?;

    table.set(
        "write",
        lua.create_function(
            |_, (path, content): (LuaPath, String)| -> mlua::Result<()> {
                std::fs::write(path, content).map_err(mlua::Error::external)
            },
        )?,
    )?;

    table.set(
        "set_permissions",
        lua.create_function(
            |_, (path, perm): (LuaPath, LuaPermissions)| -> mlua::Result<()> {
                std::fs::set_permissions(path, perm.as_ref().clone()).map_err(mlua::Error::external)
            },
        )?,
    )?;

    table.set(
        "set_times",
        lua.create_function(
            |_, (path, atime, mtime): (LuaPath, Option<u64>, Option<u64>)| -> mlua::Result<()> {
                let now = SystemTime::now();

                let mtime = match mtime {
                    Some(t) => SystemTime::UNIX_EPOCH.saturating_add(Duration::from_secs(t)),
                    None => now,
                };

                let atime = match atime {
                    Some(t) => SystemTime::UNIX_EPOCH.saturating_add(Duration::from_secs(t)),
                    None => now,
                };

                let times = std::fs::FileTimes::new()
                    .set_modified(mtime)
                    .set_accessed(atime);

                std::fs::set_times(path, times).map_err(mlua::Error::external)
            },
        )?,
    )?;

    table.set(
        "set_current_dir",
        lua.create_function(|_, path: LuaPath| -> mlua::Result<()> {
            std::env::set_current_dir(path).map_err(mlua::Error::external)
        })?,
    )?;

    table.set(
        "current_dir",
        lua.create_function(|_, _: mlua::Value| -> mlua::Result<LuaPath> {
            std::env::current_dir()
                .map(|p| p.into())
                .map_err(mlua::Error::external)
        })?,
    )?;

    table.set(
        "home_dir",
        lua.create_function(|_, _: mlua::Value| -> mlua::Result<Option<LuaPath>> {
            Ok(std::env::home_dir().map(|p| p.into()))
        })?,
    )?;

    table.set(
        "temp_dir",
        lua.create_function(|_, _: mlua::Value| -> mlua::Result<LuaPath> {
            Ok(std::env::temp_dir().into())
        })?,
    )?;

    Ok(table)
}
