use std::ffi::OsString;

use crate::headers::LuaHeaders;

#[derive(mlua::UserData)]
pub(crate) struct LuaForm(reqwest::multipart::Form);

impl From<reqwest::multipart::Form> for LuaForm {
    fn from(value: reqwest::multipart::Form) -> Self {
        Self(value)
    }
}

impl From<LuaForm> for reqwest::multipart::Form {
    fn from(value: LuaForm) -> Self {
        value.0
    }
}

impl mlua::FromLua for LuaForm {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        match value.as_userdata() {
            Some(ud) => ud.take::<LuaForm>(),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "Part".to_string(),
                message: None,
            }),
        }
    }
}

#[mlua::userdata_impl]
impl LuaForm {
    #[lua(name = "new", infallible)]
    fn lua_new() -> Self {
        reqwest::multipart::Form::new().into()
    }

    #[lua(name = "boundary", infallible)]
    fn lua_boundary(&self) -> String {
        self.0.boundary().to_string()
    }

    #[lua(name = "text", infallible)]
    fn lua_text(self, name: String, value: String) -> Self {
        self.0.text(name, value).into()
    }

    #[lua(name = "file")]
    async fn lua_file(self, name: String, path: OsString) -> mlua::Result<Self> {
        self.0
            .file(name, path)
            .await
            .map_err(mlua::Error::external)
            .map(Self::from)
    }

    #[lua(name = "part", infallible)]
    fn lua_part(self, name: String, part: LuaPart) -> Self {
        self.0.part(name, part.into()).into()
    }

    #[lua(name = "percent_encode_path_segment", infallible)]
    fn lua_percent_encode_path_segment(self) -> Self {
        self.0.percent_encode_path_segment().into()
    }

    #[lua(name = "percent_encode_attr_chars", infallible)]
    fn lua_percent_encode_attr_chars(self) -> Self {
        self.0.percent_encode_attr_chars().into()
    }

    #[lua(name = "percent_encode_noop", infallible)]
    fn lua_percent_encode_noop(self) -> Self {
        self.0.percent_encode_noop().into()
    }
}

#[derive(mlua::UserData)]
pub(crate) struct LuaPart(reqwest::multipart::Part);

impl From<reqwest::multipart::Part> for LuaPart {
    fn from(value: reqwest::multipart::Part) -> Self {
        Self(value)
    }
}

impl From<LuaPart> for reqwest::multipart::Part {
    fn from(value: LuaPart) -> Self {
        value.0
    }
}

impl mlua::FromLua for LuaPart {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        match value.as_userdata() {
            Some(ud) => ud.take::<LuaPart>(),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "Part".to_string(),
                message: None,
            }),
        }
    }
}

#[mlua::userdata_impl]
impl LuaPart {
    #[lua(name = "text", infallible)]
    fn lua_text(text: String) -> Self {
        reqwest::multipart::Part::text(text).into()
    }

    #[lua(name = "bytes", infallible)]
    fn lua_bytes(bytes: &[u8]) -> Self {
        reqwest::multipart::Part::bytes(bytes.to_vec()).into()
    }

    #[lua(name = "file")]
    async fn lua_file(file: OsString) -> mlua::Result<Self> {
        reqwest::multipart::Part::file(file)
            .await
            .map_err(mlua::Error::external)
            .map(Self::from)
    }

    #[lua(name = "mime_str")]
    fn lua_mime_str(self, mime: &str) -> mlua::Result<Self> {
        self.0
            .mime_str(mime)
            .map_err(mlua::Error::external)
            .map(Self::from)
    }

    #[lua(name = "file_name", infallible)]
    fn lua_file_name(self, filename: String) -> Self {
        self.0.file_name(filename).into()
    }

    #[lua(name = "headers", infallible)]
    fn lua_headers(self, headers: LuaHeaders) -> Self {
        self.0.headers(headers.into()).into()
    }
}
