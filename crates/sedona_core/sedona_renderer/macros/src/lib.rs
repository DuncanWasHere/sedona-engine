extern crate proc_macro;

mod uniform;

use proc_macro::TokenStream;

#[proc_macro_derive(Uniform)]
pub fn derive_uniform(input: TokenStream) -> TokenStream {
    uniform::derive_uniform(input)
}
