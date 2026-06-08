use std::{
    env::{current_dir, set_current_dir},
    fs,
    path::PathBuf,
    time::{Duration, SystemTime},
};

use crate::fs::*;

pub(crate) struct LfsAttributes {
    pub(crate) dev: u64,
    pub(crate) ino: u64,
    pub(crate) mode: String,
    pub(crate) nlink: u64,
    pub(crate) uid: u32,
    pub(crate) gid: u32,
    pub(crate) rdev: u64,
    pub(crate) access: u64,
    pub(crate) modification: u64,
    pub(crate) change: i64,
    pub(crate) size: u64,
    pub(crate) permissions: u32,
    pub(crate) blocks: Option<u64>,
    pub(crate) blksize: Option<u64>,
    pub(crate) target: Option<PathBuf>,
}

impl LfsAttributes {
    pub(crate) fn new(meta: &LuaFsMetadata) -> Self {
        #[cfg(unix)]
        return Self {
            dev: meta.dev(),
            ino: meta.ino(),
            mode: "".to_string(),
            nlink: meta.nlink(),
            uid: meta.uid(),
            gid: meta.gid(),
            rdev: meta.rdev(),
            access: meta.accessed().unwrap_or_default(),
            modification: meta.modified().unwrap_or_default(),
            change: meta.ctime(),
            size: meta.size(),
            permissions: meta.permissions().mode(),
            blocks: Some(meta.blocks()),
            blksize: Some(meta.blksize()),
            target: None,
        };

        #[cfg(windows)]
        return Self {
            dev: meta.dev(),
            ino: meta.ino(),
            mode: "".to_string(),
            nlink: meta.nlink(),
            uid: 0,
            gid: 0,
            rdev: meta.rdev(),
            access: meta.accessed().unwrap_or_default(),
            modification: meta.modified().unwrap_or_default(),
            change: meta.ctime(),
            size: meta.file_size(),
            permissions: meta.file_attributes(),
            blocks: None,
            blksize: None,
            target: None,
        };
    }

    pub(crate) fn to_lua_table(
        &self,
        lua: &mlua::Lua,
        attr: Option<&str>,
        table: Option<mlua::Table>,
    ) -> mlua::Result<mlua::Table> {
        let table = table.unwrap_or(lua.create_table()?);

        match attr {
            Some(v) if v != "dev" => (),
            _ => table.set("dev", self.dev)?,
        }
        match attr {
            Some(v) if v != "ino" => (),
            _ => table.set("ino", self.ino)?,
        }
        match attr {
            Some(v) if v != "mode" => (),
            _ => table.set("mode", self.mode.clone())?,
        }
        match attr {
            Some(v) if v != "nlink" => (),
            _ => table.set("nlink", self.nlink)?,
        }
        match attr {
            Some(v) if v != "uid" => (),
            _ => table.set("uid", self.uid)?,
        }
        match attr {
            Some(v) if v != "gid" => (),
            _ => table.set("gid", self.gid)?,
        }
        match attr {
            Some(v) if v != "rdev" => (),
            _ => table.set("rdev", self.rdev)?,
        }
        match attr {
            Some(v) if v != "access" => (),
            _ => table.set("access", self.access)?,
        }
        match attr {
            Some(v) if v != "modification" => (),
            _ => table.set("modification", self.modification)?,
        }
        match attr {
            Some(v) if v != "change" => (),
            _ => table.set("change", self.change)?,
        }
        match attr {
            Some(v) if v != "size" => (),
            _ => table.set("size", self.size)?,
        }
        match attr {
            Some(v) if v != "permissions" => (),
            _ => table.set("permissions", self.permissions)?,
        }
        match attr {
            Some(v) if v != "blocks" => (),
            _ => table.set("blocks", self.blocks)?,
        }
        match attr {
            Some(v) if v != "blksize" => (),
            _ => table.set("blksize", self.blksize)?,
        }
        match attr {
            Some(v) if v != "target" => (),
            _ => table.set("target", self.target.clone())?,
        }

        Ok(table)
    }
}

impl mlua::IntoLua for LfsAttributes {
    fn into_lua(self, lua: &mlua::prelude::Lua) -> mlua::Result<mlua::Value> {
        let table = self.to_lua_table(lua, None, None)?;

        Ok(mlua::Value::Table(table))
    }
}

pub(crate) fn lfs_lua(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let table = lua.create_table()?;

    table.set(
        "attributes",
        lua.create_function(
            |lua,
             (filepath, attr, table): (String, Option<String>, Option<mlua::Table>)|
             -> mlua::Result<mlua::Table> {
                let meta = fs::metadata(filepath).map_err(mlua::Error::external)?;
                let attr =
                    LfsAttributes::new(&meta.into()).to_lua_table(lua, attr.as_deref(), table)?;

                Ok(attr)
            },
        )?,
    )?;

    table.set(
        "symlinkattributes",
        lua.create_function(
            |lua,
             (filepath, attr, table): (String, Option<String>, Option<mlua::Table>)|
             -> mlua::Result<mlua::Table> {
                let meta = fs::symlink_metadata(&filepath).map_err(mlua::Error::external)?;
                let target = fs::read_link(&filepath);

                match target {
                    Ok(path) => {
                        let mut lfsattr = LfsAttributes::new(&meta.into());
                        lfsattr.target = Some(path);

                        lfsattr.to_lua_table(lua, attr.as_deref(), table)
                    },
                    Err(_) => {
                        LfsAttributes::new(&meta.into()).to_lua_table(lua, attr.as_deref(), table)
                    },
                }
            },
        )?,
    )?;

    table.set(
        "chdir",
        lua.create_function(|_, path: String| -> mlua::Result<()> {
            set_current_dir(path).map_err(mlua::Error::external)
        })?,
    )?;

    table.set(
        "currentdir",
        lua.create_function(|_, _: mlua::Value| -> mlua::Result<PathBuf> {
            current_dir().map_err(mlua::Error::external)
        })?,
    )?;

    table.set(
        "mkdir",
        lua.create_function(|_, dirname: String| -> mlua::Result<()> {
            fs::create_dir(dirname).map_err(mlua::Error::external)
        })?,
    )?;

    table.set(
        "dir",
        lua.create_function(|_, path: String| -> mlua::Result<LuaFsReadDir> {
            fs::read_dir(path)
                .map(|it| it.into())
                .map_err(mlua::Error::external)
        })?,
    )?;

    table.set(
        "lock_dir",
        lua.create_function(|_, (_path, _stale): (String, u64)| Ok(()))?,
    )?;

    table.set(
        "lock",
        lua.create_function(
            |_, (_fh, _mode, _start, _length): (mlua::Value, String, u64, u64)| Ok(()),
        )?,
    )?;

    table.set(
        "unlock",
        lua.create_function(|_, (_fh, _start, _length): (mlua::Value, u64, u64)| Ok(()))?,
    )?;

    table.set(
        "link",
        lua.create_function(|_, (_old, _new, _symlink): (String, String, bool)| Ok(()))?,
    )?;

    table.set(
        "setmode",
        lua.create_function(|_, (_file, _mode): (mlua::Value, String)| Ok(()))?,
    )?;

    table.set(
        "touch",
        lua.create_function(
            |_,
             (filepath, atime, mtime): (String, Option<u64>, Option<u64>)|
             -> mlua::Result<bool> {
                let file = fs::File::create_new(filepath).map_err(mlua::Error::external)?;
                let now = SystemTime::now();

                let atime = atime
                    .and_then(|t| SystemTime::UNIX_EPOCH.checked_add(Duration::from_secs(t)))
                    .unwrap_or(now);

                let mtime = mtime
                    .and_then(|t| SystemTime::UNIX_EPOCH.checked_add(Duration::from_secs(t)))
                    .unwrap_or(now);

                let times = fs::FileTimes::new().set_accessed(atime).set_accessed(mtime);

                file.set_times(times).map_err(mlua::Error::external)?;

                Ok(true)
            },
        )?,
    )?;

    Ok(table)
}
