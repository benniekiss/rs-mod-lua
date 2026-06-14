use std::{
    ffi::{OsStr, OsString},
    ops::Deref,
    path,
};

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

impl From<mlua::String> for LuaPath {
    fn from(value: mlua::String) -> Self {
        cfg_select! {
            unix => {
                use std::os::unix::ffi::OsStrExt;

                let b = value.as_bytes();
                LuaPath(OsStr::from_bytes(&b).into())
            },
            windows => {
                use std::os::windows::ffi::OsStrExt;

                let b = value.as_bytes();
                LuaPath(OsString::from_wide(&b).into())
            },
            _ => value.to_string_lossy().into(),
        }
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

impl AsRef<OsStr> for LuaPath {
    fn as_ref(&self) -> &OsStr {
        self.0.as_os_str()
    }
}

impl Deref for LuaPath {
    type Target = path::PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl mlua::FromLua for LuaPath {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::UserData(ud) => ud.borrow::<LuaPath>().map(|r| r.clone()),
            mlua::Value::String(s) => Ok(s.into()),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "LuaPath".to_string(),
                message: Some("could not convert to Path".to_string()),
            }),
        }
    }
}

#[mlua::userdata_impl]
impl LuaPath {
    #[lua(meta, name = "__tostring", infallible)]
    pub(crate) fn lua_tostring(&self) -> OsString {
        self.0.clone().into_os_string()
    }

    #[lua(name = "new", infallible)]
    pub(crate) fn lua_new() -> Self {
        path::PathBuf::new().into()
    }

    #[lua(name = "from", infallible)]
    pub(crate) fn lua_from(path: mlua::String) -> Self {
        path.into()
    }

    #[lua(name = "push", infallible)]
    pub(crate) fn lua_push(&mut self, path: LuaPath) {
        self.0.push(path)
    }

    #[lua(name = "pop", infallible)]
    pub(crate) fn lua_pop(&mut self) -> bool {
        self.0.pop()
    }

    #[lua(name = "set_file_name", infallible)]
    pub(crate) fn lua_set_file_name(&mut self, file_name: OsString) {
        self.0.set_file_name(file_name)
    }

    #[lua(name = "set_extension", infallible)]
    pub(crate) fn lua_set_extension(&mut self, extension: OsString) -> bool {
        self.0.set_extension(extension)
    }

    #[lua(name = "add_extension", infallible)]
    pub(crate) fn lua_add_extension(&mut self, extension: OsString) -> bool {
        self.0.add_extension(extension)
    }
}

lua_path_methods!(LuaPath);
