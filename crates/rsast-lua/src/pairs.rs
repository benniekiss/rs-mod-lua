use std::rc::Rc;

use mlua::LuaSerdeExt;

#[derive(Clone, mlua::UserData, mlua::FromLua, serde::Serialize)]
pub(crate) struct LuaPair {
    #[serde(skip)]
    #[lua(skip)]
    input: Rc<String>,
    #[lua(skip)]
    start: usize,
    #[lua(skip)]
    stop: usize,
    #[lua(skip)]
    rule: Rc<String>,
    #[lua(skip)]
    node_tag: Option<Rc<String>>,
    #[serde(skip)]
    #[lua(skip)]
    line: usize,
    #[serde(skip)]
    #[lua(skip)]
    col: usize,
    #[lua(skip)]
    pairs: LuaPairs,
}

#[mlua::userdata_impl]
impl LuaPair {
    #[lua(skip)]
    pub(crate) fn new(input: &Rc<String>, pair: pest::iterators::Pair<'_, &str>) -> Self {
        let span = pair.as_span();
        let (line, col) = pair.line_col();

        Self {
            input: Rc::clone(input),
            start: span.start(),
            stop: span.end(),
            rule: Rc::new(pair.as_rule().to_string()),
            node_tag: pair.as_node_tag().map(|s| Rc::new(s.to_string())),
            line,
            col,
            pairs: LuaPairs::new(input, pair.into_inner()),
        }
    }

    #[lua(name = "start", infallible)]
    pub(crate) fn lua_start(&self) -> usize {
        self.start
    }

    #[lua(name = "stop", infallible)]
    pub(crate) fn lua_stop(&self) -> usize {
        self.stop
    }

    #[lua(name = "as_rule", infallible)]
    pub(crate) fn lua_as_rule(&self) -> String {
        self.rule.to_string()
    }

    #[lua(name = "as_str", infallible)]
    pub(crate) fn lua_as_str(&self) -> String {
        self.input[self.start..self.stop].to_string()
    }

    #[lua(name = "as_node_tag", infallible)]
    pub(crate) fn lua_as_node_tag(&self) -> Option<String> {
        self.node_tag.clone().map(|s| s.to_string())
    }

    #[lua(name = "get_input", infallible)]
    pub(crate) fn lua_get_input(&self) -> String {
        self.input.to_string()
    }

    #[lua(name = "line_col", infallible)]
    pub(crate) fn lua_line_col(&self) -> (usize, usize) {
        (self.line, self.col)
    }

    #[lua(name = "pairs", infallible)]
    pub(crate) fn lua_pairs(&self) -> LuaPairs {
        self.pairs.clone()
    }

    #[lua(name = "dump")]
    pub(crate) fn lua_dump(&self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        let config = mlua::serde::SerializeOptions::new().serialize_none_to_null(false);
        lua.to_value_with(self, config)
    }
}

#[derive(Clone, mlua::UserData, mlua::FromLua, serde::Serialize)]
pub(crate) struct LuaPairs {
    #[serde(skip)]
    #[lua(skip)]
    input: Rc<String>,
    #[lua(skip)]
    start: usize,
    #[lua(skip)]
    stop: usize,
    #[serde(skip)]
    #[lua(skip)]
    idx: usize,
    #[serde(skip)]
    #[lua(skip)]
    rdx: usize,
    #[serde(skip)]
    #[lua(skip)]
    rem: usize,
    #[lua(skip)]
    pairs: Rc<Vec<LuaPair>>,
}

impl LuaPairs {
    fn peek(&self) -> Option<LuaPair> {
        if self.idx > self.rdx {
            return None;
        }

        Some(self.pairs[self.idx].clone())
    }
}

impl Iterator for LuaPairs {
    type Item = LuaPair;

    fn next(&mut self) -> Option<Self::Item> {
        let pair = self.peek()?;

        self.idx += 1;
        self.rem -= 1;

        Some(pair)
    }
}

impl DoubleEndedIterator for LuaPairs {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.rdx <= self.idx {
            return None;
        }

        let pair = self.pairs[self.rdx].clone();

        self.rdx -= 1;
        self.rem -= 1;

        Some(pair)
    }
}

impl ExactSizeIterator for LuaPairs {
    fn len(&self) -> usize {
        self.rem
    }
}

#[mlua::userdata_impl]
impl LuaPairs {
    #[lua(skip)]
    pub(crate) fn new(input: &Rc<String>, pairs: pest::iterators::Pairs<'_, &str>) -> Self {
        let pairs = pairs.map(|p| LuaPair::new(input, p)).collect::<Vec<_>>();

        let idx = 0;
        let mut rdx = 0;

        let mut start = 0;
        let mut stop = 0;
        let rem = pairs.len();

        if !pairs.is_empty() {
            rdx = pairs.len() - 1;

            start = pairs[0].start;
            stop = pairs[rdx].stop;
        }

        Self {
            input: Rc::clone(input),
            start,
            stop,
            idx,
            rdx,
            rem,
            pairs: Rc::new(pairs),
        }
    }

    #[lua(name = "as_str", infallible)]
    pub(crate) fn lua_as_str(&self) -> String {
        self.input[self.start..self.stop].to_string()
    }

    #[lua(name = "get_input", infallible)]
    pub(crate) fn lua_get_input(&self) -> String {
        self.input.to_string()
    }

    #[lua(name = "is_empty", infallible)]
    pub(crate) fn lua_is_empty(&self) -> bool {
        self.pairs.is_empty()
    }

    #[lua(name = "peek", infallible)]
    pub(crate) fn lua_peek(&mut self) -> Option<LuaPair> {
        self.peek()
    }

    #[lua(name = "next", infallible)]
    pub(crate) fn lua_next(&mut self) -> Option<LuaPair> {
        self.next()
    }

    #[lua(name = "next_back", infallible)]
    pub(crate) fn lua_next_back(&mut self) -> Option<LuaPair> {
        self.next_back()
    }

    #[lua(name = "iter", infallible)]
    pub(crate) fn lua_iter(&self, lua: &mlua::Lua) -> mlua::Result<mlua::Function> {
        let mut iter = self.clone();

        lua.create_function_mut(move |_, ()| Ok(iter.next()))
    }

    #[lua(name = "reviter", infallible)]
    pub(crate) fn lua_reviter(&self, lua: &mlua::Lua) -> mlua::Result<mlua::Function> {
        let mut iter = self.clone();

        lua.create_function_mut(move |_, ()| Ok(iter.next_back()))
    }

    #[lua(name = "dump")]
    pub(crate) fn lua_dump(&self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        let config = mlua::serde::SerializeOptions::new().serialize_none_to_null(false);
        lua.to_value_with(self, config)
    }
}
