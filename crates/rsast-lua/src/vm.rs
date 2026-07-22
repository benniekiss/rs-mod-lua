use std::sync::Arc;

use mlua::IntoLua;

use crate::pairs::LuaPairs;

#[derive(Clone, mlua::UserData, mlua::FromLua)]
pub(crate) struct LuaPestVm {
    #[lua(skip)]
    vm: Arc<pest_vm::Vm>,
    #[lua(skip)]
    err_handlers: Option<(mlua::Function, mlua::Function)>,
}

impl From<pest_vm::Vm> for LuaPestVm {
    fn from(value: pest_vm::Vm) -> Self {
        Self {
            vm: Arc::new(value),
            err_handlers: None,
        }
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

    #[lua(name = "set_error_formatter", infallible)]
    pub(crate) fn lua_set_error_formatter(
        &mut self,
        rule_handler: mlua::Function,
        ws_handler: mlua::Function,
    ) {
        self.err_handlers = Some((rule_handler, ws_handler));
    }

    #[lua(skip)]
    fn handle_error(&self, input: &str, err: pest::error::Error<&str>) -> String {
        if let Some((rule_handler, ws_handler)) = self.err_handlers.clone() {
            let err_fn: pest::error::RuleToMessageFn<&str> =
                Box::new(move |rule| rule_handler.call(*rule).unwrap_or(None));
            let ws_fn: pest::error::IsWhitespaceFn =
                Box::new(move |text| ws_handler.call(text).unwrap_or(false));

            if let Some(e) = err.parse_attempts_error(input, &err_fn, &ws_fn) {
                return e.to_string();
            }
        }

        err.to_string()
    }

    #[lua(name = "validate")]
    pub(crate) fn lua_validate(
        &self,
        lua: &mlua::Lua,
        rule: &str,
        input: &str,
    ) -> mlua::Result<mlua::MultiValue> {
        let pairs = self.vm.parse(rule, input);
        let mut mv = mlua::MultiValue::with_capacity(2);
        match pairs {
            Ok(_) => {
                mv.push_back(mlua::Value::Boolean(true));
                mv.push_back(mlua::Nil);
            },
            Err(err) => {
                mv.push_back(mlua::Value::Boolean(false));
                mv.push_back(self.handle_error(input, err).into_lua(lua)?);
            },
        }

        Ok(mv)
    }

    #[lua(name = "parse")]
    pub(crate) fn lua_parse(
        &self,
        rule: &str,
        input: &str,
        callback: mlua::Function,
    ) -> mlua::Result<mlua::MultiValue> {
        let pairs: LuaPairs = self
            .vm
            .parse(rule, input)
            .map_err(|err| mlua::Error::external(self.handle_error(input, err)))?
            .into();

        callback.call(pairs)
    }
}
