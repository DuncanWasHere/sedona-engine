extern crate proc_macro;

mod event;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, parse_macro_input};

#[proc_macro_attribute]
pub fn entity(_: TokenStream, input: TokenStream) -> TokenStream {
    let input_struct = parse_macro_input!(input as ItemStruct);

    quote! {
        #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
        #input_struct
    }
    .into()
}

#[proc_macro_attribute]
pub fn component(_: TokenStream, input: TokenStream) -> TokenStream {
    let input_struct = parse_macro_input!(input as ItemStruct);

    quote! {
        #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
        #input_struct
    }
    .into()
}

#[proc_macro_attribute]
pub fn system(_: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro]
pub fn create_event_structs(input: TokenStream) -> TokenStream {
    event::create_event_structs(input)
}
