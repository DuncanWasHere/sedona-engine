extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitStr, Token, parse_macro_input};

struct GlobalKeyInput {
    entries: Vec<(Ident, LitStr)>,
}

impl Parse for GlobalKeyInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut entries = Vec::new();
        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            input.parse::<Token![=>]>()?;
            let lit: LitStr = input.parse()?;
            entries.push((ident, lit));

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(Self { entries })
    }
}

#[proc_macro]
pub fn define_global_keys(input: TokenStream) -> TokenStream {
    let GlobalKeyInput { entries } = parse_macro_input!(input as GlobalKeyInput);

    let count = entries.len();

    let consts = entries.iter().map(|(ident, lit)| {
        quote! {
            pub const #ident: u64 = ahash_macro::hash_literal!(#lit);
        }
    });

    let count_const = quote! {
        pub const KEY_COUNT: usize = #count;
    };

    let expanded = quote! {
        #(#consts)*
        #count_const
    };

    TokenStream::from(expanded)
}
