#[cfg(test)]
mod tests {
    use derive::{attr_macro_no_args, clone_def, macro_reading_build_output, make_answer};

    #[test]
    fn it_works() {
        make_answer!();

        assert_eq!(answer(), 42);
    }

    #[test]
    fn boop() {
        // #[attr_macro_no_args]
        // struct boop {}
    }

    #[test]
    fn op() {
        #[attr_macro_no_args]
        fn boop() {}
    }

    macro_reading_build_output!();


    #[test]
    fn test_read_build_output_from_macro() {
        message();
    }

    clone_def!(struct  as Booper);

    #[test]
    fn clone_def_macro() {
        let boop = Booper {};
    }
}