use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

macro_rules! impl_varying_fields {
    ($ty:ident { $($field:ident),+ $(,)? }) => {
        
        impl Interpolable for $ty {
            fn interpolate(p:[&Self;3], w:[f64; 3]) -> Self {
                Self {
                    $(
                        $field: &p[0].$field * w[0] + &p[1].$field * w[1] + &p[2].$field * w[2],
                    ) +
                }
            }
        }

    }
}


#[proc_macro_derive(Interpolable)]
pub fn derive_interpolable(input:TokenStream) {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = match input.data {
        Data::Struct(ref s) => match &s.fields {
            Fields::Named(ref f) => f.named.iter().map(|f| f.ident.clone().unwrap()).collect::<Vec<_>>(),
            _ => panic!("Interpolable can only be derived for structs with named fields"),
        },
        _ => panic!("Interpolable can only be derived for structs"),
    };
    let expanded = quote! {
        impl_varying_fields(#name{ #( #fields , )* })
    };
    TokenStream::from(expanded);
}