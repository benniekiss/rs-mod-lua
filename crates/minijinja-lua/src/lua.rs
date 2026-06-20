use std::sync::atomic::{AtomicPtr, Ordering};

thread_local! {
    static CURRENT_LUA: AtomicPtr<mlua::Lua> = const { AtomicPtr::new(std::ptr::null_mut()) };
}

/// Allow access to a [`mlua::Lua`] instance across a `Send + Sync` boundary in module mode.
///
/// This code mirrors the [`minijinja-py`](https://github.com/mitsuhiko/minijinja/blob/29ac0b2936eacf83ebf781c52f4f4ffc3add4c52/minijinja-py/src/state.rs) implementation.
pub(crate) fn with_lua<R, F: FnOnce(&mlua::Lua) -> Result<R, mlua::Error>>(
    f: F,
) -> Result<R, mlua::Error> {
    CURRENT_LUA.with(|handle| {
        // SAFETY: The stored Lua pointer is only valid within the context of the `bind_lua` call
        // on the same thread which stored it. Callers must not attempt or otherwise retain
        // the `&Lua` reference, or any references to it, that could outlive the scope of the
        // `bind_lua` call.
        let ptr = unsafe { (handle.load(Ordering::Relaxed) as *const mlua::Lua).as_ref() };

        match ptr {
            Some(lua) => f(lua),
            None => Err(mlua::Error::runtime(
                "mlua::Lua state accessed outside of a render context.",
            )),
        }
    })
}

/// Invokes a function with the state stashed away.
///
/// This code mirrors the [`minijinja-py`](https://github.com/mitsuhiko/minijinja/blob/29ac0b2936eacf83ebf781c52f4f4ffc3add4c52/minijinja-py/src/state.rs) implementation.
pub(crate) fn bind_lua<R, F: FnOnce() -> R>(lua: &mlua::Lua, f: F) -> R {
    let old_handle = CURRENT_LUA
        .with(|handle| handle.swap(lua as *const mlua::Lua as *mut mlua::Lua, Ordering::Relaxed));

    let rv = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));

    CURRENT_LUA.with(|handle| handle.store(old_handle, Ordering::Relaxed));
    match rv {
        Ok(rv) => rv,
        Err(payload) => std::panic::resume_unwind(payload),
    }
}
