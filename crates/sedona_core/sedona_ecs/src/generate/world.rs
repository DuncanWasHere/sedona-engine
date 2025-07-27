use crate::{CollectedData, fident, write_token_stream_to_file};
use heck::ToSnakeCase;
use inflector::string::pluralize::to_plural;
use quote::{format_ident, quote};

pub fn generate_world_rs(
    out_dir: &str,
    include_files: &mut Vec<String>,
    collected: &CollectedData,
) {
    const FILE_NAME: &str = "world.rs";

    let mut code_rs = vec![];

    let mut world_fields = vec![];

    let entity_types = collected.entities.iter().map(|entity| {
        let entity_name = &entity.name;
        fident!(entity_name)
    });

    code_rs.push(quote! {
        #[allow(unused_imports)]
        use sedona_ecs::*;
        use std::collections::HashMap;

        #[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
        pub enum EntityType {
            #(#entity_types),*
        }

        #[allow(dead_code)]
        #[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
        pub struct Entity {
            entity_type: EntityType,
            id: Uuid,
        }

        #[allow(dead_code)]
        impl Entity {
            pub fn id(&self) -> Uuid {
                self.id
            }

            pub fn entity_type(&self) -> EntityType {
                self.entity_type
            }
        }

        #[allow(dead_code)]
        impl World {
            pub fn query_mut<'a, T: 'a + Send>(&'a mut self) -> impl Iterator<Item = T> + 'a
            where
                World: QueryMutFrom<'a, T>,
            {
                QueryMutFrom::<T>::query_mut_from(self)
            }

            pub fn par_query_mut<'a, T: 'a + Send>(&'a mut self) -> impl ParallelIterator<Item = T> + 'a
            where
                World: QueryMutFrom<'a, T>,
            {
                QueryMutFrom::<T>::par_query_mut_from(self)
            }

            pub fn get_mut<'a, T: 'a + Send>(&'a mut self, index: usize, entity_type: EntityType) -> Option<T>
            where
                World: QueryMutFrom<'a, T>,
            {
                QueryMutFrom::<T>::get_mut_from(self, index, entity_type)
            }
        }

        #[allow(dead_code)]
        impl World {
            pub fn query<'a, T: 'a + Send>(&'a self) -> impl Iterator<Item = T> + 'a
            where
                World: QueryFrom<'a, T>,
            {
                QueryFrom::<T>::query_from(self)
            }

            pub fn par_query<'a, T: 'a + Send>(&'a self) -> impl ParallelIterator<Item = T> + 'a
            where
                World: QueryFrom<'a, T>,
            {
                QueryFrom::<T>::par_query_from(self)
            }

            pub fn get<'a, T: 'a + Send>(&'a self, index: usize, entity_type: EntityType) -> Option<T>
            where
                World: QueryFrom<'a, T>,
            {
                QueryFrom::<T>::get_from(self, index, entity_type)
            }
        }

        pub trait WorldCreate<T> {
            fn create(&mut self, e: T) -> Entity;
        }
    });

    let mut offset_decls = vec![];
    let mut table_merges = vec![];
    let mut match_merge_index = vec![];
    let mut match_destroy_rs = vec![];

    for entity in collected.entities.iter() {
        let entity_name = &entity.name;
        let field_name = fident!( to_plural(&(entity_name.to_snake_case())));
        let archetype_type = fident!(to_plural(entity_name));
        let serialize = entity.serialize;

        if serialize {
            world_fields.push(quote! {
                #field_name: #archetype_type,
            });
        } else {
            world_fields.push(quote! {
                #[serde(skip)]
                #field_name: #archetype_type,
            });
        }
        
        let archetype_fields = entity.fields.iter().map(|field| {
            let field_name = format_ident!("{}", to_plural(&field.name));
            let field_type: proc_macro2::TokenStream = field.data_type.parse().unwrap();

            if field.serialize {
                quote! {
                    #field_name: Vec<#field_type>,
                }
            } else {
                quote! {
                    #[serde(skip)]
                    #field_name: Vec<#field_type>,
                }
            }
        });

        code_rs.push(quote! {

            #[derive(Default, Debug, serde::Deserialize, serde::Serialize)]
            struct #archetype_type {
                #(#archetype_fields)*
            }

            #[allow(dead_code)]
            impl #archetype_type {
                fn len(&self) -> usize {
                    self.entities.len()
                }
            }
        });

        code_rs.push(quote! {
            #[allow(dead_code)]
            impl #archetype_type {
                fn query_mut<'a, T: 'a>(&'a mut self) -> impl Iterator<Item = T> + 'a
                where
                    #archetype_type: QueryMutFrom<'a, T>,
                    T: 'a + Send,
                {
                    QueryMutFrom::<T>::query_mut_from(self)
                }
                fn par_query_mut<'a, T: 'a>(&'a mut self) -> impl ParallelIterator<Item = T> + 'a
                where
                    #archetype_type: QueryMutFrom<'a, T>,
                    T: 'a + Send,
                {
                    QueryMutFrom::<T>::par_query_mut_from(self)
                }
                fn get_mut<'a, T: 'a>(&'a mut self, index: usize, entity_type: EntityType) -> Option<T>
                where
                    #archetype_type: QueryMutFrom<'a, T>,
                    T: 'a + Send,
                {
                    QueryMutFrom::<T>::get_mut_from(self, index, entity_type)
                }
            }
        });

        code_rs.push(quote! {
            #[allow(dead_code)]
            impl #archetype_type {
                fn query<'a, T: 'a>(&'a self) -> impl Iterator<Item = T> + 'a
                where
                    #archetype_type: QueryFrom<'a, T>,
                    T: 'a + Send,
                {
                    QueryFrom::<T>::query_from(self)
                }
                fn par_query<'a, T: 'a>(&'a self) -> impl ParallelIterator<Item = T> + 'a
                where
                    #archetype_type: QueryFrom<'a, T>,
                    T: 'a + Send,
                {
                    QueryFrom::<T>::par_query_from(self)
                }
                fn get<'a, T: 'a>(&'a self, index: usize, entity_type: EntityType) -> Option<T>
                where
                    #archetype_type: QueryFrom<'a, T>,
                    T: 'a + Send,
                {
                    QueryFrom::<T>::get_from(self, index, entity_type)
                }
            }
        });

        let push_lines = entity
            .fields
            .iter()
            .filter(|e| e.data_type != "Entity")
            .map(|field| {
                let component_field_name = format_ident!("{}", to_plural(&field.name));
                let component_name = fident!(&field.name);

                quote! {
                    self.#field_name.#component_field_name.push(e.#component_name);
                }
            });

        let merge_extend_lines = entity
            .fields
            .iter()
            .filter(|f| f.data_type != "Entity")
            .map(|f| {
                let field_ident = format_ident!("{}", to_plural(&f.name));
                if f.serialize {
                    quote! {
                self.#field_ident.extend(table_to_merge.#field_ident.drain(..));
            }
                } else {
                    quote! {
                        for _ in 0..num_new {
                            self.#field_ident.push(Default::default());
                        }
                    }
                }
            });

        let merge_entities_line = quote! {
            self.entities.extend(table_to_merge.entities.drain(..));
        };

        let entity_name = fident!(entity_name);

        code_rs.push(quote! {
            impl WorldCreate<#entity_name> for World {
                fn create(&mut self, e: #entity_name) -> Entity {
                    let id = Uuid::new_v4();
                    self.index_lookup.insert(id, (self.#field_name.entities.len(), EntityType::#entity_name));
                    let entity = Entity {
                        entity_type: EntityType::#entity_name,
                        id,
                    };
                    self.#field_name.entities.push(entity);
                    #(#push_lines)*
                    entity
                }
            }
        });

        let pop_and_drop_code = entity.fields.iter().map(|field| {
            // let field_name = format_ident!("{}", to_plural(&field.name));
            let component_field_name = format_ident!("{}", to_plural(&field.name));
            quote! {
                self.#component_field_name.swap(index, last_index);
                self.#component_field_name.pop();
            }
        });

        let pop_and_drop_code_copy = pop_and_drop_code.clone();

        code_rs.push(quote! {
            #[allow(dead_code)]
            impl #archetype_type {
                fn destroy(&mut self, index: usize) {
                    let last_index = self.entities.len() - 1;

                    if index != last_index {
                        #(#pop_and_drop_code)*
                    } else {
                        #(#pop_and_drop_code_copy)*
                    }
                }

                fn merge(&mut self, table_to_merge: &mut #archetype_type) {
                    let num_new = table_to_merge.entities.len();

                    #merge_entities_line

                    #(#merge_extend_lines)*
                }
            }
        });

        let offset_ident = format_ident!("{}_offset", field_name);
        offset_decls.push(quote! {
            let #offset_ident = self.#field_name.entities.len();
        });
        
        table_merges.push(quote! {
            self.#field_name.merge(&mut other.#field_name);
        });

        match_merge_index.push(quote! {
            EntityType::#entity_name => {
                self.index_lookup.insert(uuid, (index + #offset_ident, entity_type));
            }
        });


        match_destroy_rs.push(quote! {
            EntityType::#entity_name => self.#field_name.destroy(index),
        });
    }

    code_rs.push(quote! {
        #[allow(dead_code)]
        impl World {
            pub fn merge(&mut self, mut other: World) {
                #(#offset_decls)*

                #(#table_merges)*

                for (uuid, (index, entity_type)) in other.index_lookup.drain() {
                    match entity_type {
                        #(#match_merge_index)*
                    }
                }
            }
        }
    });

    code_rs.push(quote! {
        #[allow(dead_code)]
        impl World {
            fn destroy(&mut self, id: Uuid) {
                if let Some(&(index, entity_type)) = self.index_lookup.get(&id) {
                    self.index_lookup.remove(&id);
                    match entity_type {
                        #(#match_destroy_rs)*
                    }
                }
            }
        }
    });

    code_rs.push(quote! {
        #[derive(Default, Debug, serde::Deserialize, serde::Serialize)]
        pub struct World {
            #(#world_fields)*
            index_lookup: HashMap<Uuid, (usize, EntityType)>,
        }
    });

    let code_rs = quote! {
        #(#code_rs)*
    };

    include_files.push(write_token_stream_to_file(out_dir, FILE_NAME, &code_rs.to_string()));
}
