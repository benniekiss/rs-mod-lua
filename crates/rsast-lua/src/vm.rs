use crate::pairs::LuaPairs;

#[derive(mlua::UserData)]
pub(crate) struct LuaPestVm(pest_vm::Vm);

impl From<pest_vm::Vm> for LuaPestVm {
    fn from(value: pest_vm::Vm) -> Self {
        Self(value)
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

    #[lua(name = "parse")]
    pub(crate) fn lua_parse(
        &self,
        lua: &mlua::Lua,
        rule: &str,
        input: &str,
        callback: mlua::Function,
    ) -> mlua::Result<mlua::MultiValue> {
        lua.scope(|scope| {
            let pairs = self.0.parse(rule, input).map_err(mlua::Error::runtime)?;
            let ud = scope.create_userdata::<LuaPairs>(pairs.into())?;
            callback.call(ud)
        })
    }
}
