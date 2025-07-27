use crate::{CollectedData, EntityDef, fident, write_token_stream_to_file};
use heck::ToSnakeCase;
use inflector::string::pluralize::to_plural;
use quote::{format_ident, quote};

pub fn generate_queries(out_dir: &str, include_files: &mut Vec<String>, collected: &CollectedData) {
    const FILE_NAME: &str = "implementations.rs";

    let mut code_rs = vec![];

    code_rs.push(quote! {
        pub trait LenFrom<'a, T>
        where
            T: 'a + Send
        {
            fn len(&'a self) -> usize;
            fn is_empty(&'a self) -> bool {
                self.len() == 0
            }
        }

        pub trait QueryFrom<'a, T>
        where
            T: 'a + Send
        {
            fn query_from(&'a self) -> impl Iterator<Item = T>;
            fn par_query_from(&'a self) -> impl ParallelIterator<Item = T>;
            fn get_from(&'a self, index: usize, entity_type: EntityType) -> Option<T>;
            fn at(&'a self, index: usize) -> Option<T>;
        }

        pub trait QueryMutFrom<'a, T>
        where
            T: 'a + Send
        {
            fn query_mut_from(&'a mut self) -> impl Iterator<Item = T>;
            fn par_query_mut_from(&'a mut self) -> impl ParallelIterator<Item = T>;
            fn get_mut_from(&'a mut self, index: usize, entity_type: EntityType) -> Option<T>;
            fn at_mut(&'a mut self, index: usize) -> Option<T>;
        }
    });

    for query in collected.queries.iter() {
        let mutable = !query.mut_fields.is_empty();

        let matching_entities: Vec<&EntityDef> = collected
            .entities
            .iter()
            .filter(|entity| {
                let mut all_fields_present = true;

                for query_field in query.mut_fields.iter() {
                    if !entity
                        .fields
                        .iter()
                        .any(|entity_field| entity_field.data_type == *query_field)
                    {
                        all_fields_present = false;
                        break;
                    }
                }

                for query_field in query.const_fields.iter() {
                    if !entity
                        .fields
                        .iter()
                        .any(|entity_field| entity_field.data_type == *query_field)
                    {
                        all_fields_present = false;
                        break;
                    }
                }

                all_fields_present
            })
            .collect();

        let mut data_types = vec![];

        for field in query.mut_fields.iter() {
            let field_data_type = fident!(field);
            data_types.push(quote! {
                &'a mut #field_data_type
            });
        }

        for field in query.const_fields.iter() {
            let field_data_type = fident!(field);

            data_types.push(quote! {
                &'a #field_data_type
            });
        }

        let mut match_get_rs = vec![];

        for entity in matching_entities.iter() {
            let entity_name = fident!(entity.name);

            let mut field_quotes = vec![];
            let mut par_field_quotes = vec![];
            let mut get_quotes = vec![];

            for field in query.mut_fields.iter() {
                let field_name = fident!(to_plural(
                    entity
                        .fields
                        .iter()
                        .find(|f| f.data_type == *field)
                        .unwrap()
                        .name
                        .as_str()
                ));

                field_quotes.push(quote! {
                    self.#field_name.iter_mut()
                });

                par_field_quotes.push(quote! {
                    self.#field_name.par_iter_mut()
                });

                get_quotes.push(quote! {
                    self.#field_name.get_mut(index)?
                });
            }
            for field in query.const_fields.iter() {
                let field_name = fident!(to_plural(
                    entity
                        .fields
                        .iter()
                        .find(|f| f.data_type == *field)
                        .unwrap()
                        .name
                        .as_str()
                ));

                field_quotes.push(quote! {
                    self.#field_name.iter()
                });

                par_field_quotes.push(quote! {
                    self.#field_name.par_iter()
                });

                get_quotes.push(quote! {
                    self.#field_name.get(index)?
                });
            }

            let archetype_type = fident!(to_plural(&entity.name));
            let archetype_field_name = fident!(to_plural(&entity.name.to_snake_case()));

            code_rs.push(quote! {
                #[allow(unused_parens)]
                impl<'a> LenFrom<'a, (#(#data_types),*)> for #archetype_type {
                    fn len(&'a self) -> usize {
                        self.entities.len()
                    }
                }
            });

            if mutable {
                code_rs.push(quote! {
                    #[allow(unused_parens, clippy::needless_question_mark, clippy::double_parens)]
                    impl<'a> QueryMutFrom<'a, (#(#data_types),*)> for #archetype_type {
                        fn query_mut_from(&'a mut self) -> impl Iterator<Item = (#(#data_types),*)> {
                            izip!(#(#field_quotes),*)
                        }

                        fn par_query_mut_from(&'a mut self) -> impl ParallelIterator<Item = (#(#data_types),*)> {
                            izip_par!(#(#par_field_quotes),*)
                        }

                        fn get_mut_from(&'a mut self, index: usize, _entity_type: EntityType) -> Option<(#(#data_types),*)> {
                            Some((#(#get_quotes),*))
                        }

                        fn at_mut(&'a mut self, index: usize) -> Option<(#(#data_types),*)>
                        {
                            Some((#(#get_quotes),*))
                        }
                    }
                });

                match_get_rs.push(quote! {
                    EntityType::#entity_name => self.#archetype_field_name.get_mut_from(index, entity_type),
                });
            } else {
                code_rs.push(quote! {
                    #[allow(unused_parens, clippy::needless_question_mark, clippy::double_parens)]
                    impl<'a> QueryFrom<'a, (#(#data_types),*)> for #archetype_type {
                        fn query_from(&'a self) -> impl Iterator<Item = (#(#data_types),*)> {
                            izip!(#(#field_quotes),*)
                        }

                        fn par_query_from(&'a self) -> impl ParallelIterator<Item = (#(#data_types),*)> {
                            izip_par!(#(#par_field_quotes),*)
                        }

                        fn get_from(&'a self, index: usize, _entity_type: EntityType) -> Option<(#(#data_types),*)> {
                            Some((#(#get_quotes),*))
                        }

                        fn at(&'a self, index: usize) -> Option<(#(#data_types),*)>
                        {
                            Some((#(#get_quotes),*))
                        }
                    }
                });

                match_get_rs.push(quote! {
                    EntityType::#entity_name => self.#archetype_field_name.get_from(index, entity_type),
                });
            }
        }

        let sum_args: Vec<_> = matching_entities
            .iter()
            .map(|entity| {
                let property_name =
                    format_ident!("{}", to_plural(&(&entity.name.to_snake_case())));
                quote! { self.#property_name.len() }
            })
            .collect();

        if !sum_args.is_empty() {
            code_rs.push(quote! {
                #[allow(unused_parens, unused_variables, unused_assignments)]
                impl<'a> LenFrom<'a, (#(#data_types),*)> for World {
                    fn len(&'a self) -> usize {
                        sum!(#(#sum_args),*)
                    }
                }
            });
        } else {
            code_rs.push(quote! {
                #[allow(unused_parens, unused_variables, unused_assignments)]
                impl<'a> LenFrom<'a, (#(#data_types),*)> for World {
                    fn len(&'a self) -> usize {
                        0
                    }
                }
            });
        }

        if mutable {
            let chain_args: Vec<_> = matching_entities
                .iter()
                .map(|entity| {
                    let property_name =
                        format_ident!("{}", to_plural(&(&entity.name.to_snake_case())));
                    quote! { self.#property_name.query_mut() }
                })
                .collect();

            let par_chain_args: Vec<_> = matching_entities
                .iter()
                .map(|entity| {
                    let property_name =
                        format_ident!("{}", to_plural(&(&entity.name.to_snake_case())));
                    quote! { self.#property_name.par_query_mut() }
                })
                .collect();

            let at_mut_args: Vec<_> = matching_entities
                .iter()
                .map(|entity| {
                    let property_name =
                        format_ident!("{}", to_plural(&(&entity.name.to_snake_case())));
                    quote! {
                        {
                            let len = self.#property_name.len();
                            if index < len {
                                return self.#property_name.at_mut(index);
                            }
                            index -= len;
                        }
                    }
                })
                .collect();

            code_rs.push(quote! {
                #[allow(unused_parens, unused_variables, unused_assignments)]
                impl<'a> QueryMutFrom<'a, (#(#data_types),*)> for World {
                    fn query_mut_from(&'a mut self) -> impl Iterator<Item = (#(#data_types),*)> {
                        chain!(#(#chain_args),*)
                    }

                    fn par_query_mut_from(&'a mut self) -> impl ParallelIterator<Item = (#(#data_types),*)> {
                        chain_par!(#(#par_chain_args),*)
                    }

                    #[allow(unreachable_patterns, clippy::match_single_binding)]
                    fn get_mut_from(&'a mut self, index: usize, entity_type: EntityType) -> Option<(#(#data_types),*)> {
                        match entity_type {
                            #(#match_get_rs)*
                            _ => None
                        }
                    }

                    #[allow(unused_mut)]
                    fn at_mut(&'a mut self, index: usize) -> Option<(#(#data_types),*)>
                    {
                        let mut index = index;
                        #(#at_mut_args)*
                        None
                    }
                }
            })
        } else {
            let chain_args: Vec<_> = matching_entities
                .iter()
                .map(|entity| {
                    let property_name =
                        format_ident!("{}", to_plural(&entity.name.to_snake_case()));
                    quote! { self.#property_name.query() }
                })
                .collect();

            let par_chain_args: Vec<_> = matching_entities
                .iter()
                .map(|entity| {
                    let property_name =
                        format_ident!("{}", to_plural(&entity.name.to_snake_case()));
                    quote! { self.#property_name.par_query() }
                })
                .collect();

            let at_args: Vec<_> = matching_entities
                .iter()
                .map(|entity| {
                    let property_name =
                        format_ident!("{}", to_plural(&(&entity.name.to_snake_case())));
                    quote! {
                        {
                            let len = self.#property_name.len();
                            if index < len {
                                return self.#property_name.at(index);
                            }
                            index -= len;
                        }
                    }
                })
                .collect();

            code_rs.push(quote! {
                #[allow(unused_parens, unused_variables, unused_assignments)]
                impl<'a> QueryFrom<'a, (#(#data_types),*)> for World {
                    fn query_from(&'a self) -> impl Iterator<Item = (#(#data_types),*)> {
                        chain!(#(#chain_args),*)
                    }

                    fn par_query_from(&'a self) -> impl ParallelIterator<Item = (#(#data_types),*)> {
                        chain_par!(#(#par_chain_args),*)
                    }

                    #[allow(unreachable_patterns, clippy::match_single_binding)]
                    fn get_from(&'a self, index: usize, entity_type: EntityType) -> Option<(#(#data_types),*)> {
                        match entity_type {
                            #(#match_get_rs)*
                            _ => None
                        }
                    }

                    #[allow(unused_mut)]
                    fn at(&'a self, index: usize) -> Option<(#(#data_types),*)>
                    {
                        let mut index = index;
                        #(#at_args)*
                        None
                    }
                }
            })
        }
    }

    let code_rs = quote! {
        #(#code_rs)*
    };

    include_files.push(write_token_stream_to_file(out_dir, FILE_NAME, &code_rs.to_string()));
}
