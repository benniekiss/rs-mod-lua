use std::{ffi::OsString, path};

use crate::fs::{LuaMetadata, LuaReadDir};

#[derive(mlua::UserData, Clone)]
pub(crate) struct LuaPath(path::PathBuf);

impl From<path::PathBuf> for LuaPath {
    fn from(value: path::PathBuf) -> Self {
        LuaPath(value)
    }
}

impl From<&path::Path> for LuaPath {
    fn from(value: &path::Path) -> Self {
        LuaPath(value.to_path_buf())
    }
}

impl From<path::Component<'_>> for LuaPath {
    fn from(value: path::Component) -> Self {
        let path: &path::Path = value.as_ref();

        LuaPath(path.to_path_buf())
    }
}

impl AsRef<path::PathBuf> for LuaPath {
    fn as_ref(&self) -> &path::PathBuf {
        &self.0
    }
}

impl AsRef<path::Path> for LuaPath {
    fn as_ref(&self) -> &path::Path {
        &self.0
    }
}

impl mlua::FromLua for LuaPath {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::UserData(ud) => Ok(ud.borrow::<Self>()?.clone()),
            mlua::Value::String(s) => Ok(LuaPath::new(Some(s.to_str()?.to_string()))),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "Path".to_string(),
                message: Some("could not convert to Path".to_string()),
            }),
        }
    }
}

#[mlua::userdata_impl]
impl LuaPath {
    #[lua(meta, name = "__tostring", infallible)]
    pub(crate) fn tostring(&self) -> OsString {
        self.0.clone().into_os_string()
    }

    #[lua(infallible)]
    pub(crate) fn new(path: Option<String>) -> Self {
        let buf = match path {
            Some(p) => path::PathBuf::from(p),
            None => path::PathBuf::new(),
        };

        LuaPath(buf)
    }

    #[lua(infallible)]
    pub(crate) fn push(&mut self, path: String) {
        self.0.push(path)
    }

    #[lua(infallible)]
    pub(crate) fn pop(&mut self) -> bool {
        self.0.pop()
    }

    #[lua(infallible)]
    pub(crate) fn set_file_name(&mut self, file_name: String) {
        self.0.set_file_name(file_name)
    }

    #[lua(infallible)]
    pub(crate) fn with_file_name(&self, file_name: String) -> LuaPath {
        self.0.with_file_name(file_name).into()
    }

    #[lua(infallible)]
    pub(crate) fn set_extension(&mut self, extension: String) -> bool {
        self.0.set_extension(extension)
    }

    #[lua(infallible)]
    pub(crate) fn with_extension(&self, extension: String) -> LuaPath {
        self.0.with_extension(extension).into()
    }

    #[lua(infallible)]
    pub(crate) fn add_extension(&mut self, extension: String) -> bool {
        self.0.add_extension(extension)
    }

    #[lua(infallible)]
    pub(crate) fn with_added_extension(&self, extension: String) -> LuaPath {
        self.0.with_added_extension(extension).into()
    }

    #[lua(infallible)]
    pub(crate) fn is_absolute(&self) -> bool {
        self.0.is_absolute()
    }

    #[lua(infallible)]
    pub(crate) fn is_relative(&self) -> bool {
        self.0.is_relative()
    }

    #[lua(infallible)]
    pub(crate) fn has_root(&self) -> bool {
        self.0.has_root()
    }

    #[lua(infallible)]
    pub(crate) fn parent(&self) -> Option<LuaPath> {
        self.0.parent().map(|p| p.into())
    }

    #[lua(infallible)]
    pub(crate) fn file_name(&self) -> Option<OsString> {
        self.0.file_name().map(|s| s.to_owned())
    }

    pub(crate) fn strip_prefix(&self, base: String) -> mlua::Result<LuaPath> {
        self.0
            .strip_prefix(base)
            .map(|p| p.into())
            .map_err(mlua::Error::external)
    }

    #[lua(infallible)]
    pub(crate) fn trim_prefix(&self, base: String) -> LuaPath {
        self.0.trim_prefix(base).into()
    }

    #[lua(infallible)]
    pub(crate) fn starts_with(&self, base: String) -> bool {
        self.0.starts_with(base)
    }

    #[lua(infallible)]
    pub(crate) fn ends_with(&self, base: String) -> bool {
        self.0.ends_with(base)
    }

    #[lua(infallible)]
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[lua(infallible)]
    pub(crate) fn file_stem(&self) -> Option<OsString> {
        self.0.file_stem().map(|s| s.to_owned())
    }

    #[lua(infallible)]
    pub(crate) fn file_prefix(&self) -> Option<OsString> {
        self.0.file_prefix().map(|s| s.to_owned())
    }

    #[lua(infallible)]
    pub(crate) fn extension(&self) -> Option<OsString> {
        self.0.extension().map(|s| s.to_owned())
    }

    #[lua(infallible)]
    pub(crate) fn has_trailing_sep(&self) -> bool {
        self.0.has_trailing_sep()
    }

    #[lua(infallible)]
    pub(crate) fn with_trailing_sep(&self) -> LuaPath {
        self.0.with_trailing_sep().into_owned().into()
    }

    #[lua(infallible)]
    pub(crate) fn trim_trailing_sep(&self) -> LuaPath {
        self.0.trim_trailing_sep().into()
    }

    #[lua(infallible)]
    pub(crate) fn join(&self, path: String) -> LuaPath {
        self.0.join(path).into()
    }

    #[lua(infallible)]
    pub(crate) fn ancestors(&self) -> Vec<LuaPath> {
        self.0
            .ancestors()
            .map(|c| c.into())
            .collect::<Vec<LuaPath>>()
    }

    pub(crate) fn iter_ancestors(&self, lua: &mlua::Lua) -> mlua::Result<mlua::Function> {
        let mut comps = self.ancestors().into_iter();

        lua.create_function_mut(move |_, _: mlua::Value| Ok(comps.next()))
    }

    #[lua(infallible)]
    pub(crate) fn components(&self) -> Vec<LuaPath> {
        self.0
            .components()
            .map(|c| c.into())
            .collect::<Vec<LuaPath>>()
    }

    pub(crate) fn iter(&self, lua: &mlua::Lua) -> mlua::Result<mlua::Function> {
        let mut comps = self.components().into_iter();

        lua.create_function_mut(move |_, _: mlua::Value| Ok(comps.next()))
    }

    pub(crate) fn metadata(&self) -> mlua::Result<LuaMetadata> {
        self.0
            .metadata()
            .map(|m| m.into())
            .map_err(mlua::Error::external)
    }

    pub(crate) fn symlink_metadata(&self) -> mlua::Result<LuaMetadata> {
        self.0
            .symlink_metadata()
            .map(|m| m.into())
            .map_err(mlua::Error::external)
    }

    pub(crate) fn canonicalize(&self) -> mlua::Result<LuaPath> {
        self.0
            .canonicalize()
            .map(|p| p.into())
            .map_err(mlua::Error::external)
    }

    pub(crate) fn absolute(&self) -> mlua::Result<LuaPath> {
        self.0
            .absolute()
            .map(|p| p.into())
            .map_err(mlua::Error::external)
    }

    pub(crate) fn read_link(&self) -> mlua::Result<LuaPath> {
        self.0
            .read_link()
            .map(|p| p.into())
            .map_err(mlua::Error::external)
    }

    pub(crate) fn read_dir(&self) -> mlua::Result<LuaReadDir> {
        self.0
            .read_dir()
            .map(|p| p.into())
            .map_err(mlua::Error::external)
    }

    #[lua(infallible)]
    pub(crate) fn exists(&self) -> bool {
        self.0.exists()
    }

    pub(crate) fn try_exists(&self) -> mlua::Result<bool> {
        self.0.try_exists().map_err(mlua::Error::external)
    }

    #[lua(infallible)]
    pub(crate) fn is_file(&self) -> bool {
        self.0.is_file()
    }

    #[lua(infallible)]
    pub(crate) fn is_dir(&self) -> bool {
        self.0.is_dir()
    }

    #[lua(infallible)]
    pub(crate) fn is_symlink(&self) -> bool {
        self.0.is_symlink()
    }
}
