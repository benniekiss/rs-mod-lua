use std::sync::Arc;

use mlua::IntoLua;
use pest_meta::parser::Rule;

use crate::pairs::LuaPairs;

#[derive(Clone, mlua::UserData, mlua::FromLua)]
pub(crate) struct LuaPestVm {
    #[lua(skip)]
    vm: Arc<pest_vm::Vm>,
    #[lua(skip)]
    err_handlers: Option<(mlua::Function, mlua::Function)>,
}

impl From<pest_vm::Vm> for LuaPestVm {
    fn from(value: pest_vm::Vm) -> Self {
        Self {
            vm: Arc::new(value),
            err_handlers: None,
        }
    }
}

#[mlua::userdata_impl]
impl LuaPestVm {
    #[lua(name = "new", infallible)]
    pub(crate) fn lua_new(grammar: &str) -> (Option<Self>, Option<Vec<String>>) {
        match pest_meta::parse_and_optimize(grammar) {
            Ok((_, rules)) => (Some(pest_vm::Vm::new(rules).into()), None),
            Err(err) => (
                None,
                Some(
                    err.into_iter()
                        .map(|e| e.renamed_rules(grammar_rule_name).to_string())
                        .collect::<Vec<_>>(),
                ),
            ),
        }
    }

    #[lua(name = "set_error_formatter", infallible)]
    pub(crate) fn lua_set_error_formatter(
        &mut self,
        rule_handler: mlua::Function,
        ws_handler: mlua::Function,
    ) {
        self.err_handlers = Some((rule_handler, ws_handler));
    }

    #[lua(skip)]
    fn handle_error(&self, input: &str, err: pest::error::Error<&str>) -> String {
        if let Some((rule_handler, ws_handler)) = self.err_handlers.clone() {
            let err_fn: pest::error::RuleToMessageFn<&str> =
                Box::new(move |rule| rule_handler.call(*rule).unwrap_or(None));
            let ws_fn: pest::error::IsWhitespaceFn =
                Box::new(move |text| ws_handler.call(text).unwrap_or(false));

            if let Some(e) = err.parse_attempts_error(input, &err_fn, &ws_fn) {
                return e.to_string();
            }
        }

        err.renamed_rules(|rule| (*rule).to_owned()).to_string()
    }

    #[lua(name = "validate")]
    pub(crate) fn lua_validate(
        &self,
        lua: &mlua::Lua,
        rule: &str,
        input: &str,
    ) -> mlua::Result<mlua::MultiValue> {
        let pairs = self.vm.parse(rule, input);
        let mut mv = mlua::MultiValue::with_capacity(2);
        match pairs {
            Ok(_) => {
                mv.push_back(mlua::Value::Boolean(true));
                mv.push_back(mlua::Nil);
            },
            Err(err) => {
                mv.push_back(mlua::Value::Boolean(false));
                mv.push_back(self.handle_error(input, err).into_lua(lua)?);
            },
        }

        Ok(mv)
    }

    #[lua(name = "parse")]
    pub(crate) fn lua_parse(
        &self,
        rule: &str,
        input: &str,
        callback: mlua::Function,
    ) -> mlua::Result<mlua::MultiValue> {
        let pairs: LuaPairs = self
            .vm
            .parse(rule, input)
            .map_err(|err| mlua::Error::external(self.handle_error(input, err)))?
            .into();

        callback.call(pairs)
    }
}

fn grammar_rule_name(rule: &pest_meta::parser::Rule) -> String {
    match *rule {
        Rule::EOI => "EOI",
        Rule::grammar_rules => "grammar_rules",
        Rule::grammar_rule => "grammar_rule",
        Rule::assignment_operator => "assignment_operator",
        Rule::opening_brace => "opening_brace",
        Rule::closing_brace => "closing_brace",
        Rule::opening_paren => "opening_paren",
        Rule::closing_paren => "closing_paren",
        Rule::opening_brack => "opening_brack",
        Rule::closing_brack => "closing_brack",
        Rule::modifier => "modifier",
        Rule::silent_modifier => "silent_modifier",
        Rule::atomic_modifier => "atomic_modifier",
        Rule::compound_atomic_modifier => "compound_atomic_modifier",
        Rule::non_atomic_modifier => "non_atomic_modifier",
        Rule::tag_id => "tag_id",
        Rule::node_tag => "node_tag",
        Rule::expression => "expression",
        Rule::term => "term",
        Rule::node => "node",
        Rule::terminal => "terminal",
        Rule::prefix_operator => "prefix_operator",
        Rule::infix_operator => "infix_operator",
        Rule::postfix_operator => "postfix_operator",
        Rule::positive_predicate_operator => "positive_predicate_operator",
        Rule::negative_predicate_operator => "negative_predicate_operator",
        Rule::sequence_operator => "sequence_operator",
        Rule::choice_operator => "choice_operator",
        Rule::optional_operator => "optional_operator",
        Rule::repeat_operator => "repeat_operator",
        Rule::repeat_once_operator => "repeat_once_operator",
        Rule::repeat_exact => "repeat_exact",
        Rule::repeat_min => "repeat_min",
        Rule::repeat_max => "repeat_max",
        Rule::repeat_min_max => "repeat_min_max",
        Rule::number => "number",
        Rule::integer => "integer",
        Rule::comma => "comma",
        Rule::_push => "_push",
        Rule::_push_literal => "_push_literal",
        Rule::peek_slice => "peek_slice",
        Rule::identifier => "identifier",
        Rule::alpha => "alpha",
        Rule::alpha_num => "alpha_num",
        Rule::string => "string",
        Rule::insensitive_string => "insensitive_string",
        Rule::range => "range",
        Rule::character => "character",
        Rule::inner_str => "inner_str",
        Rule::inner_chr => "inner_chr",
        Rule::escape => "escape",
        Rule::code => "code",
        Rule::unicode => "unicode",
        Rule::hex_digit => "hex_digit",
        Rule::quote => "quote",
        Rule::single_quote => "single_quote",
        Rule::range_operator => "range_operator",
        Rule::newline => "newline",
        Rule::WHITESPACE => "WHITESPACE",
        Rule::line_comment => "line_comment",
        Rule::block_comment => "block_comment",
        Rule::COMMENT => "COMMENT",
        Rule::space => "space",
        Rule::grammar_doc => "grammar_doc",
        Rule::line_doc => "line_doc",
        Rule::inner_doc => "inner_doc",
    }
    .to_string()
}
