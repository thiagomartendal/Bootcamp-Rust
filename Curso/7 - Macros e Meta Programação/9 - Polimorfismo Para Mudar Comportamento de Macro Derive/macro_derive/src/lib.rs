use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(MeuTrait)]
pub fn meu_trait_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let expanded = quote! {
        impl MeuTrait for #name {
            fn minha_funcao(&self) -> String {
                String::from("Implementação de MeuTrait")
            }
        }
    };

    TokenStream::from(expanded)
}
