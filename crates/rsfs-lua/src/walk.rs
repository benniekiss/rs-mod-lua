use std::{ffi::OsString, ops::Deref};

use crate::{
    fs::{LuaFileType, LuaMetadata},
    path::LuaPath,
};

#[derive(mlua::UserData, Clone)]
pub(crate) struct LuaWalkDirEntry(walkdir::DirEntry);

impl From<walkdir::DirEntry> for LuaWalkDirEntry {
    fn from(value: walkdir::DirEntry) -> Self {
        Self(value)
    }
}

impl From<LuaWalkDirEntry> for walkdir::DirEntry {
    fn from(value: LuaWalkDirEntry) -> Self {
        value.0
    }
}

impl Deref for LuaWalkDirEntry {
    type Target = walkdir::DirEntry;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[mlua::userdata_impl]
impl LuaWalkDirEntry {
    #[lua(name = "path", infallible)]
    pub(crate) fn lua_path(&self) -> LuaPath {
        self.0.path().into()
    }

    #[lua(name = "path_is_symlink", infallible)]
    pub(crate) fn lua_path_is_symlink(&self) -> bool {
        self.0.path_is_symlink()
    }

    #[lua(name = "metadata")]
    pub(crate) fn lua_metadata(&self) -> mlua::Result<LuaMetadata> {
        self.0
            .metadata()
            .map(|m| m.into())
            .map_err(mlua::Error::external)
    }

    #[lua(name = "file_type", infallible)]
    pub(crate) fn lua_file_type(&self) -> LuaFileType {
        self.0.file_type().into()
    }

    #[lua(name = "file_name", infallible)]
    pub(crate) fn lua_file_name(&self) -> OsString {
        self.0.file_name().to_os_string()
    }

    #[lua(name = "depth", infallible)]
    pub(crate) fn lua_depth(&self) -> usize {
        self.0.depth()
    }
}

#[derive(mlua::UserData)]
pub(crate) struct LuaWalkIter(walkdir::IntoIter);

impl From<walkdir::IntoIter> for LuaWalkIter {
    fn from(value: walkdir::IntoIter) -> Self {
        Self(value)
    }
}

impl From<LuaWalkIter> for walkdir::IntoIter {
    fn from(value: LuaWalkIter) -> Self {
        value.0
    }
}

impl Deref for LuaWalkIter {
    type Target = walkdir::IntoIter;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[mlua::userdata_impl]
impl LuaWalkIter {
    #[lua(name = "skip_current_dir", infallible)]
    pub(crate) fn lua_skip_current_dir(&mut self) {
        self.0.skip_current_dir();
    }

    #[lua(name = "next")]
    pub(crate) fn lua_next(&mut self) -> mlua::Result<Option<LuaWalkDirEntry>> {
        self.0
            .next()
            .transpose()
            .map(|opt| opt.map(|d| d.into()))
            .map_err(mlua::Error::external)
    }

    #[lua(name = "iter")]
    pub(crate) fn lua_iter(mut self, lua: &mlua::Lua) -> mlua::Result<mlua::Function> {
        lua.create_function_mut(move |_, _: ()| -> mlua::Result<Option<LuaWalkDirEntry>> {
            self.0
                .next()
                .transpose()
                .map(|opt| opt.map(|d| d.into()))
                .map_err(mlua::Error::external)
        })
    }
}

#[derive(mlua::UserData)]
pub(crate) struct LuaWalkDir(walkdir::WalkDir);

impl From<walkdir::WalkDir> for LuaWalkDir {
    fn from(value: walkdir::WalkDir) -> Self {
        Self(value)
    }
}

impl From<LuaWalkDir> for walkdir::WalkDir {
    fn from(value: LuaWalkDir) -> Self {
        value.0
    }
}

impl Deref for LuaWalkDir {
    type Target = walkdir::WalkDir;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[mlua::userdata_impl]
impl LuaWalkDir {
    #[lua(name = "new", infallible)]
    pub(crate) fn lua_new(root: LuaPath) -> Self {
        walkdir::WalkDir::new(root).into()
    }

    #[lua(name = "min_depth", infallible)]
    pub(crate) fn lua_min_depth(self, depth: usize) -> Self {
        self.0.min_depth(depth).into()
    }

    #[lua(name = "max_depth", infallible)]
    pub(crate) fn lua_max_depth(self, depth: usize) -> Self {
        self.0.max_depth(depth).into()
    }

    #[lua(name = "follow_links", infallible)]
    pub(crate) fn lua_follow_links(self, enable: bool) -> Self {
        self.0.follow_links(enable).into()
    }

    #[lua(name = "follow_root_links", infallible)]
    pub(crate) fn lua_follow_root_links(self, enable: bool) -> Self {
        self.0.follow_root_links(enable).into()
    }

    #[lua(name = "max_open", infallible)]
    pub(crate) fn lua_max_open(self, max: usize) -> Self {
        self.0.max_open(max).into()
    }

    #[lua(name = "sort_by_file_name", infallible)]
    pub(crate) fn lua_sort_by_file_name(self) -> Self {
        self.0.sort_by_file_name().into()
    }

    #[lua(name = "contents_first", infallible)]
    pub(crate) fn lua_contents_first(self, enable: bool) -> Self {
        self.0.contents_first(enable).into()
    }

    #[lua(name = "same_file_system", infallible)]
    pub(crate) fn lua_same_file_system(self, enable: bool) -> Self {
        self.0.same_file_system(enable).into()
    }

    #[lua(name = "into_iter", infallible)]
    pub(crate) fn lua_into_iter(self) -> LuaWalkIter {
        self.0.into_iter().into()
    }
}
