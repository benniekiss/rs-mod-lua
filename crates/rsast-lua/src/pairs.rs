use std::ops::ControlFlow;

use mlua::{IntoLua, LuaSerdeExt};

use crate::tokens::{LuaToken, LuaTokens};

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
            "pairs",
            |lua, this, callback: Option<mlua::Function>| -> mlua::Result<mlua::MultiValue> {
                let pairs = this.0.clone().into_inner();

                match callback {
                    Some(f) => lua.scope(|scope| {
                        let ud = scope.create_userdata::<LuaPairs>(pairs.into())?;
                        f.call(ud)
                    }),
                    None => lua
                        .to_value(&pairs)
                        .map(|v| mlua::MultiValue::from_vec(vec![v])),
                }
            },
        );
        methods.add_method(
            "tokens",
            |lua, this, callback: Option<mlua::Function>| -> mlua::Result<mlua::MultiValue> {
                let tokens = this.0.clone().tokens();
                match callback {
                    Some(f) => lua.scope(|scope| {
                        let ud = scope.create_userdata::<LuaTokens>(tokens.into())?;
                        f.call(ud)
                    }),
                    None => lua
                        .to_value(&tokens.map(LuaToken::from).collect::<Vec<_>>())
                        .map(|v| mlua::MultiValue::from_vec(vec![v])),
                }
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

impl<'scope> mlua::UserData for LuaPairs<'scope> {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("as_str", |_, this, ()| Ok(this.0.as_str().to_string()));
        methods.add_method("get_input", |_, this, ()| {
            Ok(this.0.get_input().to_string())
        });
        methods.add_method("concat", |_, this, ()| Ok(this.0.concat().to_string()));
        methods.add_method("is_empty", |_, this, ()| Ok(this.0.is_empty()));
        methods.add_method(
            "peek",
            |lua, this, callback: Option<mlua::Function>| -> mlua::Result<mlua::MultiValue> {
                match this.0.peek() {
                    Some(pair) if let Some(f) = callback => lua.scope(|scope| {
                        let ud = scope.create_userdata::<LuaPair>(pair.into());
                        f.call(ud)
                    }),
                    Some(pair) => lua
                        .to_value(&pair)
                        .map(|v| mlua::MultiValue::from_vec(vec![v])),
                    None => Ok(mlua::MultiValue::new()),
                }
            },
        );
        methods.add_method_mut(
            "next",
            |lua, this, callback: Option<mlua::Function>| -> mlua::Result<mlua::MultiValue> {
                match this.0.next() {
                    Some(pair) if let Some(f) = callback => lua.scope(|scope| {
                        let ud = scope.create_userdata::<LuaPair>(pair.into());
                        f.call(ud)
                    }),
                    Some(pair) => lua
                        .to_value(&pair)
                        .map(|v| mlua::MultiValue::from_vec(vec![v])),
                    None => Ok(mlua::MultiValue::new()),
                }
            },
        );
        methods.add_method_mut(
            "next_back",
            |lua, this, callback: Option<mlua::Function>| -> mlua::Result<mlua::MultiValue> {
                match this.0.next_back() {
                    Some(pair) if let Some(f) = callback => lua.scope(|scope| {
                        let ud = scope.create_userdata::<LuaPair>(pair.into());
                        f.call(ud)
                    }),
                    Some(pair) => lua
                        .to_value(&pair)
                        .map(|v| mlua::MultiValue::from_vec(vec![v])),
                    None => Ok(mlua::MultiValue::new()),
                }
            },
        );
        methods.add_method_mut(
            "fold",
            |lua, this, (init, callback): (mlua::Value, mlua::Function)| {
                lua.scope(|scope| {
                    match this.0.try_fold(init, |value, pair| {
                        scope
                            .create_userdata::<LuaPair>(pair.into())
                            .and_then(|ud| {
                                callback.call::<(mlua::Value, Option<bool>)>((value, ud))
                            })
                            .map(|(res, flow)| match flow {
                                Some(false) => ControlFlow::Break(res),
                                _ => ControlFlow::Continue(res),
                            })
                            .unwrap_or_else(|err| {
                                ControlFlow::Break(err.into_lua(lua).unwrap_or_default())
                            })
                    }) {
                        ControlFlow::Continue(val) | ControlFlow::Break(val) => Ok(val),
                    }
                })
            },
        );
        methods.add_method(
            "fold_flat",
            |lua, this, (init, callback): (mlua::Value, mlua::Function)| {
                lua.scope(|scope| {
                    match this.0.clone().flatten().try_fold(init, |value, pair| {
                        scope
                            .create_userdata::<LuaPair>(pair.into())
                            .and_then(|ud| {
                                callback.call::<(mlua::Value, Option<bool>)>((value, ud))
                            })
                            .map(|(res, flow)| match flow {
                                Some(false) => ControlFlow::Break(res),
                                _ => ControlFlow::Continue(res),
                            })
                            .unwrap_or_else(|err| {
                                ControlFlow::Break(err.into_lua(lua).unwrap_or_default())
                            })
                    }) {
                        ControlFlow::Continue(val) | ControlFlow::Break(val) => Ok(val),
                    }
                })
            },
        );
        methods.add_method_mut(
            "rfold",
            |lua, this, (init, callback): (mlua::Value, mlua::Function)| {
                lua.scope(|scope| {
                    match this.0.try_rfold(init, |value, pair| {
                        scope
                            .create_userdata::<LuaPair>(pair.into())
                            .and_then(|ud| {
                                callback.call::<(mlua::Value, Option<bool>)>((value, ud))
                            })
                            .map(|(res, flow)| match flow {
                                Some(false) => ControlFlow::Break(res),
                                _ => ControlFlow::Continue(res),
                            })
                            .unwrap_or_else(|err| {
                                ControlFlow::Break(err.into_lua(lua).unwrap_or_default())
                            })
                    }) {
                        ControlFlow::Continue(val) | ControlFlow::Break(val) => Ok(val),
                    }
                })
            },
        );
        methods.add_method(
            "rfold_flat",
            |lua, this, (init, callback): (mlua::Value, mlua::Function)| {
                lua.scope(|scope| {
                    match this.0.clone().flatten().try_rfold(init, |value, pair| {
                        scope
                            .create_userdata::<LuaPair>(pair.into())
                            .and_then(|ud| {
                                callback.call::<(mlua::Value, Option<bool>)>((value, ud))
                            })
                            .map(|(res, flow)| match flow {
                                Some(false) => ControlFlow::Break(res),
                                _ => ControlFlow::Continue(res),
                            })
                            .unwrap_or_else(|err| {
                                ControlFlow::Break(err.into_lua(lua).unwrap_or_default())
                            })
                    }) {
                        ControlFlow::Continue(val) | ControlFlow::Break(val) => Ok(val),
                    }
                })
            },
        );
        methods.add_method(
            "tokens",
            |lua, this, callback: Option<mlua::Function>| -> mlua::Result<mlua::MultiValue> {
                let tokens = this.0.clone().tokens();
                match callback {
                    Some(f) => lua.scope(|scope| {
                        let ud = scope.create_userdata::<LuaTokens>(tokens.into())?;
                        f.call(ud)
                    }),
                    None => lua
                        .to_value(&tokens.map(LuaToken::from).collect::<Vec<_>>())
                        .map(|v| mlua::MultiValue::from_vec(vec![v])),
                }
            },
        );
        methods.add_method(
            "tokens_flat",
            |lua, this, callback: Option<mlua::Function>| -> mlua::Result<mlua::MultiValue> {
                let tokens = this.0.clone().tokens();
                match callback {
                    Some(f) => lua.scope(|scope| {
                        let ud = scope.create_userdata::<LuaTokens>(tokens.into())?;
                        f.call(ud)
                    }),
                    None => lua
                        .to_value(&tokens.map(LuaToken::from).collect::<Vec<_>>())
                        .map(|v| mlua::MultiValue::from_vec(vec![v])),
                }
            },
        );
        methods.add_method("dump", |lua, this, ()| {
            lua.to_value(&this.0.clone().collect::<Vec<_>>())
        });
        methods.add_method("dump_flat", |lua, this, ()| {
            lua.to_value(&this.0.clone().flatten().collect::<Vec<_>>())
        });
    }
}
