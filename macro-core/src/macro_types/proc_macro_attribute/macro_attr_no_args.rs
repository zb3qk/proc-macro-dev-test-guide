use std::ops::Deref;
use crate::helpers::AttributeArgs;
use lazy_static::lazy_static;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, parse_quote, ItemFn, FnArg, Type};

pub fn attr_macro_no_args_core(args: TokenStream, input: TokenStream) -> TokenStream {
    // let args_parsed: AttributeArgs = parse_quote!(args);
    if !args.is_empty() {
        return quote! {
            compile_error!{"This attribute macro does not take any arguments"};
        };
    }

    // proc_macro2 version of "parse_macro_input!(input as ItemFn)"
    let item_fn = match parse2::<ItemFn>(input) {
        Ok(syntax_tree) => syntax_tree,
        Err(error) => return error.to_compile_error(),
    };

    quote!(#item_fn)
}

#[cfg(test)]
mod tests {
    use crate::macro_types::proc_macro_attribute::macro_attr_no_args::attr_macro_no_args_core;
    use crate::test_commons::test_commons::assert_tokens_eq;
    use quote::quote;

    #[test]
    fn attr_macro_no_params_no_function() {
        let _fun = attr_macro_no_args_core(quote!(), quote!());
        let actual = quote!(compile_error! { "unexpected end of input, expected `fn`" });

        assert_tokens_eq(&_fun, &actual)
    }

    #[test]
    fn attr_macro_with_params_no_function() {
        let attr = quote! {
            example_attribute = "example value"
        };
        let actual =  quote! {
            compile_error!{"This attribute macro does not take any arguments"};
        };

        let _fun = attr_macro_no_args_core(attr, quote!());

        assert_tokens_eq(&_fun, &actual)
    }

    #[test]
    fn attr_macro_with_params_with_function() {
        let attr = quote! {
            example_attribute = "example value"
        };
        let actual = quote! {
            compile_error!{"This attribute macro does not take any arguments"};
        };
        let function = quote! {
            fn example_function() {}
        };

        let _fun = attr_macro_no_args_core(attr, function);

        assert_tokens_eq(&_fun, &actual)
    }

    #[test]
    fn attr_macro_no_params_with_function() {
        let function = quote! {
            fn example_function() {}
        };
        let actual = function.clone();

        let _fun = attr_macro_no_args_core(quote!(), function);

        assert_tokens_eq(&_fun, &actual)
    }
}
