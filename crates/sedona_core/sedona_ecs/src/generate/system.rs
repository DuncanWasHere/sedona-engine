use crate::parse::SystemDefParamValue;
use crate::{
    CollectedData, SystemDefParam, SystemDefParamReference, fident, write_token_stream_to_file,
};
use itertools::Itertools;
use quote::{format_ident, quote};
use std::collections::HashMap;

pub fn generate_systems(out_dir: &str, include_files: &mut Vec<String>, collected: &CollectedData) {
    const FILE_NAME: &str = "systems.rs";
    
    let mut code_rs = vec![];

    // Distinct groups
    let groups: Vec<_> = collected
        .systems
        .iter()
        .map(|s| &s.group)
        .unique()
        .collect();
    
    for group in groups.iter() {
        let mut calls = vec![];

        let mut call_params: HashMap<(String, String), SystemDefParamReference> = HashMap::new();
        let mut call_params_values: HashMap<(String, String), SystemDefParamValue> = HashMap::new();

        for system in collected
            .systems
            .iter()
            .filter(|s| &s.group == *group)
            .sorted_by(|a, b| a.name.cmp(&b.name))
        {
            let mut params_rs = vec![];

            for param in system.params.iter() {
                match param {
                    SystemDefParam::Query(_name) => {
                        params_rs.push(quote! {
                            Query::new(),
                        });
                    }
                    SystemDefParam::Value(value) => {
                        let name = fident!(value.name);

                        params_rs.push(quote! {
                            #name,
                        });

                        let key = (value.name.clone(), value.ty.clone());
                        let item = value.clone();
                        call_params_values.entry(key).or_insert(item);
                    }
                    SystemDefParam::Reference(reference) => {
                        let name = fident!(reference.name);

                        params_rs.push(quote! {
                            #name,
                        });

                        let key = (reference.name.clone(), reference.ty.clone());
                        let item = reference.clone();
                        call_params
                            .entry(key)
                            .and_modify(|e| {
                                if reference.mutable {
                                    e.mutable = true;
                                }
                            })
                            .or_insert(item);
                    }
                }
            }

            let system_function_name = fident!(&system.name);

            calls.push(quote! {
                #system_function_name(#(#params_rs)*);
            })
        }

        let function_name = format_ident!("systems_{}", group);

        // get values of call_params, ignoring the key
        let call_params: Vec<_> = call_params.values().collect();
        let call_params_values: Vec<_> = call_params_values.values().collect();

        // order call_params by name
        let call_params = call_params
            .iter()
            .sorted_by(|a, b| a.name.cmp(&b.name))
            .collect::<Vec<_>>();

        let call_params_values = call_params_values
            .iter()
            .sorted_by(|a, b| a.name.cmp(&b.name))
            .collect::<Vec<_>>();

        let call_params_rs = call_params.iter().map(|r| {
            let name = fident!(r.name);
            let ty = fident!(r.ty);

            if r.mutable {
                quote! {
                   #name: &mut #ty
                }
            } else {
                quote! {

                    #name: &#ty
                }
            }
        });

        let call_params_values_rs = call_params_values.iter().map(|v| {
            let name = fident!(v.name);
            let ty = syn::parse_str::<syn::Type>(&v.ty).expect("invalid type for value param");
            quote! {
                #name: #ty
            }
        });

        let all_call_params_rs = call_params_rs.chain(call_params_values_rs);

        code_rs.push(quote! {
            #[allow(private_interfaces)]
            pub fn #function_name(#(#all_call_params_rs),*) {
                #(#calls)*
            }
        });
    }

    let code_rs = quote! {
        #(#code_rs)*
    };

    include_files.push(write_token_stream_to_file(out_dir, FILE_NAME, &code_rs.to_string()));
}
