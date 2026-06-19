// SPDX-License-Identifier: MIT

use std::{
    cmp,
    fmt,
    ops::{Deref, DerefMut},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use minijinja::{
    AutoEscape,
    Error as JinjaError,
    ErrorKind as JinjaErrorKind,
    UndefinedBehavior,
    syntax::SyntaxConfig,
    value::{
        Enumerator,
        Kwargs,
        Object as JinjaObject,
        ObjectRepr as JinjaObjectRepr,
        Value as JinjaValue,
        ValueKind as JinjaValueKind,
    },
};
use mlua::{LuaSerdeExt, ObjectLike};

use crate::state::{LuaStateMut, LuaStateRef, with_lua};

pub(crate) trait LuaObject {
    /// Create a new wrapper around the [`mlua::Value`] associated with `key`
    fn new(key: mlua::RegistryKey) -> Self;

    /// Get the stored `RegistryKey`
    fn key(&self) -> Arc<mlua::RegistryKey>;

    /// Whether to pass a [`minijinja::State`] to function calls, if provided
    fn pass_state(&self) -> bool;

    /// Set whether to pass a [`minijinja::State`] to function calls, if provided
    fn set_pass_state(&mut self, enable: bool);

    /// Execute a callback with [`mlua::mlua::Lua`] and the retrieved [`mlua::Value`] as arguments
    fn with<R, F, T>(&self, f: F) -> Result<R, JinjaError>
    where
        F: FnOnce(&mlua::Lua, T) -> Result<R, mlua::Error>,
        T: mlua::FromLua,
    {
        with_lua(|lua| {
            let value = lua.registry_value::<T>(&self.key())?;

            f(lua, value)
        })
        .map_err(|err| err_to_minijinja_err(err, JinjaErrorKind::InvalidOperation))
    }
}

/// A wrapper around an [`mlua::Function`]. It provides access to the [`mlua::Function`]
/// within a `minijinja` context by dynamically getting the object via the stored
/// [`mlua::RegistryKey`].
#[derive(Debug)]
pub(crate) struct LuaFunctionObject {
    key: Arc<mlua::RegistryKey>,
    pass_state: Arc<AtomicBool>,
}

impl LuaFunctionObject {
    pub(crate) fn with_func(
        &self,
        args: &[JinjaValue],
        state: Option<&minijinja::State>,
    ) -> Result<Option<JinjaValue>, JinjaError> {
        self.with(|lua, func: mlua::Function| {
            let mut mv = minijinja_args_to_lua(lua, args);

            // Using `mlua::Lua::scope` here allows passing the `minijinja::State` to the callback.
            // Since `minijinja::State` is not `'static`, this enables passing a
            // temporarily created `mlua::UserData` to the callback, which is then
            // destructured at the end of the scope.
            //
            // This prevents misuse in lua, such as if the callback assigned the `minijinja::State`
            // to a global variable.
            lua.scope(|scope| {
                if let Some(st) = state
                    && self.pass_state()
                {
                    let userdate = scope.create_userdata::<LuaStateRef>(st.into())?;
                    mv.push_front(mlua::Value::UserData(userdate.clone()));
                };

                let res = &mut func.call(mv)?;

                Ok(lua_multi_to_minijinja(lua, res))
            })
        })
    }

    pub(crate) fn with_func_mut(
        &self,
        args: &[JinjaValue],
        state: Option<&mut minijinja::State>,
    ) -> Result<Option<JinjaValue>, JinjaError> {
        self.with(|lua, func: mlua::Function| {
            let mut mv = minijinja_args_to_lua(lua, args);

            // Using `mlua::Lua::scope` here allows passing the `minijinja::State` to the callback.
            // Since `minijinja::State` is not `'static`, this enables passing a
            // temporarily created `mlua::UserData` to the callback, which is then
            // destructured at the end of the scope.
            //
            // This prevents misuse in lua, such as if the callback assigned the `minijinja::State`
            // to a global variable.
            lua.scope(|scope| {
                if let Some(st) = state
                    && self.pass_state()
                {
                    let userdate = scope.create_userdata::<LuaStateMut>(st.into())?;
                    mv.push_front(mlua::Value::UserData(userdate.clone()));
                };

                let res = &mut func.call(mv)?;

                Ok(lua_multi_to_minijinja(lua, res))
            })
        })
    }
}

impl fmt::Display for LuaFunctionObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<function>")
    }
}

impl Drop for LuaFunctionObject {
    fn drop(&mut self) {
        let _ = with_lua(|lua| {
            lua.expire_registry_values();
            Ok(())
        });
    }
}

impl LuaObject for LuaFunctionObject {
    fn new(key: mlua::RegistryKey) -> Self {
        Self {
            key: Arc::new(key),
            pass_state: Arc::new(AtomicBool::new(false)),
        }
    }

    fn key(&self) -> Arc<mlua::RegistryKey> {
        Arc::clone(&self.key)
    }

    fn pass_state(&self) -> bool {
        self.pass_state.load(Ordering::Relaxed)
    }

    fn set_pass_state(&mut self, enable: bool) {
        self.pass_state.store(enable, Ordering::Relaxed);
    }
}

impl JinjaObject for LuaFunctionObject {
    fn repr(self: &Arc<Self>) -> JinjaObjectRepr {
        JinjaObjectRepr::Plain
    }

    fn render(self: &Arc<Self>, f: &mut fmt::Formatter<'_>) -> fmt::Result
    where
        Self: Sized + 'static,
    {
        fmt::Display::fmt(self, f)
    }

    fn call(
        self: &std::sync::Arc<Self>,
        state: &minijinja::State<'_, '_>,
        args: &[JinjaValue],
    ) -> Result<JinjaValue, minijinja::Error> {
        self.with_func(args, Some(state))?
            .ok_or_else(|| JinjaError::new(JinjaErrorKind::InvalidOperation, "no value returned"))
    }
}

/// A wrapper around an [`mlua::Table`]. It provides access to the [`mlua::Table`]
/// within a `minijinja` context by dynamically getting the object via the stored
/// [`mlua::RegistryKey`].
#[derive(Debug)]
pub(crate) struct LuaTableObject {
    key: Arc<mlua::RegistryKey>,
    pass_state: Arc<AtomicBool>,
    array_like: Arc<AtomicBool>,
}

impl LuaTableObject {
    fn array_like(&self) -> bool {
        self.array_like.load(Ordering::Relaxed)
    }

    fn set_array_like(&mut self, enable: bool) {
        self.array_like.store(enable, Ordering::Relaxed);
    }
}

impl fmt::Display for LuaTableObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = self.with(|_, table: mlua::Table| {
            Ok(table
                .call_method::<String>(mlua::MetaMethod::ToString.name(), mlua::Nil)
                .unwrap_or(JinjaValue::from_serialize(table).to_string()))
        });

        match repr {
            Ok(s) => write!(f, "{s}"),
            Err(_) => write!(f, "<table>"),
        }
    }
}

impl Drop for LuaTableObject {
    fn drop(&mut self) {
        let _ = with_lua(|lua| {
            lua.expire_registry_values();
            Ok(())
        });
    }
}

impl LuaObject for LuaTableObject {
    fn new(key: mlua::RegistryKey) -> Self {
        Self {
            key: Arc::new(key),
            pass_state: Arc::new(AtomicBool::new(false)),
            array_like: Arc::new(AtomicBool::new(false)),
        }
    }

    fn key(&self) -> Arc<mlua::RegistryKey> {
        Arc::clone(&self.key)
    }

    fn pass_state(&self) -> bool {
        self.pass_state.load(Ordering::Relaxed)
    }

    fn set_pass_state(&mut self, enable: bool) {
        self.pass_state.store(enable, Ordering::Relaxed);
    }
}

impl JinjaObject for LuaTableObject {
    fn repr(self: &Arc<Self>) -> JinjaObjectRepr {
        if self.array_like() {
            JinjaObjectRepr::Seq
        } else {
            JinjaObjectRepr::Map
        }
    }

    fn render(self: &Arc<Self>, f: &mut fmt::Formatter<'_>) -> fmt::Result
    where
        Self: Sized + 'static,
    {
        fmt::Display::fmt(self, f)
    }

    fn call(
        self: &std::sync::Arc<Self>,
        state: &minijinja::State<'_, '_>,
        args: &[JinjaValue],
    ) -> Result<JinjaValue, minijinja::Error> {
        self.with(|lua, table: mlua::Table| {
            let mut mv = minijinja_args_to_lua(lua, args);

            lua.scope(move |scope| {
                if self.pass_state() {
                    let userdate = scope.create_userdata::<LuaStateRef>(state.into())?;
                    mv.push_front(mlua::Value::UserData(userdate.clone()));
                };

                let res = &mut table.call(mv)?;

                Ok(lua_multi_to_minijinja(lua, res).unwrap_or(JinjaValue::UNDEFINED))
            })
        })
    }

    fn call_method(
        self: &std::sync::Arc<Self>,
        state: &minijinja::State<'_, '_>,
        method: &str,
        args: &[JinjaValue],
    ) -> Result<JinjaValue, JinjaError> {
        self.with(|lua, table: mlua::Table| {
            let mut mv = minijinja_args_to_lua(lua, args);

            lua.scope(move |scope| {
                if self.pass_state() {
                    let userdate = scope.create_userdata::<LuaStateRef>(state.into())?;
                    mv.push_front(mlua::Value::UserData(userdate.clone()));
                };

                let res = &mut table.call_method(method, mv)?;

                Ok(lua_multi_to_minijinja(lua, res).unwrap_or(JinjaValue::UNDEFINED))
            })
        })
        .map_err(|err| err_to_minijinja_err(err, JinjaErrorKind::UnknownMethod))
    }

    fn get_value(self: &Arc<Self>, key: &JinjaValue) -> Option<JinjaValue> {
        self.with(|lua, table: mlua::Table| {
            let mut key = lua.to_value(key)?;

            // Since lua is 1-indexed, if the provided value is an integer,
            // and the table is array-like, assume it is meant as an index
            // into the table and +1 to it
            if let Some(num) = key.as_integer()
                && self.array_like()
            {
                key = mlua::Value::Integer(num + 1)
            }

            let value: mlua::Value = table.get(key)?;

            // If the table did not return a value, return `None` to fallback to global lookups.
            // Otherwise, this prevents the use of global variables when the render context
            // is cast as a LuaTableObject
            if value.is_nil() {
                return Ok(None);
            }

            Ok(lua_to_minijinja(lua, &value))
        })
        .ok()
        .flatten()
    }

    fn enumerate(self: &std::sync::Arc<Self>) -> Enumerator {
        let items = self.with(|lua, table: mlua::Table| {
            if self.array_like() {
                table
                    .sequence_values::<mlua::Value>()
                    .map(|v| {
                        let v = v?;
                        let value = lua_to_minijinja(lua, &v).unwrap_or(JinjaValue::UNDEFINED);

                        Ok(value)
                    })
                    .collect::<Result<Vec<JinjaValue>, mlua::Error>>()
            } else {
                table
                    .pairs::<mlua::Value, mlua::Value>()
                    .map(|pair| {
                        let (k, _v) = pair?;

                        let key = lua_to_minijinja(lua, &k).unwrap_or(JinjaValue::UNDEFINED);

                        Ok(key)
                    })
                    .collect::<Result<Vec<JinjaValue>, mlua::Error>>()
            }
        });

        match items {
            Ok(items) => Enumerator::Iter(Box::new(items.into_iter())),
            Err(_) => Enumerator::NonEnumerable,
        }
    }

    fn custom_cmp(self: &Arc<Self>, other: &minijinja::value::DynObject) -> Option<cmp::Ordering> {
        let other = other.downcast_ref::<LuaTableObject>()?;

        self.with(|lua, table: mlua::Table| {
            let other_table = lua.registry_value::<mlua::Table>(&other.key)?;

            if table.equals(&other_table)? {
                return Ok(Some(cmp::Ordering::Equal));
            };

            let res = match table.call_method::<bool>("__lt", other_table) {
                Ok(true) => Some(cmp::Ordering::Less),
                Ok(false) => Some(cmp::Ordering::Greater),
                Err(_) => None,
            };

            Ok(res)
        })
        .ok()
        .flatten()
    }
}

/// A wrapper around an [`mlua::UserData`]. It provides access to the [`mlua::UserData`]
/// within a `minijinja` context by dynamically getting the object via the stored
/// [`mlua::RegistryKey`].
#[derive(Debug)]
pub(crate) struct LuaUserDataObject {
    key: Arc<mlua::RegistryKey>,
    pass_state: Arc<AtomicBool>,
}

impl fmt::Display for LuaUserDataObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = self.with(|_, userdata: mlua::AnyUserData| userdata.to_string());

        match repr {
            Ok(s) => write!(f, "{s}"),
            Err(_) => write!(f, "<userdata>"),
        }
    }
}

impl Drop for LuaUserDataObject {
    fn drop(&mut self) {
        let _ = with_lua(|lua| {
            lua.expire_registry_values();
            Ok(())
        });
    }
}

impl LuaObject for LuaUserDataObject {
    fn new(key: mlua::RegistryKey) -> Self {
        Self {
            key: Arc::new(key),
            pass_state: Arc::new(AtomicBool::new(false)),
        }
    }

    fn key(&self) -> Arc<mlua::RegistryKey> {
        Arc::clone(&self.key)
    }

    fn pass_state(&self) -> bool {
        self.pass_state.load(Ordering::Relaxed)
    }

    fn set_pass_state(&mut self, enable: bool) {
        self.pass_state.store(enable, Ordering::Relaxed);
    }
}

impl JinjaObject for LuaUserDataObject {
    fn repr(self: &Arc<Self>) -> JinjaObjectRepr {
        JinjaObjectRepr::Plain
    }

    fn render(self: &Arc<Self>, f: &mut fmt::Formatter<'_>) -> fmt::Result
    where
        Self: Sized + 'static,
    {
        fmt::Display::fmt(self, f)
    }

    fn call(
        self: &std::sync::Arc<Self>,
        state: &minijinja::State<'_, '_>,
        args: &[JinjaValue],
    ) -> Result<JinjaValue, minijinja::Error> {
        self.with(|lua, userdata: mlua::AnyUserData| {
            let mut mv = minijinja_args_to_lua(lua, args);

            lua.scope(move |scope| {
                if self.pass_state() {
                    let ud = scope.create_userdata::<LuaStateRef>(state.into())?;
                    mv.push_front(mlua::Value::UserData(ud.clone()));
                };

                let res = &mut userdata.call(mv)?;

                Ok(lua_multi_to_minijinja(lua, res).unwrap_or(JinjaValue::UNDEFINED))
            })
        })
    }

    fn call_method(
        self: &std::sync::Arc<Self>,
        state: &minijinja::State<'_, '_>,
        method: &str,
        args: &[JinjaValue],
    ) -> Result<JinjaValue, JinjaError> {
        self.with(|lua, userdata: mlua::AnyUserData| {
            let mut mv = minijinja_args_to_lua(lua, args);

            lua.scope(move |scope| {
                if self.pass_state() {
                    let ud = scope.create_userdata::<LuaStateRef>(state.into())?;
                    mv.push_front(mlua::Value::UserData(ud.clone()));
                };

                let res = &mut userdata.call_method(method, mv)?;

                Ok(lua_multi_to_minijinja(lua, res).unwrap_or(JinjaValue::UNDEFINED))
            })
        })
        .map_err(|err| err_to_minijinja_err(err, JinjaErrorKind::UnknownMethod))
    }

    fn get_value(self: &Arc<Self>, key: &JinjaValue) -> Option<JinjaValue> {
        self.with(|lua, userdata: mlua::AnyUserData| {
            let key = lua.to_value(key)?;
            let value = userdata.get(key)?;

            Ok(lua_to_minijinja(lua, &value))
        })
        .ok()
        .flatten()
    }

    fn custom_cmp(self: &Arc<Self>, other: &minijinja::value::DynObject) -> Option<cmp::Ordering> {
        let other = other.downcast_ref::<LuaTableObject>()?;

        self.with(|lua: &mlua::Lua, userdata: mlua::AnyUserData| {
            let otherdata = lua.registry_value::<mlua::AnyUserData>(&other.key)?;

            if let Ok(true) = userdata.call_method::<bool>("__eq", &otherdata) {
                return Ok(Some(cmp::Ordering::Equal));
            };

            let res = match userdata.call_method::<bool>("__lt", &otherdata) {
                Ok(true) => Some(cmp::Ordering::Less),
                Ok(false) => Some(cmp::Ordering::Greater),
                Err(_) => None,
            };

            Ok(res)
        })
        .ok()
        .flatten()
    }
}

/// A wrapper around a list of [`minijinja::Value`] to preserve [`mlua::MultiValue`] characteristics
/// for Lua functions which return multiple values.
#[derive(Debug)]
pub(crate) struct LuaMultiValueObject(Vec<JinjaValue>);

impl Deref for LuaMultiValueObject {
    type Target = Vec<JinjaValue>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaMultiValueObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl JinjaObject for LuaMultiValueObject {
    fn repr(self: &Arc<Self>) -> JinjaObjectRepr {
        JinjaObjectRepr::Seq
    }

    fn get_value(self: &Arc<Self>, key: &JinjaValue) -> Option<JinjaValue> {
        match key.as_usize() {
            Some(k) => self.0.get(k).cloned(),
            None => None,
        }
    }

    fn enumerate(self: &Arc<Self>) -> Enumerator {
        Enumerator::Values(self.0.clone())
    }
}

/// Convert an [`mlua::Value`] to a [`minijinja::Value`].
///
/// If `value` is an [`mlua::Value::Table`], [`mlua::Value::Function`], or
/// [`mlua::Value::UserData`], it is wrapped in a struct implementing [`minijinja::value::Object`].
/// Otherwise, the  object is serialized to a [`minijinja::Value`] via `serde` using
/// [`minijinja::Value::from_serialize`]
pub(crate) fn lua_to_minijinja(lua: &mlua::Lua, value: &mlua::Value) -> Option<JinjaValue> {
    match value {
        mlua::Value::UserData(userdata) => {
            let res = lua.create_registry_value(userdata);

            match res {
                Ok(key) => Some(JinjaValue::from_object(LuaUserDataObject::new(key))),
                Err(_) => None,
            }
        },
        mlua::Value::Table(table) => {
            let res = lua.create_registry_value(table);

            match res {
                Ok(key) => {
                    let mut obj = LuaTableObject::new(key);
                    if table_is_array_like(table, Some(false)) {
                        obj.set_array_like(true);
                    }

                    Some(JinjaValue::from_object(obj))
                },
                Err(_) => None,
            }
        },
        mlua::Value::Function(func) => {
            let res = lua.create_registry_value(func);

            match res {
                Ok(key) => Some(JinjaValue::from_object(LuaFunctionObject::new(key))),
                Err(_) => None,
            }
        },
        // minijinja::Value::from_serialize converts `mlua::Value::Nil` to
        // `minijinja::Value::None` otherwise. Semantically, `None` is more
        // similar to the `mlua::NULL` value.
        mlua::Value::Nil => Some(JinjaValue::UNDEFINED),
        v if v.is_null() => Some(JinjaValue::from(())),
        _ => Some(JinjaValue::from_serialize(value)),
    }
}

/// Convert an [`mlua::MultiValue`] to a [`minijinja::Value`].
///
/// An `mv` passed in with more than 1 value is wrapped in `LuaMultiValueObject` to preserve
/// multivalue argument semantics going from `minijinja` to Lua.
pub(crate) fn lua_multi_to_minijinja(
    lua: &mlua::Lua,
    mv: &mut mlua::MultiValue,
) -> Option<JinjaValue> {
    match mv.len() {
        0 => Some(JinjaValue::UNDEFINED),
        // If a function only returns one value, do not wrap it in a `LuaMultiValueObject`.
        // Otherwise, all Lua return values will be treated as lists by `minijinja`
        1 => lua_to_minijinja(lua, &mv.pop_front().unwrap_or_default()),
        _ => Some(JinjaValue::from_object(LuaMultiValueObject(
            mv.iter()
                .map(|v| lua_to_minijinja(lua, v).unwrap_or_default())
                .collect::<Vec<_>>(),
        ))),
    }
}

/// Convert a [`minijinja::Value`] to an [`mlua::Value`].
///
/// If the [`minijinja::Value`] is a [`LuaObject`], the underlying [`mlua::Table`] is retrieved so
/// that round trips `lua -> minijinja -> lua` maintain access to the same [`mlua::Table`].
/// Otherwise, objects are converted via `serde` using [`mlua::mlua::Lua::to_value`].
pub(crate) fn minijinja_to_lua(lua: &mlua::Lua, value: &JinjaValue) -> Option<mlua::MultiValue> {
    let mut mv = mlua::MultiValue::with_capacity(1);

    match value.kind() {
        JinjaValueKind::Undefined => mv.push_back(mlua::Value::Nil),
        JinjaValueKind::None => mv.push_back(mlua::Value::NULL),
        _ => {
            if let Some(obj) = value.downcast_object_ref::<LuaUserDataObject>() {
                let userdata = lua.registry_value::<mlua::AnyUserData>(&obj.key).ok()?;
                mv.push_back(mlua::Value::UserData(userdata))
            } else if let Some(obj) = value.downcast_object_ref::<LuaTableObject>() {
                let table = lua.registry_value::<mlua::Table>(&obj.key).ok()?;
                mv.push_back(mlua::Value::Table(table))
            } else if let Some(obj) = value.downcast_object_ref::<LuaFunctionObject>() {
                let func = lua.registry_value::<mlua::Function>(&obj.key).ok()?;
                mv.push_back(mlua::Value::Function(func))
            } else if let Some(obj) = value.downcast_object_ref::<LuaMultiValueObject>() {
                for val in obj.iter() {
                    mv.append(&mut minijinja_to_lua(lua, val).unwrap_or_default())
                }
            } else {
                mv.push_back(lua.to_value(&value).unwrap_or_default())
            }
        },
    }

    Some(mv)
}

/// Convert a slice of [`minijinja::Value`] to an [`mlua::MultiValue`]
///
/// This is used to convert arguments passed to minijinja filters, tests, and
/// functions into lua arguments to be handled by the registered lua callbacks.
pub(crate) fn minijinja_args_to_lua(lua: &mlua::Lua, args: &[JinjaValue]) -> mlua::MultiValue {
    let mut mv = mlua::MultiValue::with_capacity(args.len());
    for val in args.iter() {
        mv.append(&mut minijinja_to_lua(lua, val).unwrap_or_default());
    }

    mv
}

/// Convert [`mlua::MultiValue`] arguments into a
/// [`Vec<minijinja::Value>`](minijinja::value::ArgType)
///
/// If `accept_kwargs` is `true`, special handling is applied to the last argument if it is an
/// [`mlua::Table`] by converting it to [`minijinja::value::Kwargs`].
///
/// This is currently only used in the [`LuaState`] methods `apply_filter()`, `perform_test()`,
/// and `call_macro()` to pass keyword arguments to those callbacks.
pub(crate) fn lua_args_to_minijinja(
    lua: &mlua::Lua,
    args: &mut mlua::MultiValue,
    accept_kwargs: bool,
) -> Vec<JinjaValue> {
    let kwargs = args
        .pop_back_if(|v| accept_kwargs && v.is_table())
        .map(|v| {
            v.as_table()
                .map(|tbl| {
                    JinjaValue::from(Kwargs::from_iter(
                        tbl.pairs::<mlua::Value, mlua::Value>().filter_map(|pair| {
                            let (k, v) = pair.ok()?;

                            let key = k.to_string().ok()?;
                            let value = lua_to_minijinja(lua, &v)?;

                            Some((key, value))
                        }),
                    ))
                })
                // If for some reason `.as_table()` fails, follow through with a regular conversion.
                .unwrap_or_else(|| lua_to_minijinja(lua, &v).unwrap_or(JinjaValue::UNDEFINED))
        });

    let mut args = args
        .iter()
        .map(|v| lua_to_minijinja(lua, v).unwrap_or(JinjaValue::UNDEFINED))
        .collect::<Vec<JinjaValue>>();

    if let Some(kw) = kwargs {
        args.push(kw)
    };

    args
}

/// Convert an [`mlua::Error`] error into the specified [`minijinja::ErrorKind`]
pub(crate) fn err_to_minijinja_err<T: std::error::Error>(
    err: T,
    kind: JinjaErrorKind,
) -> JinjaError {
    JinjaError::new(kind, err.to_string())
}

/// Convert a [`minijinja::AutoEscape`] variant to a string
pub(crate) fn auto_escape_to_lua(autoescape: AutoEscape) -> Option<String> {
    match autoescape {
        AutoEscape::Html => Some("html".to_string()),
        AutoEscape::Json => Some("json".to_string()),
        AutoEscape::None => Some("none".to_string()),
        AutoEscape::Custom(s) => Some(s.to_string()),
        _ => None,
    }
}

/// Convert a string to a [`minijinja::AutoEscape`] variant
pub(crate) fn lua_to_auto_escape(autoescape: &str) -> Result<AutoEscape, mlua::Error> {
    let au = match autoescape.to_lowercase().as_str() {
        "html" => AutoEscape::Html,
        "json" => AutoEscape::Json,
        "none" => AutoEscape::None,
        _ => return Err(mlua::Error::FromLuaConversionError { from: "auto_escape", to: "minijinja::AutoEscape".to_string(), message: Some("Failed to convert {} to minijinja::AutoEscape. Arguments must be one of 'html', 'json', or 'none'".to_string()) })};

    Ok(au)
}

/// Convert a [`minijinja::UndefinedBehavior`] variant to a string
///
/// The conversion is case-insensitive
pub(crate) fn undefined_behavior_to_lua(behavior: UndefinedBehavior) -> Option<String> {
    match behavior {
        UndefinedBehavior::Chainable => Some("chainable".to_string()),
        UndefinedBehavior::Lenient => Some("lenient".to_string()),
        UndefinedBehavior::SemiStrict => Some("semi-strict".to_string()),
        UndefinedBehavior::Strict => Some("strict".to_string()),
        _ => None,
    }
}

/// Convert a string to a [`minijinja::UndefinedBehavior`] variant.
///
/// The conversion is case-insensitive
pub(crate) fn lua_to_undefined_behavior(behavior: &str) -> Result<UndefinedBehavior, mlua::Error> {
    let ub = match behavior.to_lowercase().as_str() {
        "chainable" => UndefinedBehavior::Chainable,
        "lenient" => UndefinedBehavior::Lenient,
        "semi-strict" => UndefinedBehavior::SemiStrict,
        "strict" => UndefinedBehavior::Strict,
        _ => return Err(mlua::Error::FromLuaConversionError { from: "undefined_behavior", to: "minijinja::UndefinedBehavior".to_string(), message: Some("Failed to convert {} to minijinja::UndefinedBehavior. Arguments must be one of 'chainable', 'lenient', 'semi-strict', or 'strict'".to_string()) })
    };

    Ok(ub)
}

/// Convert an [`mlua::Table`] to a [`minijinja::syntax::SyntaxConfig`]
pub(crate) fn lua_to_syntax_config(syntax: mlua::Table) -> Result<SyntaxConfig, JinjaError> {
    let defaults = SyntaxConfig::default();

    let (block_s, block_e) =
        optional_delimiter_pair(&syntax, "block_delimiters")?.unwrap_or_else(|| {
            let (s, e) = defaults.block_delimiters();
            (s.to_string(), e.to_string())
        });

    let (var_s, var_e) =
        optional_delimiter_pair(&syntax, "variable_delimiters")?.unwrap_or_else(|| {
            let (s, e) = defaults.variable_delimiters();
            (s.to_string(), e.to_string())
        });

    let (com_s, com_e) =
        optional_delimiter_pair(&syntax, "comment_delimiters")?.unwrap_or_else(|| {
            let (s, e) = defaults.comment_delimiters();
            (s.to_string(), e.to_string())
        });

    let line_statement = optional_string(&syntax, "line_statement_prefix")?
        .unwrap_or_else(|| defaults.line_statement_prefix().unwrap_or("").to_string());

    let line_comment = optional_string(&syntax, "line_comment_prefix")?
        .unwrap_or_else(|| defaults.line_comment_prefix().unwrap_or("").to_string());

    SyntaxConfig::builder()
        .block_delimiters(block_s, block_e)
        .variable_delimiters(var_s, var_e)
        .comment_delimiters(com_s, com_e)
        .line_statement_prefix(line_statement)
        .line_comment_prefix(line_comment)
        .build()
}

/// Returns `Some((start, end))` if the key is present, `None` if absent or
/// nil, or an error if the value is present but malformed.
fn optional_delimiter_pair(
    syntax: &mlua::Table,
    name: &str,
) -> Result<Option<(String, String)>, JinjaError> {
    match syntax.get::<mlua::Value>(name) {
        Ok(mlua::Value::Nil) | Err(_) => Ok(None),
        Ok(mlua::Value::Table(table)) => table_to_syntax_args(&table, name).map(Some),
        Ok(_) => Err(JinjaError::new(
            JinjaErrorKind::InvalidDelimiter,
            format!("{name} must be an array-like table of 2 strings"),
        )),
    }
}

/// Returns `Some(string)` if the key is present, `None` if absent or nil.
fn optional_string(syntax: &mlua::Table, name: &str) -> Result<Option<String>, JinjaError> {
    match syntax.get::<mlua::Value>(name) {
        Ok(mlua::Value::Nil) | Err(_) => Ok(None),
        Ok(mlua::Value::String(s)) => s
            .to_str()
            .map(|s| Some(s.to_string()))
            .map_err(|err| err_to_minijinja_err(err, JinjaErrorKind::InvalidDelimiter)),
        Ok(_) => Err(JinjaError::new(
            JinjaErrorKind::InvalidDelimiter,
            format!("{name} must be a string"),
        )),
    }
}

/// Helper to parse an [`mlua::Table`] into [`minijinja::syntax::SyntaxConfig`] setting arguments.
///
/// Valid values are array-like tables with only 2 items.
fn table_to_syntax_args(table: &mlua::Table, name: &str) -> Result<(String, String), JinjaError> {
    if table_is_array_like(table, None) {
        match table.len() {
            Ok(2) => {
                let a: String = table
                    .get(1)
                    .map_err(|err| err_to_minijinja_err(err, JinjaErrorKind::InvalidDelimiter))?;
                let b: String = table
                    .get(2)
                    .map_err(|err| err_to_minijinja_err(err, JinjaErrorKind::InvalidDelimiter))?;

                Ok((a, b))
            },
            Ok(len) => {
                let message = if len > 2 {
                    format!("Too many args provided. Expected 2, provided {}", len)
                } else {
                    format!("Not enough args provided. Expected 2, provided {}", len)
                };

                Err(JinjaError::new(JinjaErrorKind::InvalidDelimiter, message))
            },
            Err(err) => Err(err_to_minijinja_err(err, JinjaErrorKind::InvalidDelimiter)),
        }
    } else {
        Err(JinjaError::new(
            JinjaErrorKind::InvalidDelimiter,
            format!("{} should be an array-like table of 2 strings", name),
        ))
    }
}

/// Check if an [`mlua::Table`] is array-like. That is, check if all of the
/// keys are sequential numbers with no holes.
///
/// Empty tables can optionally be encoded as arrays.
fn table_is_array_like(table: &mlua::Table, empty_as_array: Option<bool>) -> bool {
    let seq_len = table.raw_len();

    if seq_len == 0 {
        return empty_as_array.unwrap_or(false) & table.is_empty();
    }

    // If the sequence length matches the total number of pairs,
    // there are no non-integer or out-of-sequence keys
    seq_len == table.pairs::<mlua::Value, mlua::Value>().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> mlua::Lua {
        mlua::Lua::new()
    }

    // TYPE CONVERSION TESTS //

    #[test]
    fn test_lua_minijinja_roundtrip_nil() {
        let lua = setup();
        let jinja = lua_to_minijinja(&lua, &mlua::Value::Nil).unwrap();
        assert!(jinja.is_undefined());

        let value = minijinja_to_lua(&lua, &jinja).unwrap();
        assert!(value.back().unwrap().is_nil());
    }

    #[test]
    fn test_lua_minijinja_roundtrip_null() {
        let lua = setup();
        let jinja = lua_to_minijinja(&lua, &mlua::Value::NULL).unwrap();
        assert!(jinja.is_none());

        let value = minijinja_to_lua(&lua, &jinja).unwrap();
        assert!(value.back().unwrap().is_null());
    }

    #[test]
    fn test_lua_minijinja_roundtrip_bool() {
        let lua = setup();
        let jinja = lua_to_minijinja(&lua, &mlua::Value::Boolean(true)).unwrap();
        assert_eq!(jinja.kind(), JinjaValueKind::Bool);

        let value = minijinja_to_lua(&lua, &jinja).unwrap();
        assert!(value.back().unwrap().is_boolean());
    }

    #[test]
    fn test_lua_minijinja_roundtrip_string() {
        let lua = setup();
        let jinja = lua_to_minijinja(
            &lua,
            &mlua::Value::String(lua.create_string("test").unwrap()),
        )
        .unwrap();
        assert_eq!(jinja.as_str().unwrap(), "test");

        let value = minijinja_to_lua(&lua, &jinja).unwrap();
        assert_eq!(value.back().unwrap().as_string().unwrap(), "test");
    }

    #[test]
    fn test_lua_minijinja_roundtrip_number() {
        let lua = setup();
        let jinja = lua_to_minijinja(&lua, &mlua::Value::Number(99.99f64)).unwrap();
        assert!(jinja.is_number());

        let value = minijinja_to_lua(&lua, &jinja).unwrap();
        assert!(value.back().unwrap().is_number());
    }

    #[test]
    fn test_lua_minijinja_roundtrip_integer() {
        let lua = setup();
        let jinja = lua_to_minijinja(&lua, &mlua::Value::Integer(99i64)).unwrap();
        assert!(jinja.is_integer());

        let value = minijinja_to_lua(&lua, &jinja).unwrap();
        assert!(value.back().unwrap().is_integer());
    }

    #[test]
    fn test_lua_minijinja_roundtrip_table() {
        let lua = setup();
        let table = lua.create_table().unwrap();
        table.set("a", 1).unwrap();

        let jinja = lua_to_minijinja(&lua, &mlua::Value::Table(table)).unwrap();
        assert_eq!(jinja.kind(), JinjaValueKind::Map);
        assert!(jinja.downcast_object_ref::<LuaTableObject>().is_some());

        let value = minijinja_to_lua(&lua, &jinja).unwrap();
        assert!(value.back().unwrap().is_table());
    }

    #[test]
    fn test_lua_minijinja_roundtrip_array() {
        let lua = setup();
        let table = lua.create_table().unwrap();
        table.set(1, "a").unwrap();
        table.set(2, "b").unwrap();

        let jinja = lua_to_minijinja(&lua, &mlua::Value::Table(table)).unwrap();
        assert_eq!(jinja.kind(), JinjaValueKind::Seq);
        assert!(jinja.downcast_object_ref::<LuaTableObject>().is_some());

        let value = minijinja_to_lua(&lua, &jinja).unwrap();
        assert!(value.front().unwrap().is_table());
    }

    #[test]
    fn test_lua_minijinja_roundtrip_function() {
        let lua = setup();
        let func = lua
            .create_function(|_: &mlua::Lua, _: mlua::Value| Ok("BOO"))
            .unwrap();

        let jinja = lua_to_minijinja(&lua, &mlua::Value::Function(func)).unwrap();
        assert_eq!(jinja.kind(), JinjaValueKind::Plain);
        assert!(jinja.downcast_object_ref::<LuaFunctionObject>().is_some());

        let value = minijinja_to_lua(&lua, &jinja).unwrap();
        assert!(value.front().unwrap().is_function());
    }

    #[test]
    fn test_lua_minijinja_roundtrip_userdata() {
        struct TestData {}

        impl mlua::UserData for TestData {}

        let lua = setup();
        let userdata = lua.create_userdata(TestData {}).unwrap();

        let jinja = lua_to_minijinja(&lua, &mlua::Value::UserData(userdata)).unwrap();
        assert_eq!(jinja.kind(), JinjaValueKind::Plain);
        assert!(jinja.downcast_object_ref::<TestData>().is_some());

        let value = minijinja_to_lua(&lua, &jinja).unwrap();
        assert!(value.front().unwrap().is_userdata());
    }

    #[test]
    fn test_lua_minijinja_roundtrip_multivalue() {
        let lua = setup();
        let table = lua.create_table().unwrap();
        let func = lua.create_function(|_, ()| Ok(())).unwrap();

        let mv = &mut mlua::MultiValue::from_vec(vec![
            mlua::Value::Table(table),
            mlua::Value::Function(func),
            mlua::Value::Integer(1),
        ]);

        let jinja = lua_multi_to_minijinja(&lua, mv).unwrap();
        assert_eq!(jinja.kind(), JinjaValueKind::Seq);

        let obj = jinja.downcast_object_ref::<LuaMultiValueObject>().unwrap();
        assert_eq!(obj.len(), 3);
        assert!(
            obj.first()
                .unwrap()
                .downcast_object_ref::<LuaTableObject>()
                .is_some()
        );
        assert!(
            obj.get(1)
                .unwrap()
                .downcast_object_ref::<LuaFunctionObject>()
                .is_some()
        );
        assert!(obj.get(2).unwrap().is_integer());

        let value = minijinja_to_lua(&lua, &jinja).unwrap();
        assert_eq!(value.len(), 3);
        assert!(value.front().unwrap().is_table());
        assert!(value.get(1).unwrap().is_function());
        assert!(value.get(2).unwrap().is_integer());
    }

    #[test]
    fn test_lua_minijinja_roundtrip_kwargs() {
        let lua = setup();
        let table = lua.create_table().unwrap();
        table.set("foo", "bar").unwrap();

        let args = &mut mlua::MultiValue::from_iter(vec![
            mlua::Value::Integer(1),
            mlua::Value::Integer(2),
            mlua::Value::Table(table),
        ]);

        let jinja = lua_args_to_minijinja(&lua, args, true);
        assert!(jinja.last().unwrap().is_kwargs());
        assert_eq!(
            jinja.last().unwrap().get_attr("foo").unwrap().to_string(),
            "bar"
        );

        let value = minijinja_args_to_lua(&lua, &jinja);
        assert!(value.iter().last().unwrap().is_table());
        assert_eq!(
            value
                .iter()
                .last()
                .unwrap()
                .as_table()
                .unwrap()
                .get::<String>("foo")
                .unwrap(),
            "bar"
        );
    }

    #[test]
    fn test_lua_minijinja_roundtrip_no_kwargs() {
        let lua = setup();
        let table = lua.create_table().unwrap();
        table.set("foo", "bar").unwrap();

        let args = &mut mlua::MultiValue::from_iter(vec![
            mlua::Value::Integer(1),
            mlua::Value::Integer(2),
            mlua::Value::Table(table),
        ]);

        let jinja = lua_args_to_minijinja(&lua, args, false);
        assert!(
            jinja
                .last()
                .unwrap()
                .downcast_object_ref::<LuaTableObject>()
                .is_some()
        );

        let value = minijinja_args_to_lua(&lua, &jinja);
        assert!(value.iter().last().unwrap().is_table());
        assert_eq!(
            value
                .iter()
                .last()
                .unwrap()
                .as_table()
                .unwrap()
                .get::<String>("foo")
                .unwrap(),
            "bar"
        );
    }

    // AUTO ESCAPE CONVERSION TESTS //

    #[test]
    fn test_autoescape_roundtrip_html() {
        let lua_ae = auto_escape_to_lua(AutoEscape::Html).unwrap();
        assert_eq!(lua_ae, "html");

        let jinja_ae: AutoEscape = lua_to_auto_escape(&lua_ae).unwrap();
        assert_eq!(jinja_ae, AutoEscape::Html);
    }

    #[test]
    fn test_autoescape_roundtrip_json() {
        let lua_ae = auto_escape_to_lua(AutoEscape::Json).unwrap();
        assert_eq!(lua_ae, "json");

        let jinja_ae: AutoEscape = lua_to_auto_escape(&lua_ae).unwrap();
        assert_eq!(jinja_ae, AutoEscape::Json);
    }

    #[test]
    fn test_autoescape_roundtrip_none() {
        let lua_ae = auto_escape_to_lua(AutoEscape::None).unwrap();
        assert_eq!(lua_ae, "none");

        let jinja_ae: AutoEscape = lua_to_auto_escape(&lua_ae).unwrap();
        assert_eq!(jinja_ae, AutoEscape::None);
    }

    #[test]
    fn test_autoescape_custom() {
        let lua_ae = auto_escape_to_lua(AutoEscape::Custom("test custom")).unwrap();
        assert_eq!(lua_ae, "test custom");

        let jinja_ae = lua_to_auto_escape(&lua_ae);
        assert!(jinja_ae.is_err());
    }

    #[test]
    fn test_autoescape_roundtrip_invalid() {
        assert!(lua_to_auto_escape("xml").is_err());
    }

    // UNDEFINED BEHAVIOR CONVERSION TESTS //

    #[test]
    fn test_undefined_behavior_roundtrip_chainable() {
        let lua_ae = undefined_behavior_to_lua(UndefinedBehavior::Chainable).unwrap();
        assert_eq!(lua_ae, "chainable");

        let jinja_ae: UndefinedBehavior = lua_to_undefined_behavior(&lua_ae).unwrap();
        assert_eq!(jinja_ae, UndefinedBehavior::Chainable);
    }

    #[test]
    fn test_undefined_behavior_roundtrip_lenient() {
        let lua_ae = undefined_behavior_to_lua(UndefinedBehavior::Lenient).unwrap();
        assert_eq!(lua_ae, "lenient");

        let jinja_ae: UndefinedBehavior = lua_to_undefined_behavior(&lua_ae).unwrap();
        assert_eq!(jinja_ae, UndefinedBehavior::Lenient);
    }

    #[test]
    fn test_undefined_behavior_roundtrip_semi_strict() {
        let lua_ae = undefined_behavior_to_lua(UndefinedBehavior::SemiStrict).unwrap();
        assert_eq!(lua_ae, "semi-strict");

        let jinja_ae: UndefinedBehavior = lua_to_undefined_behavior(&lua_ae).unwrap();
        assert_eq!(jinja_ae, UndefinedBehavior::SemiStrict);
    }

    #[test]
    fn test_undefined_behavior_roundtrip_strict() {
        let lua_ae = undefined_behavior_to_lua(UndefinedBehavior::Strict).unwrap();
        assert_eq!(lua_ae, "strict");

        let jinja_ae: UndefinedBehavior = lua_to_undefined_behavior(&lua_ae).unwrap();
        assert_eq!(jinja_ae, UndefinedBehavior::Strict);
    }

    #[test]
    fn test_undefined_behavior_roundtrip_invalid() {
        assert!(lua_to_undefined_behavior("none").is_err());
    }

    // SYNTAX CONFIG TESTS //

    #[test]
    fn test_table_to_syntax_args_valid() {
        let lua = setup();
        let args = lua.create_sequence_from(["START", "END"]).unwrap();
        let (s, e) = table_to_syntax_args(&args, "block_delimiters").unwrap();
        assert_eq!(s, "START");
        assert_eq!(e, "END");
    }

    #[test]
    fn test_table_to_syntax_args_too_many() {
        let lua = setup();
        let args = lua
            .create_sequence_from(["START", "END", "too many"])
            .unwrap();
        assert!(table_to_syntax_args(&args, "block_delimiters").is_err());
    }

    #[test]
    fn test_table_to_syntax_args_too_few() {
        let lua = setup();
        let args = lua.create_sequence_from(["START"]).unwrap();
        assert!(table_to_syntax_args(&args, "block_delimiters").is_err());
    }

    #[test]
    fn test_table_to_syntax_args_not_array() {
        let lua = setup();
        let table = lua.create_table().unwrap();
        table.set("start", "START").unwrap();
        table.set("end", "END").unwrap();

        assert!(table_to_syntax_args(&table, "block_delimiters").is_err());
    }

    #[test]
    fn test_syntax_config_defaults() {
        let lua = setup();
        let table = lua.create_table().unwrap();
        let config = lua_to_syntax_config(table).unwrap();
        let defaults = SyntaxConfig::default();
        assert_eq!(config.block_delimiters(), defaults.block_delimiters());
        assert_eq!(config.variable_delimiters(), defaults.variable_delimiters());
        assert_eq!(config.comment_delimiters(), defaults.comment_delimiters());
        assert_eq!(
            config.line_statement_prefix(),
            defaults.line_statement_prefix()
        );
        assert_eq!(config.line_comment_prefix(), defaults.line_comment_prefix());
    }

    #[test]
    fn test_syntax_config() {
        let lua = setup();
        let table = lua.create_table().unwrap();
        table
            .set("block_delimiters", ["BLOCK_S", "BLOCK_E"])
            .unwrap();
        table
            .set("variable_delimiters", ["VAR_S", "VAR_E"])
            .unwrap();
        table.set("comment_delimiters", ["COM_S", "COM_E"]).unwrap();
        table.set("line_statement_prefix", "LS").unwrap();
        table.set("line_comment_prefix", "LC").unwrap();

        let config = lua_to_syntax_config(table).unwrap();
        assert_eq!(config.block_delimiters(), ("BLOCK_S", "BLOCK_E"));
        assert_eq!(config.variable_delimiters(), ("VAR_S", "VAR_E"));
        assert_eq!(config.comment_delimiters(), ("COM_S", "COM_E"));
        assert_eq!(config.line_statement_prefix(), Some("LS"));
        assert_eq!(config.line_comment_prefix(), Some("LC"));
    }

    // ARRAY-LIKE TABLE TESTS //

    #[test]
    fn test_table_array_like() {
        let lua = setup();
        let table = lua.create_table().unwrap();
        table.set(1, "a").unwrap();
        table.set(2, "b").unwrap();
        table.set(3, "c").unwrap();
        assert!(table_is_array_like(&table, None));
    }

    #[test]
    fn test_table_array_like_hole() {
        let lua = setup();
        let table = lua.create_table().unwrap();
        table.set(1, "a").unwrap();
        table.set(3, "c").unwrap();
        assert!(!table_is_array_like(&table, None));
    }

    #[test]
    fn test_table_array_like_sparse() {
        let lua = setup();
        let table = lua.create_table().unwrap();
        table.set(3, "c").unwrap();
        assert!(!table_is_array_like(&table, None));
    }

    #[test]
    fn test_table_array_like_mixed() {
        let lua = setup();
        let table = lua.create_table().unwrap();
        table.set(1, "a").unwrap();
        table.set("2", "b").unwrap();
        assert!(!table_is_array_like(&table, None));
    }

    #[test]
    fn test_table_array_like_empty() {
        let lua = setup();
        let table = lua.create_table().unwrap();
        assert!(!table_is_array_like(&table, None));
        assert!(table_is_array_like(&table, Some(true)));
    }
}
