//! # Find Dependencies
//! When you are querying for a particular data structure in code, the definition of that
//! data structure may require other dependencies to compile. To successfully compile,
//! the dependency tree needs to be recursed and all dependencies in this tree need to be
//! identified. Notably, these dependencies can exist in separate cargo crates. 
//!
//! ### Potential Features
//! 1. In find_mod get Items in scope so that dependency analysis can be executed.

// TODO: Develop use case for `pub use crate_name::path::to::mod::{DefName1, DefName2}
// Implementation ideas:
// `pub use mod::...` can be used at the level of any module, but can only be chained across `pub use expressions`
// Meaning, we can call `find_mod` along the module path of `pub use mod::` until either we hit the definition  or another `pub use mod::`
// find_mod needs to be re-worked for this use case? - Not really since we can just call `find_module` at each level of `pub use mod`
// https://doc.rust-lang.org/reference/items/use-declarations.html#use-visibility
// https://doc.rust-lang.org/std/keyword.use.html
// Cases:
// 1. relative paths (need to keep track of `mod.rs` for current module directory
//    a. super
//    b. self
// 2. absolute paths (need to keep track of src_path/lib.rs)
//    a. crate
// 3. `pub use` (module recursion)
//    a. renaming
//    b. `*` export (need to recurse each module of this type since it is not obvious which one exported the definition)
//    c. from another crate
// 4. `pub struct`
// 5. `pub mod::path::to::mod` export module defined in find_mod

// Implementation idea:
// 1. For relative paths, keep track of the current path and update the path as it continues

use syn::{Ident, Item};

#[derive(PartialEq, Copy, Clone)]
pub enum Definition {
    Struct,
    Fn,
    Enum
}

pub fn get_definition<T>() {}

impl Definition {
    pub fn get_definition_name(item: &Item, def_type: Definition) -> Option<(Ident, &Item)> {
        match item {
            Item::Fn(f) =>
                if def_type == Definition::Fn { Some((f.sig.ident.clone(), item)) } else {None},
            Item::Struct(s) =>
                if def_type == Definition::Struct { Some((s.ident.clone(), item)) } else {None},
            Item::Enum(e) =>
                if def_type == Definition::Enum { Some((e.ident.clone(), item)) } else {None},
            _ => None
        }
    }
}

pub fn find_definition_in_scope(scope: Vec<Item>, definition_type: Definition, name: Ident) -> Item {
    scope.iter()
        .filter_map(|item| Definition::get_definition_name(item, definition_type))
        .find(|(ident, item)| ident.to_string() == name.to_string()).unwrap().1.to_owned()
}


#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use maplit::btreemap;
    use quote::quote;
    use syn::{ItemStruct, parse2};
    use crate::helpers::test::mock_file::tests::{mock_file_for_path, random_module_contents, file};
    use crate::r#impl::find_mod::find_mod_in_crate_core;

    pub fn crate_path<'a>() -> &'a Path { Path::new("/crate") }
    pub fn src_path() -> PathBuf { crate_path().join("src") }
    pub fn lib_src_path() -> PathBuf { src_path().join("lib.rs") }

    // #[test]
    // fn get_defintion_example() {
    //     let (expectation, tokens) = random_module_contents();
    //
    //     mock_file_for_path(btreemap! { lib_src_path() => file(quote! { #tokens }) });
    //
    //     find_mod_in_crate_core(crate_path(), module_path);
    // }

    #[test]
    fn pub_use_with_ambiguous_wildcard_exports() {
        let (_, tokens_a) = random_module_contents();
        let (_, tokens_b) = random_module_contents();

        let path : syn::Path = parse2(quote!(crate::Definition)).unwrap();
        mock_file_for_path(btreemap! {
            lib_src_path() => file(quote! {
                mod ambiguous;
                pub use ambiguous::Definition;
            }),
            src_path().join("ambiguous/mod.rs") => file(quote! {
                pub use module_a::*; // Is Result in here?
                pub use module_b::*; // Or in here? Need to check all wild-card exports.
            }),
            src_path().join("ambiguous/module_a.rs") => file(quote! {
                #tokens_a
                pub struct Definition {}
            }),
            src_path().join("ambiguous/module_b.rs") => file(quote! {
                #tokens_b
            })
        });

        let expected: ItemStruct = parse2(quote!(pub struct Definition {})).unwrap();
        // assert_eq!(expected, )
    }

    #[test]
    fn pub_use_with_relative_path_export() {
        let (_, tokens_a) = random_module_contents();

        let path : syn::Path = parse2(quote!(crate::with_relative_path::Definition)).unwrap();
        mock_file_for_path(btreemap! {
            lib_src_path() => file(quote! {
                pub mod with_relative_path;
                mod with_definition;
            }),
            src_path().join("with_relative_path.rs") => file(quote! {
                pub use with_definition::Definition;
            }),
            src_path().join("with_definition.rs") => file(quote! {
                #tokens_a
                pub struct Definition {}
            })
        });

        let expected: ItemStruct = parse2(quote!(pub struct Definition {})).unwrap();
        // assert_eq!(expected, )
    }

    #[test]
    fn pub_use_with_renamed_export() {
        let (_, tokens_a) = random_module_contents();

        let path : syn::Path = parse2(quote!(crate::renamed_export::RenamedDefinition)).unwrap();
        mock_file_for_path(btreemap! {
            lib_src_path() => file(quote! {
                pub mod renamed_export;
                mod with_definition;
            }),
            src_path().join("renamed_export.rs") => file(quote! {
                pub use with_definition::Definition as RenamedDefinition;
            }),
            src_path().join("with_definition.rs") => file(quote! {
                #tokens_a
                pub struct Definition {}
            })
        });

        let expected: ItemStruct = parse2(quote!(pub struct Definition {})).unwrap();
        // assert_eq!(expected, )
    }

    #[test]
    fn pub_use_with_export_from_another_crate() {
        let (_, tokens_a) = random_module_contents();

        pub fn crate_path_other<'a>() -> &'a Path { Path::new("/crate") }
        pub fn src_path_other() -> PathBuf { crate_path_other().join("src") }
        pub fn lib_src_path_other() -> PathBuf { src_path_other().join("lib.rs") }

        // TODO: Add mocks for crate paths here

        let path : syn::Path = parse2(quote!(crate::with_relative_path::Definition)).unwrap();
        mock_file_for_path(btreemap! {
            lib_src_path() => file(quote! {
                pub use another_crate::Definition;
            }),
            lib_src_path_other().join("with_relative_path.rs") => file(quote! {
                pub struct Definition {};
            }),
        });

        let expected: ItemStruct = parse2(quote!(pub struct Definition {})).unwrap();
        // assert_eq!(expected, )
    }
}