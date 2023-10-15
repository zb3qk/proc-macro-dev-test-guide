//!
//! ### Example
//!
//! `clone_def!(struct syn::Path as NewPath)`

use proc_macro2::{Ident, TokenStream};
use proc_macro_error::{Diagnostic, Level};
use quote::{quote, ToTokens};
use syn::{Path, Token};
use syn::parse::{Parse, ParseStream};

/// clone_def supports cloning the definitions of the below structures and none else.
enum DefinitionType {
    Struct(Token![struct]),
    Enum(Token![enum]),
    Function(Token![fn])
}

impl ToTokens for DefinitionType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            DefinitionType::Struct(v) => quote!(#v),
            DefinitionType::Function(f) => quote!(#f),
            DefinitionType::Enum(e) => quote!(#e),
        });
    }
}

struct CloneDefinitionExpression {
    def_type: DefinitionType,
    path: Path,
    _as: Token![as],
    def_name: Ident
}

// TODO: Write nicer error messages

impl Parse for DefinitionType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![struct]) { Ok(Self::Struct(input.parse()?)) }
        else if lookahead.peek(Token![enum]) { Ok(Self::Enum(input.parse()?)) }
        else if lookahead.peek(Token![fn]) { Ok(Self::Function(input.parse()?)) }
        else { Err(lookahead.error()) }
    }
}

impl Parse for CloneDefinitionExpression {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(CloneDefinitionExpression {
            def_type: input.parse()?,
            path: input.parse()?,
            _as: input.parse()?,
            def_name: input.parse()?
        })
    }
}

impl ToTokens for CloneDefinitionExpression {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let CloneDefinitionExpression { def_name, def_type, .. } = self;
        let definition = quote!(#def_type #def_name);
        tokens.extend(match self.def_type {
            DefinitionType::Struct(_) | DefinitionType::Enum(_) => quote!(#definition {}),
            DefinitionType::Function(_) => quote!(#definition())
        });
    }
}

pub fn clone_def(tokens: TokenStream) -> TokenStream {
    let clone_def: CloneDefinitionExpression = match syn::parse2(tokens) {
        Ok(exp) => exp,
        Err(e) => return e.to_compile_error()
    };
    clone_def.into_token_stream()
}

pub fn clone_def_attr(item: TokenStream, attrs: TokenStream) -> TokenStream {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use crate::macro_types::custom_syntax::clone_def_syntax::clone_def;
    use crate::test_commons::test_commons::assert_tokens_eq;

    #[test]
    fn parse_expression() {
        let expression = quote! {
            struct path::to::structure as NewStructName
        };
        let def = clone_def(expression);
        assert_tokens_eq( &quote!(struct NewStructName {}), &def)
    }

    #[test]
    fn parse_failed_expression() {
        let expression = quote! {
            boop path::to::structure as NewStructName
        };
        let def = clone_def(expression);
        let expected = quote! {
            compile_error ! { "expected one of: `struct`, `enum`, `fn`" }
        };
        assert_tokens_eq(&expected, &def)
    }

    #[test]
    fn parse_failed_name_expression() {
        let expression = quote! {
            struct path::to::structure as struct
        };
        let def = clone_def(expression);
        let expected = quote! {
            compile_error ! { "expected identifier" }
        };
        assert_tokens_eq(&expected, &def)
    }
}
