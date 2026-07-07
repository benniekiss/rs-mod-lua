use mlua::LuaSerdeExt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct YamlBudget {
    pub(crate) max_reader_input_bytes: Option<usize>,
    pub(crate) max_events: usize,
    pub(crate) max_aliases: usize,
    pub(crate) max_anchors: usize,
    pub(crate) max_depth: usize,
    pub(crate) max_inclusion_depth: u32,
    pub(crate) max_documents: usize,
    pub(crate) max_nodes: usize,
    pub(crate) max_total_scalar_bytes: usize,
    pub(crate) max_total_comment_bytes: usize,
    pub(crate) max_merge_keys: usize,
    pub(crate) enforce_alias_anchor_ratio: bool,
    pub(crate) alias_anchor_min_aliases: usize,
    pub(crate) alias_anchor_ratio_multiplier: usize,
}

#[allow(deprecated)]
impl From<serde_saphyr::Budget> for YamlBudget {
    fn from(value: serde_saphyr::Budget) -> Self {
        Self {
            max_reader_input_bytes: value.max_reader_input_bytes,
            max_events: value.max_events,
            max_aliases: value.max_aliases,
            max_anchors: value.max_anchors,
            max_depth: value.max_depth,
            max_inclusion_depth: value.max_inclusion_depth,
            max_documents: value.max_documents,
            max_nodes: value.max_nodes,
            max_total_scalar_bytes: value.max_total_scalar_bytes,
            max_total_comment_bytes: value.max_total_comment_bytes,
            max_merge_keys: value.max_merge_keys,
            enforce_alias_anchor_ratio: value.enforce_alias_anchor_ratio,
            alias_anchor_min_aliases: value.alias_anchor_min_aliases,
            alias_anchor_ratio_multiplier: value.alias_anchor_ratio_multiplier,
        }
    }
}

#[allow(deprecated)]
impl From<YamlBudget> for serde_saphyr::Budget {
    fn from(value: YamlBudget) -> Self {
        Self {
            max_reader_input_bytes: value.max_reader_input_bytes,
            max_events: value.max_events,
            max_aliases: value.max_aliases,
            max_anchors: value.max_anchors,
            max_depth: value.max_depth,
            max_inclusion_depth: value.max_inclusion_depth,
            max_documents: value.max_documents,
            max_nodes: value.max_nodes,
            max_total_scalar_bytes: value.max_total_scalar_bytes,
            max_total_comment_bytes: value.max_total_comment_bytes,
            max_merge_keys: value.max_merge_keys,
            enforce_alias_anchor_ratio: value.enforce_alias_anchor_ratio,
            alias_anchor_min_aliases: value.alias_anchor_min_aliases,
            alias_anchor_ratio_multiplier: value.alias_anchor_ratio_multiplier,
        }
    }
}

impl mlua::IntoLua for YamlBudget {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        lua.to_value(&self)
    }
}

impl mlua::FromLua for YamlBudget {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        lua.from_value(value)
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) enum YamlBudgetBreach {
    Events { events: usize },
    Aliases { aliases: usize },
    Anchors { anchors: usize },
    Depth { depth: usize },
    InclusionDepth { depth: u32 },
    Documents { documents: usize },
    Nodes { nodes: usize },
    ScalarBytes { total_scalar_bytes: usize },
    CommentBytes { total_comment_bytes: usize },
    MergeKeys { merge_keys: usize },
    AliasAnchorRatio { aliases: usize, anchors: usize },
    SequenceUnbalanced,
    InputBytes { input_bytes: usize },
}

impl From<serde_saphyr::budget::BudgetBreach> for YamlBudgetBreach {
    fn from(value: serde_saphyr::budget::BudgetBreach) -> Self {
        match value {
            serde_saphyr::budget::BudgetBreach::Events { events } => {
                YamlBudgetBreach::Events { events }
            },
            serde_saphyr::budget::BudgetBreach::Aliases { aliases } => {
                YamlBudgetBreach::Aliases { aliases }
            },
            serde_saphyr::budget::BudgetBreach::Anchors { anchors } => {
                YamlBudgetBreach::Anchors { anchors }
            },
            serde_saphyr::budget::BudgetBreach::Depth { depth } => {
                YamlBudgetBreach::Depth { depth }
            },
            serde_saphyr::budget::BudgetBreach::InclusionDepth { depth } => {
                YamlBudgetBreach::InclusionDepth { depth }
            },
            serde_saphyr::budget::BudgetBreach::Documents { documents } => {
                YamlBudgetBreach::Documents { documents }
            },
            serde_saphyr::budget::BudgetBreach::Nodes { nodes } => {
                YamlBudgetBreach::Nodes { nodes }
            },
            serde_saphyr::budget::BudgetBreach::ScalarBytes { total_scalar_bytes } => {
                YamlBudgetBreach::ScalarBytes { total_scalar_bytes }
            },
            serde_saphyr::budget::BudgetBreach::CommentBytes {
                total_comment_bytes,
            } => YamlBudgetBreach::CommentBytes {
                total_comment_bytes,
            },
            serde_saphyr::budget::BudgetBreach::MergeKeys { merge_keys } => {
                YamlBudgetBreach::MergeKeys { merge_keys }
            },
            serde_saphyr::budget::BudgetBreach::AliasAnchorRatio { aliases, anchors } => {
                YamlBudgetBreach::AliasAnchorRatio { aliases, anchors }
            },
            serde_saphyr::budget::BudgetBreach::SequenceUnbalanced => {
                YamlBudgetBreach::SequenceUnbalanced
            },
            serde_saphyr::budget::BudgetBreach::InputBytes { input_bytes } => {
                YamlBudgetBreach::InputBytes { input_bytes }
            },
            _ => YamlBudgetBreach::InputBytes { input_bytes: 0 },
        }
    }
}

impl From<YamlBudgetBreach> for serde_saphyr::budget::BudgetBreach {
    fn from(value: YamlBudgetBreach) -> Self {
        match value {
            YamlBudgetBreach::Events { events } => {
                serde_saphyr::budget::BudgetBreach::Events { events }
            },

            YamlBudgetBreach::Aliases { aliases } => {
                serde_saphyr::budget::BudgetBreach::Aliases { aliases }
            },
            YamlBudgetBreach::Anchors { anchors } => {
                serde_saphyr::budget::BudgetBreach::Anchors { anchors }
            },
            YamlBudgetBreach::Depth { depth } => {
                serde_saphyr::budget::BudgetBreach::Depth { depth }
            },
            YamlBudgetBreach::InclusionDepth { depth } => {
                serde_saphyr::budget::BudgetBreach::InclusionDepth { depth }
            },
            YamlBudgetBreach::Documents { documents } => {
                serde_saphyr::budget::BudgetBreach::Documents { documents }
            },
            YamlBudgetBreach::Nodes { nodes } => {
                serde_saphyr::budget::BudgetBreach::Nodes { nodes }
            },
            YamlBudgetBreach::ScalarBytes { total_scalar_bytes } => {
                serde_saphyr::budget::BudgetBreach::ScalarBytes { total_scalar_bytes }
            },
            YamlBudgetBreach::CommentBytes {
                total_comment_bytes,
            } => serde_saphyr::budget::BudgetBreach::CommentBytes {
                total_comment_bytes,
            },
            YamlBudgetBreach::MergeKeys { merge_keys } => {
                serde_saphyr::budget::BudgetBreach::MergeKeys { merge_keys }
            },
            YamlBudgetBreach::AliasAnchorRatio { aliases, anchors } => {
                serde_saphyr::budget::BudgetBreach::AliasAnchorRatio { aliases, anchors }
            },
            YamlBudgetBreach::SequenceUnbalanced => {
                serde_saphyr::budget::BudgetBreach::SequenceUnbalanced
            },
            YamlBudgetBreach::InputBytes { input_bytes } => {
                serde_saphyr::budget::BudgetBreach::InputBytes { input_bytes }
            },
        }
    }
}

impl mlua::IntoLua for YamlBudgetBreach {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        lua.to_value(&self)
    }
}

impl mlua::FromLua for YamlBudgetBreach {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        lua.from_value(value)
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct YamlBudgetReport {
    pub breached: Option<YamlBudgetBreach>,
    pub events: usize,
    pub aliases: usize,
    pub anchors: usize,
    pub documents: usize,
    pub nodes: usize,
    pub max_depth: usize,
    pub total_scalar_bytes: usize,
    pub total_comment_bytes: usize,
    pub merge_keys: usize,
}

impl From<serde_saphyr::budget::BudgetReport> for YamlBudgetReport {
    fn from(value: serde_saphyr::budget::BudgetReport) -> Self {
        Self {
            breached: value.breached.map(YamlBudgetBreach::from),
            events: value.events,
            aliases: value.aliases,
            anchors: value.anchors,
            documents: value.documents,
            nodes: value.nodes,
            max_depth: value.max_depth,
            total_scalar_bytes: value.total_scalar_bytes,
            total_comment_bytes: value.total_comment_bytes,
            merge_keys: value.merge_keys,
        }
    }
}

impl From<YamlBudgetReport> for serde_saphyr::budget::BudgetReport {
    fn from(value: YamlBudgetReport) -> Self {
        Self {
            breached: value.breached.map(serde_saphyr::budget::BudgetBreach::from),
            events: value.events,
            aliases: value.aliases,
            anchors: value.anchors,
            documents: value.documents,
            nodes: value.nodes,
            max_depth: value.max_depth,
            total_scalar_bytes: value.total_scalar_bytes,
            total_comment_bytes: value.total_comment_bytes,
            merge_keys: value.merge_keys,
        }
    }
}

impl mlua::IntoLua for YamlBudgetReport {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        lua.to_value(&self)
    }
}

impl mlua::FromLua for YamlBudgetReport {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        lua.from_value(value)
    }
}
