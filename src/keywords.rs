use serde::{Deserialize, Serialize};

mod additional_properties;
mod all_of;
mod anchor;
mod any_of;
mod comment;
mod constant;
mod contains;
mod default;
mod definitions;
mod defs;
mod deprecated;
mod description;
mod dynamic_anchor;
mod dynamic_ref;
mod r#enum;
mod examples;
mod exclusive_maximum;
mod exclusive_minimum;
mod format;
mod id;
mod if_then_else;
mod items;
mod max_contains;
mod max_items;
mod max_length;
mod max_props;
mod maximum;
mod min_contains;
mod min_items;
mod min_length;
mod min_props;
mod minimum;
mod multiple;
mod not;
mod one_of;
mod pattern;
mod pattern_properties;
mod prefix_items;
mod properties;
mod property_names;
mod read_only;
mod required;
mod schema;
mod title;
mod r#type;
mod unevaluated_items;
mod unevaluated_properties;
mod unique_items;
mod write_only;

pub use additional_properties::AdditionalPropertiesKeyword;
pub use all_of::AllOfKeyword;
pub use anchor::AnchorKeyword;
pub use any_of::AnyOfKeyword;
pub use comment::CommentKeyword;
pub use constant::ConstantKeyword;
pub use contains::ContainsKeyword;
pub use default::DefaultKeyword;
pub use definitions::DefinitionsKeyword;
pub use defs::DefsKeyword;
pub use deprecated::DeprecatedKeyword;
pub use description::DescriptionKeyword;
pub use dynamic_anchor::DynamicAnchorKeyword;
pub use dynamic_ref::DynamicRefKeyword;
pub use examples::ExamplesKeyword;
pub use exclusive_maximum::ExclusiveMaximumKeyword;
pub use exclusive_minimum::ExclusiveMinimumKeyword;
pub use format::FormatKeyword;
pub use id::IdKeyword;
pub use if_then_else::IfThenElseKeyword;
pub use items::ItemsKeyword;
pub use max_contains::MaxContainsKeyword;
pub use max_items::MaxItemsKeyword;
pub use max_length::MaxLengthKeyword;
pub use max_props::MaxPropertiesKeyword;
pub use maximum::MaximumKeyword;
pub use min_contains::MinContainsKeyword;
pub use min_items::MinItemsKeyword;
pub use min_length::MinLengthKeyword;
pub use min_props::MinPropertiesKeyword;
pub use minimum::MinimumKeyword;
pub use multiple::MultipleOfKeyword;
pub use not::NotKeyword;
pub use one_of::OneOfKeyword;
pub use pattern::PatternKeyword;
pub use pattern_properties::PatternPropertiesKeyword;
pub use prefix_items::PrefixItemsKeyword;
pub use properties::PropertiesKeyword;
pub use property_names::PropertyNamesKeyword;
pub use r#enum::EnumKeyword;
pub use r#type::TypeKeyword;
pub use read_only::ReadOnlyKeyword;
pub use required::RequiredKeyword;
pub use schema::SchemaKeyword;
pub use title::TitleKeyword;
pub use unevaluated_items::UnevaluatedItemsKeyword;
pub use unevaluated_properties::UnevaluatedPropertiesKeyword;
pub use unique_items::UniqueItemsKeyword;
pub use write_only::WriteOnlyKeyword;

use crate::{
    context::{Compiler, Context},
    validator::Validator,
};
use url::Url;

/// A trait that all keywords need to implement, it is responsible to taking the
/// relevant key out of the schema and implmenting behavior in the stages of validation.
/// Validation is broken up into three distinct phases run in order.
/// 1. The resolve phase
///     keywords are allowed to resolve their value to other schema objects, this allows keywords like "$ref" to
///     be replaced with their proper schema object
/// 2. The patch phase
///     keywords are allowed to alter the document before validation begins, this allows keywords like "default" to
///     patch the document before
/// 3. The validate phase
///     keywords are tested against the relevant value individually, keywords can return one of 4 values
///         - Pass
///         - Fail(reason)
///         - N/A
///         - Defer(keywords)
///     if any keywords return Fail, the schema does not validate. All keywords are checked in order to provide better error
///     handling. Deffering allows keywords to refer to the results of dependents, although it generally wiser to factor out
///     dependent keywords.
pub trait Keyword {
    fn compile(&mut self, compiler: &mut Compiler);
    fn patch(&self, validator: Validator);
    fn validate(&self, validator: Validator);
}

macro_rules! def_keywords {
    ($($ident:ident: $ty:ty = $name:literal $([$($derive:tt)*])?)*) => {
        #[derive(Deserialize, Serialize, Default)]
        pub struct Keywords {
            $(
                #[serde($($($derive)*)?)]
                #[serde(rename = $name, skip_serializing_if = "Option::is_none")]
                pub $ident: Option<$ty>,
            )*
        }

        impl Keywords {
            pub fn compile(&mut self, compiler: &mut Compiler) {
                $(
                    if let Some(keyword) = &mut self.$ident {
                        Keyword::compile(keyword, compiler);
                    }
                )*
            }
        }
    };
}

def_keywords!(
    additional_properties: AdditionalPropertiesKeyword = "additionalProperties"
    all_of: AllOfKeyword = "allOf"
    anchor: AnchorKeyword = "$anchor"
    any_of: AnyOfKeyword = "anyOf"
    comment: CommentKeyword = "$comment"
    constant: ConstantKeyword = "const"
    contains: ContainsKeyword = "contains"
    default: DefaultKeyword = "default"
    definitions: DefinitionsKeyword = "definitions"
    defs: DefsKeyword = "$defs"
    deprecated: DeprecatedKeyword = "deprecated"
    description: DescriptionKeyword = "description"
    dynamic_anchor: DynamicAnchorKeyword = "$dynamicAnchor"
    dynamic_ref: DynamicRefKeyword = "$dynamicRef"
    enum_: EnumKeyword = "enum"
    examples: ExamplesKeyword = "examples"
    excl_max: ExclusiveMaximumKeyword = "exclusiveMaximum"
    excl_min: ExclusiveMinimumKeyword = "exclusiveMinimum"
    format: FormatKeyword = "format"
    id: IdKeyword = "$id"
    if_then_else: IfThenElseKeyword = "if_then_else" [flatten]
    items: ItemsKeyword = "items"
    max_contains: MaxContainsKeyword = "maxContains"
    max_items: MaxItemsKeyword = "maxItems"
    max_length: MaxLengthKeyword = "maxLength"
    max_props: MaxPropertiesKeyword = "maxProperties"
    max: MaximumKeyword = "maximum"
    min_contains: MinContainsKeyword = "minContains"
    min_items: MinItemsKeyword = "minItems"
    min_length: MinLengthKeyword = "minLength"
    min_props: MinPropertiesKeyword = "minProperties"
    min: MinimumKeyword = "minimum"
    multiple: MultipleOfKeyword = "multipleOf"
    not: NotKeyword = "not"
    one_of: OneOfKeyword = "oneOf"
    pat_props: PatternPropertiesKeyword = "patternProperties"
    pat: PatternKeyword = "pattern"
    prefix_items: PrefixItemsKeyword = "prefixItems"
    props: PropertiesKeyword = "properties"
    prop_names: PropertyNamesKeyword = "propertyNames"
    read_only: ReadOnlyKeyword = "readOnly"
    required: RequiredKeyword = "required"
    schema: SchemaKeyword = "$schema"
    title: TitleKeyword = "title"
    type_: TypeKeyword = "type"
    unevaluated_items: UnevaluatedItemsKeyword = "unevaluatedItems"
    unique_items: UniqueItemsKeyword = "uniqueItems"
    write_only: WriteOnlyKeyword = "writeOnly"
);

pub enum IllogicalSchema {}

impl Keywords {
    /// test whether or not the keywords in the schema form a logical schema.
    pub fn is_logical(&self) -> bool {
        match self {
            _ => false,
        }
    }
}
