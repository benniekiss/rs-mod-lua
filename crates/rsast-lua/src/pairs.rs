use std::rc::Rc;

use mlua::LuaSerdeExt;

#[derive(Debug, Clone, PartialEq, mlua::UserData, mlua::FromLua, serde::Serialize)]
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
    pairs: Option<LuaPairs>,
}

#[mlua::userdata_impl]
impl LuaPair {
    #[lua(skip)]
    pub(crate) fn new(input: &Rc<String>, pair: pest::iterators::Pair<'_, &str>) -> Self {
        let span = pair.as_span();
        let rule = Rc::new(pair.as_rule().to_string());
        let node_tag = pair.as_node_tag().map(|s| Rc::new(s.to_string()));
        let (line, col) = pair.line_col();

        let inner = pair.into_inner();
        let mut pairs = None;
        if !inner.is_empty() {
            pairs = Some(inner.into())
        }

        Self {
            input: Rc::clone(input),
            start: span.start(),
            stop: span.end(),
            rule,
            node_tag,
            line,
            col,
            pairs,
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
    pub(crate) fn lua_pairs(&self) -> Option<LuaPairs> {
        self.pairs.clone()
    }

    #[lua(name = "dump")]
    pub(crate) fn lua_dump(&self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        let config = mlua::serde::SerializeOptions::new().serialize_none_to_null(false);
        lua.to_value_with(self, config)
    }
}

#[derive(Debug, Clone, PartialEq, mlua::UserData, mlua::FromLua, serde::Serialize)]
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

impl From<pest::iterators::Pairs<'_, &str>> for LuaPairs {
    fn from(value: pest::iterators::Pairs<'_, &str>) -> Self {
        let input = Rc::new(value.get_input().to_string());
        let pairs = value.map(|p| LuaPair::new(&input, p)).collect::<Vec<_>>();
        Self::new(&input, pairs)
    }
}

impl LuaPairs {
    fn peek(&self) -> Option<LuaPair> {
        if self.idx >= self.rdx {
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

        self.rdx -= 1;
        self.rem -= 1;

        Some(self.pairs[self.rdx].clone())
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
    pub(crate) fn new(input: &Rc<String>, pairs: Vec<LuaPair>) -> Self {
        let idx = 0;
        let rdx = pairs.len();
        let rem = pairs.len();

        let mut start = 0;
        let mut stop = 0;

        if !pairs.is_empty() {
            start = pairs[0].start;
            stop = pairs[rdx - 1].stop;
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

    #[lua(skip)]
    fn flatten_into(&self) -> LuaPairs {
        let mut out = Vec::with_capacity(self.pairs.len());
        let mut stack = vec![self.clone()];

        while let Some(mut iter) = stack.pop() {
            if let Some(pair) = iter.next() {
                if !iter.len() != 0 {
                    stack.push(iter);
                }

                out.push(pair.clone());

                if let Some(p) = pair.pairs {
                    stack.push(p.clone());
                }
            }
        }

        Self::new(&self.input, out)
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
        self.len() == 0
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

    #[lua(name = "flatten", infallible)]
    pub(crate) fn lua_flatten(&self) -> LuaPairs {
        self.flatten_into()
    }

    #[lua(name = "dump")]
    pub(crate) fn lua_dump(&self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        let config = mlua::serde::SerializeOptions::new().serialize_none_to_null(false);
        lua.to_value_with(self, config)
    }
}

#[cfg(test)]
#[allow(clippy::while_let_on_iterator)]
mod tests {

    use super::*;

    const GRAMMAR: &str = r#"
    field = { (ASCII_DIGIT | "." | "-")+ }
    record = { #tag = field ~ ("," ~ field)* }
    "#;

    const INPUT: &str = "65279,1179403647,1463895090";

    fn setup() -> pest_vm::Vm {
        let (_, rules) = pest_meta::parse_and_optimize(GRAMMAR).unwrap();
        pest_vm::Vm::new(rules)
    }

    #[test]
    fn test_iteration() {
        let vm = setup();

        let input = Rc::new(INPUT.to_string());
        let pairs = vm.parse("record", INPUT).unwrap();
        let lua_pairs: LuaPairs = pairs.clone().into();

        let mapped_pairs = pairs.map(|p| LuaPair::new(&input, p)).collect::<Vec<_>>();

        assert_eq!(lua_pairs.collect::<Vec<_>>(), mapped_pairs)
    }

    #[test]
    fn test_next() {
        let vm = setup();

        let input = Rc::new(INPUT.to_string());
        let mut pairs = vm.parse("record", INPUT).unwrap();
        let mut lua_pairs: LuaPairs = pairs.clone().into();

        let mut pest_vec = vec![];
        while let Some(p) = pairs.next() {
            pest_vec.push(LuaPair::new(&input, p))
        }

        let mut lua_vec = vec![];
        while let Some(p) = lua_pairs.next() {
            lua_vec.push(p)
        }

        assert_eq!(lua_vec, pest_vec)
    }

    #[test]
    fn test_next_back() {
        let vm = setup();

        let input = Rc::new(INPUT.to_string());
        let mut pairs = vm.parse("record", INPUT).unwrap();
        let mut lua_pairs: LuaPairs = pairs.clone().into();

        let mut pest_vec = vec![];
        while let Some(p) = pairs.next_back() {
            pest_vec.push(LuaPair::new(&input, p))
        }

        let mut lua_vec = vec![];
        while let Some(p) = lua_pairs.next_back() {
            lua_vec.push(p)
        }

        assert_eq!(lua_vec, pest_vec)
    }

    #[test]
    fn test_flat_iteration() {
        let vm = setup();

        let input = Rc::new(INPUT.to_string());
        let pairs = vm.parse("record", INPUT).unwrap();
        let lua_pairs: LuaPairs = pairs.clone().into();

        let mapped_pairs = pairs
            .flatten()
            .map(|p| LuaPair::new(&input, p))
            .collect::<Vec<_>>();

        assert_eq!(lua_pairs.flatten_into().collect::<Vec<_>>(), mapped_pairs)
    }

    #[test]
    fn test_flat_next() {
        let vm = setup();

        let input = Rc::new(INPUT.to_string());
        let pairs = vm.parse("record", INPUT).unwrap();
        let lua_pairs: LuaPairs = pairs.clone().into();

        let mut flat_pairs = pairs.flatten();
        let mut pest_vec = vec![];
        while let Some(p) = flat_pairs.next() {
            pest_vec.push(LuaPair::new(&input, p))
        }

        let mut flat_lua_pairs = lua_pairs.flatten_into();
        let mut lua_vec = vec![];
        while let Some(p) = flat_lua_pairs.next() {
            lua_vec.push(p)
        }

        assert_eq!(lua_vec, pest_vec)
    }

    #[test]
    fn test_flat_next_back() {
        let vm = setup();

        let input = Rc::new(INPUT.to_string());
        let pairs = vm.parse("record", INPUT).unwrap();
        let lua_pairs: LuaPairs = pairs.clone().into();

        let mut flat_pairs = pairs.flatten();
        let mut pest_vec = vec![];
        while let Some(p) = flat_pairs.next_back() {
            pest_vec.push(LuaPair::new(&input, p))
        }

        let mut flat_lua_pairs = lua_pairs.flatten_into();
        let mut lua_vec = vec![];
        while let Some(p) = flat_lua_pairs.next_back() {
            lua_vec.push(p)
        }

        assert_eq!(lua_vec, pest_vec)
    }
}
