use syn::{Item, ItemUse, UseTree};
use crate::errors::CargoQueryError;
use crate::helpers::module_path::ModulePath;
use crate::r#impl::find_mod::FindModuleContext;

fn get_exports_from_item_scope(items: Vec<Item>) -> Vec<ItemUse> {
    items.iter().filter(|&item| matches!(item, Item::Use(_))).map(|item| match &item {
        Item::Use(module) => module.to_owned(),
        _ => unreachable!()
    }).collect()
}

fn find_export_from_scope(exports: Vec<ItemUse>,  query_context: &FindModuleContext) -> Option<ItemUse> {
    unimplemented!()
    // exports.iter().find(|item| item.ident == module_name.ident).cloned()
}

enum ExportPath {
    Direct(ModulePath),
    Wildcard(ModulePath),
}

// Return Vec<Iter<&PathSegment>> instead of () so that parent function can have something to work with
fn recurse_export_path<'a>(tree: &UseTree, mut current_module_path: ModulePath) -> Result<(), CargoQueryError> {
    let mut next_module_ref = current_module_path.next().unwrap();
    let next_module = next_module_ref.clone();
    let parsed_export = match tree {

        // A single threaded export path:
        // ex: `use crate::module::submodule::...`
        UseTree::Path(p) => // continue iterating through path
            if p.ident == next_module { recurse_export_path(&p.tree, current_module_path) }
            else { Err(CargoQueryError::generic_error()) },

        // A single export leaf module.
        // ex: `use crate::module::submodule::leaf_module;`
        UseTree::Name(p) =>  if p.ident == next_module { unimplemented!() // navigate to definition
        } else { Err(CargoQueryError::generic_error()) },

        // A single export leaf module which has been renamed.
        // ex: `use crate::module::submodule::leaf_module as renamed_leaf_module;`
        UseTree::Rename(p) => if p.rename == next_module { // navigate to definition and add original name to start of path
            // This value will remain changed even when doing a back-recursion from a Group check.
            // This is not ok, since if there is a failure, this will taint the module_path for the rest of the recursion.
            // Should we clone instead here? Only return the module_path if the recursion is successful?
            // This actually may not be a problem since there will always be a success if the Rename is the last item in the export path.
            // *next_module_ref = &PathSegment::from(p.ident.clone());
            // let cloned_current_module_path: Vec<&PathSegment> = current_module_path.cloned().collect();
            // ExportPath::Direct(cloned_current_module_path.into());
            Ok(())
        } else { unimplemented!() },

        // A wild card export path.
        // ex: `use crate::module::submodule::*`
        UseTree::Glob(p) => unimplemented!(), // wild card recursion

        // A group of export paths.
        // ex: `use crate::module::submodule::{leaf_module, leaf_module2, submodule2::*};`
        UseTree::Group(p) => {
            // let forked_export_paths: Vec<()> = p.items.iter()
            //     .filter_map(|tree| recurse_export_path(tree, current_module_path).ok())
            //     .collect();
            // match forked_export_paths.len() {
            //     // No path is valid
            //     0 => Err(CargoQueryError::generic_error()),
            //     // A single path is valid
            //     1 => Ok(()),
            //     // More than 1 path is valid, which does not make sense
            //     _ => Err(CargoQueryError::generic_error())
            // }
            Ok(())
        }
    };
    // current_module_path.advance_back_by(1).expect("TODO: panic message");
    parsed_export
}

fn find_export(scope_content: Vec<Item>, query_context: FindModuleContext) -> Result<(), CargoQueryError> {
    let exports = get_exports_from_item_scope(scope_content);
    let export = find_export_from_scope(exports, &query_context).unwrap();

    // Always make sure original query_context module path can be accessed
    recurse_export_path(&export.tree, query_context.current_module_path)
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use maplit::btreemap;
    use quote::quote;
    use syn::parse2;
    use crate::helpers::test::initialize::test::initialize;
    use crate::helpers::test::mock_file::tests::{mock_file_for_path, random_module_contents, file};

    pub fn crate_path<'a>() -> &'a Path { Path::new("/example") }
    pub fn src_path() -> PathBuf { crate_path().join("src") }
    pub fn lib_src_path() -> PathBuf { src_path().join("lib.rs") }

    fn leaf_module() {
        initialize();
        let (expectation, tokens) = random_module_contents();

        mock_file_for_path(btreemap! {
            lib_src_path() => file(quote!{
                mod foo { #tokens_foo }
                mod bar { #tokens_bar }
            })
        });

        let mod_path: syn::Path = parse2(quote! { foo }).unwrap();

    }
}