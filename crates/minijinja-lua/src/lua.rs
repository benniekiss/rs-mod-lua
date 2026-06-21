use std::cell::RefCell;

thread_local! {
    static LUA: RefCell<Option<mlua::WeakLua>> = const { RefCell::new(None) };
}

struct LuaGuard<'r> {
    prev: Option<mlua::WeakLua>,
    store: &'r RefCell<Option<mlua::WeakLua>>,
}

impl<'r> LuaGuard<'r> {
    fn new(lua: &mlua::Lua, store: &'r RefCell<Option<mlua::WeakLua>>) -> Self {
        let weak = lua.weak();
        let prev = store.replace(Some(weak));
        Self { prev, store }
    }

    fn bind<R, F>(lua: &mlua::Lua, store: &'r RefCell<Option<mlua::WeakLua>>, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let _guard = Self::new(lua, store);
        f()
    }

    fn with<R, F>(store: &'r RefCell<Option<mlua::WeakLua>>, f: F) -> Result<R, mlua::Error>
    where
        F: FnOnce(&mlua::Lua) -> Result<R, mlua::Error>,
    {
        let weak = store.borrow();
        let weak = weak.as_ref().ok_or_else(|| {
            mlua::Error::runtime("`mlua::Lua` instance accessed outside of a render context")
        })?;
        let lua = weak
            .try_upgrade()
            .ok_or_else(|| mlua::Error::runtime("`mlua::Lua` instance is not available"))?;

        f(&lua)
    }
}

impl Drop for LuaGuard<'_> {
    fn drop(&mut self) {
        self.store.replace(self.prev.take());
    }
}

pub(crate) fn bind_lua<R, F>(lua: &mlua::Lua, f: F) -> R
where
    F: FnOnce() -> R,
{
    LUA.with(|slot| LuaGuard::bind(lua, slot, f))
}

pub(crate) fn with_lua<R, F>(f: F) -> Result<R, mlua::Error>
where
    F: FnOnce(&mlua::Lua) -> Result<R, mlua::Error>,
{
    LUA.with(|store| LuaGuard::with(store, f))
}
