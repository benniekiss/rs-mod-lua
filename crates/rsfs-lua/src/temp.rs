use std::{
    ffi::{OsStr, OsString},
    io::{Read, Seek, SeekFrom, Write},
    ops::Deref,
    path,
};

use crate::{
    file::LuaFile,
    fs::{LuaMetadata, LuaPermissions, LuaReadDir},
    path::LuaPath,
};

#[derive(mlua::UserData)]
pub(crate) struct LuaTempPath(tempfile::TempPath);

impl From<tempfile::TempPath> for LuaTempPath {
    fn from(value: tempfile::TempPath) -> Self {
        Self(value)
    }
}

impl From<LuaTempPath> for tempfile::TempPath {
    fn from(value: LuaTempPath) -> Self {
        value.0
    }
}

impl AsRef<path::Path> for LuaTempPath {
    fn as_ref(&self) -> &path::Path {
        &self.0
    }
}

impl AsRef<OsStr> for LuaTempPath {
    fn as_ref(&self) -> &OsStr {
        self.0.as_os_str()
    }
}

impl Deref for LuaTempPath {
    type Target = tempfile::TempPath;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl mlua::FromLua for LuaTempPath {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::UserData(ud) => ud.take(),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "LuaTempPath".to_string(),
                message: Some("could not convert to LuaTempPath".to_string()),
            }),
        }
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

lua_path_methods!(LuaTempPath);

#[derive(mlua::UserData)]
pub(crate) struct LuaTempDir(tempfile::TempDir);

impl From<tempfile::TempDir> for LuaTempDir {
    fn from(value: tempfile::TempDir) -> Self {
        Self(value)
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

#[derive(mlua::UserData)]
pub(crate) struct LuaNamedTempFile(tempfile::NamedTempFile);

impl From<tempfile::NamedTempFile> for LuaNamedTempFile {
    fn from(value: tempfile::NamedTempFile) -> Self {
        Self(value)
    }
}

#[mlua::userdata_impl]
impl LuaNamedTempFile {
    #[lua(name = "new")]
    pub(crate) fn lua_new() -> mlua::Result<Self> {
        tempfile::NamedTempFile::new()
            .map(|nt| nt.into())
            .map_err(mlua::Error::external)
    }

    #[lua(name = "new_in")]
    pub(crate) fn lua_new_in(dir: LuaPath) -> mlua::Result<Self> {
        tempfile::NamedTempFile::new_in(dir)
            .map(|nt| nt.into())
            .map_err(mlua::Error::external)
    }

    #[lua(name = "with_prefix")]
    pub(crate) fn lua_with_prefix(prefix: LuaPath) -> mlua::Result<Self> {
        tempfile::NamedTempFile::with_prefix(prefix)
            .map(|nt| nt.into())
            .map_err(mlua::Error::external)
    }

    #[lua(name = "with_prefix_in")]
    pub(crate) fn lua_with_prefix_in(prefix: LuaPath, dir: LuaPath) -> mlua::Result<Self> {
        tempfile::NamedTempFile::with_prefix_in(prefix, dir)
            .map(|nt| nt.into())
            .map_err(mlua::Error::external)
    }

    #[lua(name = "with_suffix")]
    pub(crate) fn lua_with_suffix(suffix: LuaPath) -> mlua::Result<Self> {
        tempfile::NamedTempFile::with_suffix(suffix)
            .map(|nt| nt.into())
            .map_err(mlua::Error::external)
    }

    #[lua(name = "with_suffix_in")]
    pub(crate) fn lua_with_suffix_in(suffix: LuaPath, dir: LuaPath) -> mlua::Result<Self> {
        tempfile::NamedTempFile::with_suffix_in(suffix, dir)
            .map(|nt| nt.into())
            .map_err(mlua::Error::external)
    }

    #[lua(name = "path", infallible)]
    pub(crate) fn lua_path(&self) -> LuaPath {
        self.0.path().into()
    }

    #[lua(name = "close")]
    pub(crate) fn lua_close(self) -> mlua::Result<()> {
        self.0.close().map_err(mlua::Error::external)
    }

    #[lua(name = "persist")]
    pub(crate) fn lua_persist(self, new_path: LuaPath) -> mlua::Result<LuaFile> {
        self.0
            .persist(new_path)
            .map(|nt| nt.into())
            .map_err(mlua::Error::external)
    }

    #[lua(name = "persist_noclobber")]
    pub(crate) fn lua_persist_noclobber(self, new_path: LuaPath) -> mlua::Result<LuaFile> {
        self.0
            .persist_noclobber(new_path)
            .map(|nt| nt.into())
            .map_err(mlua::Error::external)
    }

    #[lua(name = "keep")]
    pub(crate) fn lua_keep(self) -> mlua::Result<(LuaFile, LuaPath)> {
        let (f, p) = self.0.keep().map_err(mlua::Error::external)?;
        Ok((f.into(), p.into()))
    }

    #[lua(name = "disable_cleanup", infallible)]
    pub(crate) fn lua_disable_cleanup(&mut self, disable_cleanup: bool) {
        self.0.disable_cleanup(disable_cleanup)
    }

    #[lua(name = "into_file", infallible)]
    pub(crate) fn lua_into_file(self) -> LuaFile {
        self.0.into_file().into()
    }

    #[lua(name = "into_temp_path", infallible)]
    pub(crate) fn lua_into_temp_path(self) -> LuaTempPath {
        self.0.into_temp_path().into()
    }

    #[lua(name = "into_parts", infallible)]
    pub(crate) fn lua_into_parts(self) -> (LuaFile, LuaTempPath) {
        let (f, p) = self.0.into_parts();
        (f.into(), p.into())
    }

    #[lua(name = "from_parts", infallible)]
    pub(crate) fn lua_from_parts(file: LuaFile, path: LuaTempPath) -> Self {
        tempfile::NamedTempFile::from_parts(file.into(), path.into()).into()
    }

    #[lua(name = "reopen")]
    pub(crate) fn lua_reopen(&self) -> mlua::Result<LuaFile> {
        self.0
            .reopen()
            .map(|nt| nt.into())
            .map_err(mlua::Error::external)
    }
}

lua_read_methods!(LuaNamedTempFile);

lua_write_methods!(LuaNamedTempFile);

#[derive(mlua::UserData)]
pub(crate) struct LuaSpooledTempfile(tempfile::SpooledTempFile);

impl From<tempfile::SpooledTempFile> for LuaSpooledTempfile {
    fn from(value: tempfile::SpooledTempFile) -> Self {
        Self(value)
    }
}

impl Deref for LuaSpooledTempfile {
    type Target = tempfile::SpooledTempFile;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[mlua::userdata_impl]
impl LuaSpooledTempfile {
    #[lua(name = "new", infallible)]
    pub(crate) fn lua_new(max_size: usize) -> Self {
        tempfile::SpooledTempFile::new(max_size).into()
    }

    #[lua(name = "new_in", infallible)]
    pub(crate) fn lua_new_in(max_size: usize, dir: LuaPath) -> Self {
        tempfile::SpooledTempFile::new_in(max_size, dir).into()
    }

    #[lua(name = "is_rolled", infallible)]
    pub(crate) fn lua_is_rolled(&self) -> bool {
        self.0.is_rolled()
    }

    #[lua(name = "roll")]
    pub(crate) fn lua_roll(&mut self) -> mlua::Result<()> {
        self.0.roll().map_err(mlua::Error::external)
    }

    #[lua(name = "set_len")]
    pub(crate) fn lua_set_len(&mut self, size: u64) -> mlua::Result<()> {
        self.0.set_len(size).map_err(mlua::Error::external)
    }

    #[lua(name = "into_file")]
    pub(crate) fn lua_into_file(self) -> mlua::Result<LuaFile> {
        self.0
            .into_file()
            .map(|st| st.into())
            .map_err(mlua::Error::external)
    }
}

lua_read_methods!(LuaSpooledTempfile);

lua_write_methods!(LuaSpooledTempfile);

#[derive(mlua::UserData, Clone)]
pub(crate) struct LuaTempBuilder {
    #[lua(skip)]
    prefix: LuaPath,
    #[lua(skip)]
    suffix: LuaPath,
    #[lua(skip)]
    rand_bytes: usize,
    #[lua(skip)]
    append: bool,
    #[lua(skip)]
    permissions: Option<LuaPermissions>,
    #[lua(skip)]
    disable_cleanup: bool,
}

#[mlua::userdata_impl]
impl LuaTempBuilder {
    #[lua(skip)]
    fn build<F, R>(&self, f: F) -> R
    where
        F: FnOnce(tempfile::Builder) -> R,
    {
        let mut builder = tempfile::Builder::new();
        builder
            .prefix(&self.prefix)
            .suffix(&self.suffix)
            .rand_bytes(self.rand_bytes)
            .append(self.append)
            .disable_cleanup(self.disable_cleanup);

        if let Some(perms) = &self.permissions {
            builder.permissions(perms.clone().into());
        };

        f(builder)
    }

    #[lua(name = "new", infallible)]
    pub(crate) fn lua_new() -> Self {
        Self {
            prefix: LuaPath::lua_new(),
            suffix: LuaPath::lua_new(),
            rand_bytes: 6,
            append: false,
            permissions: None,
            disable_cleanup: false,
        }
    }

    #[lua(name = "prefix", infallible)]
    pub(crate) fn lua_prefix(&mut self, prefix: LuaPath) -> Self {
        self.prefix = prefix;
        self.clone()
    }

    #[lua(name = "suffix", infallible)]
    pub(crate) fn lua_suffix(&mut self, suffix: LuaPath) -> Self {
        self.suffix = suffix;
        self.clone()
    }

    #[lua(name = "rand_bytes", infallible)]
    pub(crate) fn lua_rand_bytes(&mut self, rand: usize) -> Self {
        self.rand_bytes = rand;
        self.clone()
    }

    #[lua(name = "append", infallible)]
    pub(crate) fn lua_append(&mut self, append: bool) -> Self {
        self.append = append;
        self.clone()
    }

    #[lua(name = "permissions", infallible)]
    pub(crate) fn lua_permissions(&mut self, permissions: LuaPermissions) -> Self {
        self.permissions = Some(permissions);
        self.clone()
    }

    #[lua(name = "disable_cleanup", infallible)]
    pub(crate) fn lua_disable_cleanup(&mut self, disable_cleanup: bool) -> Self {
        self.disable_cleanup = disable_cleanup;
        self.clone()
    }

    #[lua(name = "tempfile", infallible)]
    pub(crate) fn lua_tempfile(&self) -> mlua::Result<LuaNamedTempFile> {
        self.build(|builder| builder.tempfile())
            .map(|nt| nt.into())
            .map_err(mlua::Error::external)
    }

    #[lua(name = "tempfile_in", infallible)]
    pub(crate) fn lua_tempfile_in(&self, dir: LuaPath) -> mlua::Result<LuaNamedTempFile> {
        self.build(|builder| builder.tempfile_in(dir))
            .map(|nt| nt.into())
            .map_err(mlua::Error::external)
    }

    #[lua(name = "tempdir", infallible)]
    pub(crate) fn lua_tempdir(&self) -> mlua::Result<LuaTempDir> {
        self.build(|builder| builder.tempdir())
            .map(|td| td.into())
            .map_err(mlua::Error::external)
    }

    #[lua(name = "tempdir_in", infallible)]
    pub(crate) fn lua_tempdir_in(&self, dir: LuaPath) -> mlua::Result<LuaTempDir> {
        self.build(|builder| builder.tempdir_in(dir))
            .map(|td| td.into())
            .map_err(mlua::Error::external)
    }

    #[lua(name = "make", infallible)]
    pub(crate) fn lua_make(&self, f: mlua::Function) -> mlua::Result<LuaNamedTempFile> {
        self.build(|builder| {
            builder.make(|p| {
                f.call::<LuaFile>(p)
                    .map(|f| f.into())
                    .map_err(|err| std::io::Error::other(err.to_string()))
            })
        })
        .map(|nt| nt.into())
        .map_err(mlua::Error::external)
    }

    #[lua(name = "make_in", infallible)]
    pub(crate) fn lua_make_in(
        &self,
        dir: LuaPath,
        f: mlua::Function,
    ) -> mlua::Result<LuaNamedTempFile> {
        self.build(|builder| {
            builder.make_in(dir, |p| {
                f.call::<LuaFile>(p)
                    .map(|f| f.into())
                    .map_err(|err| std::io::Error::other(err.to_string()))
            })
        })
        .map(|nt| nt.into())
        .map_err(mlua::Error::external)
    }
}

pub(crate) fn temp_lua(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let table = lua.create_table()?;

    table.set("Builder", lua.create_proxy::<LuaTempBuilder>()?)?;

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
        "spooled_tempfile",
        lua.create_function(|_, max_size: usize| -> mlua::Result<LuaSpooledTempfile> {
            Ok(tempfile::spooled_tempfile(max_size).into())
        })?,
    )?;

    table.set(
        "spooled_tempfile_in",
        lua.create_function(
            |_, (max_size, dir): (usize, LuaPath)| -> mlua::Result<LuaSpooledTempfile> {
                Ok(tempfile::spooled_tempfile_in(max_size, dir).into())
            },
        )?,
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
