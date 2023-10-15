use proc_macro2::{Ident, TokenStream};
use syn::parse::{Parse, Parser, ParseStream};
use syn::{Token, token};
use syn::token::Token;
use crate::define_keywords;
use crate::macro_types::custom_syntax::custom_syntax_macro::keywords::{KeyOf, ValueOf};

// TODO: Implement Parse for each Struct
// Based on Yew: https://github.dev/yewstack/yew/blob/master/packages/yew-macro/src/html_tree/html_block.rs


/// Specify keywords based on example [1]
///
/// [1] C:/Users/zbilm/.cargo/registry/src/github.com-1ecc6299db9ec823/syn-1.0.107/src/token.rs:241
mod keywords {
    use syn::custom_keyword;
    pub struct KeyOf;
    pub struct ValueOf;

    custom_keyword!(keyOf);
    custom_keyword!(valueOf);

    // TODO: Implement Parse for these two keywords. May need to create a macro for this.
}

/// Based on this [1] example
/// [1] https://docs.rs/syn/latest/syn/struct.Path.html#method.parse_mod_style
struct TemplateMapDefinition {
    key: Ident,
    colon: Token![:],
    expr: TemplateExpression
}

enum TemplateExpressionPath {
    KeyOf(KeyOf),
    ValueOf(ValueOf),
    /// Identifier of a Struct or Enum
    Identifier(Ident),
    TemplateIndex(TemplateIndex),
    /// Indicating the end of the expression chain
    End
}

/// Example:
/// `keyOf valueOf StructName['fieldName']`
struct TemplateExpression {
    /// Passed to a Template expression for dynamic templating ex.
    ///
    /// `[key: keyOf StructName]: AnotherStruct[key]`
    ///
    /// where `key` is the parameter.
    parameter: Option<Ident>,
    template_expression: Vec<TemplateExpressionPath>
}

enum TemplateIndexBrackets {
    Ident(token::Bracket, syn::Ident),
    Literal(token::Bracket, syn::LitStr)
}

/// Example:
///
/// `StructName[parameterName]` or `EnumName['literal']`
struct TemplateIndex {
    /// Name of the struct/enum
    name: Ident,
    brackets: TemplateIndexBrackets
}

/// Example:
/// `[key: keyOf StructName]: String`
struct TemplateSyntax {
    template_definition: TemplateMapDefinition,
    colon: Token![:],
    template_value: TemplateExpression

}


#[cfg(test)]
mod tests {

}