#[cfg(unix)]
use std::os::unix::fs::{DirEntryExt, FileTypeExt, MetadataExt, PermissionsExt};
#[cfg(windows)]
use std::os::windows::fs::{FileTypeExt, MetadataExt};
use std::{ffi::OsString, fs, time::SystemTime};

use crate::path::LuaPath;

#[derive(mlua::UserData)]
pub(crate) struct LuaMetadata(fs::Metadata);

impl From<fs::Metadata> for LuaMetadata {
    fn from(value: fs::Metadata) -> Self {
        LuaMetadata(value)
    }
}

#[mlua::userdata_impl]
impl LuaMetadata {
    #[lua(infallible)]
    pub(crate) fn file_type(&self) -> LuaFileType {
        self.0.file_type().into()
    }

    #[lua(infallible)]
    pub(crate) fn is_dir(&self) -> bool {
        self.0.is_dir()
    }

    #[lua(infallible)]
    pub(crate) fn is_file(&self) -> bool {
        self.0.is_file()
    }

    #[lua(infallible)]
    pub(crate) fn is_symlink(&self) -> bool {
        self.0.is_symlink()
    }

    #[lua(infallible)]
    pub(crate) fn len(&self) -> u64 {
        self.0.len()
    }

    #[lua(infallible)]
    pub(crate) fn permissions(&self) -> LuaPermissions {
        self.0.permissions().into()
    }

    pub(crate) fn modified(&self) -> mlua::Result<u64> {
        let time = self.0.modified().map_err(mlua::Error::external)?;
        time.duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .map_err(mlua::Error::external)
    }

    pub(crate) fn accessed(&self) -> mlua::Result<u64> {
        let time = self.0.accessed().map_err(mlua::Error::external)?;
        time.duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .map_err(mlua::Error::external)
    }

    pub(crate) fn created(&self) -> mlua::Result<u64> {
        let time = self.0.created().map_err(mlua::Error::external)?;
        time.duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .map_err(mlua::Error::external)
    }
}

#[cfg(unix)]
#[mlua::userdata_impl]
impl LuaMetadata {
    #[lua(infallible)]
    pub(crate) fn dev(&self) -> u64 {
        self.0.dev()
    }

    #[lua(infallible)]
    pub(crate) fn ino(&self) -> u64 {
        self.0.ino()
    }

    #[lua(infallible)]
    pub(crate) fn mode(&self) -> u32 {
        self.0.mode()
    }

    #[lua(infallible)]
    pub(crate) fn nlink(&self) -> u64 {
        self.0.nlink()
    }

    #[lua(infallible)]
    pub(crate) fn uid(&self) -> u32 {
        self.0.uid()
    }

    #[lua(infallible)]
    pub(crate) fn gid(&self) -> u32 {
        self.0.gid()
    }

    #[lua(infallible)]
    pub(crate) fn rdev(&self) -> u64 {
        self.0.rdev()
    }

    #[lua(infallible)]
    pub(crate) fn size(&self) -> u64 {
        self.0.size()
    }

    #[lua(infallible)]
    pub(crate) fn atime(&self) -> i64 {
        self.0.atime()
    }

    #[lua(infallible)]
    pub(crate) fn mtime(&self) -> i64 {
        self.0.mtime()
    }

    #[lua(infallible)]
    pub(crate) fn ctime(&self) -> i64 {
        self.0.ctime()
    }

    #[lua(infallible)]
    pub(crate) fn blksize(&self) -> u64 {
        self.0.blksize()
    }

    #[lua(infallible)]
    pub(crate) fn blocks(&self) -> u64 {
        self.0.blocks()
    }
}

#[cfg(windows)]
#[mlua::userdata_impl]
impl LuaMetadata {
    #[lua(infallible)]
    pub(crate) fn file_attributes(&self) -> u32 {
        self.0.file_attributes()
    }

    #[lua(infallible)]
    pub(crate) fn creation_time(&self) -> u64 {
        self.0.creation_time()
    }

    #[lua(infallible)]
    pub(crate) fn last_access_time(&self) -> u64 {
        self.0.last_access_time()
    }

    #[lua(infallible)]
    pub(crate) fn last_write_time(&self) -> u64 {
        self.0.last_write_time()
    }

    #[lua(infallible)]
    pub(crate) fn file_size(&self) -> u64 {
        self.0.file_size()
    }
}

#[derive(mlua::UserData)]
pub(crate) struct LuaFileType(fs::FileType);

impl From<fs::FileType> for LuaFileType {
    fn from(value: fs::FileType) -> Self {
        LuaFileType(value)
    }
}

#[mlua::userdata_impl]
impl LuaFileType {
    #[lua(infallible)]
    pub(crate) fn is_dir(&self) -> bool {
        self.0.is_dir()
    }

    #[lua(infallible)]
    pub(crate) fn is_file(&self) -> bool {
        self.0.is_file()
    }

    #[lua(infallible)]
    pub(crate) fn is_symlink(&self) -> bool {
        self.0.is_symlink()
    }
}

#[cfg(unix)]
#[mlua::userdata_impl]
impl LuaFileType {
    #[lua(infallible)]
    pub(crate) fn is_block_device(&self) -> bool {
        self.0.is_block_device()
    }

    #[lua(infallible)]
    pub(crate) fn is_char_device(&self) -> bool {
        self.0.is_char_device()
    }

    #[lua(infallible)]
    pub(crate) fn is_fifo(&self) -> bool {
        self.0.is_fifo()
    }

    #[lua(infallible)]
    pub(crate) fn is_socket(&self) -> bool {
        self.0.is_fifo()
    }
}

#[cfg(windows)]
#[mlua::userdata_impl]
impl LuaFileType {
    #[lua(infallible)]
    pub(crate) fn is_symlink_dir(&self) -> bool {
        self.0.is_symlink_dir()
    }

    #[lua(infallible)]
    pub(crate) fn is_symlink_file(&self) -> bool {
        self.0.is_symlink_file()
    }
}

#[derive(mlua::UserData, Clone)]
pub(crate) struct LuaPermissions(fs::Permissions);

impl From<fs::Permissions> for LuaPermissions {
    fn from(value: fs::Permissions) -> Self {
        LuaPermissions(value)
    }
}

impl AsRef<fs::Permissions> for LuaPermissions {
    fn as_ref(&self) -> &fs::Permissions {
        &self.0
    }
}

impl mlua::FromLua for LuaPermissions {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::UserData(ud) => Ok(ud.borrow::<Self>()?.clone()),
            mlua::Value::String(s) => Ok(LuaPermissions::from_perms(s.to_str()?.to_string())?),
            mlua::Value::Integer(int) => Ok(LuaPermissions::from_mode(
                u32::try_from(int).map_err(mlua::Error::external)?,
            )),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "LuaPermissions".to_string(),
                message: Some("could not convert to Permissions".to_string()),
            }),
        }
    }
}

#[mlua::userdata_impl]
impl LuaPermissions {
    #[lua(infallible)]
    pub(crate) fn readonly(&self) -> bool {
        self.0.readonly()
    }

    #[lua(infallible)]
    pub(crate) fn set_readonly(&mut self, readonly: bool) {
        self.0.set_readonly(readonly);
    }
}

#[cfg(unix)]
#[mlua::userdata_impl]
impl LuaPermissions {
    #[lua(infallible)]
    pub(crate) fn from_mode(mode: u32) -> LuaPermissions {
        fs::Permissions::from_mode(mode).into()
    }

    #[lua(infallible)]
    pub(crate) fn mode(&self) -> u32 {
        self.0.mode()
    }

    #[lua(infallible)]
    pub(crate) fn set_mode(&mut self, mode: u32) {
        self.0.set_mode(mode);
    }

    #[lua(infallible)]
    pub(crate) fn from_perms(perms: String) -> mlua::Result<LuaPermissions> {
        let mode = u32::from_str_radix(&perms, 8).map_err(mlua::Error::external)?;

        Ok(fs::Permissions::from_mode(mode).into())
    }

    #[lua(infallible)]
    pub(crate) fn perms(&self) -> String {
        format!("{:o}", self.0.mode())
    }

    #[lua(infallible)]
    pub(crate) fn set_perms(&mut self, perms: String) -> mlua::Result<()> {
        let mode = u32::from_str_radix(&perms, 8).map_err(mlua::Error::external)?;
        self.0.set_mode(mode);

        Ok(())
    }
}

#[derive(mlua::UserData)]
pub(crate) struct LuaDirEntry(fs::DirEntry);

impl From<fs::DirEntry> for LuaDirEntry {
    fn from(value: fs::DirEntry) -> Self {
        LuaDirEntry(value)
    }
}

#[mlua::userdata_impl]
impl LuaDirEntry {
    #[lua(infallible)]
    pub(crate) fn path(&self) -> LuaPath {
        self.0.path().into()
    }

    #[lua(infallible)]
    pub(crate) fn metadata(&self) -> mlua::Result<LuaMetadata> {
        self.0
            .metadata()
            .map(|m| m.into())
            .map_err(mlua::Error::external)
    }

    #[lua(infallible)]
    pub(crate) fn file_type(&self) -> mlua::Result<LuaFileType> {
        self.0
            .file_type()
            .map(|ft| ft.into())
            .map_err(mlua::Error::external)
    }

    #[lua(infallible)]
    pub(crate) fn file_name(&self) -> OsString {
        self.0.file_name()
    }
}

#[cfg(unix)]
#[mlua::userdata_impl]
impl LuaDirEntry {
    #[lua(infallible)]
    pub(crate) fn ino(&self) -> u64 {
        self.0.ino()
    }
}

#[derive(mlua::UserData)]
pub(crate) struct LuaReadDir(fs::ReadDir);

impl From<fs::ReadDir> for LuaReadDir {
    fn from(value: fs::ReadDir) -> Self {
        LuaReadDir(value)
    }
}

#[mlua::userdata_impl]
impl LuaReadDir {
    pub(crate) fn next(&mut self) -> mlua::Result<Option<LuaDirEntry>> {
        self.0
            .next()
            .transpose()
            .map(|v| v.map(|d| d.into()))
            .map_err(mlua::Error::external)
    }
}
