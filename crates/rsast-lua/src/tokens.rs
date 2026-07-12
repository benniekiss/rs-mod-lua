use std::iter::Peekable;

use mlua::LuaSerdeExt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub(crate) enum LuaToken {
    Start { rule: String, pos: usize },
    End { rule: String, pos: usize },
}

impl From<pest::Token<'_, &str>> for LuaToken {
    fn from(value: pest::Token<'_, &str>) -> Self {
        match value {
            pest::Token::Start { rule, pos } => LuaToken::Start {
                rule: rule.to_string(),
                pos: pos.pos(),
            },
            pest::Token::End { rule, pos } => LuaToken::End {
                rule: rule.to_string(),
                pos: pos.pos(),
            },
        }
    }
}

impl From<&pest::Token<'_, &str>> for LuaToken {
    fn from(value: &pest::Token<'_, &str>) -> Self {
        match value {
            pest::Token::Start { rule, pos } => LuaToken::Start {
                rule: rule.to_string(),
                pos: pos.pos(),
            },
            pest::Token::End { rule, pos } => LuaToken::End {
                rule: rule.to_string(),
                pos: pos.pos(),
            },
        }
    }
}

impl mlua::IntoLua for LuaToken {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        lua.to_value(&self)
    }
}

impl mlua::FromLua for LuaToken {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        lua.from_value(value)
    }
}

pub(crate) struct LuaTokens<'scope>(Peekable<pest::iterators::Tokens<'scope, &'scope str>>);

impl<'scope> From<pest::iterators::Tokens<'scope, &'scope str>> for LuaTokens<'scope> {
    fn from(value: pest::iterators::Tokens<'scope, &'scope str>) -> Self {
        Self(value.peekable())
    }
}

impl<'scope> mlua::UserData for LuaTokens<'scope> {
    fn add_methods<M: mlua::prelude::LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("peek", |_, this, ()| Ok(this.0.peek().map(LuaToken::from)));

        methods.add_method_mut("next", |_, this, ()| Ok(this.0.next().map(LuaToken::from)));

        methods.add_method_mut("next_back", |_, this, ()| {
            Ok(this.0.next_back().map(LuaToken::from))
        });
    }
}
