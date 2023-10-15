extern crate proc_macro;
use proc_macro::TokenStream;
use macro_core::macro_types::custom_syntax::clone_def_syntax::clone_def as core_clone_def;
use macro_core::macro_types::proc_macro_attribute::macro_attr_no_args::attr_macro_no_args_core;
use macro_core::macro_types::proc_macro_attribute::macro_attr_with_args::attr_macro_with_args_core;
use macro_core::macro_types::proc_macro_read_build_file::proc_macro_read_build_file;


#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

#[proc_macro_attribute]
pub fn attr_macro_with_args(args: TokenStream, input: TokenStream) -> TokenStream {
    attr_macro_with_args_core(args.into(), input.into()).into()
}

#[proc_macro]
pub fn macro_reading_build_output(item: TokenStream) -> TokenStream {
    proc_macro_read_build_file().into()
}

/// Defines a function specific attribute macro where the attribute explicitly takes no parameters.
///
/// # Example
///
/// ## Compilation Failure examples
///
/// Expecting a Function
/// ```compile_fail
/// #[attr_macro_no_args]
/// struct example {}
/// ```
///
///  No arguments should be passed
/// ```compile_fail
/// #[attr_macro_no_args("boop")]
/// fn example () {}
/// ```
///
/// ## Expected use case example
/// ```
/// use derive::attr_macro_no_args;
/// #[attr_macro_no_args]
/// fn example () {}
/// ```
#[proc_macro_attribute]
pub fn attr_macro_no_args(args: TokenStream, input: TokenStream) -> TokenStream {
    attr_macro_no_args_core(args.into(), input.into()).into()
}

#[proc_macro]
pub fn clone_def(tokens: TokenStream) -> TokenStream {
    core_clone_def(tokens.into()).into()
}
