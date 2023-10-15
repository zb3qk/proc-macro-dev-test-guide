use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use cargo_metadata::{CargoOpt, MetadataCommand};
use proc_macro2::TokenStream;

pub fn proc_macro_read_build_file() -> TokenStream {
    let out_dir = match env::var("OUT_DIR") {
        Ok(out_dir) => out_dir,
        Err(e) => panic!("Could not find OUT_DIR env variable")
    };
    let dest_path = Path::new(&out_dir).join("hello1.rs");
    let mut file = File::open(dest_path).expect("Unable to open file");
    let mut src = String::new();
    file.read_to_string(&mut src).expect("Unable to read file");
    match src.parse() {
        Ok(val) => val,
        Err(err) => panic!("Could not parse string data")
    }
}

pub fn parse_cargo_metadata() {

}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::File;
    use std::io::Read;
    use cargo_metadata::{CargoOpt, MetadataCommand};

    #[test]
    fn read_file() {
        env::set_var("OUT_DIR", "")
    }

    #[test]
    fn read_cargo_metadata() {
        let _metadata = MetadataCommand::new()
            .manifest_path("./Cargo.toml")
            .features(CargoOpt::AllFeatures)
            .exec()
            .unwrap();
        let src_path = &_metadata.packages.get(0).unwrap()
            .targets.get(0).unwrap()
            .src_path;
        let mut file = File::open(src_path).expect("Unable to open file");

        let mut src = String::new();
        file.read_to_string(&mut src).expect("Unable to read file");

        let syntax = syn::parse_file(&src).expect("Unable to parse file");
        println!("{:#?}", syntax);
        let items = syntax.items;
        println!("{:#?}", items);

    }
}