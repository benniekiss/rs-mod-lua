use crate::{
    request::{LuaRequest, LuaRequestBuilder},
    response::LuaResponse,
};

#[derive(mlua::UserData, mlua::FromLua, Clone)]
pub(crate) struct LuaClient(reqwest::Client);

impl From<reqwest::Client> for LuaClient {
    fn from(value: reqwest::Client) -> Self {
        Self(value)
    }
}

impl From<LuaClient> for reqwest::Client {
    fn from(value: LuaClient) -> Self {
        value.0
    }
}

#[mlua::userdata_impl]
impl LuaClient {
    #[lua(name = "new", infallible)]
    fn lua_new() -> Self {
        reqwest::Client::new().into()
    }

    #[lua(name = "builder", infallible)]
    fn lua_builder() -> LuaClientBuilder {
        reqwest::ClientBuilder::new().into()
    }

    #[lua(name = "get", infallible)]
    fn lua_get(&self, url: String) -> LuaRequestBuilder {
        self.0.get(url).into()
    }

    #[lua(name = "post", infallible)]
    fn lua_post(&self, url: String) -> LuaRequestBuilder {
        self.0.post(url).into()
    }

    #[lua(name = "put", infallible)]
    fn lua_put(&self, url: String) -> LuaRequestBuilder {
        self.0.put(url).into()
    }

    #[lua(name = "patch", infallible)]
    fn lua_patch(&self, url: String) -> LuaRequestBuilder {
        self.0.patch(url).into()
    }

    #[lua(name = "delete", infallible)]
    fn lua_delete(&self, url: String) -> LuaRequestBuilder {
        self.0.delete(url).into()
    }

    #[lua(name = "head", infallible)]
    fn lua_head(&self, url: String) -> LuaRequestBuilder {
        self.0.head(url).into()
    }

    #[lua(name = "request")]
    fn lua_request(&self, method: &[u8], url: String) -> mlua::Result<LuaRequestBuilder> {
        reqwest::Method::from_bytes(method)
            .map_err(mlua::Error::external)
            .map(|m| self.0.request(m, url).into())
    }

    #[lua(name = "execute")]
    async fn lua_execute(&self, request: LuaRequest) -> mlua::Result<LuaResponse> {
        self.0
            .execute(request.into())
            .await
            .map_err(mlua::Error::external)
            .map(|v| v.into())
    }
}

#[derive(mlua::UserData)]
pub(crate) struct LuaClientBuilder(reqwest::ClientBuilder);

impl From<reqwest::ClientBuilder> for LuaClientBuilder {
    fn from(value: reqwest::ClientBuilder) -> Self {
        Self(value)
    }
}

#[mlua::userdata_impl]
impl LuaClientBuilder {}
