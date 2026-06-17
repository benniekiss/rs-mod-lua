#[cfg(unix)]
use std::os::unix::fs::{DirEntryExt, FileTypeExt, MetadataExt, OpenOptionsExt, PermissionsExt};
#[cfg(windows)]
use std::os::windows::fs::{FileTypeExt, MetadataExt, OpenOptionsExt};
use std::{
    ffi::OsString,
    fs,
    io::{BufReader, Lines, Split},
    ops::Deref,
    sync::{Arc, Mutex},
    time::SystemTime,
};

use crate::{file::LuaFile, path::LuaPath};

#[derive(mlua::UserData, Clone)]
pub(crate) struct LuaMetadata(fs::Metadata);

impl From<fs::Metadata> for LuaMetadata {
    fn from(value: fs::Metadata) -> Self {
        LuaMetadata(value)
    }
}

impl From<LuaMetadata> for fs::Metadata {
    fn from(value: LuaMetadata) -> Self {
        value.0
    }
}

impl Deref for LuaMetadata {
    type Target = fs::Metadata;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[mlua::userdata_impl]
impl LuaMetadata {
    /// Returns the file type for this metadata.
    #[lua(name = "file_type", infallible)]
    pub(crate) fn lua_file_type(&self) -> LuaFileType {
        self.0.file_type().into()
    }

    /// Returns true if this metadata is for a directory. The result is mutually exclusive to the
    /// result of Metadata::is_file, and will be false for symlink metadata obtained from
    /// symlink_metadata.
    #[lua(name = "is_dir", infallible)]
    pub(crate) fn lua_is_dir(&self) -> bool {
        self.0.is_dir()
    }

    /// Returns true if this metadata is for a regular file. The result is mutually exclusive to the
    /// result of Metadata::is_dir, and will be false for symlink metadata obtained from
    /// symlink_metadata.
    ///
    /// When the goal is simply to read from (or write to) the source, the most
    /// reliable way to test the source can be read (or written to) is to open it. Only using
    /// is_file can break workflows like diff <( prog_a ) on a Unix-like system for example. See
    /// File::open or OpenOptions::open for more information.
    #[lua(name = "is_file", infallible)]
    pub(crate) fn lua_is_file(&self) -> bool {
        self.0.is_file()
    }

    /// Returns true if this metadata is for a symbolic link.
    #[lua(name = "is_symlink", infallible)]
    pub(crate) fn lua_is_symlink(&self) -> bool {
        self.0.is_symlink()
    }

    /// Returns the size of the file, in bytes, this metadata is for.
    #[lua(name = "len", infallible)]
    pub(crate) fn lua_len(&self) -> u64 {
        self.0.len()
    }

    /// Returns the permissions of the file this metadata is for.
    #[lua(name = "permissions", infallible)]
    pub(crate) fn lua_permissions(&self) -> LuaPermissions {
        self.0.permissions().into()
    }

    /// Returns the last modification time listed in this metadata.
    ///
    /// The returned value corresponds to the value of the mtime field of stat on Unix platforms and
    /// the ftLastWriteTime field on Windows platforms as the number of seconds since the Unix
    /// Epoch
    #[lua(name = "modified")]
    pub(crate) fn lua_modified(&self) -> mlua::Result<u64> {
        let time = self.0.modified().map_err(mlua::Error::external)?;
        time.duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .map_err(mlua::Error::external)
    }

    ///Returns the last access time of this metadata.
    ///
    /// The returned value corresponds to the value of the atime field of stat on Unix platforms and
    /// the ftLastAccessTime field on Windows platforms as the number of seconds since the Unix
    /// Epoch
    ///
    /// Note that not all platforms will keep this field update in a file's metadata, for example
    /// Windows has an option to disable updating this time when files are accessed and Linux
    /// similarly has noatime.
    #[lua(name = "accessed")]
    pub(crate) fn lua_accessed(&self) -> mlua::Result<u64> {
        let time = self.0.accessed().map_err(mlua::Error::external)?;
        time.duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .map_err(mlua::Error::external)
    }

    /// Returns the creation time listed in this metadata.
    ///
    /// The returned value corresponds to the value of the btime field of statx on Linux kernel
    /// starting from to 4.11, the birthtime field of stat on other Unix platforms, and the
    /// ftCreationTime field on Windows platforms as the number of seconds since the Unix Epoch
    #[lua(name = "created")]
    pub(crate) fn lua_created(&self) -> mlua::Result<u64> {
        let time = self.0.created().map_err(mlua::Error::external)?;
        time.duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .map_err(mlua::Error::external)
    }
}

#[cfg(unix)]
#[mlua::userdata_impl]
impl LuaMetadata {
    /// Returns the ID of the device containing the file.
    #[lua(name = "dev", infallible)]
    pub(crate) fn lua_dev(&self) -> u64 {
        self.0.dev()
    }

    /// Returns the inode number.
    #[lua(name = "ino", infallible)]
    pub(crate) fn lua_ino(&self) -> u64 {
        self.0.ino()
    }

    /// Returns the rights applied to this file.
    #[lua(name = "mode", infallible)]
    pub(crate) fn lua_mode(&self) -> u32 {
        self.0.mode()
    }

    /// Returns the number of hard links pointing to this file.
    #[lua(name = "nlink", infallible)]
    pub(crate) fn lua_nlink(&self) -> u64 {
        self.0.nlink()
    }

    /// Returns the user ID of the owner of this file.
    #[lua(name = "uid", infallible)]
    pub(crate) fn lua_uid(&self) -> u32 {
        self.0.uid()
    }

    /// Returns the group ID of the owner of this file.
    #[lua(name = "gid", infallible)]
    pub(crate) fn lua_gid(&self) -> u32 {
        self.0.gid()
    }

    /// Returns the device ID of this file (if it is a special one).
    #[lua(name = "rdev", infallible)]
    pub(crate) fn lua_rdev(&self) -> u64 {
        self.0.rdev()
    }

    /// Returns the total size of this file in bytes.
    #[lua(name = "size", infallible)]
    pub(crate) fn lua_size(&self) -> u64 {
        self.0.size()
    }

    /// Returns the last access time of the file, in seconds since Unix Epoch.
    #[lua(name = "atime", infallible)]
    pub(crate) fn lua_atime(&self) -> i64 {
        self.0.atime()
    }

    /// Returns the last modification time of the file, in seconds since Unix Epoch.
    #[lua(name = "mtime", infallible)]
    pub(crate) fn lua_mtime(&self) -> i64 {
        self.0.mtime()
    }

    /// Returns the last status change time of the file, in seconds since Unix Epoch.
    #[lua(name = "ctime", infallible)]
    pub(crate) fn lua_ctime(&self) -> i64 {
        self.0.ctime()
    }

    /// Returns the block size for filesystem I/O.
    #[lua(name = "blksize", infallible)]
    pub(crate) fn lua_blksize(&self) -> u64 {
        self.0.blksize()
    }

    /// Returns the number of blocks allocated to the file, in 512-byte units.
    ///
    /// Please note that this may be smaller than st_size / 512 when the file has holes.
    #[lua(name = "blocks", infallible)]
    pub(crate) fn lua_blocks(&self) -> u64 {
        self.0.blocks()
    }
}

#[cfg(windows)]
#[mlua::userdata_impl]
impl LuaMetadata {
    #[lua(name = "file_attributes", infallible)]
    pub(crate) fn lua_file_attributes(&self) -> u32 {
        self.0.file_attributes()
    }

    #[lua(name = "creation_time", infallible)]
    pub(crate) fn lua_creation_time(&self) -> u64 {
        self.0.creation_time()
    }

    #[lua(name = "last_access_time", infallible)]
    pub(crate) fn lua_last_access_time(&self) -> u64 {
        self.0.last_access_time()
    }

    #[lua(name = "last_write_time", infallible)]
    pub(crate) fn lua_last_write_time(&self) -> u64 {
        self.0.last_write_time()
    }

    #[lua(name = "file_size", infallible)]
    pub(crate) fn lua_file_size(&self) -> u64 {
        self.0.file_size()
    }
}

#[derive(mlua::UserData, Clone)]
pub(crate) struct LuaFileType(fs::FileType);

impl From<fs::FileType> for LuaFileType {
    fn from(value: fs::FileType) -> Self {
        LuaFileType(value)
    }
}

impl From<LuaFileType> for fs::FileType {
    fn from(value: LuaFileType) -> Self {
        value.0
    }
}

impl Deref for LuaFileType {
    type Target = fs::FileType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[mlua::userdata_impl]
impl LuaFileType {
    #[lua(name = "is_dir", infallible)]
    pub(crate) fn lua_is_dir(&self) -> bool {
        self.0.is_dir()
    }

    #[lua(name = "is_file", infallible)]
    pub(crate) fn lua_is_file(&self) -> bool {
        self.0.is_file()
    }

    #[lua(name = "is_symlink", infallible)]
    pub(crate) fn lua_is_symlink(&self) -> bool {
        self.0.is_symlink()
    }
}

#[cfg(unix)]
#[mlua::userdata_impl]
impl LuaFileType {
    #[lua(name = "is_block_device", infallible)]
    pub(crate) fn lua_is_block_device(&self) -> bool {
        self.0.is_block_device()
    }

    #[lua(name = "is_char_device", infallible)]
    pub(crate) fn lua_is_char_device(&self) -> bool {
        self.0.is_char_device()
    }

    #[lua(name = "is_fifo", infallible)]
    pub(crate) fn lua_is_fifo(&self) -> bool {
        self.0.is_fifo()
    }

    #[lua(name = "is_socket", infallible)]
    pub(crate) fn lua_is_socket(&self) -> bool {
        self.0.is_fifo()
    }
}

#[cfg(windows)]
#[mlua::userdata_impl]
impl LuaFileType {
    #[lua(name = "is_symlink_dir", infallible)]
    pub(crate) fn lua_is_symlink_dir(&self) -> bool {
        self.0.is_symlink_dir()
    }

    #[lua(name = "is_symlink_file", infallible)]
    pub(crate) fn lua_is_symlink_file(&self) -> bool {
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

impl From<LuaPermissions> for fs::Permissions {
    fn from(value: LuaPermissions) -> Self {
        value.0
    }
}

impl AsRef<fs::Permissions> for LuaPermissions {
    fn as_ref(&self) -> &fs::Permissions {
        &self.0
    }
}

impl Deref for LuaPermissions {
    type Target = fs::Permissions;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl mlua::FromLua for LuaPermissions {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::UserData(ud) => ud.borrow::<LuaPermissions>().map(|r| r.clone()),
            mlua::Value::String(s) => Ok(LuaPermissions::lua_from_perms(s.to_str()?.to_string())?),
            mlua::Value::Integer(int) => Ok(LuaPermissions::lua_from_mode(
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
    #[lua(name = "readonly", infallible)]
    pub(crate) fn lua_readonly(&self) -> bool {
        self.0.readonly()
    }

    #[lua(name = "set_readonly", infallible)]
    pub(crate) fn lua_set_readonly(&mut self, readonly: bool) {
        self.0.set_readonly(readonly);
    }
}

#[cfg(unix)]
#[mlua::userdata_impl]
impl LuaPermissions {
    #[lua(name = "from_mode", infallible)]
    pub(crate) fn lua_from_mode(mode: u32) -> LuaPermissions {
        fs::Permissions::from_mode(mode).into()
    }

    #[lua(name = "mode", infallible)]
    pub(crate) fn lua_mode(&self) -> u32 {
        self.0.mode()
    }

    #[lua(name = "set_mode", infallible)]
    pub(crate) fn lua_set_mode(&mut self, mode: u32) {
        self.0.set_mode(mode);
    }

    #[lua(name = "from_perms", infallible)]
    pub(crate) fn lua_from_perms(perms: String) -> mlua::Result<LuaPermissions> {
        let mode = u32::from_str_radix(&perms, 8).map_err(mlua::Error::external)?;

        Ok(fs::Permissions::from_mode(mode).into())
    }

    #[lua(name = "perms", infallible)]
    pub(crate) fn lua_perms(&self) -> String {
        format!("{:o}", self.0.mode())
    }

    #[lua(name = "set_perms", infallible)]
    pub(crate) fn lua_set_perms(&mut self, perms: String) -> mlua::Result<()> {
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

impl From<LuaDirEntry> for fs::DirEntry {
    fn from(value: LuaDirEntry) -> Self {
        value.0
    }
}

impl Deref for LuaDirEntry {
    type Target = fs::DirEntry;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[mlua::userdata_impl]
impl LuaDirEntry {
    #[lua(name = "path", infallible)]
    pub(crate) fn lua_path(&self) -> LuaPath {
        self.0.path().into()
    }

    #[lua(name = "metadata")]
    pub(crate) fn lua_metadata(&self) -> mlua::Result<LuaMetadata> {
        self.0
            .metadata()
            .map(|m| m.into())
            .map_err(mlua::Error::external)
    }

    #[lua(name = "file_type")]
    pub(crate) fn lua_file_type(&self) -> mlua::Result<LuaFileType> {
        self.0
            .file_type()
            .map(|ft| ft.into())
            .map_err(mlua::Error::external)
    }

    #[lua(name = "file_name", infallible)]
    pub(crate) fn lua_file_name(&self) -> OsString {
        self.0.file_name()
    }
}

#[cfg(unix)]
#[mlua::userdata_impl]
impl LuaDirEntry {
    #[lua(name = "ino", infallible)]
    pub(crate) fn lua_ino(&self) -> u64 {
        self.0.ino()
    }
}

#[derive(mlua::UserData, mlua::FromLua, Clone)]
pub(crate) struct LuaReadDir(Arc<Mutex<fs::ReadDir>>);

impl From<fs::ReadDir> for LuaReadDir {
    fn from(value: fs::ReadDir) -> Self {
        LuaReadDir(Arc::new(Mutex::new(value)))
    }
}

#[mlua::userdata_impl]
impl LuaReadDir {
    #[lua(name = "next")]
    pub(crate) fn lua_next(&self) -> mlua::Result<Option<LuaDirEntry>> {
        self.0
            .lock()
            .map_err(mlua::Error::runtime)?
            .next()
            .transpose()
            .map(|v| v.map(|d| d.into()))
            .map_err(mlua::Error::external)
    }

    #[lua(name = "iter")]
    pub(crate) fn lua_iter(&self, lua: &mlua::Lua) -> mlua::Result<(mlua::Function, Self)> {
        let iter =
            lua.create_function_mut(|_, this: LuaReadDir| -> mlua::Result<Option<LuaDirEntry>> {
                this.lua_next()
            })?;

        Ok((iter, self.clone()))
    }
}

#[derive(mlua::UserData, Clone)]
pub(crate) struct LuaOpenOptions(fs::OpenOptions);

impl From<fs::OpenOptions> for LuaOpenOptions {
    fn from(value: fs::OpenOptions) -> Self {
        LuaOpenOptions(value)
    }
}

impl From<LuaOpenOptions> for fs::OpenOptions {
    fn from(value: LuaOpenOptions) -> Self {
        value.0
    }
}

impl Deref for LuaOpenOptions {
    type Target = fs::OpenOptions;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[mlua::userdata_impl]
impl LuaOpenOptions {
    #[lua(name = "new", infallible)]
    pub(crate) fn lua_new() -> Self {
        fs::OpenOptions::new().into()
    }

    #[lua(name = "read", infallible)]
    pub(crate) fn lua_read(&mut self, read: bool) -> Self {
        self.0.read(read);
        self.clone()
    }

    #[lua(name = "write", infallible)]
    pub(crate) fn lua_write(&mut self, write: bool) -> Self {
        self.0.write(write);
        self.clone()
    }

    #[lua(name = "append", infallible)]
    pub(crate) fn lua_append(&mut self, append: bool) -> Self {
        self.0.append(append);
        self.clone()
    }

    #[lua(name = "truncate", infallible)]
    pub(crate) fn lua_truncate(&mut self, truncate: bool) -> Self {
        self.0.truncate(truncate);
        self.clone()
    }

    #[lua(name = "create", infallible)]
    pub(crate) fn lua_create(&mut self, create: bool) -> Self {
        self.0.create(create);
        self.clone()
    }

    #[lua(name = "create_new", infallible)]
    pub(crate) fn lua_create_new(&mut self, create_new: bool) -> Self {
        self.0.create_new(create_new);
        self.clone()
    }

    #[lua(name = "open")]
    pub(crate) fn lua_open(&mut self, path: LuaPath) -> mlua::Result<LuaFile> {
        self.0
            .open(path)
            .map(|f| f.into())
            .map_err(mlua::Error::external)
    }
}

#[cfg(unix)]
#[mlua::userdata_impl]
impl LuaOpenOptions {
    #[lua(name = "mode", infallible)]
    pub(crate) fn lua_mode(&mut self, mode: LuaPermissions) -> Self {
        self.0.mode(mode.mode());

        self.clone()
    }

    #[lua(name = "custom_flags", infallible)]
    pub(crate) fn lua_custom_flags(&mut self, flags: i32) -> Self {
        self.0.custom_flags(flags);
        self.clone()
    }
}

#[cfg(windows)]
#[mlua::userdata_impl]
impl LuaOpenOptions {
    #[cfg(not(unix))]
    #[lua(name = "custom_flags", infallible)]
    pub(crate) fn lua_custom_flags(&mut self, flags: u32) -> Self {
        self.0.custom_flags(flags);
        self.clone()
    }

    #[lua(name = "access_mode", infallible)]
    pub(crate) fn lua_access_mode(&mut self, mode: u32) -> Self {
        self.0.access_mode(mode);
        self.clone()
    }

    #[lua(name = "share_mode", infallible)]
    pub(crate) fn lua_share_mode(&mut self, mode: u32) -> Self {
        self.0.access_mode(mode);
        self.clone()
    }

    #[lua(name = "attributes", infallible)]
    pub(crate) fn lua_attributes(&mut self, attributes: u32) -> Self {
        self.0.attributes(mode);
        self.clone()
    }

    #[lua(name = "security_qos_flags", infallible)]
    pub(crate) fn lua_security_qos_flags(&mut self, flags: u32) -> Self {
        self.0.security_qos_flags(flags);
        self.clone()
    }
}

#[derive(mlua::UserData, mlua::FromLua, Clone)]
pub(crate) struct LuaLines(Arc<Mutex<Lines<BufReader<fs::File>>>>);

impl From<Lines<BufReader<fs::File>>> for LuaLines {
    fn from(value: Lines<BufReader<fs::File>>) -> Self {
        LuaLines(Arc::new(Mutex::new(value)))
    }
}

#[mlua::userdata_impl]
impl LuaLines {
    #[lua(name = "next")]
    pub(crate) fn lua_next(&self, lua: &mlua::Lua) -> mlua::Result<Option<mlua::String>> {
        self.0
            .lock()
            .map_err(mlua::Error::runtime)?
            .next()
            .transpose()
            .map_err(mlua::Error::external)?
            .map(|v| lua.create_string(v))
            .transpose()
    }

    #[lua(name = "iter")]
    pub(crate) fn lua_iter(&self, lua: &mlua::Lua) -> mlua::Result<(mlua::Function, Self)> {
        let iter = lua.create_function_mut(
            |lua, this: LuaLines| -> mlua::Result<Option<mlua::String>> { this.lua_next(lua) },
        )?;

        Ok((iter, self.clone()))
    }
}

#[derive(mlua::UserData, mlua::FromLua, Clone)]
pub(crate) struct LuaSplit(Arc<Mutex<Split<BufReader<fs::File>>>>);

impl From<Split<BufReader<fs::File>>> for LuaSplit {
    fn from(value: Split<BufReader<fs::File>>) -> Self {
        LuaSplit(Arc::new(Mutex::new(value)))
    }
}

#[mlua::userdata_impl]
impl LuaSplit {
    #[lua(name = "next")]
    pub(crate) fn lua_next(&self, lua: &mlua::Lua) -> mlua::Result<Option<mlua::String>> {
        self.0
            .lock()
            .map_err(mlua::Error::runtime)?
            .next()
            .transpose()
            .map_err(mlua::Error::external)?
            .map(|v| lua.create_string(v))
            .transpose()
    }

    #[lua(name = "iter")]
    pub(crate) fn lua_iter(&self, lua: &mlua::Lua) -> mlua::Result<(mlua::Function, Self)> {
        let iter = lua.create_function_mut(
            |lua, this: LuaSplit| -> mlua::Result<Option<mlua::String>> { this.lua_next(lua) },
        )?;

        Ok((iter, self.clone()))
    }
}
