extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Data,
    DataStruct,
    Fields,
};

#[proc_macro_derive(New)]
pub fn macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    //  that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_macro(ast)
}

fn impl_macro(ast: syn::DeriveInput) -> TokenStream {
    // https://docs.rs/syn/1.0.109/syn/struct.DeriveInput.html
    
    let struct_name = ast.ident;
    
    let fields = match ast.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => fields.named,
        _ => panic!("this derive macro only works on structs with named fields"),
    };

    let prop_new_q = fields.into_iter().map(|f| {
        let field_name = f.ident;

        quote! {
            #field_name: "".to_string(),
        }
    });

    quote! {
        impl GetFromEnv for #struct_name {}
        impl New for #struct_name {
            fn new() -> Self {
                Self {
                    #(#prop_new_q)*
                }
            }
        }
    }.into()
}
