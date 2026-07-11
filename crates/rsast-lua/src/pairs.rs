use mlua::LuaSerdeExt;

use crate::tokens::LuaTokens;

pub(crate) struct LuaPair<'scope>(pest::iterators::Pair<'scope, &'scope str>);

impl<'scope> From<pest::iterators::Pair<'scope, &'scope str>> for LuaPair<'scope> {
    fn from(value: pest::iterators::Pair<'scope, &'scope str>) -> Self {
        Self(value)
    }
}

impl<'scope> mlua::UserData for LuaPair<'scope> {
    fn add_methods<M: mlua::prelude::LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("start", |_, this, ()| Ok(this.0.as_span().start()));
        methods.add_method("end", |_, this, ()| Ok(this.0.as_span().end()));
        methods.add_method("as_rule", |_, this, ()| Ok(this.0.as_rule().to_string()));
        methods.add_method("as_str", |_, this, ()| Ok(this.0.as_str().to_string()));
        methods.add_method("as_node_tag", |_, this, ()| {
            Ok(this.0.as_node_tag().map(|s| s.to_string()))
        });
        methods.add_method("get_input", |_, this, ()| {
            Ok(this.0.get_input().to_string())
        });
        methods.add_method("line_col", |_, this, ()| Ok(this.0.line_col()));
        methods.add_method(
            "tokens",
            |lua, this, callback: mlua::Function| -> mlua::Result<mlua::MultiValue> {
                lua.scope(|scope| {
                    let tokens = this.0.clone().tokens();
                    let ud = scope.create_userdata::<LuaTokens>(tokens.into())?;
                    callback.call(ud)
                })
            },
        );
        methods.add_method(
            "pairs",
            |lua, this, callback: mlua::Function| -> mlua::Result<mlua::MultiValue> {
                lua.scope(|scope| {
                    let inner = this.0.clone().into_inner();
                    let ud = scope.create_userdata::<LuaPairs>(inner.into())?;
                    callback.call(ud)
                })
            },
        );
        methods.add_method("dump", |lua, this, ()| lua.to_value(&this.0));
    }
}

pub(crate) struct LuaPairs<'scope>(pest::iterators::Pairs<'scope, &'scope str>);

impl<'scope> From<pest::iterators::Pairs<'scope, &'scope str>> for LuaPairs<'scope> {
    fn from(value: pest::iterators::Pairs<'scope, &'scope str>) -> Self {
        Self(value)
    }
}

impl<'scope> From<LuaPairs<'scope>> for pest::iterators::Pairs<'scope, &'scope str> {
    fn from(value: LuaPairs<'scope>) -> Self {
        value.0
    }
}

impl<'scope> mlua::UserData for LuaPairs<'scope> {
    fn add_methods<M: mlua::prelude::LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("as_str", |_, this, ()| Ok(this.0.as_str().to_string()));
        methods.add_method("get_input", |_, this, ()| {
            Ok(this.0.get_input().to_string())
        });
        methods.add_method("concat", |_, this, ()| Ok(this.0.concat().to_string()));
        methods.add_method("is_empty", |_, this, ()| Ok(this.0.is_empty()));
        methods.add_method(
            "peek",
            |lua, this, callback: mlua::Function| -> mlua::Result<mlua::MultiValue> {
                match this.0.peek() {
                    Some(pair) => lua.scope(|scope| {
                        let ud = scope.create_userdata::<LuaPair>(pair.into());
                        callback.call(ud)
                    }),
                    None => Ok(mlua::MultiValue::new()),
                }
            },
        );
        methods.add_method(
            "tokens",
            |lua, this, callback: mlua::Function| -> mlua::Result<mlua::MultiValue> {
                lua.scope(|scope| {
                    let tokens = this.0.clone().tokens();
                    let ud = scope.create_userdata::<LuaTokens>(tokens.into())?;
                    callback.call(ud)
                })
            },
        );
        methods.add_method(
            "tokens_flat",
            |lua, this, callback: mlua::Function| -> mlua::Result<mlua::MultiValue> {
                lua.scope(|scope| {
                    let tokens = this.0.clone().flatten().tokens();
                    let ud = scope.create_userdata::<LuaTokens>(tokens.into())?;
                    callback.call(ud)
                })
            },
        );
        methods.add_method("for_each", |lua, this, callback: mlua::Function| {
            lua.scope(|scope| {
                for pair in this.0.clone() {
                    let ud = scope.create_userdata::<LuaPair>(pair.into())?;
                    if let Some(false) = callback.call::<Option<bool>>(ud)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            })
        });
        methods.add_method("for_each_flat", |lua, this, callback: mlua::Function| {
            lua.scope(|scope| {
                for pair in this.0.clone().flatten() {
                    let ud = scope.create_userdata::<LuaPair>(pair.into())?;
                    if let Some(false) = callback.call::<Option<bool>>(ud)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            })
        });
        methods.add_method("dump", |lua, this, ()| {
            lua.to_value(&this.0.clone().collect::<Vec<_>>())
        });
        methods.add_method("dump_flat", |lua, this, ()| {
            lua.to_value(&this.0.clone().flatten().collect::<Vec<_>>())
        });
    }
}
