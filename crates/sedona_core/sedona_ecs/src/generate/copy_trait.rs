use crate::{CollectedData, fident, write_token_stream_to_file};
use quote::{format_ident, quote};

pub fn generate_copy_traits(
    out_dir: &str,
    include_files: &mut Vec<String>,
    collected: &CollectedData,
) {
    const FILE_NAME: &str = "copy_traits.rs";
    
    let mut code_rs = vec![];

    code_rs.push(quote! {});

    for query in collected.queries.iter() {
        let mut data_types = vec![];

        for field in query.mut_fields.iter() {
            let field_data_type = fident!(field);
            data_types.push(quote! {
                &mut #field_data_type
            });
        }

        for field in query.const_fields.iter() {
            let field_data_type = fident!(field);
            data_types.push(quote! {
                &#field_data_type
            });
        }

        if data_types.len() > 1 {
            code_rs.push(quote! {
                impl Copy for Query<(#(#data_types),*)> {}
            });
        } else if let Some(data_type) = data_types.first() {
            code_rs.push(quote! {
                impl Copy for Query<#data_type> {}
            });
        }
    }

    let code_rs = quote! {
        #(#code_rs)*
    };

    include_files.push(write_token_stream_to_file(out_dir, FILE_NAME, &code_rs.to_string()));
}
