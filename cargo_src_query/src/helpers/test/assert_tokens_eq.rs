#[cfg(test)]
pub mod test_commons {
    use proc_macro2::TokenStream;

    /// [1] https://github.com/CarlKCarlK/anyinput/blob/d59d04146a8154015e21e2aaf924ee6140393073/anyinput-core/src/tests.rs#L31
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
}