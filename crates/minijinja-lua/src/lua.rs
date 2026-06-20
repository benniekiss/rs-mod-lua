// SPDX-License-Identifier: MIT

use std::sync::atomic::{AtomicPtr, Ordering};

struct LuaGuard(*mut mlua::Lua);

impl LuaGuard {
    thread_local! {
        static LUA_POINTER: AtomicPtr<mlua::Lua> = const { AtomicPtr::new(std::ptr::null_mut()) };
    }

    /// Create a new guard, swapping the stored pointer with a pointer of the provided
    /// `&mlua::Lua`. The original pointer will be restored when the guard is dropped.
    fn new(lua: &mlua::Lua) -> Self {
        let ptr = Self::LUA_POINTER.with(|handle| {
            handle.swap(lua as *const mlua::Lua as *mut mlua::Lua, Ordering::Relaxed)
        });

        Self(ptr)
    }

    /// Invoke a function with access to a reference to the stored Lua pointer.
    fn with_lua<R, F>(f: F) -> Result<R, mlua::Error>
    where
        F: FnOnce(&mlua::Lua) -> Result<R, mlua::Error>,
    {
        Self::LUA_POINTER.with(|handle| {
            let ptr = handle.load(Ordering::Relaxed) as *const mlua::Lua;

            // SAFETY: The stored Lua pointer is only valid within the context of the `bind_lua`
            // call on the same thread which stored it, and the Lua reference must not outlive the
            // closure.
            let lua = unsafe { ptr.as_ref() }.ok_or_else(|| {
                mlua::Error::runtime("mlua::Lua state accessed outside of a render context.")
            })?;

            f(lua)
        })
    }

    /// Store a pointer to an `&mlua::Lua` reference and execute a function
    fn bind_lua<R, F>(lua: &mlua::Lua, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let _guard = Self::new(lua);
        f()
    }
}

impl Drop for LuaGuard {
    fn drop(&mut self) {
        Self::LUA_POINTER.with(|handle| handle.store(self.0, Ordering::Relaxed));
    }
}

/// Allow access to an [`mlua::Lua`] reference across a `Send + Sync` boundary in module mode.
///
/// This code mirrors the [`minijinja-py`](https://github.com/mitsuhiko/minijinja/blob/29ac0b2936eacf83ebf781c52f4f4ffc3add4c52/minijinja-py/src/state.rs) implementation.
pub(crate) fn with_lua<R, F>(f: F) -> Result<R, mlua::Error>
where
    F: FnOnce(&mlua::Lua) -> Result<R, mlua::Error>,
{
    LuaGuard::with_lua(f)
}

/// Invokes a function with the Lua state stashed away.
///
/// This code mirrors the [`minijinja-py`](https://github.com/mitsuhiko/minijinja/blob/29ac0b2936eacf83ebf781c52f4f4ffc3add4c52/minijinja-py/src/state.rs) implementation.
pub(crate) fn bind_lua<R, F>(lua: &mlua::Lua, f: F) -> R
where
    F: FnOnce() -> R,
{
    LuaGuard::bind_lua(lua, f)
}
