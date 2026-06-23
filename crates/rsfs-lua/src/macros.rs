macro_rules! lua_read_methods {
    ($ud:ident) => {
        #[mlua::userdata_impl]
        impl $ud {
            #[lua(name = "read")]
            pub(crate) fn lua_read(
                &mut self,
                lua: &mlua::Lua,
                chunk: Option<usize>,
            ) -> mlua::Result<(mlua::String, usize)> {
                let size = chunk.unwrap_or(16);
                let mut buf = vec![0u8; size];
                let n_read = self.0.read(&mut buf).map_err(mlua::Error::external)?;

                let s = lua.create_string(buf)?;

                Ok((s, n_read))
            }

            #[lua(name = "read_exact")]
            pub(crate) fn lua_read_exact(
                &mut self,
                lua: &mlua::Lua,
                size: usize,
            ) -> mlua::Result<(mlua::String, usize)> {
                let mut buf = vec![0u8; size];

                self.0.read_exact(&mut buf).map_err(mlua::Error::external)?;

                let s = lua.create_string(buf)?;

                Ok((s, size))
            }

            #[lua(name = "read_to_end")]
            pub(crate) fn lua_read_to_end(
                &mut self,
                lua: &mlua::Lua,
            ) -> mlua::Result<(mlua::String, usize)> {
                let mut buf = Vec::new();

                let n_read = self
                    .0
                    .read_to_end(&mut buf)
                    .map_err(mlua::Error::external)?;

                let s = lua.create_string(buf)?;

                Ok((s, n_read))
            }
        }
    };
}

macro_rules! lua_write_methods {
    ($ud:ident) => {
        #[mlua::userdata_impl]
        impl $ud {
            #[lua(name = "write")]
            pub(crate) fn lua_write(&mut self, buf: &[u8]) -> mlua::Result<usize> {
                self.0.write(buf).map_err(mlua::Error::external)
            }

            #[lua(name = "write_all")]
            pub(crate) fn lua_write_all(&mut self, buf: &[u8]) -> mlua::Result<()> {
                self.0.write_all(buf).map_err(mlua::Error::external)
            }

            #[lua(name = "seek")]
            pub(crate) fn lua_seek(
                &mut self,
                offset: Option<i64>,
                whence: Option<String>,
            ) -> mlua::Result<u64> {
                let offset = offset.unwrap_or_default();

                let whence = match whence {
                    Some(s) if s == "start" => {
                        SeekFrom::Start(u64::try_from(offset).map_err(mlua::Error::external)?)
                    },
                    Some(s) if s == "end" => SeekFrom::End(offset),
                    Some(s) if s == "current" => SeekFrom::Current(offset),
                    None => SeekFrom::Current(offset),
                    Some(s) => Err(mlua::Error::runtime(format!(
                        "invalid option. Must be one of `start`, `end`, or `current`: {s}"
                    )))?,
                };

                self.0.seek(whence).map_err(mlua::Error::external)
            }

            #[lua(name = "flush")]
            pub(crate) fn lua_flush(&mut self) -> mlua::Result<()> {
                self.0.flush().map_err(mlua::Error::external)
            }
        }
    };
}

macro_rules! lua_path_methods {
    ($ud:ident) => {
        #[mlua::userdata_impl]
        impl $ud {
            #[lua(name = "with_file_name", infallible)]
            pub(crate) fn lua_with_file_name(&self, file_name: String) -> LuaPath {
                self.0.with_file_name(file_name).into()
            }

            #[lua(name = "with_extension", infallible)]
            pub(crate) fn lua_with_extension(&self, extension: String) -> LuaPath {
                self.0.with_extension(extension).into()
            }

            #[lua(name = "with_added_extension", infallible)]
            pub(crate) fn lua_with_added_extension(&self, extension: String) -> LuaPath {
                self.0.with_added_extension(extension).into()
            }

            #[lua(name = "is_absolute", infallible)]
            pub(crate) fn lua_is_absolute(&self) -> bool {
                self.0.is_absolute()
            }

            #[lua(name = "is_relative", infallible)]
            pub(crate) fn lua_is_relative(&self) -> bool {
                self.0.is_relative()
            }

            #[lua(name = "has_root", infallible)]
            pub(crate) fn lua_has_root(&self) -> bool {
                self.0.has_root()
            }

            #[lua(name = "parent", infallible)]
            pub(crate) fn lua_parent(&self) -> Option<LuaPath> {
                self.0.parent().map(|p| p.into())
            }

            #[lua(name = "file_name", infallible)]
            pub(crate) fn lua_file_name(&self) -> Option<OsString> {
                self.0.file_name().map(|s| s.to_owned())
            }

            #[lua(name = "strip_prefix")]
            pub(crate) fn lua_strip_prefix(&self, base: String) -> mlua::Result<LuaPath> {
                self.0
                    .strip_prefix(base)
                    .map(|p| p.into())
                    .map_err(mlua::Error::external)
            }

            #[lua(name = "trim_prefix", infallible)]
            pub(crate) fn lua_trim_prefix(&self, base: String) -> LuaPath {
                self.0.trim_prefix(base).into()
            }

            #[lua(name = "starts_with", infallible)]
            pub(crate) fn lua_starts_with(&self, base: LuaPath) -> bool {
                self.0.starts_with(base)
            }

            #[lua(name = "ends_with", infallible)]
            pub(crate) fn lua_ends_with(&self, base: LuaPath) -> bool {
                self.0.ends_with(base)
            }

            #[lua(name = "is_empty", infallible)]
            pub(crate) fn lua_is_empty(&self) -> bool {
                self.0.is_empty()
            }

            #[lua(name = "file_stem", infallible)]
            pub(crate) fn lua_file_stem(&self) -> Option<OsString> {
                self.0.file_stem().map(|s| s.to_owned())
            }

            #[lua(name = "file_prefix", infallible)]
            pub(crate) fn lua_file_prefix(&self) -> Option<OsString> {
                self.0.file_prefix().map(|s| s.to_owned())
            }

            #[lua(name = "extension", infallible)]
            pub(crate) fn lua_extension(&self) -> Option<OsString> {
                self.0.extension().map(|s| s.to_owned())
            }

            #[lua(name = "has_trailing_sep", infallible)]
            pub(crate) fn lua_has_trailing_sep(&self) -> bool {
                self.0.has_trailing_sep()
            }

            #[lua(name = "with_trailing_sep", infallible)]
            pub(crate) fn lua_with_trailing_sep(&self) -> LuaPath {
                self.0.with_trailing_sep().into_owned().into()
            }

            #[lua(name = "trim_trailing_sep", infallible)]
            pub(crate) fn lua_trim_trailing_sep(&self) -> LuaPath {
                self.0.trim_trailing_sep().into()
            }

            #[lua(name = "join", infallible)]
            pub(crate) fn lua_join(&self, path: LuaPath) -> LuaPath {
                self.0.join(path).into()
            }

            #[lua(name = "ancestors", infallible)]
            pub(crate) fn lua_ancestors(&self) -> Vec<LuaPath> {
                self.0
                    .ancestors()
                    .map(|c| c.into())
                    .collect::<Vec<LuaPath>>()
            }

            #[lua(name = "iter_ancestors")]
            pub(crate) fn lua_iter_ancestors(
                &self,
                lua: &mlua::Lua,
            ) -> mlua::Result<mlua::Function> {
                let mut comps = self.lua_ancestors().into_iter();

                lua.create_function_mut(move |_, ()| Ok(comps.next()))
            }

            #[lua(name = "components", infallible)]
            pub(crate) fn lua_components(&self) -> Vec<LuaPath> {
                self.0
                    .components()
                    .map(|c| c.into())
                    .collect::<Vec<LuaPath>>()
            }

            #[lua(name = "iter")]
            pub(crate) fn lua_iter(&self, lua: &mlua::Lua) -> mlua::Result<mlua::Function> {
                let mut comps = self.lua_components().into_iter();

                lua.create_function_mut(move |_, ()| Ok(comps.next()))
            }

            #[lua(name = "metadata")]
            pub(crate) fn lua_metadata(&self) -> mlua::Result<LuaMetadata> {
                self.0
                    .metadata()
                    .map(|m| m.into())
                    .map_err(mlua::Error::external)
            }

            #[lua(name = "symlink_metadata")]
            pub(crate) fn lua_symlink_metadata(&self) -> mlua::Result<LuaMetadata> {
                self.0
                    .symlink_metadata()
                    .map(|m| m.into())
                    .map_err(mlua::Error::external)
            }

            #[lua(name = "canonicalize")]
            pub(crate) fn lua_canonicalize(&self) -> mlua::Result<LuaPath> {
                self.0
                    .canonicalize()
                    .map(|p| p.into())
                    .map_err(mlua::Error::external)
            }

            #[lua(name = "absolute")]
            pub(crate) fn lua_absolute(&self) -> mlua::Result<LuaPath> {
                self.0
                    .absolute()
                    .map(|p| p.into())
                    .map_err(mlua::Error::external)
            }

            #[lua(name = "read_link")]
            pub(crate) fn lua_read_link(&self) -> mlua::Result<LuaPath> {
                self.0
                    .read_link()
                    .map(|p| p.into())
                    .map_err(mlua::Error::external)
            }

            #[lua(name = "read_dir")]
            pub(crate) fn lua_read_dir(&self) -> mlua::Result<LuaReadDir> {
                self.0
                    .read_dir()
                    .map(|p| p.into())
                    .map_err(mlua::Error::external)
            }

            #[lua(name = "exists", infallible)]
            pub(crate) fn lua_exists(&self) -> bool {
                self.0.exists()
            }

            #[lua(name = "try_exists")]
            pub(crate) fn lua_try_exists(&self) -> mlua::Result<bool> {
                self.0.try_exists().map_err(mlua::Error::external)
            }

            #[lua(name = "is_file", infallible)]
            pub(crate) fn lua_is_file(&self) -> bool {
                self.0.is_file()
            }

            #[lua(name = "is_dir", infallible)]
            pub(crate) fn lua_is_dir(&self) -> bool {
                self.0.is_dir()
            }

            #[lua(name = "is_symlink", infallible)]
            pub(crate) fn lua_is_symlink(&self) -> bool {
                self.0.is_symlink()
            }
        }
    };
}
