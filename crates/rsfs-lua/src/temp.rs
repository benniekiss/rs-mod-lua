use std::ops::Deref;

use crate::{file::LuaFile, path::LuaPath};

#[derive(mlua::UserData)]
pub(crate) struct LuaTempPath(tempfile::TempPath);

impl From<tempfile::TempPath> for LuaTempPath {
    fn from(value: tempfile::TempPath) -> Self {
        Self(value)
    }
}

impl Deref for LuaTempPath {
    type Target = tempfile::TempPath;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[mlua::userdata_impl]
impl LuaTempPath {
    #[lua(name = "close")]
    pub(crate) fn lua_close(self) -> mlua::Result<()> {
        self.0.close().map_err(mlua::Error::external)
    }

    #[lua(name = "persist")]
    pub(crate) fn lua_persist(self, new_path: LuaPath) -> mlua::Result<()> {
        self.0.persist(new_path).map_err(mlua::Error::external)
    }

    #[lua(name = "persist_noclobber")]
    pub(crate) fn lua_persist_noclobber(self, new_path: LuaPath) -> mlua::Result<()> {
        self.0
            .persist_noclobber(new_path)
            .map_err(mlua::Error::external)
    }

    #[lua(name = "keep")]
    pub(crate) fn lua_keep(self) -> mlua::Result<LuaPath> {
        self.0
            .keep()
            .map(|p| p.into())
            .map_err(mlua::Error::external)
    }

    #[lua(name = "disable_cleanup", infallible)]
    pub(crate) fn lua_disable_cleanup(&mut self, disable_cleanup: bool) {
        self.0.disable_cleanup(disable_cleanup)
    }

    #[lua(name = "try_from_path", infallible)]
    pub(crate) fn lua_try_from_path(path: LuaPath) -> mlua::Result<Self> {
        tempfile::TempPath::try_from_path(&path)
            .map(|tp| tp.into())
            .map_err(mlua::Error::external)
    }
}

#[derive(mlua::UserData)]
pub(crate) struct LuaTempDir(tempfile::TempDir);

impl From<tempfile::TempDir> for LuaTempDir {
    fn from(value: tempfile::TempDir) -> Self {
        Self(value)
    }
}

impl Deref for LuaTempDir {
    type Target = tempfile::TempDir;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[mlua::userdata_impl]
impl LuaTempDir {
    #[lua(name = "new")]
    pub(crate) fn lua_new() -> mlua::Result<Self> {
        tempfile::TempDir::new()
            .map(|td| td.into())
            .map_err(mlua::Error::external)
    }

    #[lua(name = "new_in")]
    pub(crate) fn lua_new_in(dir: LuaPath) -> mlua::Result<Self> {
        tempfile::TempDir::new_in(dir)
            .map(|td| td.into())
            .map_err(mlua::Error::external)
    }

    #[lua(name = "with_prefix")]
    pub(crate) fn lua_with_prefix(prefix: LuaPath) -> mlua::Result<Self> {
        tempfile::TempDir::with_prefix(prefix)
            .map(|td| td.into())
            .map_err(mlua::Error::external)
    }

    #[lua(name = "with_prefix_in")]
    pub(crate) fn lua_with_prefix_in(prefix: LuaPath, dir: LuaPath) -> mlua::Result<Self> {
        tempfile::TempDir::with_prefix_in(prefix, dir)
            .map(|td| td.into())
            .map_err(mlua::Error::external)
    }

    #[lua(name = "with_suffix")]
    pub(crate) fn lua_with_suffix(suffix: LuaPath) -> mlua::Result<Self> {
        tempfile::TempDir::with_suffix(suffix)
            .map(|td| td.into())
            .map_err(mlua::Error::external)
    }

    #[lua(name = "with_suffix_in")]
    pub(crate) fn lua_with_suffix_in(suffix: LuaPath, dir: LuaPath) -> mlua::Result<Self> {
        tempfile::TempDir::with_suffix_in(suffix, dir)
            .map(|td| td.into())
            .map_err(mlua::Error::external)
    }

    #[lua(name = "path", infallible)]
    pub(crate) fn lua_path(&self) -> LuaPath {
        self.0.path().into()
    }

    #[lua(name = "keep", infallible)]
    pub(crate) fn lua_keep(self) -> LuaPath {
        self.0.keep().into()
    }

    #[lua(name = "disable_cleanup", infallible)]
    pub(crate) fn lua_disable_cleanup(&mut self, disable_cleanup: bool) {
        self.0.disable_cleanup(disable_cleanup)
    }

    #[lua(name = "close")]
    pub(crate) fn lua_close(self) -> mlua::Result<()> {
        self.0.close().map_err(mlua::Error::external)
    }
}

// pub(crate) struct LuaNamedTempFile(tempfile::NamedTempFile);

// pub(crate) struct LuaSpooledTempfile(tempfile::SpooledTempFile);

// pub(crate) struct LuaTempBuilder(tempfile::Builder<'static, 'static>);

pub(crate) fn temp_lua(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let table = lua.create_table()?;

    table.set(
        "tempfile",
        lua.create_function(|_, _: ()| -> mlua::Result<LuaFile> {
            tempfile::tempfile()
                .map(|f| f.into())
                .map_err(mlua::Error::external)
        })?,
    )?;

    table.set(
        "tempfile_in",
        lua.create_function(|_, dir: LuaPath| -> mlua::Result<LuaFile> {
            tempfile::tempfile_in(dir)
                .map(|f| f.into())
                .map_err(mlua::Error::external)
        })?,
    )?;

    table.set(
        "tempdir",
        lua.create_function(|_, _: ()| -> mlua::Result<LuaTempDir> {
            tempfile::tempdir()
                .map(|d| d.into())
                .map_err(mlua::Error::external)
        })?,
    )?;

    table.set(
        "tempdir_in",
        lua.create_function(|_, dir: LuaPath| -> mlua::Result<LuaTempDir> {
            tempfile::tempdir_in(dir)
                .map(|d| d.into())
                .map_err(mlua::Error::external)
        })?,
    )?;

    Ok(table)
}
