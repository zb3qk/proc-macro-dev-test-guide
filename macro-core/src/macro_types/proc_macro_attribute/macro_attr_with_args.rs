use crate::helpers::AttributeArgs;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, parse_quote, ItemFn};
use syn::parse::Parser;

pub fn attr_macro_with_args_core(args: TokenStream, input: TokenStream) -> TokenStream {
    // let args_parsed = match parse2::<AttributeArgs>(args) {
    //     Ok(abstract_syntax_tree) => abstract_syntax_tree,
    //     Err(error) => return error.to_compile_error(),
    // };
    //
    // let args_parsed: AttributeArgs = parse_quote!(args);

    let args_parsed = syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated
        .parse2(args)
        .unwrap(); // Better to turn it into a `compile_error!()`

    // args_parsed.first().unwrap().segments.last().unwrap().ident;

    // proc_macro2 version of "parse_macro_input!(input as ItemFn)"
    let item_fn = match parse2::<ItemFn>(input) {
        Ok(syntax_tree) => syntax_tree,
        Err(error) => return error.to_compile_error(),
    };

    quote!(#item_fn)
}
