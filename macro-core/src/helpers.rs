use std::ops::Deref;
use syn::parse::{Parse, ParseStream, Result};
use syn::Token;
use serde::{Deserialize, Serialize};

/// A newtype for testing
///
/// This needed because AttributeArgs from syn crate is not a newtype and does not implements `Parse` trait
///
/// [1] https://github.dev/jozanza/re-winit/blob/ada5c6929b5c9a31a901aaabaecaaf4c5931e2da/vendor/ndk-macro/src/helper.rs#L12
#[derive(Debug)]
pub struct AttributeArgs(syn::AttributeArgs);

impl Deref for AttributeArgs {
    type Target = syn::AttributeArgs;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Parse for AttributeArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut metas = Vec::new();

        loop {
            if input.is_empty() {
                break;
            }
            let value = input.parse()?;
            metas.push(value);
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
        }

        Ok(Self(metas))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "TopLevelStruct")]
struct DurationDef {
    secs: i64,
    nanos: i32,
}

#[derive(Clone)]
struct TopLevelStruct {
    secs: i64,
    nanos: i32,
}

struct IntermediatStruct {

}

pub struct Flag {
    short: char,
    name: &'static str,
    /* ... */
}

include!(concat!(env!("OUT_DIR"), "/hello.rs"));
// const boop: &str = message();





