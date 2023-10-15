
#[cfg(test)]
pub mod tests {
    use std::collections::BTreeMap;
    use std::path::PathBuf;
    use anyhow::Error;
    use mocktopus::mocking::{Mockable, MockResult};
    use proc_macro2::{Ident, TokenStream};
    use quote::{quote, ToTokens};
    use rand::Rng;
    use syn::{Item, parse2, parse_str};
    use uuid::Uuid;
    use crate::r#impl::find_mod::parse_file_from_path;

    pub fn mock_file_for_path(path_to_file: BTreeMap<PathBuf, syn::File>) {
        parse_file_from_path.mock_safe(move |path| {
            match path_to_file.get(path) {
                Some(file) => MockResult::Return(Ok(file.clone())),
                None => MockResult::Return(Err(Error::msg(format!(
                    "Error generated from Mocking function. \
                    Path `{}` does not exist map of paths. \
                    Map of path to files: \
                    {:#?}",
                    path.to_str().unwrap(), path_to_file
                ))))
            }
        });
    }

    // Rust Construct Generators
    pub fn random_struct() -> TokenStream {
        let uuid = "name_".to_string() + &*Uuid::new_v4().to_string().replace('-', "_");
        let ident = Ident::new(&uuid, proc_macro2::Span::call_site());
        quote!( struct #ident {} ).into()
    }

    pub fn random_module_contents() -> (Vec<Item>, TokenStream) {
        let mut rng = rand::thread_rng();
        let num_items = rng.gen_range(2..10);
        let mut tokens: Vec<String> = vec![];
        for i in 0..num_items { tokens.push(random_struct().to_string()) };
        let tokens = tokens.join("\n");
        println!("{:#?}", tokens);
        let file: syn::File = parse_str(&tokens).unwrap();
        // TODO: Fix this ... Will make tests clearer
        // let items = tokens.iter().map(|s| parse2(quote!(
        //     struct #s {}
        // )).unwrap()).collect();
        let tokens = file.to_token_stream();
        return (file.items, tokens.into())
    }

    pub fn file(tokens: TokenStream) -> syn::File {
        parse2(tokens.into()).unwrap()
    }
}