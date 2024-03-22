extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Drawable)]
pub fn tsuika_drawable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields = if let Data::Struct(data) = &input.data {
        &data.fields
    } else {
        panic!("Drawable can only be derived for structs");
    };
    let field_names = fields
        .iter()
        .map(|field| field.ident.as_ref().unwrap().to_string())
        .collect::<Vec<_>>();
    let field_count = fields.len();
    let gen = quote! {
        impl Drawable for #name
        where
            Self: std::fmt::Debug + 'static
        {
            fn draw(&self) -> String {
                format!("{:#?}", self)
            }

            /* fn select(&self, index: usize) -> String {
                vec![#(#field_names),*].get(index % #field_count).unwrap().to_string()
            } */
        }
    };
    gen.into()
}

#[proc_macro_derive(Manager)]
pub fn tsuika_manager_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields = if let Data::Struct(data) = &input.data {
        &data.fields
    } else {
        panic!("Manager can only be derived for structs");
    };
    let gen = quote! {
        impl #name {
            fn 
        }
    };

    gen.into()
}
