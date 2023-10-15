#![feature(proc_macro_diagnostic)]
#![feature(exact_size_is_empty)]
#![feature(iter_advance_by)]
//!
//!
//!
//!
//!


use quote::quote;
use quote::spanned::Spanned;
use syn::{Ident, Item, parse2, Path};
pub use crate::core::{Crate, Query, QueryType};
use crate::core::query_cargo_src_core;
use crate::errors::CargoQueryError;
use crate::flags::{Flags};
use proc_macro2::Span;
use proc_macro2::TokenStream;

pub mod flags;
mod core;
pub mod errors;
pub mod helpers;
mod macros;
mod bench;
mod r#impl;
mod logger;

// TODO: Use https://github.com/bheisler/criterion.rs for benchmarking
// TODO: Set up logger. Loggers can be defined per scope (so we can have a library specific logger according to ChatGPT)
// Logger Info: https://stackoverflow.com/questions/61810740/log-source-file-and-line-numbers
// TODO: Set up tracker which generates a Github issue each time a Rust Reference file is updated to review specification updates

// TODO: Feature 1: () Dependency list for each queried item (e.g. struct, enum, function, etc.)
// TODO: Feature 2: (Comprehensive) Expand files with macros using cargo-expand (before parsing) - Macros are very freaky and can affect the AST fundamentally. Meaning the only AST output we can trust is the one from cargo-expand.
// TODO: Cache files/queries to avoid re-parsing the same file/request multiple times

// TODO: Use rust-analyzer instead of implementing everything ourselves: ex. (https://rust-analyzer.github.io/manual.html#go-to-definition) - use this to find definitions

pub fn get_module(flags: Flags, crate_name: Crate, module_path: Path) -> Result<Vec<Item>, CargoQueryError> {
    query_cargo_src_core(flags, Query {
        crate_name,
        module_path,
        query_type: QueryType::Mod,
    })
}

pub fn get_definition(flags: Flags, crate_name: Crate, module_path: Path, definition_name: Ident) {}

fn boop() {
    let token_stream: proc_macro2::TokenStream = quote! {
        pub struct Definition {}
    }.into();

    let item: Item = parse2(token_stream.into()).unwrap();
    item.__span().source_file().path();
}