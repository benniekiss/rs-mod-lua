use std::iter::Peekable;

use mlua::{IntoLua, IntoLuaMulti};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct LuaSpan {
    start: usize,
    end: usize,
    text: String,
}

impl From<pest::Span<'_>> for LuaSpan {
    fn from(value: pest::Span) -> Self {
        Self {
            start: value.start(),
            end: value.end(),
            text: value.as_str().to_string(),
        }
    }
}

impl From<&pest::Span<'_>> for LuaSpan {
    fn from(value: &pest::Span) -> Self {
        Self {
            start: value.start(),
            end: value.end(),
            text: value.as_str().to_string(),
        }
    }
}

impl mlua::IntoLuaMulti for LuaSpan {
    fn into_lua_multi(self, lua: &mlua::Lua) -> mlua::Result<mlua::MultiValue> {
        let mut mv = mlua::MultiValue::with_capacity(3);
        mv.push_back(self.text.into_lua(lua)?);
        mv.push_back(self.start.into_lua(lua)?);
        mv.push_back(self.end.into_lua(lua)?);
        Ok(mv)
    }
}

pub(crate) struct LuaLinesIter<'scope> {
    span: pest::Span<'scope>,
    pos: usize,
}

impl<'scope> From<pest::Span<'scope>> for LuaLinesIter<'scope> {
    fn from(value: pest::Span<'scope>) -> Self {
        Self {
            span: value,
            pos: value.start(),
        }
    }
}

impl<'scope> LuaLinesIter<'scope> {
    fn find_line_start(&self) -> usize {
        // SPDX-SnippetBegin
        // SPDX-SnippetCopyrightText: 2018 Dragoș Tiselice
        // SPDX-License-Identifier: MIT
        //
        // Copied from the [`pest::Position::find_line_start`] implementation
        // https://github.com/pest-parser/pest
        let input = self.span.get_input();
        if input.is_empty() {
            return 0;
        };
        // pos is always a UTF-8 border.
        let start = input
            .char_indices()
            .rev()
            .skip_while(|&(i, _)| i >= self.pos)
            .find(|&(_, c)| c == '\n');
        match start {
            Some((i, _)) => i + 1,
            None => 0,
        }
        // SPDX-SnippetEnd
    }

    fn find_line_end(&self) -> usize {
        // SPDX-SnippetBegin
        // SPDX-FileCopyrightText: 2018 Dragoș Tiselice
        // SPDX-License-Identifier: MIT
        //
        // Copied from the [`pest::Position::find_line_end`] implementation
        // https://github.com/pest-parser/pest
        let input = self.span.get_input();
        if input.is_empty() {
            0
        } else if self.pos == input.len() - 1 {
            input.len()
        } else {
            // pos is always a UTF-8 border.
            let end = input
                .char_indices()
                .skip_while(|&(i, _)| i < self.pos)
                .find(|&(_, c)| c == '\n');
            match end {
                Some((i, _)) => i + 1,
                None => input.len(),
            }
        }
        // SPDX-SnippetEnd
    }
}

impl<'scope> Iterator for LuaLinesIter<'scope> {
    type Item = LuaSpan;

    fn next(&mut self) -> Option<Self::Item> {
        // SPDX-SnippetBegin
        // SPDX-FileCopyrightText: 2018 Dragoș Tiselice
        // SPDX-License-Identifier: MIT
        //
        // Copied from the [`pest::LinesSpan::next`] implementation
        // https://github.com/pest-parser/pest
        if self.pos >= self.span.end() {
            return None;
        }

        let start = self.find_line_start();
        self.pos = self.find_line_end();

        Some(LuaSpan {
            start,
            end: self.pos,
            text: self.span.get_input()[start..self.pos].to_string(),
        })
        // SPDX-SnippetEnd
    }
}

pub(crate) struct LuaLines<'scope>(Peekable<LuaLinesIter<'scope>>);

impl<'scope> From<pest::Span<'scope>> for LuaLines<'scope> {
    fn from(value: pest::Span<'scope>) -> Self {
        Self(LuaLinesIter::from(value).peekable())
    }
}

impl<'scope> mlua::UserData for LuaLines<'scope> {
    fn add_methods<M: mlua::prelude::LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("peek", |lua, this, ()| {
            this.0
                .peek()
                .map(|v| v.clone().into_lua_multi(lua))
                .transpose()
                .map(|v| v.unwrap_or_default())
        });

        methods.add_method_mut("next", |lua, this, ()| {
            this.0
                .next()
                .map(|v| v.into_lua_multi(lua))
                .transpose()
                .map(|v| v.unwrap_or_default())
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_iteration() {
        let input = "one\ntwo\nthree";

        let span = pest::Span::new(input, 0, input.len()).unwrap();
        let ex = span.lines_span().map(LuaSpan::from).collect::<Vec<_>>();

        let lines: LuaLines = span.into();
        let res = lines.0.collect::<Vec<_>>();

        assert_eq!(ex, res)
    }
}
