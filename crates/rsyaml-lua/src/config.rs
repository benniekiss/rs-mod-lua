// SPDX-License-Identifier: MIT

use std::ops::Deref;

use crate::{
    budget::{YamlBudget, YamlBudgetReport},
    policy::{
        YamlAliasLimits,
        YamlCommentPosition,
        YamlDuplicateKeyPolicy,
        YamlMergeKeyPolicy,
        YamlRequireIndent,
    },
};

#[derive(mlua::UserData, mlua::FromLua, Clone)]
pub struct EncodeConfig(mlua::serde::DeserializeOptions);

impl From<mlua::serde::DeserializeOptions> for EncodeConfig {
    fn from(value: mlua::serde::DeserializeOptions) -> Self {
        Self(value)
    }
}

impl From<EncodeConfig> for mlua::serde::DeserializeOptions {
    fn from(value: EncodeConfig) -> Self {
        value.0
    }
}

impl AsRef<mlua::serde::DeserializeOptions> for EncodeConfig {
    fn as_ref(&self) -> &mlua::serde::DeserializeOptions {
        &self.0
    }
}

impl Deref for EncodeConfig {
    type Target = mlua::serde::DeserializeOptions;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for EncodeConfig {
    fn default() -> Self {
        mlua::serde::DeserializeOptions::new().into()
    }
}

#[mlua::userdata_impl]
impl EncodeConfig {
    #[lua(name = "new", infallible)]
    pub(crate) fn lua_new() -> Self {
        Self::default()
    }

    #[lua(name = "deny_unsupported_types", getter, infallible)]
    pub(crate) fn lua_deny_unsupported_types(&self) -> bool {
        self.0.deny_unsupported_types
    }

    #[lua(name = "set_deny_unsupported_types", infallible)]
    pub(crate) fn lua_set_deny_unsupported_types(&mut self, enable: bool) -> Self {
        self.0 = self.0.deny_unsupported_types(enable);
        self.clone()
    }

    #[lua(name = "deny_recursive_tables", getter, infallible)]
    pub(crate) fn lua_deny_recursive_tables(&self) -> bool {
        self.0.deny_recursive_tables
    }

    #[lua(name = "set_deny_recursive_tables", infallible)]
    pub(crate) fn lua_set_deny_recursive_tables(&mut self, enable: bool) -> Self {
        self.0 = self.0.deny_recursive_tables(enable);
        self.clone()
    }

    #[lua(name = "sort_keys", getter, infallible)]
    pub(crate) fn lua_sort_keys(&self) -> bool {
        self.0.sort_keys
    }

    #[lua(name = "set_sort_keys", infallible)]
    pub(crate) fn lua_set_sort_keys(&mut self, enable: bool) -> Self {
        self.0 = self.0.sort_keys(enable);
        self.clone()
    }

    #[lua(name = "encode_empty_tables_as_array", getter, infallible)]
    pub(crate) fn lua_encode_empty_tables_as_array(&self) -> bool {
        self.0.encode_empty_tables_as_array
    }

    #[lua(name = "set_encode_empty_tables_as_array", infallible)]
    pub(crate) fn lua_set_encode_empty_tables_as_array(&mut self, enable: bool) -> Self {
        self.0 = self.0.encode_empty_tables_as_array(enable);
        self.clone()
    }

    #[lua(name = "detect_mixed_tables", getter, infallible)]
    pub(crate) fn lua_detect_mixed_tables(&self) -> bool {
        self.0.detect_mixed_tables
    }

    #[lua(name = "set_detect_mixed_tables", infallible)]
    pub(crate) fn lua_set_detect_mixed_tables(&mut self, enable: bool) -> Self {
        self.0 = self.0.detect_mixed_tables(enable);
        self.clone()
    }
}

#[derive(mlua::UserData, mlua::FromLua, Clone)]
pub struct DecodeConfig {
    #[lua(skip)]
    options: mlua::serde::SerializeOptions,
    #[lua(skip)]
    pub(crate) cast_u64_to_f64: bool,
}

impl From<mlua::serde::SerializeOptions> for DecodeConfig {
    fn from(value: mlua::serde::SerializeOptions) -> Self {
        DecodeConfig {
            options: value,
            cast_u64_to_f64: false,
        }
    }
}

impl From<DecodeConfig> for mlua::serde::SerializeOptions {
    fn from(value: DecodeConfig) -> Self {
        value.options
    }
}

impl AsRef<mlua::serde::SerializeOptions> for DecodeConfig {
    fn as_ref(&self) -> &mlua::serde::SerializeOptions {
        &self.options
    }
}

impl Deref for DecodeConfig {
    type Target = mlua::serde::SerializeOptions;

    fn deref(&self) -> &Self::Target {
        &self.options
    }
}

impl Default for DecodeConfig {
    fn default() -> Self {
        mlua::serde::SerializeOptions::new()
            .detect_serde_json_arbitrary_precision(true)
            .into()
    }
}

#[mlua::userdata_impl]
impl DecodeConfig {
    #[lua(name = "new", infallible)]
    pub(crate) fn lua_new() -> Self {
        Self::default()
    }

    #[lua(name = "null", getter, infallible)]
    pub(crate) fn lua_null(&self) -> bool {
        self.options.serialize_unit_to_null && self.options.serialize_none_to_null
    }

    #[lua(name = "set_null", infallible)]
    pub(crate) fn lua_set_null(&mut self, enable: bool) -> Self {
        self.options = self
            .options
            .serialize_unit_to_null(enable)
            .serialize_none_to_null(enable);
        self.clone()
    }

    #[lua(name = "cast_u64_to_f64", getter, infallible)]
    pub(crate) fn lua_cast_u64_to_f64(&self) -> bool {
        self.cast_u64_to_f64
    }

    #[lua(name = "set_cast_u64_to_f64", infallible)]
    pub(crate) fn lua_set_cast_u64_to_f64(&mut self, enable: bool) -> Self {
        self.cast_u64_to_f64 = enable;
        self.clone()
    }

    #[lua(name = "array_metatable", getter, infallible)]
    pub(crate) fn lua_array_metatable(&self) -> bool {
        self.options.set_array_metatable
    }

    #[lua(name = "set_array_metatable", infallible)]
    pub(crate) fn lua_set_array_metatable(&mut self, enable: bool) -> Self {
        self.options = self.options.set_array_metatable(enable);
        self.clone()
    }
}

#[derive(mlua::UserData, mlua::FromLua, Clone)]
pub(crate) struct YamlEncodeOptions(serde_saphyr::SerializerOptions);

impl From<serde_saphyr::SerializerOptions> for YamlEncodeOptions {
    fn from(value: serde_saphyr::SerializerOptions) -> Self {
        Self(value)
    }
}

impl From<YamlEncodeOptions> for serde_saphyr::SerializerOptions {
    fn from(value: YamlEncodeOptions) -> Self {
        value.0
    }
}

impl AsRef<serde_saphyr::SerializerOptions> for YamlEncodeOptions {
    fn as_ref(&self) -> &serde_saphyr::SerializerOptions {
        &self.0
    }
}

impl Deref for YamlEncodeOptions {
    type Target = serde_saphyr::SerializerOptions;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for YamlEncodeOptions {
    fn default() -> Self {
        serde_saphyr::SerializerOptions::default().into()
    }
}

#[allow(deprecated)]
#[mlua::userdata_impl]
impl YamlEncodeOptions {
    #[lua(name = "new", infallible)]
    pub(crate) fn lua_new() -> Self {
        Self::default()
    }

    #[lua(name = "empty_as_braces", getter, infallible)]
    pub(crate) fn lua_empty_as_braces(&self) -> bool {
        self.0.empty_as_braces
    }

    #[lua(name = "set_empty_as_braces", infallible)]
    pub(crate) fn lua_set_empty_as_braces(&mut self, enable: bool) -> Self {
        self.0.empty_as_braces = enable;
        self.clone()
    }

    #[lua(name = "indent_step", getter, infallible)]
    pub(crate) fn lua_indent_step(&self) -> usize {
        self.0.indent_step
    }

    #[lua(name = "set_indent_step", infallible)]
    pub(crate) fn lua_set_indent_step(&mut self, indent: usize) -> Self {
        self.0.indent_step = indent;
        self.clone()
    }

    #[lua(name = "compact_list_indent", getter, infallible)]
    pub(crate) fn lua_compact_list_indent(&self) -> bool {
        self.0.compact_list_indent
    }

    #[lua(name = "set_compact_list_indent", infallible)]
    pub(crate) fn lua_set_compact_list_indent(&mut self, enable: bool) -> Self {
        self.0.compact_list_indent = enable;
        self.clone()
    }

    // #[lua(name = "anchor_generator", getter, infallible)]
    // pub(crate) fn lua_anchor_generator(&self) -> Option<impl mlua::IntoLua> {
    //     self.0.anchor_generator.map(mlua::Function::wrap_raw)
    // }

    // #[lua(name = "set_anchor_generator", infallible)]
    // pub(crate) fn lua_set_anchor_generator(&mut self, func: mlua::Function) -> Self {
    //     self.0.anchor_generator = Some(|anchor: usize| func.call::<String>(anchor));
    //     self.clone()
    // }

    #[lua(name = "min_fold_chars", getter, infallible)]
    pub(crate) fn lua_min_fold_chars(&self) -> usize {
        self.0.min_fold_chars
    }

    #[lua(name = "set_min_fold_chars", infallible)]
    pub(crate) fn lua_set_min_fold_chars(&mut self, chars: usize) -> Self {
        self.0.min_fold_chars = chars;
        self.clone()
    }

    #[lua(name = "folded_wrap_chars", getter, infallible)]
    pub(crate) fn lua_folded_wrap_chars(&self) -> usize {
        self.0.folded_wrap_chars
    }

    #[lua(name = "set_folded_wrap_chars", infallible)]
    pub(crate) fn lua_set_folded_wrap_chars(&mut self, chars: usize) -> Self {
        self.0.folded_wrap_chars = chars;
        self.clone()
    }

    #[lua(name = "tagged_enums", getter, infallible)]
    pub(crate) fn lua_tagged_enums(&self) -> bool {
        self.0.tagged_enums
    }

    #[lua(name = "set_tagged_enums", infallible)]
    pub(crate) fn lua_set_tagged_enums(&mut self, enable: bool) -> Self {
        self.0.tagged_enums = enable;
        self.clone()
    }

    #[lua(name = "prefer_block_scalars", getter, infallible)]
    pub(crate) fn lua_prefer_block_scalars(&self) -> bool {
        self.0.prefer_block_scalars
    }

    #[lua(name = "set_prefer_block_scalars", infallible)]
    pub(crate) fn lua_set_prefer_block_scalars(&mut self, enable: bool) -> Self {
        self.0.prefer_block_scalars = enable;
        self.clone()
    }

    #[lua(name = "quote_all", getter, infallible)]
    pub(crate) fn lua_quote_all(&self) -> bool {
        self.0.quote_all
    }

    #[lua(name = "set_quote_all", infallible)]
    pub(crate) fn lua_set_quote_all(&mut self, enable: bool) -> Self {
        self.0.quote_all = enable;
        self.clone()
    }

    #[lua(name = "comment_position", getter, infallible)]
    pub(crate) fn lua_comment_position(&self) -> YamlCommentPosition {
        self.0.comment_position.into()
    }

    #[lua(name = "set_comment_position", infallible)]
    pub(crate) fn lua_set_comment_position(&mut self, pos: YamlCommentPosition) -> Self {
        self.0.comment_position = pos.into();
        self.clone()
    }

    #[lua(name = "yaml_12", getter, infallible)]
    pub(crate) fn lua_yaml_12(&self) -> bool {
        self.0.yaml_12
    }

    #[lua(name = "yaml_12", infallible)]
    pub(crate) fn lua_set_yaml_12(&mut self, enable: bool) -> Self {
        self.0.yaml_12 = enable;
        self.clone()
    }
}

#[derive(mlua::UserData, mlua::FromLua, Clone)]
pub(crate) struct YamlDecodeOptions(serde_saphyr::Options);

impl From<serde_saphyr::Options> for YamlDecodeOptions {
    fn from(value: serde_saphyr::Options) -> Self {
        Self(value)
    }
}

impl From<YamlDecodeOptions> for serde_saphyr::Options {
    fn from(value: YamlDecodeOptions) -> Self {
        value.0
    }
}

impl AsRef<serde_saphyr::Options> for YamlDecodeOptions {
    fn as_ref(&self) -> &serde_saphyr::Options {
        &self.0
    }
}

impl Deref for YamlDecodeOptions {
    type Target = serde_saphyr::Options;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for YamlDecodeOptions {
    fn default() -> Self {
        serde_saphyr::Options::default().into()
    }
}

#[allow(deprecated)]
#[mlua::userdata_impl]
impl YamlDecodeOptions {
    #[lua(name = "new", infallible)]
    pub(crate) fn lua_new() -> Self {
        Self::default()
    }

    #[lua(name = "budget", getter, infallible)]
    pub(crate) fn lua_budget(&self) -> Option<YamlBudget> {
        self.0.budget.clone().map(YamlBudget::from)
    }

    #[lua(name = "set_budget", infallible)]
    pub(crate) fn lua_set_budget(&mut self, budget: Option<YamlBudget>) -> Self {
        self.0.budget = budget.map(serde_saphyr::Budget::from);
        self.clone()
    }

    #[lua(name = "with_budget_report", infallible)]
    pub(crate) fn lua_with_budget_report(&mut self, callback: mlua::Function) -> Self {
        self.0 = self.0.clone().with_budget_report(move |budget| {
            let budget: YamlBudgetReport = budget.into();
            let _ = callback.call::<()>(budget);
        });
        self.clone()
    }

    #[lua(name = "duplicate_keys", getter, infallible)]
    pub(crate) fn lua_duplicate_keys(&self) -> YamlDuplicateKeyPolicy {
        self.0.duplicate_keys.into()
    }

    #[lua(name = "set_duplicate_keys", infallible)]
    pub(crate) fn lua_set_duplicate_keys(&mut self, policy: YamlDuplicateKeyPolicy) -> Self {
        self.0.duplicate_keys = policy.into();
        self.clone()
    }

    #[lua(name = "merge_keys", getter, infallible)]
    pub(crate) fn lua_merge_keys(&self) -> YamlMergeKeyPolicy {
        self.0.merge_keys.into()
    }

    #[lua(name = "set_merge_keys", infallible)]
    pub(crate) fn lua_set_merge_keys(&mut self, policy: YamlMergeKeyPolicy) -> Self {
        self.0.merge_keys = policy.into();
        self.clone()
    }

    #[lua(name = "alias_limits", getter, infallible)]
    pub(crate) fn lua_alias_limits(&self) -> YamlAliasLimits {
        self.0.alias_limits.into()
    }

    #[lua(name = "set_alias_limits", infallible)]
    pub(crate) fn lua_set_alias_limits(&mut self, limit: YamlAliasLimits) -> Self {
        self.0.alias_limits = limit.into();
        self.clone()
    }

    #[lua(name = "legacy_octal_numbers", getter, infallible)]
    pub(crate) fn lua_legacy_octal_numbers(&self) -> bool {
        self.0.legacy_octal_numbers
    }

    #[lua(name = "set_legacy_octal_numbers", infallible)]
    pub(crate) fn lua_set_legacy_octal_numbers(&mut self, enable: bool) -> Self {
        self.0.legacy_octal_numbers = enable;
        self.clone()
    }

    #[lua(name = "strict_booleans", getter, infallible)]
    pub(crate) fn lua_strict_booleans(&self) -> bool {
        self.0.strict_booleans
    }

    #[lua(name = "set_strict_booleans", infallible)]
    pub(crate) fn lua_set_strict_booleans(&mut self, enable: bool) -> Self {
        self.0.strict_booleans = enable;
        self.clone()
    }

    #[lua(name = "ignore_binary_tag_for_string", getter, infallible)]
    pub(crate) fn lua_ignore_binary_tag_for_string(&self) -> bool {
        self.0.ignore_binary_tag_for_string
    }

    #[lua(name = "set_ignore_binary_tag_for_string", infallible)]
    pub(crate) fn lua_set_ignore_binary_tag_for_string(&mut self, enable: bool) -> Self {
        self.0.ignore_binary_tag_for_string = enable;
        self.clone()
    }

    #[lua(name = "angle_conversions", getter, infallible)]
    pub(crate) fn lua_angle_conversions(&self) -> bool {
        self.0.angle_conversions
    }

    #[lua(name = "set_angle_conversions", infallible)]
    pub(crate) fn lua_set_angle_conversions(&mut self, enable: bool) -> Self {
        self.0.angle_conversions = enable;
        self.clone()
    }

    #[lua(name = "no_schema", getter, infallible)]
    pub(crate) fn lua_no_schema(&self) -> bool {
        self.0.no_schema
    }

    #[lua(name = "set_no_schema", infallible)]
    pub(crate) fn lua_set_no_schema(&mut self, enable: bool) -> Self {
        self.0.no_schema = enable;
        self.clone()
    }

    #[lua(name = "with_snippet", getter, infallible)]
    pub(crate) fn lua_with_snippet(&self) -> bool {
        self.0.with_snippet
    }

    #[lua(name = "set_with_snippet", infallible)]
    pub(crate) fn lua_set_with_snippet(&mut self, enable: bool) -> Self {
        self.0.with_snippet = enable;
        self.clone()
    }

    #[lua(name = "crop_radius", getter, infallible)]
    pub(crate) fn lua_crop_radius(&self) -> usize {
        self.0.crop_radius
    }

    #[lua(name = "set_crop_radius", infallible)]
    pub(crate) fn lua_set_crop_radius(&mut self, radius: usize) -> Self {
        self.0.crop_radius = radius;
        self.clone()
    }

    #[lua(name = "require_indent", getter, infallible)]
    pub(crate) fn lua_require_indent(&self) -> YamlRequireIndent {
        self.0.require_indent.into()
    }

    #[lua(name = "set_require_indent", infallible)]
    pub(crate) fn lua_set_require_indent(&mut self, indent: YamlRequireIndent) -> Self {
        self.0.require_indent = indent.into();
        self.clone()
    }
}
