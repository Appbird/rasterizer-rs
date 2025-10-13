use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(Interpolate)]
pub fn derive_interpolable(input:TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = match input.data {
        Data::Struct(ref s) => match &s.fields {
            Fields::Named(f) => f.named.iter().map(|f| f.ident.clone().unwrap()).collect::<Vec<_>>(),
            _ => panic!("Interpolableは名前付きフィールドしか受け取らないけど、なんか変なもの入れてない？"),
        },
        _ => panic!("Interpolableは構造体しか受け取らんよ～他のもの入れてない？"),
    };
    let expanded = quote! {
        impl Interpolable for #name {
            fn interpolate(p:&[&Self;3], w:&[f64; 3]) -> Self {
                Self {
                    #(
                        #fields: &p[0].#fields * w[0] + &p[1].#fields * w[1] + &p[2].#fields * w[2],
                    )*
                }
            }
        }
    };
    TokenStream::from(expanded)
}