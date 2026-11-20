use std::time::Duration;

use crate::{
    client::LuaClient,
    headers::LuaHeaders,
    multipart::LuaForm,
    response::LuaResponse,
    version::LuaVersion,
};

#[derive(mlua::UserData)]
pub(crate) struct LuaRequest(reqwest::Request);

impl From<reqwest::Request> for LuaRequest {
    fn from(value: reqwest::Request) -> Self {
        Self(value)
    }
}

impl From<LuaRequest> for reqwest::Request {
    fn from(value: LuaRequest) -> Self {
        value.0
    }
}

impl mlua::FromLua for LuaRequest {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        match value.as_userdata() {
            Some(ud) => ud.take::<LuaRequest>(),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "Request".to_string(),
                message: None,
            }),
        }
    }
}

#[mlua::userdata_impl]
impl LuaRequest {
    #[lua(name = "method", infallible)]
    fn lua_method(&self) -> String {
        self.0.method().as_str().to_string()
    }

    #[lua(name = "url", infallible)]

    fn lua_url(&self) {}

    #[lua(name = "headers", infallible)]

    fn lua_headers(&self) {}

    #[lua(name = "body")]

    fn lua_body(&self, lua: &mlua::Lua) -> mlua::Result<Option<mlua::LuaString>> {
        if let Some(body) = self.0.body()
            && let Some(b) = body.as_bytes()
        {
            lua.create_string(b).map(Some)
        } else {
            Ok(None)
        }
    }

    #[lua(name = "timeout", infallible)]
    fn lua_timeout(&self) -> Option<u64> {
        self.0.timeout().map(|d| d.as_secs())
    }

    #[lua(name = "version", infallible)]

    fn lua_version(&self) {}
}

#[derive(mlua::UserData)]
pub(crate) struct LuaRequestBuilder(reqwest::RequestBuilder);

impl From<reqwest::RequestBuilder> for LuaRequestBuilder {
    fn from(value: reqwest::RequestBuilder) -> Self {
        Self(value)
    }
}

#[mlua::userdata_impl]
impl LuaRequestBuilder {
    #[lua(name = "from_parts", infallible)]
    fn lua_from_parts(client: LuaClient, request: LuaRequest) -> Self {
        reqwest::RequestBuilder::from_parts(client.into(), request.into()).into()
    }

    #[lua(name = "build")]
    fn lua_build(self) -> mlua::Result<LuaRequest> {
        self.0
            .build()
            .map_err(mlua::Error::external)
            .map(LuaRequest::from)
    }

    #[lua(name = "build_split")]
    fn lua_build_split(self) -> mlua::Result<(LuaClient, LuaRequest)> {
        let (client, response) = self.0.build_split();

        let res = response.map_err(mlua::Error::external)?;

        Ok((client.into(), res.into()))
    }

    #[lua(name = "send")]
    async fn lua_send(self) -> mlua::Result<LuaResponse> {
        self.0
            .send()
            .await
            .map_err(mlua::Error::external)
            .map(LuaResponse::from)
    }

    #[must_use]
    #[lua(name = "header", infallible)]
    fn lua_header(self, key: &[u8], value: &[u8]) -> Self {
        self.0.header(key, value).into()
    }

    #[must_use]
    #[lua(name = "headers", infallible)]
    fn lua_headers(self, headers: LuaHeaders) -> Self {
        self.0.headers(headers.into()).into()
    }

    #[must_use]
    #[lua(name = "basic_auth", infallible)]
    fn lua_basic_auth(self, username: &str, password: Option<&str>) -> Self {
        self.0.basic_auth(username, password).into()
    }

    #[must_use]
    #[lua(name = "bearer_auth", infallible)]
    fn lua_bearer_auth(self, token: &str) -> Self {
        self.0.bearer_auth(token).into()
    }

    #[must_use]
    #[lua(name = "body", infallible)]
    fn lua_body(self, body: &[u8]) -> Self {
        self.0.body(body.to_vec()).into()
    }

    #[must_use]
    #[lua(name = "timeout", infallible)]
    fn lua_timeout(self, timeout: u64) -> Self {
        self.0.timeout(Duration::from_secs(timeout)).into()
    }

    #[must_use]
    #[lua(name = "multipart", infallible)]
    fn lua_multipart(self, multipart: LuaForm) -> Self {
        self.0.multipart(multipart.into()).into()
    }

    #[must_use]
    #[lua(name = "query", infallible)]
    fn lua_query(self, query: mlua::Value) -> Self {
        self.0.query(&query.to_serializable()).into()
    }

    #[must_use]
    #[lua(name = "version", infallible)]
    fn lua_version(self, version: LuaVersion) -> Self {
        self.0.version(version.into()).into()
    }

    #[must_use]
    #[lua(name = "form", infallible)]
    fn lua_form(self, form: mlua::Value) -> Self {
        self.0.form(&form.to_serializable()).into()
    }

    #[must_use]
    #[lua(name = "json", infallible)]
    fn lua_json(self, json: mlua::Value) -> Self {
        self.0.json(&json.to_serializable()).into()
    }
}
