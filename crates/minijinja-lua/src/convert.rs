// SPDX-License-Identifier: MIT

use std::{
    cmp,
    fmt,
    fmt::Debug,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    sync::{
        Arc,
        Mutex,
        atomic::{AtomicBool, Ordering},
    },
};

use minijinja::{
    AutoEscape,
    Error as JinjaError,
    ErrorKind as JinjaErrorKind,
    UndefinedBehavior,
    syntax::{SyntaxConfig, SyntaxConfigBuilder},
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

use crate::{
    lua::with_lua,
    state::{LuaStateMut, LuaStateRef},
};

#[derive(Debug, Clone)]
pub(crate) struct LuaJinjaObjectWrapper<V> {
    key: Arc<mlua::RegistryKey>,
    pass_state: Arc<AtomicBool>,
    array_like: Arc<AtomicBool>,
    lua_type: PhantomData<fn() -> V>,
}

impl<V> LuaJinjaObjectWrapper<V>
where
    V: Clone + mlua::FromLua + mlua::IntoLua + 'static,
    LuaJinjaObjectWrapper<V>: JinjaObject,
{
    /// Create a new wrapper around the [`mlua::Value`] associated with `key`
    pub(crate) fn new(key: mlua::RegistryKey) -> Self {
        Self::from(key)
    }

    /// Get the stored `RegistryKey`
    pub(crate) fn key(&self) -> Arc<mlua::RegistryKey> {
        self.key.clone()
    }

    /// Create a wrapper from an [`mlua::Value`]
    pub(crate) fn from_value(lua: &mlua::Lua, value: &V) -> mlua::Result<Self> {
        lua.create_registry_value(value.clone()).map(Self::new)
    }

    /// Convert the wrapper to an [`mlua::Value`]
    pub(crate) fn to_value(&self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        lua.registry_value::<V>(&self.key())
            .and_then(|v| v.into_lua(lua))
    }

    /// Create a reference to a wrapper from a [`minijinja::Value`]
    pub(crate) fn from_jinja_ref(value: &JinjaValue) -> Result<&Self, JinjaError> {
        value
            .downcast_object_ref::<Self>()
            .ok_or_else(|| JinjaError::new(JinjaErrorKind::CannotDeserialize, ""))
    }

    /// Convert the wrapper to a [`minijinja::Value`]
    pub(crate) fn to_jinja(&self) -> JinjaValue {
        JinjaValue::from_object(self.clone())
    }

    /// Whether to pass a [`minijinja::State`] to function calls, if provided
    pub(crate) fn pass_state(&self) -> bool {
        self.pass_state.load(Ordering::Relaxed)
    }

    /// Set whether to pass a [`minijinja::State`] to function calls, if provided
    pub(crate) fn set_pass_state(&mut self, enable: bool) {
        self.pass_state.store(enable, Ordering::Relaxed);
    }

    /// Whether to treat the wrapper as an array
    pub(crate) fn array_like(&self) -> bool {
        self.array_like.load(Ordering::Relaxed)
    }

    /// Set whether to treat the wrapper as an array
    pub(crate) fn set_array_like(&mut self, enable: bool) {
        self.array_like.store(enable, Ordering::Relaxed);
    }

    /// Execute a callback with [`mlua::mlua::Lua`] and the retrieved [`mlua::Value`] as arguments
    pub(crate) fn with<R, F, T>(&self, f: F) -> Result<R, JinjaError>
    where
        F: FnOnce(&mlua::Lua, T) -> Result<R, mlua::Error>,
        T: mlua::FromLua,
    {
        with_lua(|lua| {
            lua.registry_value::<T>(&self.key())
                .and_then(|value| f(lua, value))
        })
        .map_err(|err| err_to_minijinja_err(err, JinjaErrorKind::InvalidOperation))
    }
}

impl<V> Drop for LuaJinjaObjectWrapper<V> {
    fn drop(&mut self) {
        let _ = with_lua(|lua| {
            lua.expire_registry_values();
            Ok(())
        });
    }
}

impl<V> From<mlua::RegistryKey> for LuaJinjaObjectWrapper<V> {
    fn from(value: mlua::RegistryKey) -> Self {
        Self {
            key: Arc::new(value),
            pass_state: Arc::new(AtomicBool::new(false)),
            array_like: Arc::new(AtomicBool::new(false)),
            lua_type: PhantomData,
        }
    }
}

impl<V> From<LuaJinjaObjectWrapper<V>> for JinjaValue
where
    V: Clone + mlua::FromLua + mlua::IntoLua + 'static,
    LuaJinjaObjectWrapper<V>: JinjaObject,
{
    fn from(value: LuaJinjaObjectWrapper<V>) -> Self {
        value.to_jinja()
    }
}

impl<V> TryFrom<JinjaValue> for LuaJinjaObjectWrapper<V>
where
    V: Clone + mlua::FromLua + mlua::IntoLua + 'static,
    LuaJinjaObjectWrapper<V>: JinjaObject,
{
    type Error = JinjaError;

    fn try_from(value: JinjaValue) -> Result<Self, Self::Error> {
        Self::from_jinja_ref(&value).cloned()
    }
}

impl<V> mlua::FromLua for LuaJinjaObjectWrapper<V>
where
    V: Clone + mlua::FromLua + mlua::IntoLua + 'static,
    LuaJinjaObjectWrapper<V>: JinjaObject,
{
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        let value = V::from_lua(value, lua)?;
        LuaJinjaObjectWrapper::from_value(lua, &value)
    }
}

impl<V> mlua::IntoLua for LuaJinjaObjectWrapper<V>
where
    V: Clone + mlua::FromLua + mlua::IntoLua + 'static,
    LuaJinjaObjectWrapper<V>: JinjaObject,
{
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        self.to_value(lua)
    }
}

/// A wrapper around an [`mlua::Function`]. It provides access to the [`mlua::Function`]
/// within a `minijinja` context by dynamically getting the object via the stored
/// [`mlua::RegistryKey`].
pub(crate) type LuaFunctionObject = LuaJinjaObjectWrapper<mlua::Function>;

impl fmt::Display for LuaFunctionObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<function>")
    }
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
                if self.pass_state() {
                    let val = state
                        .map(|s| scope.create_userdata::<LuaStateRef>(s.into()))
                        .transpose()?
                        .map(mlua::Value::UserData)
                        .unwrap_or_default();
                    mv.push_front(val);
                }
                func.call(mv)
                    .map(|mut v| lua_multi_to_minijinja(lua, &mut v))
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
                if self.pass_state() {
                    let val = state
                        .map(|s| scope.create_userdata::<LuaStateMut>(s.into()))
                        .transpose()?
                        .map(mlua::Value::UserData)
                        .unwrap_or_default();
                    mv.push_front(val);
                }
                func.call(mv)
                    .map(|mut v| lua_multi_to_minijinja(lua, &mut v))
            })
        })
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
pub(crate) type LuaTableObject = LuaJinjaObjectWrapper<mlua::Table>;

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
                    let val = scope
                        .create_userdata::<LuaStateRef>(state.into())
                        .map(mlua::Value::UserData)?;
                    mv.push_front(val);
                }
                table
                    .call(mv)
                    .map(|mut v| lua_multi_to_minijinja(lua, &mut v).unwrap_or_default())
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
                    let val = scope
                        .create_userdata::<LuaStateRef>(state.into())
                        .map(mlua::Value::UserData)?;
                    mv.push_front(val);
                }
                table
                    .call_method(method, mv)
                    .map(|mut v| lua_multi_to_minijinja(lua, &mut v).unwrap_or_default())
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
        self.with(|lua, table: mlua::Table| {
            let items = match self.array_like() {
                true => table
                    .sequence_values::<mlua::Value>()
                    .map(|v| {
                        let v = v.unwrap_or_default();
                        lua_to_minijinja(lua, &v).unwrap_or_default()
                    })
                    .collect::<Vec<JinjaValue>>(),
                _ => table
                    .pairs::<mlua::Value, mlua::Value>()
                    .map(|pair| {
                        let k = pair.unwrap_or_default().0;
                        lua_to_minijinja(lua, &k).unwrap_or_default()
                    })
                    .collect::<Vec<JinjaValue>>(),
            };

            Ok(items)
        })
        .map(|items| Enumerator::Iter(Box::new(items.into_iter())))
        .unwrap_or(Enumerator::NonEnumerable)
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
pub(crate) type LuaUserDataObject = LuaJinjaObjectWrapper<mlua::AnyUserData>;

impl fmt::Display for LuaUserDataObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = self.with(|_, userdata: mlua::AnyUserData| userdata.to_string());

        match repr {
            Ok(s) => write!(f, "{s}"),
            Err(_) => write!(f, "<userdata>"),
        }
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
                    let val = scope
                        .create_userdata::<LuaStateRef>(state.into())
                        .map(mlua::Value::UserData)?;
                    mv.push_front(val);
                }
                userdata
                    .call(mv)
                    .map(|mut v| lua_multi_to_minijinja(lua, &mut v).unwrap_or_default())
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
                    let val = scope
                        .create_userdata::<LuaStateRef>(state.into())
                        .map(mlua::Value::UserData)?;
                    mv.push_front(val);
                }
                userdata
                    .call_method(method, mv)
                    .map(|mut v| lua_multi_to_minijinja(lua, &mut v).unwrap_or_default())
            })
        })
        .map_err(|err| err_to_minijinja_err(err, JinjaErrorKind::UnknownMethod))
    }

    fn get_value(self: &Arc<Self>, key: &JinjaValue) -> Option<JinjaValue> {
        self.with(|lua, userdata: mlua::AnyUserData| {
            lua.to_value(key)
                .and_then(|k| userdata.get(k))
                .map(|v| lua_to_minijinja(lua, &v))
        })
        .ok()
        .flatten()
    }

    fn custom_cmp(self: &Arc<Self>, other: &minijinja::value::DynObject) -> Option<cmp::Ordering> {
        let other = other.downcast_ref::<LuaUserDataObject>()?;

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

#[derive(Clone)]
pub(crate) struct LuaAutoEscape(AutoEscape);

impl Default for LuaAutoEscape {
    fn default() -> Self {
        Self(AutoEscape::None)
    }
}

impl Deref for LuaAutoEscape {
    type Target = AutoEscape;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<AutoEscape> for LuaAutoEscape {
    fn from(value: AutoEscape) -> Self {
        LuaAutoEscape(value)
    }
}

impl From<LuaAutoEscape> for AutoEscape {
    fn from(value: LuaAutoEscape) -> Self {
        value.0
    }
}

impl TryFrom<&str> for LuaAutoEscape {
    type Error = mlua::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "html" => Ok(AutoEscape::Html.into()),
            "json" => Ok(AutoEscape::Json.into()),
            "none" => Ok(AutoEscape::None.into()),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: "string",
                to: "AutoEscape".to_string(),
                message: Some(format!(
                    "arguments must be one of 'html', 'json', or 'none': {}",
                    value
                )),
            }),
        }
    }
}

impl mlua::FromLua for LuaAutoEscape {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        let autoescape = value.to_string()?;
        match autoescape.to_lowercase().as_str() {
            "html" => Ok(AutoEscape::Html.into()),
            "json" => Ok(AutoEscape::Json.into()),
            "none" => Ok(AutoEscape::None.into()),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "AutoEscape".to_string(),
                message: Some("arguments must be one of 'html', 'json', or 'none'".to_string()),
            }),
        }
    }
}

impl mlua::IntoLua for LuaAutoEscape {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        match self.deref() {
            AutoEscape::Html => "html".into_lua(lua),
            AutoEscape::Json => "json".into_lua(lua),
            AutoEscape::None => "none".into_lua(lua),
            AutoEscape::Custom(s) => s.into_lua(lua),
            _ => Err(mlua::Error::runtime("invalid AutoEscape value")),
        }
    }
}

#[derive(Clone)]
pub(crate) struct LuaUndefinedBehavior(UndefinedBehavior);

impl Default for LuaUndefinedBehavior {
    fn default() -> Self {
        Self(UndefinedBehavior::Lenient)
    }
}

impl Deref for LuaUndefinedBehavior {
    type Target = UndefinedBehavior;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<UndefinedBehavior> for LuaUndefinedBehavior {
    fn from(value: UndefinedBehavior) -> Self {
        LuaUndefinedBehavior(value)
    }
}

impl From<LuaUndefinedBehavior> for UndefinedBehavior {
    fn from(value: LuaUndefinedBehavior) -> Self {
        value.0
    }
}

impl TryFrom<&str> for LuaUndefinedBehavior {
    type Error = mlua::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "chainable" => Ok(UndefinedBehavior::Chainable.into()),
            "lenient" => Ok(UndefinedBehavior::Lenient.into()),
            "semi-strict" => Ok(UndefinedBehavior::SemiStrict.into()),
            "strict" => Ok(UndefinedBehavior::Strict.into()),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: "string",
                to: "UndefinedBehavior".to_string(),
                message: Some(
                    "arguments must be one of 'chainable', 'lenient', 'semi-strict', or 'strict'"
                        .to_string(),
                ),
            }),
        }
    }
}

impl mlua::FromLua for LuaUndefinedBehavior {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        let ub = value.to_string()?;
        match ub.to_lowercase().as_str() {
            "chainable" => Ok(UndefinedBehavior::Chainable.into()),
            "lenient" => Ok(UndefinedBehavior::Lenient.into()),
            "semi-strict" => Ok(UndefinedBehavior::SemiStrict.into()),
            "strict" => Ok(UndefinedBehavior::Strict.into()),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "UndefinedBehavior".to_string(),
                message: Some(
                    "arguments must be one of 'chainable', 'lenient', 'semi-strict', or 'strict'"
                        .to_string(),
                ),
            }),
        }
    }
}

impl mlua::IntoLua for LuaUndefinedBehavior {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        match self.deref() {
            UndefinedBehavior::Chainable => "chainable".into_lua(lua),
            UndefinedBehavior::Lenient => "lenient".into_lua(lua),
            UndefinedBehavior::SemiStrict => "semi-strict".into_lua(lua),
            UndefinedBehavior::Strict => "strict".into_lua(lua),
            _ => Err(mlua::Error::runtime("invalid UndefinedBehavior value")),
        }
    }
}

#[derive(mlua::UserData, mlua::FromLua, Clone)]
pub(crate) struct LuaSyntaxConfig(SyntaxConfig);

impl From<SyntaxConfig> for LuaSyntaxConfig {
    fn from(value: SyntaxConfig) -> Self {
        Self(value)
    }
}

impl From<LuaSyntaxConfig> for SyntaxConfig {
    fn from(value: LuaSyntaxConfig) -> Self {
        value.0
    }
}

impl Deref for LuaSyntaxConfig {
    type Target = SyntaxConfig;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[mlua::userdata_impl]
impl LuaSyntaxConfig {
    #[lua(name = "builder", infallible)]
    pub(crate) fn lua_builder() -> LuaSyntaxConfigBuilder {
        SyntaxConfig::builder().into()
    }

    #[lua(name = "block_delimiters", infallible)]
    pub(crate) fn lua_block_delimiters(&self) -> (String, String) {
        let (s, e) = self.0.block_delimiters();
        (s.to_string(), e.to_string())
    }

    #[lua(name = "variable_delimiters", infallible)]
    pub(crate) fn lua_variable_delimiters(&self) -> (String, String) {
        let (s, e) = self.0.variable_delimiters();
        (s.to_string(), e.to_string())
    }

    #[lua(name = "comment_delimiters", infallible)]
    pub(crate) fn lua_comment_delimiters(&self) -> (String, String) {
        let (s, e) = self.0.comment_delimiters();
        (s.to_string(), e.to_string())
    }

    #[lua(name = "line_statement_prefix", infallible)]
    pub(crate) fn lua_line_statement_prefix(&self) -> Option<String> {
        self.0.line_statement_prefix().map(|s| s.to_string())
    }

    #[lua(name = "line_comment_prefix", infallible)]
    pub(crate) fn lua_line_comment_prefix(&self) -> Option<String> {
        self.0.line_comment_prefix().map(|s| s.to_string())
    }
}

#[derive(mlua::UserData, mlua::FromLua, Clone)]
pub(crate) struct LuaSyntaxConfigBuilder(Arc<Mutex<SyntaxConfigBuilder>>);

impl From<SyntaxConfigBuilder> for LuaSyntaxConfigBuilder {
    fn from(value: SyntaxConfigBuilder) -> Self {
        Self(Arc::new(Mutex::new(value)))
    }
}

#[mlua::userdata_impl]
impl LuaSyntaxConfigBuilder {
    #[lua(name = "build")]
    pub(crate) fn lua_build(&self) -> mlua::Result<LuaSyntaxConfig> {
        self.0
            .lock()
            .map_err(mlua::Error::runtime)?
            .build()
            .map(|c| c.into())
            .map_err(mlua::Error::external)
    }

    #[lua(name = "block_delimiters")]
    pub(crate) fn lua_block_delimiters(&self, start: String, end: String) -> mlua::Result<Self> {
        self.0
            .lock()
            .map_err(mlua::Error::runtime)?
            .block_delimiters(start, end);

        Ok(self.clone())
    }

    #[lua(name = "variable_delimiters")]
    pub(crate) fn lua_variable_delimiters(&self, start: String, end: String) -> mlua::Result<Self> {
        self.0
            .lock()
            .map_err(mlua::Error::runtime)?
            .variable_delimiters(start, end);

        Ok(self.clone())
    }

    #[lua(name = "comment_delimiters")]
    pub(crate) fn lua_comment_delimiters(&self, start: String, end: String) -> mlua::Result<Self> {
        self.0
            .lock()
            .map_err(mlua::Error::runtime)?
            .comment_delimiters(start, end);

        Ok(self.clone())
    }

    #[lua(name = "line_statement_prefix")]
    pub(crate) fn lua_line_statement_prefix(&self, prefix: String) -> mlua::Result<Self> {
        self.0
            .lock()
            .map_err(mlua::Error::runtime)?
            .line_statement_prefix(prefix);

        Ok(self.clone())
    }

    #[lua(name = "line_comment_prefix")]
    pub(crate) fn lua_line_comment_prefix(&self, prefix: String) -> mlua::Result<Self> {
        self.0
            .lock()
            .map_err(mlua::Error::runtime)?
            .line_comment_prefix(prefix);

        Ok(self.clone())
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
        mlua::Value::UserData(ud) => LuaUserDataObject::from_value(lua, ud)
            .map(|v| v.into())
            .ok(),

        mlua::Value::Table(table) => LuaTableObject::from_value(lua, table)
            .map(|mut obj| {
                if table_is_array_like(table, Some(false)) {
                    obj.set_array_like(true);
                };
                obj.into()
            })
            .ok(),

        mlua::Value::Function(func) => LuaFunctionObject::from_value(lua, func)
            .map(|v| v.into())
            .ok(),
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
            if let Ok(obj) = LuaUserDataObject::from_jinja_ref(value) {
                mv.push_back(obj.to_value(lua).ok()?)
            } else if let Ok(obj) = LuaTableObject::from_jinja_ref(value) {
                mv.push_back(obj.to_value(lua).ok()?)
            } else if let Ok(obj) = LuaFunctionObject::from_jinja_ref(value) {
                mv.push_back(obj.to_value(lua).ok()?)
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
                .unwrap_or_else(|| lua_to_minijinja(lua, &v).unwrap_or_default())
        });

    let mut args = args
        .iter()
        .map(|v| lua_to_minijinja(lua, v).unwrap_or_default())
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
    use mlua::{FromLua, IntoLua};

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
        let func = lua.create_function(|_: &mlua::Lua, ()| Ok("BOO")).unwrap();

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
        assert!(jinja.downcast_object_ref::<LuaUserDataObject>().is_some());

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
        let lua = setup();

        let lua_ae: LuaAutoEscape = AutoEscape::Html.into();
        let lua_ae_str = lua_ae.into_lua(&lua).unwrap().to_string().unwrap();
        assert_eq!(lua_ae_str, "html");

        let jinja_ae = LuaAutoEscape::from_lua(lua_ae_str.into_lua(&lua).unwrap(), &lua).unwrap();
        assert_eq!(*jinja_ae, AutoEscape::Html);
    }

    #[test]
    fn test_autoescape_roundtrip_json() {
        let lua = setup();

        let lua_ae: LuaAutoEscape = AutoEscape::Json.into();
        let lua_ae_str = lua_ae.into_lua(&lua).unwrap().to_string().unwrap();
        assert_eq!(lua_ae_str, "json");

        let jinja_ae = LuaAutoEscape::from_lua(lua_ae_str.into_lua(&lua).unwrap(), &lua).unwrap();
        assert_eq!(*jinja_ae, AutoEscape::Json);
    }

    #[test]
    fn test_autoescape_roundtrip_none() {
        let lua = setup();

        let lua_ae: LuaAutoEscape = AutoEscape::None.into();
        let lua_ae_str = lua_ae.into_lua(&lua).unwrap().to_string().unwrap();
        assert_eq!(lua_ae_str, "none");

        let jinja_ae = LuaAutoEscape::from_lua(lua_ae_str.into_lua(&lua).unwrap(), &lua).unwrap();
        assert_eq!(*jinja_ae, AutoEscape::None);
    }

    #[test]
    fn test_autoescape_custom() {
        let lua = setup();

        let lua_ae: LuaAutoEscape = AutoEscape::Custom("test custom").into();
        let lua_ae_str = lua_ae.into_lua(&lua).unwrap().to_string().unwrap();
        assert_eq!(lua_ae_str, "test custom");

        assert!(LuaAutoEscape::from_lua(lua_ae_str.into_lua(&lua).unwrap(), &lua).is_err());
    }

    #[test]
    fn test_autoescape_roundtrip_invalid() {
        let lua = setup();

        assert!(LuaAutoEscape::from_lua("xml".into_lua(&lua).unwrap(), &lua).is_err());
    }

    // UNDEFINED BEHAVIOR CONVERSION TESTS //

    #[test]
    fn test_undefined_behavior_roundtrip_chainable() {
        let lua = setup();

        let lua_ae: LuaUndefinedBehavior = UndefinedBehavior::Chainable.into();
        let lua_ae_str = lua_ae.into_lua(&lua).unwrap().to_string().unwrap();
        assert_eq!(lua_ae_str, "chainable");

        let jinja_ae =
            LuaUndefinedBehavior::from_lua(lua_ae_str.into_lua(&lua).unwrap(), &lua).unwrap();
        assert_eq!(*jinja_ae, UndefinedBehavior::Chainable);
    }

    #[test]
    fn test_undefined_behavior_roundtrip_lenient() {
        let lua = setup();

        let lua_ae: LuaUndefinedBehavior = UndefinedBehavior::Lenient.into();
        let lua_ae_str = lua_ae.into_lua(&lua).unwrap().to_string().unwrap();
        assert_eq!(lua_ae_str, "lenient");

        let jinja_ae =
            LuaUndefinedBehavior::from_lua(lua_ae_str.into_lua(&lua).unwrap(), &lua).unwrap();
        assert_eq!(*jinja_ae, UndefinedBehavior::Lenient);
    }

    #[test]
    fn test_undefined_behavior_roundtrip_semi_strict() {
        let lua = setup();

        let lua_ae: LuaUndefinedBehavior = UndefinedBehavior::SemiStrict.into();
        let lua_ae_str = lua_ae.into_lua(&lua).unwrap().to_string().unwrap();
        assert_eq!(lua_ae_str, "semi-strict");

        let jinja_ae =
            LuaUndefinedBehavior::from_lua(lua_ae_str.into_lua(&lua).unwrap(), &lua).unwrap();
        assert_eq!(*jinja_ae, UndefinedBehavior::SemiStrict);
    }

    #[test]
    fn test_undefined_behavior_roundtrip_strict() {
        let lua = setup();

        let lua_ae: LuaUndefinedBehavior = UndefinedBehavior::Strict.into();
        let lua_ae_str = lua_ae.into_lua(&lua).unwrap().to_string().unwrap();
        assert_eq!(lua_ae_str, "strict");

        let jinja_ae =
            LuaUndefinedBehavior::from_lua(lua_ae_str.into_lua(&lua).unwrap(), &lua).unwrap();
        assert_eq!(*jinja_ae, UndefinedBehavior::Strict);
    }

    #[test]
    fn test_undefined_behavior_roundtrip_invalid() {
        let lua = setup();

        assert!(LuaUndefinedBehavior::from_lua("none".into_lua(&lua).unwrap(), &lua).is_err());
    }

    // SYNTAX CONFIG TESTS //

    #[test]
    fn test_syntax_config() {
        let builder = LuaSyntaxConfig::lua_builder();
        builder
            .lua_block_delimiters("BLOCK_S".to_string(), "BLOCK_E".to_string())
            .unwrap();
        builder
            .lua_variable_delimiters("VAR_S".to_string(), "VAR_E".to_string())
            .unwrap();
        builder
            .lua_comment_delimiters("COM_S".to_string(), "COM_E".to_string())
            .unwrap();
        builder.lua_line_statement_prefix("LS".to_string()).unwrap();
        builder.lua_line_comment_prefix("LC".to_string()).unwrap();

        let config = builder.lua_build().unwrap();

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
