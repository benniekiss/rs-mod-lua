use std::cell::RefCell;

use mlua::LuaSerdeExt;
use rsjson_lua::config::EncodeConfig;

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
        // The borrow from `store` must be dropped before calling `f` to prevent panics in case `f`
        // itself calls `bind`
        let lua = store
            .try_borrow()
            .map_err(mlua::Error::runtime)?
            .as_ref()
            .ok_or_else(|| {
                mlua::Error::runtime("`mlua::Lua` instance accessed outside of a render context")
            })?
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

pub(crate) fn lua_to_json(
    lua: &mlua::Lua,
    value: mlua::Value,
    options: Option<EncodeConfig>,
) -> mlua::Result<serde_json::Value> {
    match value.as_string() {
        Some(s) => serde_json::from_str(&s.to_string_lossy()).map_err(mlua::Error::external),
        None => lua.from_value_with(value, *options.unwrap_or_default()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_outside_context() {
        let err = with_lua(|_| Ok(()));

        assert!(err.is_err());
        assert!(
            err.unwrap_err()
                .to_string()
                .contains("accessed outside of a render context")
        );
    }

    #[test]
    fn test_recursive_bind() {
        let lua = mlua::Lua::new();

        let res = bind_lua(&lua, || {
            with_lua(|lua| bind_lua(lua, || with_lua(|lua| bind_lua(lua, || Ok(())))))
        });

        assert!(res.is_ok())
    }

    #[test]
    fn test_drop_and_restore() {
        let lua1 = &mlua::Lua::new();
        let lua2 = &mlua::Lua::new();

        let reg1 = lua1.create_registry_value(1).unwrap();
        let reg2 = lua2.create_registry_value(2).unwrap();

        bind_lua(lua1, || {
            let res1 = with_lua(|lua1| Ok(lua1.owns_registry_value(&reg1))).unwrap();
            assert!(res1);

            let res2 = bind_lua(lua2, || {
                with_lua(|lua2| Ok(lua2.owns_registry_value(&reg2)))
            })
            .unwrap();
            assert!(res2);

            let res1 = with_lua(|lua1| Ok(lua1.owns_registry_value(&reg1))).unwrap();
            assert!(res1);
        })
    }

    #[test]
    fn test_panic_and_restore() {
        let lua1 = &mlua::Lua::new();
        let lua2 = &mlua::Lua::new();

        let reg1 = lua1.create_registry_value(1).unwrap();

        bind_lua(lua1, || {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                bind_lua(lua2, || {
                    with_lua(|_| -> Result<(), mlua::Error> { panic!("pow") }).unwrap();
                });
            }));

            let res1 = with_lua(|lua1| Ok(lua1.owns_registry_value(&reg1))).unwrap();
            assert!(res1);
        })
    }
}
