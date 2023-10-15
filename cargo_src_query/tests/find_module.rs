#[cfg(test)]
mod tests {
    use color_eyre::owo_colors::OwoColorize;
    use proc_macro2::Span;
    use quote::quote;
    use syn::{Ident, parse2, Path};
    use cargo_src_query::flags::{DEFAULT_FLAGS, Flags};
    use cargo_src_query::{Crate, get_module};

    pub const CARGO_ENV_VAR: &str = "CARGO_MANIFEST_DIR";
    pub const INTEGRATION_TEST_CRATE: &str = "integration_test_crate";

    #[test]
    fn successfully_find_module_in_integration_test_crate() {
        let module = get_module(DEFAULT_FLAGS, Crate::External(Ident::new(INTEGRATION_TEST_CRATE, Span::call_site().into())),
                                parse2(quote!(nested_directory::another_file)).unwrap())
            .expect("Could not find module");

        let expected = quote!(pub struct ExampleStruct {});
        let expected: syn::File = parse2(expected).unwrap();
        let expected = expected.items;

        assert_eq!(format!("{module:#?}"), format!("{expected:#?}"))
    }
}