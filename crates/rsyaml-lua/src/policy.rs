use mlua::LuaSerdeExt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct YamlAliasLimits {
    pub(crate) max_total_replayed_events: usize,
    pub(crate) max_replay_stack_depth: usize,
    pub(crate) max_alias_expansions_per_anchor: usize,
}

#[allow(deprecated)]
impl From<serde_saphyr::options::AliasLimits> for YamlAliasLimits {
    fn from(value: serde_saphyr::options::AliasLimits) -> Self {
        Self {
            max_total_replayed_events: value.max_total_replayed_events,
            max_replay_stack_depth: value.max_replay_stack_depth,
            max_alias_expansions_per_anchor: value.max_alias_expansions_per_anchor,
        }
    }
}

#[allow(deprecated)]
impl From<YamlAliasLimits> for serde_saphyr::options::AliasLimits {
    fn from(value: YamlAliasLimits) -> Self {
        Self {
            max_total_replayed_events: value.max_total_replayed_events,
            max_replay_stack_depth: value.max_replay_stack_depth,
            max_alias_expansions_per_anchor: value.max_alias_expansions_per_anchor,
        }
    }
}

impl mlua::IntoLua for YamlAliasLimits {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        lua.to_value(&self)
    }
}

impl mlua::FromLua for YamlAliasLimits {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        lua.from_value(value)
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) enum YamlMergeKeyPolicy {
    Merge,
    AsOrdinary,
    Error,
}

impl From<serde_saphyr::MergeKeyPolicy> for YamlMergeKeyPolicy {
    fn from(value: serde_saphyr::MergeKeyPolicy) -> Self {
        match value {
            serde_saphyr::MergeKeyPolicy::Merge => YamlMergeKeyPolicy::Merge,
            serde_saphyr::MergeKeyPolicy::AsOrdinary => YamlMergeKeyPolicy::AsOrdinary,
            serde_saphyr::MergeKeyPolicy::Error => YamlMergeKeyPolicy::Error,
            _ => YamlMergeKeyPolicy::Error,
        }
    }
}

impl From<YamlMergeKeyPolicy> for serde_saphyr::MergeKeyPolicy {
    fn from(value: YamlMergeKeyPolicy) -> Self {
        match value {
            YamlMergeKeyPolicy::Merge => serde_saphyr::MergeKeyPolicy::Merge,
            YamlMergeKeyPolicy::AsOrdinary => serde_saphyr::MergeKeyPolicy::AsOrdinary,
            YamlMergeKeyPolicy::Error => serde_saphyr::MergeKeyPolicy::Error,
        }
    }
}

impl mlua::IntoLua for YamlMergeKeyPolicy {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        lua.to_value(&self)
    }
}

impl mlua::FromLua for YamlMergeKeyPolicy {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        lua.from_value(value)
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) enum YamlDuplicateKeyPolicy {
    Error,
    FirstWins,
    LastWins,
}

impl From<serde_saphyr::DuplicateKeyPolicy> for YamlDuplicateKeyPolicy {
    fn from(value: serde_saphyr::DuplicateKeyPolicy) -> Self {
        match value {
            serde_saphyr::DuplicateKeyPolicy::Error => YamlDuplicateKeyPolicy::Error,
            serde_saphyr::DuplicateKeyPolicy::FirstWins => YamlDuplicateKeyPolicy::FirstWins,
            serde_saphyr::DuplicateKeyPolicy::LastWins => YamlDuplicateKeyPolicy::LastWins,
            _ => YamlDuplicateKeyPolicy::Error,
        }
    }
}

impl From<YamlDuplicateKeyPolicy> for serde_saphyr::DuplicateKeyPolicy {
    fn from(value: YamlDuplicateKeyPolicy) -> Self {
        match value {
            YamlDuplicateKeyPolicy::Error => serde_saphyr::DuplicateKeyPolicy::Error,
            YamlDuplicateKeyPolicy::FirstWins => serde_saphyr::DuplicateKeyPolicy::FirstWins,
            YamlDuplicateKeyPolicy::LastWins => serde_saphyr::DuplicateKeyPolicy::LastWins,
        }
    }
}

impl mlua::IntoLua for YamlDuplicateKeyPolicy {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        lua.to_value(&self)
    }
}

impl mlua::FromLua for YamlDuplicateKeyPolicy {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        lua.from_value(value)
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) enum YamlRequireIndent {
    Unchecked,
    Divisible(usize),
    Even,
    Uniform(Option<usize>),
}

impl From<serde_saphyr::RequireIndent> for YamlRequireIndent {
    fn from(value: serde_saphyr::RequireIndent) -> Self {
        match value {
            serde_saphyr::RequireIndent::Unchecked => YamlRequireIndent::Unchecked,
            serde_saphyr::RequireIndent::Divisible(n) => YamlRequireIndent::Divisible(n),
            serde_saphyr::RequireIndent::Even => YamlRequireIndent::Even,
            serde_saphyr::RequireIndent::Uniform(n) => YamlRequireIndent::Uniform(n),
            _ => YamlRequireIndent::Unchecked,
        }
    }
}

impl From<YamlRequireIndent> for serde_saphyr::RequireIndent {
    fn from(value: YamlRequireIndent) -> Self {
        match value {
            YamlRequireIndent::Unchecked => serde_saphyr::RequireIndent::Unchecked,
            YamlRequireIndent::Divisible(n) => serde_saphyr::RequireIndent::Divisible(n),
            YamlRequireIndent::Even => serde_saphyr::RequireIndent::Even,
            YamlRequireIndent::Uniform(n) => serde_saphyr::RequireIndent::Uniform(n),
        }
    }
}

impl mlua::IntoLua for YamlRequireIndent {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        lua.to_value(&self)
    }
}

impl mlua::FromLua for YamlRequireIndent {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        lua.from_value(value)
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) enum YamlCommentPosition {
    Inline,
    Above,
}

impl From<serde_saphyr::CommentPosition> for YamlCommentPosition {
    fn from(value: serde_saphyr::CommentPosition) -> Self {
        match value {
            serde_saphyr::CommentPosition::Above => YamlCommentPosition::Above,
            serde_saphyr::CommentPosition::Inline => YamlCommentPosition::Inline,
            _ => YamlCommentPosition::Inline,
        }
    }
}

impl From<YamlCommentPosition> for serde_saphyr::CommentPosition {
    fn from(value: YamlCommentPosition) -> Self {
        match value {
            YamlCommentPosition::Above => serde_saphyr::CommentPosition::Above,
            YamlCommentPosition::Inline => serde_saphyr::CommentPosition::Inline,
        }
    }
}

impl mlua::IntoLua for YamlCommentPosition {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        lua.to_value(&self)
    }
}

impl mlua::FromLua for YamlCommentPosition {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        lua.from_value(value)
    }
}
