// build.rs

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("hello.rs");
    let mut f = File::create(dest_path).unwrap();

    f.write_all(b"
        pub fn message() -> &'static str {
            \"Hello, World!\"
        }
    ").unwrap();

    let mut file = File::open("./src/helpers.rs").expect("Unable to open file");
    // file.write_all(b"
    //     pub fn message() -> &'static str {
    //         \"Hello, World!\"
    //     }
    // ").unwrap();

    let mut src = String::new();
    file.read_to_string(&mut src).expect("Unable to read file");

    let syntax = syn::parse_file(&src).expect("Unable to parse file");
    // p!("{:#?}", syntax);
}