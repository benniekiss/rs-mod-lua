#[derive(mlua::UserData, Clone)]
pub(crate) struct LuaClient(reqwest::Client);

impl From<reqwest::Client> for LuaClient {
    fn from(value: reqwest::Client) -> Self {
        Self(value)
    }
}

#[mlua::userdata_impl]
impl LuaClient {
    #[lua(name = "new", infallible)]
    fn lua_new() -> Self {
        reqwest::Client::new().into()
    }
}
