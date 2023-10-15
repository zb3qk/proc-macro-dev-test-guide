//! # Find Modules
//! The following is based on [the Rust Specification for Modules](https://doc.rust-lang.org/reference/items/modules.html).
//! A module path of the form `crate_name::path::to::module` with the type `syn::path::Path` maps directly to
//! scoped rust code which contains different data structures, definitions, and implementation.
//! Using the file path to the `src` of the cargo crate, we can examine rust code as it was written.
//! In terms parsing, there are 3 different ways to define a module:
//! 1. As the the lib.rs file
//! 2. As a mod.rs file
//! 3. As a file adjacent to the lib.rs file a mod.rs file
//! 4. Inline within a file or within another `mod` definition
//!
//! ## Module Type Examples
//!
//!
//! ### lib.rs
//! ```text
//! .
//! ├── *lib.rs*
//! └── Cargo.toml
//! ```
//!
//! ### mod.rs
//! ```text
//! .
//! ├── lib.rs
//! ├── mod_name
//! │    └── *mod.rs*
//! └── Cargo.toml
//! ```
//!
//! ### file_mod_name.rs
//!
//! ```text
//! .
//! ├── lib.rs
//! ├── *file_mod_next_to_lib.rs*
//! ├── mod_name
//! │    ├── mod.rs
//! │    └── *file_mod_name.rs*
//! └── Cargo.toml
//! ```
//!
//! ### Inline within a File
//!
//! ```
//! // lib.rs
//! mod name {
//!     mod another_module {}
//! }
//! ```
//!


use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use mocktopus::macros::mockable;
use syn::{Attribute, Item, ItemMod, ItemUse, PathSegment, token, UseTree};
use anyhow::Result;
use log::{debug, error, info, trace};
use log_derive::logfn;
use maplit::btreemap;
use proc_macro2::Ident;
use quote::__private::ext::RepToTokensExt;
use syn::token::Brace;
use crate::errors::CargoQueryError;
use quote::ToTokens;
use syn::punctuated::{Iter, IterMut};
use crate::errors::message_identifier::{MODULE_PATH, SOURCE_PATH};
use crate::errors::{IntoProcMacroError, AddMessages};
use crate::helpers::module_path::ModulePath;
use crate::r#impl::external::parse_file_from_path;

// TODO: Add visibility level checks at each level of module recursion based on visibility flag
// TODO: Implement this use case: https://doc.rust-lang.org/reference/items/modules.html#the-path-attribute
// TODO: Implement pub use crate::path::to::module (may be worth a separate file to parse this case)

pub type ModuleContents = Vec<Item>;
pub struct ModuleContentsTemp {
    /// Rust code defined within a given module verbatim.
    contents: Vec<Item>,
    /// Dependencies which are accessible by the module. The definitions do not need to
    /// strictly exist within the contents of the module. These can exist in parent contexts
    /// within the same file.
    accessible_dependencies: Vec<syn::Path>
}

#[derive(Debug)]
pub struct FindModuleContext<'a> {
    crate_path: &'a std::path::Path,
    current_directory_path: &'a std::path::Path,
    pub current_module_path: ModulePath
}

impl<'a: 'new, 'new> FindModuleContext<'a> {
    fn current_module(&self) -> Result<Ident, CargoQueryError> {
        self.current_module_path.get_current_module().ok_or(CargoQueryError::generic_error())
    }

    fn clone_and_update_current_directory_path<>(&self, new_directory_path: &'new std::path::Path) -> FindModuleContext<'new> {
        FindModuleContext {
            crate_path: self.crate_path,
            current_directory_path: new_directory_path,
            current_module_path: self.current_module_path.clone()
        }
    }
}


#[logfn(Trace)]
// #[logfn_inputs(Info)]
pub fn find_mod_in_crate(crate_dir_path: &std::path::Path, module_path: ModulePath) -> Result<ModuleContents, CargoQueryError> {
    let cloned_mod_path = module_path.clone();

    find_mod_in_crate_core(crate_dir_path, module_path).map_err(|e|
        e.add_messages(btreemap! {
            // This may be better suited for the top level
            SOURCE_PATH => crate_dir_path.to_str().expect("The source path does not contain valid unicode.").to_string(),
            MODULE_PATH => cloned_mod_path.to_string()
        })
    )
}

pub fn find_mod_in_crate_core(crate_dir_path: &std::path::Path, mut module_path: ModulePath) -> Result<ModuleContents, CargoQueryError> {
    // Add `src` to `src_path` since all source files exist under the `src` directory
    // TODO: Need to fact check this rule ...
    let src_path = crate_dir_path.join("src");
    trace!("crate /src path: {src_path:#?}");
    // Set lib.rs as the root of search
    // TODO: Validate: Is this path to lib.rs always correct?
    let lib_path = src_path.join("lib.rs");
    trace!("lib.rs path: {lib_path:#?}");

    let lib_rs = parse_file_from_path(&lib_path).map_err(|e| {
        CargoQueryError::could_not_process_lib_rs(e)
    })?;

    // Get name of module to search for
    let first_module = if let Some(module) = module_path.get_current_module() {
        module
    } else { return Ok(lib_rs.items) }; // If there is no path specified, return the contents of the root (lib.rs)
    trace!("first module: {first_module:#?}");

    // TODO: Implement private edge case. example: `use syn::__private::TokenStream;`

    // Find module in file
    let modules = get_modules_from_item_scope(lib_rs.items);
    let module = find_module_from_scope(modules, &first_module).ok_or(CargoQueryError::generic_error())?;

    let query_context = FindModuleContext {
        crate_path: crate_dir_path,
        current_directory_path: &src_path,
        current_module_path: module_path
    };

    trace!("initial query context: {query_context:#?}");
    // Iterate through file structure to find module
    recurse_module_hierarchy(module, query_context)
}

fn find_next_module_definition(scope_content: Vec<Item>, query_context: FindModuleContext) -> Result<ModuleContents, CargoQueryError> {
    let modules = get_modules_from_item_scope(scope_content);
    let module = find_module_from_scope(modules, &query_context.current_module()?).unwrap();

    recurse_module_hierarchy(module, query_context)

    // TODO: Extract this into core ...
}



fn find_next_module(scope_content: Vec<Item>, query_context: FindModuleContext) -> Result<ModuleContents, CargoQueryError> {
    let modules = get_modules_from_item_scope(scope_content);
    let module = find_module_from_scope(modules, &query_context.current_module()?).unwrap();

    return recurse_module_hierarchy(module.to_owned(), query_context)

    // TODO: Extract this into core ...
}

///
/// If a module cannot be found in the above circumstances, then the module may exist as
/// an export with the `use crate::module_name` syntax.
fn recurse_dependencies(scope_content: Vec<Item>, query_context: FindModuleContext) {
    // TODO: Errors vs. Options? Are these errors needed?
    // test recursion for module hierarchy
    find_next_module_definition(scope_content, query_context);
    // test recursion for `use` exports
    // find_export(scope_content.clone(), query_context.clone());
    // test recursion for wildcard `use` exports

    unimplemented!()
}

/// There are 3 implementation scenarios a module can exist in:
/// 1. A module `name` can be implemented in a `mod.rs` file which exists in directory `name`
/// 2. A module can be implemented inline, within a file of the form `mod name { ... }`
/// 3. A module can be implemented in a file adjacent to its parent's `mod.rs` called `name.rs`
///
/// Here are a few assumptions based on the Rust specification [1]
/// 1. A given `mod.rs` file defines each of that module's children
/// 2. If a module path does not fit the above criteria, then the module is misconfigured, or does
/// not exist.
///
/// [1] https://doc.rust-lang.org/reference/items/modules.html
fn recurse_module_hierarchy(module: ItemMod,
                            query_context: FindModuleContext) -> Result<ModuleContents, CargoQueryError> {
    let FindModuleContext { current_directory_path,  .. } = &query_context;
    let current_module = query_context.current_module()?;

    debug!("query context: {query_context:#?}");
    let module_content = module.content;
    // Module implementation is scoped with it's definition: `mod name {}`
    if let Some(m) = module_content {
        debug!("Module implementation is scoped with it's definition: `mod name {}`", current_module);
        return recurse_inline_mods(m.1, query_context)
    }

    // Checks if module has `path attribute`
    // if module.attrs.contains(&Attribute {});

    // Module definition refers to a separate implementation: `mod name;`
    match parse_file_from_path(
        &current_directory_path.join(format!("{}.rs", current_module.to_string()))
    ) {
        // Module implementation exists within a file in the directory
        // ../
        //    ├── mod.rs
        //    └── *name.rs*
        Ok(f) => {
            debug!("Module implementation exists within a file in the directory: {}/{}.rs",
                current_directory_path.to_str().unwrap(), current_module.to_string());
            recurse_file(f, query_context)
        },
        // Module implementation within mod.rs in a sub-directory in the current directory
        // ../
        //    ├── mod.rs
        //    └── name
        //         └── mod.rs
        Err(_) => {
            debug!("Module implementation within mod.rs in sub-directory {} in the current directory: {}/mod.rs",
                current_module.to_string(),
                current_directory_path.join(current_module.to_string()).to_str().unwrap());
            recurse_mod_rs(query_context.clone_and_update_current_directory_path(
                &current_directory_path.join(current_module.to_string())
            ))
        }
    }
}

/// Recurses through `mod.rs` to:
/// 1. Determine whether the file is the module being searched for
/// 2. Parse the module hierarchy to find the next child module
fn recurse_mod_rs(mut query_context: FindModuleContext) -> Result<ModuleContents, CargoQueryError> {
    let FindModuleContext { current_directory_path, ref mut current_module_path, .. } = query_context;
    let path_to_mod_rs = current_directory_path.join("mod.rs");

    let mod_rs = parse_file_from_path(&path_to_mod_rs).map_err(
        |e| CargoQueryError::could_not_process_file(&path_to_mod_rs, e)
    )?.items;

    // Final module definition implementation is in `mod.rs`
    let next_mod: Ident = if let Some(module) = next_module(current_module_path) {
        module
    } else { return Ok(mod_rs) };

    // Determine implementation type of next module
    let modules = get_modules_from_item_scope(mod_rs);

    let next_module_content = find_module_from_scope(modules, &next_mod).ok_or(CargoQueryError::generic_error())?;

    recurse_module_hierarchy(next_module_content, query_context)
}

/// Parse a given file to:
/// 1. Determine whether the file is the module being searched for
/// 2. To parse nested modules
fn recurse_file(file: syn::File, mut query_context: FindModuleContext) -> Result<ModuleContents, CargoQueryError> {
    let FindModuleContext { ref mut current_module_path, .. } = query_context;
    let next_mod: Ident = if let Some(module) = next_module(current_module_path) {
        module
    } else { return Ok(file.items) };
    let module = find_module_from_items(file.items, &next_mod)?
        .ok_or(CargoQueryError::could_not_find_module_in_file(&next_mod))?.1;
    recurse_inline_mods(module, query_context)
}

/// Parses nested modules recursively to find the queried module based on module_path
fn recurse_inline_mods(module_contents: Vec<Item>, mut query_context: FindModuleContext) -> Result<ModuleContents, CargoQueryError> {
    let FindModuleContext { ref mut current_module_path, .. } = query_context;
    let next_mod: Ident = if let Some(module) = next_module(current_module_path) {
        module
    } else { return Ok(module_contents) };
    let module = find_module_from_items(module_contents, &next_mod)?
        .ok_or(CargoQueryError::could_not_find_module_in_file(&next_mod))?.1;
    recurse_inline_mods(module, query_context)
}

/// Filters a vector of Rust Items (units of implementation defined by `syn`)
/// to only include module definitions
fn get_modules_from_item_scope(items: Vec<Item>) -> Vec<ItemMod> {
    items.iter().filter(|&item| matches!(item, Item::Mod(_))).map(|item| match &item {
        Item::Mod(module) => module.to_owned(),
        _ => unreachable!()
    }).collect()
}

/// Finds the definition of a module within a given scope. Otherwise, returns an Error.
fn find_module_from_items(item_scope: Vec<Item>, module: &Ident) -> Result<Option<(token::Brace, Vec<Item>)>, CargoQueryError>{
    let modules = get_modules_from_item_scope(item_scope);

    Ok(modules.iter().find(|item| item.ident == module.clone()).ok_or(
        CargoQueryError::could_not_find_defined_module(module))?.to_owned().content)
}

/// Retrieves the next module in the module path and removes it from the path
fn next_module(current_module_path: &mut ModulePath) -> Option<Ident> {
    debug!("Current module path: {:?}", current_module_path);
    let next_module = current_module_path.next();
    debug!("Next module: {:?}", next_module);
    next_module
}

fn find_module_from_scope(modules: Vec<ItemMod>, module_name: &Ident) -> Option<ItemMod> {
    debug!("Finding module: {} in scope: {:?}", &module_name, &modules);
    modules.iter().find(|item| item.ident == module_name.clone()).cloned()
}

#[cfg(test)]
mod test {
    use mocktopus::mocking::{Mockable};
    use test_log::test;

    mod helpers {
        use std::path::{Path, PathBuf};
        use maplit::btreemap;
        use syn::parse2;
        use quote::{quote, ToTokens};
        use crate::helpers::module_path::ModulePath;
        use crate::helpers::test::initialize::test::initialize;
        use crate::helpers::test::mock_file::tests::{mock_file_for_path, random_module_contents, file};
        use crate::r#impl::find_mod::find_mod_in_crate;

        // Constants
        // Paths
        pub fn crate_path<'a>() -> &'a Path { Path::new("/example") }
        pub fn src_path() -> PathBuf { crate_path().join("src") }
        pub fn lib_src_path() -> PathBuf { src_path().join("lib.rs") }

        // TODO: Write failure tests to validate Error messages

        #[test]
        /// ```text
        /// .
        /// ├── *lib.rs*
        /// └── Cargo.toml
        /// ```
        fn DIRECTORY_root_FILE_lib_CONTENTS_root_path() {
            initialize();
            let (expectation, tokens) = random_module_contents();

            mock_file_for_path(btreemap! { lib_src_path() => file(quote! { #tokens }) });

            assert_eq!(find_mod_in_crate(crate_path(), ModulePath::default()).unwrap(), expectation)
        }

        #[test]
        fn DIRECTORY_root_FILE_lib_CONTENTS_no_module() {
            initialize();
            let (_, tokens) = random_module_contents();

            mock_file_for_path(btreemap! { lib_src_path() => file(quote! { #tokens }) });

            let mod_path: syn::Path = parse2(quote! { foo }).unwrap();

            assert!(matches!(find_mod_in_crate(crate_path(), mod_path.into()), Err(_)))
        }

        #[test]
        fn DIRECTORY_root_FILE_lib_CONTENTS_multiple_inline_modules() {
            initialize();
            let (expectation_foo, tokens_foo) = random_module_contents();
            let (expectation_bar, tokens_bar) = random_module_contents();

            mock_file_for_path(btreemap! {
                lib_src_path() => file(quote!{
                    mod foo { #tokens_foo }
                    mod bar { #tokens_bar }
                })
            });

            let mod_path: syn::Path = parse2(quote! { foo }).unwrap();
            assert_eq!(find_mod_in_crate(crate_path(), mod_path.into()).unwrap(), expectation_foo);

            let mod_path: syn::Path = parse2(quote! { bar }).unwrap();
            assert_eq!(find_mod_in_crate(crate_path(), mod_path.into()).unwrap(), expectation_bar);
        }

        #[test]
        fn DIRECTORY_root_FILE_lib_CONTENTS_nested_inline_modules() {
            initialize();
            let (expectation, tokens) = random_module_contents();

            mock_file_for_path(btreemap! {
                lib_src_path() => file( quote! {
                    mod foo {
                        mod bar {
                            #tokens
                        }
                    }
                })
            });

            let mod_path: syn::Path = parse2(quote! { foo::bar }).unwrap();
            assert_eq!(find_mod_in_crate(crate_path(), mod_path.into()).unwrap(), expectation);
        }

        #[test]
        /// ```text
        /// .
        /// ├── lib.rs
        /// ├── *foo.rs*
        /// └── Cargo.toml
        /// ```
        fn DIRECTORY_root_FILE_adjacent_CONTENT_nested_inline_modules() {
            initialize();
            mock_file_for_path(btreemap! {
                lib_src_path() => file(quote!( mod foo; )),
                src_path().join("foo.rs") => file(quote! {

                })
            });

            let mod_path: syn::Path = parse2(quote! { foo }).unwrap();
        }

        #[test]
        /// ```text
        /// .
        /// ├── lib.rs
        /// ├── foo/
        /// │   └── *mod.rs*
        /// └── Cargo.toml
        /// ```
        fn DIRECTORY_nested_FILE_mod_CONTENT_nested_modules() {
            initialize();
            let (expectation, tokens) = random_module_contents();
            mock_file_for_path(btreemap! {
                lib_src_path() => file(quote!( mod foo; )),
                src_path().join("foo/mod.rs") => file(quote!( #tokens )),
            });

            let mod_path: syn::Path = parse2(quote! { foo }).unwrap();
            assert_eq!(find_mod_in_crate(crate_path(), mod_path.into()).unwrap(), expectation)
        }


        #[test]
        /// ```text
        /// .
        /// ├── lib.rs
        /// ├── foo/
        /// │   ├── mod.rs
        /// │   └── *bar.rs*
        /// └── Cargo.toml
        /// ```
        fn DIRECTORY_nested_FILE_adjacent_CONTENT_nested_modules() {
            // initialize();
            let (expectation, tokens) = random_module_contents();
            mock_file_for_path(btreemap! {
                lib_src_path() => file(quote!( mod foo; )),
                src_path().join("foo/mod.rs") => file(quote!( mod bar; )),
                src_path().join("foo/bar.rs") => file(quote!( #tokens )),
            });

            let mod_path: syn::Path = parse2(quote! { foo::bar }).unwrap();
            assert_eq!(find_mod_in_crate(crate_path(), mod_path.into()).unwrap(), expectation)
        }

        /// ```text
        /// .
        /// ├── lib.rs
        /// ├── foo/
        /// │   ├── mod.rs
        /// │   ├── bar.rs
        /// │   └── baz/
        /// │       ├── mod.rs
        /// │       └── *boop.rs*
        /// └── Cargo.toml
        /// ```
        #[test]
        fn DIRECTORY_deeply_nested_FILE_adjacent_CONTENT_nested_modules() {
            initialize();
            let (expectation, tokens) = random_module_contents();
            mock_file_for_path(btreemap! {
                lib_src_path() => file(quote! { mod foo; }),
                src_path().join("foo/mod.rs") => file(quote! {
                    mod hoop { }
                    mod bar;
                    mod baz;
                }),
                src_path().join("foo/baz/mod.rs") => file(quote! {
                    mod boop;
                }),
                src_path().join("foo/baz/boop.rs") => file(quote! {
                    mod loo;
                    mod poo {}
                    mod waldo {
                        mod corge {
                            #tokens
                        }
                    }
                })
            });

            let mod_path: syn::Path = parse2(quote! { foo::baz::boop::waldo::corge }).unwrap();
            assert_eq!(find_mod_in_crate(crate_path(), mod_path.into()).unwrap(), expectation)
        }
    }
}