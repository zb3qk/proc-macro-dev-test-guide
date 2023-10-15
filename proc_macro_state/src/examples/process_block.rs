use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse2, Token};
use syn::parse::{Parse, ParseStream};
use crate::MacroState;

/// Enumeration of supported Block types for this procedural macro
#[non_exhaustive]
enum BlockType {
    Struct(Token![struct]),
    Enum(Token![enum]),
    Function(Token![fn])
}

impl Parse for MacroState<BlockType> {
    fn parse(input: ParseStream) -> syn::Result<Self> {

    }
}

struct ScopedBlock {
    block_type: BlockType,
    name: String
}

impl Parse for MacroState<ScopedBlock> {
    fn parse(input: ParseStream) -> syn::Result<Self> {

    }
}




type ScopedBlockState = MacroState<ScopedBlock>;

/// This is where we would define `#[proc_macro]` or any of the procedural macro variants.
/// For the sake of example and unit testing,
fn macro_ingress(tokens: TokenStream) -> TokenStream {
    let macro_state: ScopedBlockState = parse2(tokens)?;
}

#[cfg(test)]
mod tests {

}