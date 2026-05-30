// SPDX-License-Identifier: MIT

use mlua::prelude::{Lua, LuaError, LuaFunction, LuaValue};

use crate::LuaEnvironment;

/// Helper to get the lua type for minijinja wrapper userdata.
///
/// Returns `environment`, `state`, `none`, or any other regular lua type name.
pub(crate) fn minijinja_types(val: &LuaValue) -> Result<&'static str, LuaError> {
    match val {
        LuaValue::UserData(ud) if ud.is::<LuaEnvironment>() => Ok("environment"),
        LuaValue::UserData(ud) if ud.type_name()? == Some("state".to_string()) => Ok("state"),
        val if val.is_null() => Ok("none"),
        _ => Ok(val.type_name()),
    }
}

/// Helper to load templates from a directory.
///
/// The returned function can be provided to `Environment:set_loader`
pub(crate) fn minijinja_path_loader(lua: &Lua) -> Result<LuaFunction, LuaError> {
    lua.load(
        r#"
        local function path_loader(paths)
            if type(paths) == "string" then
                paths = { paths }
            end

            local function loader(name)
                if name:match("\\") then return nil end

                name = name:gsub("^/+", ""):gsub("/+$", "")

                local sep = package.config:sub(1,1)
                local pattern = "([^" .. sep .. "]*)"

                local splits = {}
                for piece in name:gmatch(pattern) do
                    if ".." == piece then return nil end
                    table.insert(splits, piece)
                end

                for _, path in ipairs(paths) do
                    local p = path .. sep .. table.concat(splits, sep)
                    local file = io.open(p, "r")

                    if file then
                        local source = file:read("a")
                        file:close()

                        return source
                    end
                end
            end

            return loader
        end

        return path_loader
    "#,
    )
    .eval()
}

/// Filters to work with JSON strings and objects.
#[cfg(feature = "json")]
pub mod json {
    use minijinja::{Error as JinjaError, ErrorKind as JinjaErrorKind, State, Value as JinjaValue};

    use crate::convert::err_to_minijinja_err;

    /// Add the filters to the environment
    pub fn add_to_environment(env: &mut minijinja::Environment) {
        env.add_filter("fromjson", fromjson);
    }

    /// This filter allows loading minijinja objects from a JSON string.
    ///
    /// In lua, this allows loading a JSON object while preserving key order.
    pub fn fromjson(_: &State, json: &[u8]) -> Result<JinjaValue, JinjaError> {
        serde_json::from_slice(json)
            .map_err(|err| err_to_minijinja_err(err, JinjaErrorKind::BadSerialization))
    }
}

/// Filters to format date and time strings.
#[cfg(feature = "datetime")]
pub mod datetime {
    use jiff::civil::{Date, Time};
    use minijinja::{
        Error as JinjaError,
        ErrorKind as JinjaErrorKind,
        State,
        Value as JinjaValue,
        value::Kwargs,
    };

    use crate::convert::err_to_minijinja_err;

    /// Add the filters to the environment
    pub fn add_to_environment(env: &mut minijinja::Environment) {
        env.add_filter("datefmt", datefmt);
        env.add_filter("timefmt", timefmt);
    }

    /// Formats a string into a date using the [`jiff`] crate.
    ///
    /// If the `format` keyword is provided, the date will be formatted according to the `strftime`
    /// format. Otherwise, the value from [`date.to_string`](jiff::civil::Date) is returned.
    ///
    /// If the `patterns` keyword is provided, it must be a list of `strptime` format strings to
    /// parse the input. Multiple patterns can be provided to allow support for various date
    /// formats. If no patterns are provided or matched, then the default [`jiff`] formatting is
    /// used by calling `.parse()`
    ///
    /// See here for available formatting patterns: <https://docs.rs/jiff/latest/jiff/fmt/strtime/index.html>
    pub fn datefmt(_: &State, value: JinjaValue, kwargs: Kwargs) -> Result<String, JinjaError> {
        let format = kwargs.get::<Option<&str>>("format")?;
        let patterns = kwargs.get::<Option<Vec<String>>>("patterns")?;
        kwargs.assert_all_used()?;

        let date = match value.as_str() {
            Some(s) => {
                // Try the provided patterns
                if let Some(date) = patterns
                    .iter()
                    .flatten()
                    .find_map(|f| Date::strptime(f, s).ok())
                {
                    Ok(date)
                } else {
                    // Or fallback to the `jiff` parser
                    s.parse::<Date>()
                        .map_err(|err| err_to_minijinja_err(err, JinjaErrorKind::CannotDeserialize))
                }
            },
            None => Err(JinjaError::new(
                JinjaErrorKind::CannotDeserialize,
                "could not parse value as a string",
            )),
        }?;

        Ok(match format {
            Some(f) => date.strftime(f).to_string(),
            None => date.to_string(),
        })
    }

    /// Formats a string into a time using the [`jiff`] crate.
    ///
    /// If `format` is provided, the time will be formatted according to the `strftime` format.
    /// Otherwise, the value from [`time.to_string()`](jiff::civil::Time) is returned.
    ///
    /// If `patterns` is provided, it must be a list of `strptime` format strings to parse the
    /// input. Multiple patterns can be provided to allow support for various date formats. If no
    /// patterns are provided or matched, then the default [`jiff`] formatting is used by calling
    /// `.parse()`
    ///
    /// See here for available formatting patterns: <https://docs.rs/jiff/latest/jiff/fmt/strtime/index.html>
    pub fn timefmt(_: &State, value: JinjaValue, kwargs: Kwargs) -> Result<String, JinjaError> {
        let format = kwargs.get::<Option<&str>>("format")?;
        let patterns = kwargs.get::<Option<Vec<String>>>("patterns")?;
        kwargs.assert_all_used()?;

        let time = match value.as_str() {
            Some(s) => {
                // Try the provided patterns
                if let Some(date) = patterns
                    .iter()
                    .flatten()
                    .find_map(|f| Time::strptime(f, s).ok())
                {
                    Ok(date)
                } else {
                    // Or fallback to the `jiff` parser
                    s.parse::<Time>()
                        .map_err(|err| err_to_minijinja_err(err, JinjaErrorKind::CannotDeserialize))
                }
            },
            None => Err(JinjaError::new(
                JinjaErrorKind::CannotDeserialize,
                "could not parse value as a string",
            )),
        }?;

        Ok(match format {
            Some(f) => time.strftime(f).to_string(),
            None => time.to_string(),
        })
    }
}

#[cfg(test)]
mod test {
    use minijinja::context;
    use mlua::Lua;
    use serde_json::json;

    use super::*;
    use crate::state::LuaState;

    fn setup() -> Lua {
        Lua::new()
    }

    #[test]
    fn test_minijinja_types_environment() {
        let lua = setup();
        let env = lua.create_userdata(LuaEnvironment::new()).unwrap();

        assert_eq!(
            minijinja_types(&LuaValue::UserData(env)).unwrap(),
            "environment"
        );
    }

    #[test]
    fn test_minijinja_types_state() {
        let lua = setup();
        let env = minijinja::Environment::new();
        let state = env.empty_state();

        lua.scope(|scope| {
            let ud = scope.create_userdata(LuaState::new(&state)).unwrap();
            assert_eq!(minijinja_types(&LuaValue::UserData(ud)).unwrap(), "state");
            Ok(())
        })
        .unwrap();
    }

    #[test]
    fn test_minijinja_types_none() {
        assert_eq!(minijinja_types(&LuaValue::NULL).unwrap(), "none");
    }

    #[test]
    fn test_minijinja_types_lua() {
        let lua = setup();

        assert_eq!(minijinja_types(&LuaValue::Nil).unwrap(), "nil");
        assert_eq!(
            minijinja_types(&LuaValue::Boolean(true)).unwrap(),
            "boolean"
        );
        assert_eq!(
            minijinja_types(&LuaValue::Function(
                lua.create_function(|_, _: LuaValue| Ok(())).unwrap()
            ))
            .unwrap(),
            "function"
        );
        assert_eq!(minijinja_types(&LuaValue::Integer(99)).unwrap(), "integer");
        assert_eq!(minijinja_types(&LuaValue::Number(99.99)).unwrap(), "number");
        assert_eq!(
            minijinja_types(&LuaValue::String(lua.create_string("foo").unwrap())).unwrap(),
            "string"
        );
        assert_eq!(
            minijinja_types(&LuaValue::Table(lua.create_table().unwrap())).unwrap(),
            "table"
        );
        assert_eq!(
            minijinja_types(&LuaValue::Thread(
                lua.create_thread(lua.create_function(|_, _: LuaValue| Ok(())).unwrap())
                    .unwrap()
            ))
            .unwrap(),
            "thread"
        );
    }

    #[test]
    #[cfg(feature = "json")]
    fn test_minijinja_from_json_filter() {
        let mut env = minijinja::Environment::new();
        json::add_to_environment(&mut env);

        let ex = json!({"1": 1, "2": 2, "three": [1,2,3]});
        let expr = env.compile_expression("te | fromjson").unwrap();

        let res = expr.eval(context! { te => ex.to_string() }).unwrap();

        assert_eq!(res, minijinja::Value::from_serialize(ex));
    }

    #[test]
    #[cfg(feature = "datetime")]
    fn test_minijinja_datefmt_filter() {
        let mut env = minijinja::Environment::new();
        datetime::add_to_environment(&mut env);

        let date = "2000-01-01";
        let ex = "2000-01-01";

        let expr = env.compile_expression("te | datefmt").unwrap();
        let res = expr.eval(context! { te => date }).unwrap();

        assert_eq!(res.as_str().unwrap(), ex, "{} should parse to {}", date, ex);
    }

    #[test]
    #[cfg(feature = "datetime")]
    fn test_minijinja_datefmt_filter_format() {
        let mut env = minijinja::Environment::new();
        datetime::add_to_environment(&mut env);

        let date: &str = "2000-01-01T11:12:13";
        let ex = "January 1, 2000";
        let fmt = "%B %-d, %Y";

        let te = format!("te | datefmt(format='{}')", fmt);
        let expr = env.compile_expression(&te).unwrap();

        let res = expr.eval(context! { te => date }).unwrap();

        assert_eq!(res.as_str().unwrap(), ex, "{} should parse to {}", date, ex);
    }

    #[test]
    #[cfg(feature = "datetime")]
    fn test_minijinja_datefmt_filter_parse() {
        let mut env = minijinja::Environment::new();
        datetime::add_to_environment(&mut env);

        let date = "2026 1 January";
        let ex = "2026-01-01";
        let patt = "%Y %-d %B";

        let te = format!("te | datefmt(patterns=['{}'])", patt);
        let expr = env.compile_expression(&te).unwrap();

        let res = expr.eval(context! { te => date }).unwrap();

        assert_eq!(res.as_str().unwrap(), ex, "{} should parse to {}", date, ex);
    }

    #[test]
    #[cfg(feature = "datetime")]
    fn test_minijinja_timefmt_filter() {
        let mut env = minijinja::Environment::new();
        datetime::add_to_environment(&mut env);

        let time = "2000-01-01T11:12:13";
        let ex = "11:12:13";

        let expr = env.compile_expression("te | timefmt").unwrap();
        let res = expr.eval(context! { te => time }).unwrap();

        assert_eq!(res.as_str().unwrap(), ex, "{} should parse to {}", time, ex);
    }

    #[test]
    #[cfg(feature = "datetime")]
    fn test_minijinja_timefmt_filter_format() {
        let mut env = minijinja::Environment::new();
        datetime::add_to_environment(&mut env);

        let time = "12:02:31";
        let ex = "31:02:12";
        let fmt = "%S:%M:%H";

        let te = format!("te | timefmt(format='{}')", fmt);
        let expr = env.compile_expression(&te).unwrap();

        let res = expr.eval(context! { te => time }).unwrap();

        assert_eq!(res.as_str().unwrap(), ex, "{} should parse to {}", time, ex);
    }

    #[test]
    #[cfg(feature = "datetime")]
    fn test_minijinja_timefmt_filter_parse() {
        let mut env = minijinja::Environment::new();
        datetime::add_to_environment(&mut env);

        let time = "04 02 09";
        let ex = "02:04:09";
        let patt = "%M %H %S";

        let te = format!("te | timefmt(patterns=['{}'])", patt);
        let expr = env.compile_expression(&te).unwrap();

        let res = expr.eval(context! { te => time }).unwrap();

        assert_eq!(res.as_str().unwrap(), ex, "{} should parse to {}", time, ex);
    }
}
