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
