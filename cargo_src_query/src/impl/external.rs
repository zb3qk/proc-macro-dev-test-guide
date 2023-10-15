//! # External
//! This module contains all the external functions that are used by the `cargo_src_query` crate.
//!


use mocktopus::macros::mockable;
use std::fs::File;
use anyhow::Result;
use std::io::Read;
use syn::Path;

#[mockable]
#[allow(clippy::forget_ref)]
#[allow(clippy::forget_copy)]
/// Reads a file based on a file path and converts it into a parseable `syn::File` data structure.
///
/// **Warning**: This code reads the source file from a directory. This operation does not scale
/// very well given reading from HDD is a time consuming operation.
pub fn parse_file_from_path(path: &std::path::Path) -> Result<syn::File> {
    dbg!(&path);
    let mut file = File::open(path)?;
    let mut src = String::new();
    file.read_to_string(&mut src)?;
    Ok(syn::parse_file(&src)?)
}

#[mockable]
#[allow(clippy::forget_ref)]
#[allow(clippy::forget_copy)]
/// Get the path to the crate's implementation directory from it's definition in
/// the given `Cargo.toml` file.
pub fn get_crate_path_from_cargo_toml(cargo_toml_path: &Path) -> Result<std::path::PathBuf> {
    unimplemented!()
}