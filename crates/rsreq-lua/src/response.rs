#[derive(mlua::UserData)]
pub(crate) struct LuaResponse(reqwest::Response);

impl From<reqwest::Response> for LuaResponse {
    fn from(value: reqwest::Response) -> Self {
        Self(value)
    }
}

#[mlua::userdata_impl]
impl LuaResponse {}
