#[cfg(test)]
mod tests {
    use crate::assert_tokens_eq;
    use macro_core::attr_macro_no_args;
    use quote::quote;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic]
    fn attr_macro_no_params_no_function() {
        let _fun = attr_macro_no_args(quote!(), quote!());
    }

    #[test]
    #[should_panic]
    fn attr_macro_no_params() {
        let attr = quote! {
            ndk_glue = "my::re::exported::ndk_glue"
        };

        let _fun = attr_macro_no_args(
            attr,
            quote!(
                fn boop() {}
            ),
        );
    }

    #[test]
    fn attr_macro_with_params() {
        let _fun = attr_macro_no_args(quote!(), quote!());
        let actual = quote!();

        assert_tokens_eq(&_fun, &actual)
    }

    include!(concat!(env!("OUT_DIR"), "/hello.rs"));
    #[test]
    fn build_test() {
        message()
    }
}

use proc_macro2::TokenStream;
pub fn assert_tokens_eq(expected: &TokenStream, actual: &TokenStream) {
    let expected = expected.to_string();
    let actual = actual.to_string();

    if expected != actual {
        println!(
            "{}",
            colored_diff::PrettyDifference {
                expected: &expected,
                actual: &actual,
            }
        );
        println!("expected: {}", &expected);
        println!("actual  : {}", &actual);
        panic!("expected != actual");
    }
}
