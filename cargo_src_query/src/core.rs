use cargo_metadata::{CargoOpt, MetadataCommand};
use cargo_metadata::camino::Utf8PathBuf;
use quote::quote;
use syn::{Ident, Item, PathSegment, Token};
use syn::spanned::Spanned;
use crate::errors::CargoQueryError;

use crate::flags::Flags;
use crate::r#impl::find_dependencies::Definition;
use crate::r#impl::find_mod::find_mod_in_crate;

// TODO: Update this with correct environment variable
/// Ref: [1] https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
pub(crate) const CARGO_ENV_VAR: &str = "CARGO_MANIFEST_DIR";

pub enum Crate {
    // ex. crate::in::this::crate
    Internal,
    // ex. Ident::in::another::crate
    External(Ident)
}

pub struct Query {
    pub(crate) crate_name: Crate, pub(crate) module_path: syn::Path,
    pub(crate) query_type: QueryType
}

pub enum QueryType {
    Definition(Ident, Definition),
    Mod
}

// TODO: Implement batch search (multiple Items along same path)
// TODO: Implement private edge case. example: `use syn::__private::TokenStream;`
pub fn query_cargo_src_core(flags: Flags, query: Query) -> Result<Vec<Item>, CargoQueryError> {
    let Query { crate_name, module_path, query_type } = query;

    // TODO: Is this env_var the correct way to go about this? Should we be using cargo_metadata instead?
    let env_var = std::env::var(CARGO_ENV_VAR).unwrap();
    let crate_src_path = match crate_name {
        Crate::Internal => env_var,
        Crate::External(src_crate_name) => get_cargo_metadata_src_path(src_crate_name)
    };
    dbg!(&crate_src_path);
    let crate_src_path = std::path::Path::new(crate_src_path.as_str());

    // TODO: Implement logic
    match query_type {
        QueryType::Definition(_, _) => unimplemented!(),
        QueryType::Mod => find_mod_in_crate(crate_src_path, module_path.into())
    }
}


fn get_cargo_metadata_src_path(src_crate: Ident) -> String {
    let src_crate_name = &src_crate.to_string();
    let _metadata = MetadataCommand::new()
        .manifest_path("./Cargo.toml")
        .features(CargoOpt::AllFeatures)
        .exec()
        .unwrap();

    // TODO: Can this be processed more efficiently?
    let package =
        _metadata.packages.iter().find(|&p| p.name.eq(src_crate_name));
    let mut path = match package {
        Some(p) => &p.manifest_path,
        None => {
            crate::errors::invalid_crate_name(src_crate_name.span().into(), &src_crate_name);
            unreachable!("The macro aborts here")
        }
    }.to_owned();
    // Remove `Cargo.toml` from end of src_path
    // Run `cargo metadata` and search for `manifest_path` to see an example of a path
    path.pop();
    path.into_string()
}

#[cfg(test)]
mod tests {
    use cargo_metadata::{CargoOpt, MetadataCommand};

    #[test]
    fn test_targets() {
        let _metadata = &MetadataCommand::new()
            .manifest_path("./Cargo.toml")
            .features(CargoOpt::AllFeatures)
            .exec()
            .unwrap();

        println!("{:#?}", _metadata.packages[0].name);
        println!("{:#?}", _metadata.packages[0].targets);
    }
}