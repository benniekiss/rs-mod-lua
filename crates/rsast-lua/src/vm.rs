use std::{rc::Rc, sync::Arc};

use mlua::IntoLua;

use crate::pairs::LuaPairs;

#[derive(Clone, mlua::UserData, mlua::FromLua)]
pub(crate) struct LuaPestVm(Arc<pest_vm::Vm>);

impl From<pest_vm::Vm> for LuaPestVm {
    fn from(value: pest_vm::Vm) -> Self {
        Self(Arc::new(value))
    }
}

#[mlua::userdata_impl]
impl LuaPestVm {
    #[lua(name = "new", infallible)]
    pub(crate) fn lua_new(grammar: &str) -> (Option<Self>, Option<Vec<String>>) {
        match pest_meta::parse_and_optimize(grammar) {
            Ok((_, rules)) => (Some(pest_vm::Vm::new(rules).into()), None),
            Err(err) => (
                None,
                Some(err.iter().map(|e| e.to_string()).collect::<Vec<_>>()),
            ),
        }
    }

    #[lua(name = "validate")]
    pub(crate) fn lua_validate(
        &self,
        lua: &mlua::Lua,
        rule: &str,
        input: &str,
    ) -> mlua::Result<mlua::MultiValue> {
        let pairs = self.0.parse(rule, input);
        let mut mv = mlua::MultiValue::with_capacity(2);
        match pairs {
            Ok(_) => {
                mv.push_back(mlua::Value::Boolean(true));
                mv.push_back(mlua::Nil);
            },
            Err(err) => {
                mv.push_back(mlua::Value::Boolean(false));
                mv.push_back(err.to_string().into_lua(lua)?);
            },
        }

        Ok(mv)
    }

    #[lua(name = "parse")]
    pub(crate) fn lua_parse(
        &self,
        lua: &mlua::Lua,
        rule: &str,
        input: &str,
        callback: mlua::Function,
    ) -> mlua::Result<mlua::MultiValue> {
        let pairs = self.0.parse(rule, input).map_err(mlua::Error::runtime)?;
        lua.create_userdata::<LuaPairs>(LuaPairs::new(&Rc::new(input.to_string()), pairs))
            .and_then(|ud| callback.call(ud))
    }
}
