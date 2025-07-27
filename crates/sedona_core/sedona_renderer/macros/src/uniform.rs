use heck::ToSnakeCase;
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn derive_uniform(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let struct_name_snake = struct_name.to_string().to_snake_case();

    let fields = match &input.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("Uniform can only be derived for named structs"),
    };

    let field_info_entries = fields.iter().map(|f| {
        let name_ident = &f.ident;
        let name_str = name_ident.as_ref().unwrap().to_string();
        let ty = &f.ty;

        quote! {
            sedona_renderer::types::UniformFieldInfo {
                name: #name_str,
                offset: ::core::mem::offset_of!(#struct_name, #name_ident),
                size: ::core::mem::size_of::<#ty>(),
            }
        }
    });

    let expanded = quote! {
        impl sedona_renderer::types::Uniform for #struct_name {
            const SIZE: u64 = size_of::<Self>() as u64;

            fn name() -> &'static str {
                #struct_name_snake
            }

            fn field_info() -> &'static [sedona_renderer::types::UniformFieldInfo] {
                &[
                    #(#field_info_entries),*
                ]
            }
        }
    };

    TokenStream::from(expanded)
}
