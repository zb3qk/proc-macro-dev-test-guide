
//! These are Contract tests [1] which specify the dependencies with Cargo, the filesystem interface,
//! and Rust Spec that this library depends on. If any test in this file fails, then parts of
//! this crate potentially needs to be re-architected.
#[cfg(test)]
mod tests {
    use cargo_metadata::{CargoOpt, MetadataCommand};

    pub const CARGO_ENV_VAR: &str = "CARGO_MANIFEST_DIR";
    pub const INTEGRATION_TEST_CRATE: &str = "integration_test_crate";

    #[test]
    fn cargo_env_variable_is_correct() {
        let env_var = &std::env::var(CARGO_ENV_VAR).unwrap();
        println!("`{CARGO_ENV_VAR}` Environment variable: {env_var}");

        let path =  std::path::Path::new(env_var.as_str());
        let crate_name = path.iter().last();
        let crate_name = crate_name.unwrap_or_else(||
            panic!("The environment variable had no defined path! \n
                   `{CARGO_ENV_VAR}` Environment variable: {env_var}")
        );

        let expected_crate_name = "cargo_src_query";
        assert_eq!(crate_name, expected_crate_name);
    }

    #[test]
    fn cargo_metadata_contains_integration_test_crate() {
        let metadata = MetadataCommand::new()
            .manifest_path("./Cargo.toml")
            .features(CargoOpt::AllFeatures)
            .exec()
            .unwrap();
        let metadata = format!("{metadata:#?}");
        let test_crate_exists = metadata.contains(INTEGRATION_TEST_CRATE);
        assert!(test_crate_exists)
    }
}