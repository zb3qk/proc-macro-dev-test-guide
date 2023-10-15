use serde::{Deserialize, Serialize};

mod nested_directory;

pub struct PublicExampleStruct {}

#[derive(Clone, Copy)]
pub struct ExampleStructWithStdAttributeMacros {}

#[derive(Serialize, Deserialize)]
pub struct ExampleStructWithNonStdAttributeMacros {}

pub mod nested_in_lib {
    pub struct PublicExampleStruct {}
    struct PrivateExampleStruct {}

    pub fn public_example_function() {}

    mod deeply_nested {
        pub struct PublicExampleStruct {}
    }
}
