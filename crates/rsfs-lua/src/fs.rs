struct FsAttributes {
    dev: u64,
    ino: u64,
    mode: String,
    nlink: u64,
    uid: u64,
    gid: u64,
    rdev: u64,
    access: u64,
    modification: u64,
    change: u64,
    size: u64,
    permissions: u64,
    blocks: u64,
    blksize: u64,
    target: String,
}

pub(crate) fn lfs_lua(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let table = lua.create_table()?;

    table.set(
        "attributes",
        lua.create_function(|_, (_filepath, _attr, _table): (String, String, mlua::Table)| Ok(()))?,
    )?;

    table.set("chdir", lua.create_function(|_, _path: String| Ok(()))?)?;

    table.set(
        "lock_dir",
        lua.create_function(|_, (_path, _stale): (String, u64)| Ok(()))?,
    )?;

    table.set(
        "currentdir",
        lua.create_function(|_, _: mlua::Value| Ok(()))?,
    )?;

    table.set("dir", lua.create_function(|_, _path: String| Ok(()))?)?;

    table.set(
        "lock",
        lua.create_function(
            |_, (_fh, _mode, _start, _length): (mlua::Value, String, u64, u64)| Ok(()),
        )?,
    )?;

    table.set(
        "link",
        lua.create_function(|_, (_old, _new, _symlink): (String, String, bool)| Ok(()))?,
    )?;

    table.set("mkdir", lua.create_function(|_, _dirname: String| Ok(()))?)?;

    table.set(
        "setmode",
        lua.create_function(|_, (_file, _mode): (mlua::Value, String)| Ok(()))?,
    )?;

    table.set(
        "symlinkattributes",
        lua.create_function(|_, (_filepath, _attr, _table): (String, String, mlua::Table)| Ok(()))?,
    )?;

    table.set(
        "touch",
        lua.create_function(|_, (_filepath, _atime, _mtime): (String, u64, u64)| Ok(()))?,
    )?;

    table.set(
        "unlock",
        lua.create_function(|_, (_fh, _start, _length): (mlua::Value, u64, u64)| Ok(()))?,
    )?;

    Ok(table)
}
