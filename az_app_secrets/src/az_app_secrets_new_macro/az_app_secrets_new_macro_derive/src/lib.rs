extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Data,
    DataStruct,
    Fields,
};

#[proc_macro_derive(AzAppSecretsNew)]
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
        #[async_trait]
        impl GetFromKeyVault for #struct_name {}
        impl AzAppSecretsNew for #struct_name {
            fn new() -> Self {
                Self {
                    #(#prop_new_q)*
                }
            }
        }
    }.into()
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    // let name: &Ident = &ast.ident;

    // println!("{}", name);

    // // The data could be a Struct, Enum or Union, we
    // //  only want to work on Struct
    // let data: &Data = &ast.data;

    // match &data {
    //     Struct(data_struct) => {
    //         match &data_struct.fields {
    //             Named(fields_named) => {
    //                 for field in fields_named.into_iter() {
    //                     println!("{}", field.ident);
    //                 }

    //                 println!("{}", fields_named.named.ident);
    //                 let gen = quote!(
    //                     impl HelloMacro for #name {
    //                         fn hello_macro() {
    //                             println!("#name {}", stringify!(#name));
    //                             // for field in #fields_named.named {
    //                             //     println!("{}", stringify!(field.ident));
    //                             // }
    //                         }
    //                     }

    //                 ).into();
    //                 return gen
    //             }
    //             _ => return quote! {#ast}.into()
    //         }
    //     }
    //     _ => {
    //         return quote! {#ast}.into()
    //     }

    // }

    // match &mut ast.data {
    //     syn::Data::Struct(ref mut struct_data) => {  
    //         let mut item_struct = parse_macro_input!(input as ItemStruct);
    //         let _ = parse_macro_input!(args as parse::Nothing);         
    //         match &mut struct_data.fields {
    //             syn::Fields::Named(fields) => {
    //                 fields
    //                     .named
    //                     .push(syn::Field::parse_named.parse2(quote! { pub a: String }).unwrap());
    //             }   
    //             _ => {
    //                 ()
    //             }
    //         }              
            
    //         return quote! {#ast}.into();
    //     }
    //     _ => panic!("`add_field` macro has to be used with structs "),
    // }

    // let gen = quote! {
    //     impl HelloMacro for #name {
    //         fn hello_macro() {
    //             println!("#name {}", stringify!(#name));
    //         }
    //     }
    // };
    // dbg!(&gen);
    // return gen.into()
}

// #[macro_export]
// macro_rules! add_app_variables_new {
//     (pub struct $name:ident { $(pub $prop_name:ident : $prop_type:ty),* $(,),* }) => {
//         pub struct $name {
//             $(pub $prop_name : $prop_type),*
//         }

//         impl $name {
//             pub fn new() -> $name {
//                 $name {
//                     $($prop_name : "".to_string()),*
//                 }
//             }
//         }
//     }
// }