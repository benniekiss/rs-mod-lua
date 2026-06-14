use std::{
    fs,
    io::{BufRead, Read, Seek, SeekFrom, Write},
    time::{Duration, SystemTime},
};

use crate::{
    fs::{LuaLines, LuaMetadata, LuaOpenOptions, LuaPermissions, LuaSplit},
    path::LuaPath,
};

#[derive(mlua::UserData)]
pub(crate) struct LuaFile(fs::File);

impl From<fs::File> for LuaFile {
    fn from(value: fs::File) -> Self {
        LuaFile(value)
    }
}

impl From<LuaFile> for fs::File {
    fn from(value: LuaFile) -> Self {
        value.0
    }
}

impl mlua::FromLua for LuaFile {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::UserData(ud) => ud.take(),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "LuaFile".to_string(),
                message: Some("could not convert to File".to_string()),
            }),
        }
    }
}

#[mlua::userdata_impl]
impl LuaFile {
    pub(crate) fn split(&self, byte: u8) -> mlua::Result<LuaSplit> {
        let file = self.try_clone()?;
        let buf = std::io::BufReader::new(file.0);

        Ok(buf.split(byte).into())
    }

    pub(crate) fn lines(&self) -> mlua::Result<LuaLines> {
        let file = self.try_clone()?;
        let buf = std::io::BufReader::new(file.0);

        Ok(buf.lines().into())
    }

    pub(crate) fn open(path: LuaPath) -> mlua::Result<LuaFile> {
        fs::File::open(path)
            .map(|p| p.into())
            .map_err(mlua::Error::external)
    }

    pub(crate) fn create(path: LuaPath) -> mlua::Result<LuaFile> {
        fs::File::create(path)
            .map(|f| f.into())
            .map_err(mlua::Error::external)
    }

    pub(crate) fn create_new(path: LuaPath) -> mlua::Result<LuaFile> {
        fs::File::create_new(path)
            .map(|f| f.into())
            .map_err(mlua::Error::external)
    }

    #[lua(infallible)]
    pub(crate) fn options() -> LuaOpenOptions {
        fs::File::options().into()
    }

    #[lua(infallible)]
    pub(crate) fn close(self) {
        drop(self)
    }

    pub(crate) fn sync_all(&self) -> mlua::Result<()> {
        self.0.sync_all().map_err(mlua::Error::external)
    }

    pub(crate) fn sync_data(&self) -> mlua::Result<()> {
        self.0.sync_data().map_err(mlua::Error::external)
    }

    pub(crate) fn lock(&self) -> mlua::Result<()> {
        self.0.lock().map_err(mlua::Error::external)
    }

    pub(crate) fn lock_shared(&self) -> mlua::Result<()> {
        self.0.lock_shared().map_err(mlua::Error::external)
    }

    pub(crate) fn try_lock(&self) -> mlua::Result<()> {
        self.0.try_lock().map_err(mlua::Error::external)
    }

    pub(crate) fn try_lock_shared(&self) -> mlua::Result<()> {
        self.0.try_lock_shared().map_err(mlua::Error::external)
    }

    pub(crate) fn unlock(&self) -> mlua::Result<()> {
        self.0.unlock().map_err(mlua::Error::external)
    }

    pub(crate) fn set_len(&self, size: u64) -> mlua::Result<()> {
        self.0.set_len(size).map_err(mlua::Error::external)
    }

    pub(crate) fn metadata(&self) -> mlua::Result<LuaMetadata> {
        self.0
            .metadata()
            .map(|m| m.into())
            .map_err(mlua::Error::external)
    }

    #[lua(infallible)]
    pub(crate) fn try_clone(&self) -> mlua::Result<LuaFile> {
        self.0
            .try_clone()
            .map(|f| f.into())
            .map_err(mlua::Error::external)
    }

    #[lua(infallible)]
    pub(crate) fn set_permissions(&self, perm: LuaPermissions) -> mlua::Result<()> {
        self.0
            .set_permissions(perm.as_ref().clone())
            .map_err(mlua::Error::external)
    }

    #[lua(infallible)]
    pub(crate) fn set_times(&self, mtime: Option<u64>, atime: Option<u64>) -> mlua::Result<()> {
        let times = std::fs::FileTimes::new();

        let times = match mtime {
            Some(t) => {
                let mtime = SystemTime::UNIX_EPOCH.saturating_add(Duration::from_secs(t));
                times.set_modified(mtime)
            },
            None => times,
        };

        let times = match atime {
            Some(t) => {
                let atime = SystemTime::UNIX_EPOCH.saturating_add(Duration::from_secs(t));
                times.set_accessed(atime)
            },
            None => times,
        };

        self.0.set_times(times).map_err(mlua::Error::external)
    }
}

lua_read_methods!(LuaFile);

lua_write_methods!(LuaFile);
