use crate::write_token_stream_to_file;
use quote::quote;

pub fn generate_default_queries(out_dir: &str) -> String {
    const FILE_NAME: &str = "queries.rs";

    let code_rs = quote! {
        use std::marker::PhantomData;

        #[derive(Default, Debug)]
        pub struct Query<T> {
            phantom: PhantomData<T>,
        }
        
        pub struct WithQuery<'a, T> {
            query: Query<T>,
            world: &'a World,
        }
        
        pub struct WithQueryMut<'a, T> {
            query: Query<T>,
            world: &'a mut World,
        }

        impl<T> Clone for Query<T> {
            fn clone(&self) -> Self {
                Query {
                    phantom: PhantomData,
                }
            }
        }

        #[allow(dead_code)]
        impl<T> Query<T> {
            pub fn new() -> Query<T> {
                Query {
                    phantom: PhantomData,
                }
            }
        }

        impl<'a, T: 'a + Send> Query<T>
        {
            pub fn iter(&self, world: &'a World) -> impl Iterator<Item = T> + 'a
            where
                World: QueryFrom<'a, T>,
            {
                world.query_from()
            }
        }
        
        impl<'a, T: 'a + Send> Query<T>
        {
            pub fn par_iter(&self, world: &'a World) -> impl ParallelIterator<Item = T> + 'a
            where
                World: QueryFrom<'a, T>,
            {
                world.par_query_from()
            }
        }
        
        impl<'a, T: 'a + Send> Query<T> {
            pub fn iter_mut(&self, world: &'a mut World) -> impl Iterator<Item = T> + 'a
            where
                World: QueryMutFrom<'a, T>,
            {
                world.query_mut_from()
            }
        }
        
        impl<'a, T: 'a + Send> Query<T>
        {
            pub fn par_iter_mut(&self, world: &'a mut World) -> impl ParallelIterator<Item = T> + 'a
            where
                World: QueryMutFrom<'a, T>,
            {
                world.par_query_mut_from()
            }
        }
        
        impl<'a, T: 'a + Send> Query<T> {
            pub fn get(&self, world: &'a World, index: usize, entity_type: EntityType) -> Option<T>
            where
                World: QueryFrom<'a, T>,
            {
                world.get_from(index, entity_type)
            }
        }

        impl<'a, T: 'a + Send> Query<T> {
            pub fn get_mut(&self, world: &'a mut World, index: usize, entity_type: EntityType) -> Option<T>
            where
                World: QueryMutFrom<'a, T>,
            {
                world.get_mut_from(index, entity_type)
            }
        }

        // Implement len
        impl<'a, T: 'a + Send> Query<T> {
            pub fn len(&self, world: &'a World) -> usize
            where
                World: LenFrom<'a, T>,
            {
                world.len()
            }
        }

        // Impl at_mut
        impl<'a, T: 'a + Send> Query<T> {
            pub fn at_mut(&self, world: &'a mut World, index: usize) -> Option<T>
            where
                World: QueryMutFrom<'a, T>,
            {
                world.at_mut(index)
            }
        }

        // Impl at
        impl<'a, T: 'a + Send> Query<T> {
            pub fn at(&self, world: &'a World, index: usize) -> Option<T>
            where
                World: QueryFrom<'a, T>,
            {
                world.at(index)
            }
        }
        
        #[allow(dead_code)]
        impl<'a, T> WithQuery<'a, T>
            where World: QueryFrom<'a, T>,
                World: LenFrom<'a, T>,
                T: 'a + Send,
        {
            pub fn iter(&'a self) -> impl Iterator<Item = T> + 'a {
                self.query.iter(self.world)
            }
            
            pub fn par_iter(&'a self) -> impl ParallelIterator<Item = T> + 'a {
                self.query.par_iter(self.world)
            }
            
            pub fn get(&'a self, id: Uuid) -> Option<T> {
                if let Some(&(index, entity_type)) = self.world.index_lookup.get(&id) {
                    self.query.get(self.world, index, entity_type)
                } else {
                    None
                }
            }
            
            pub fn len(&'a self) -> usize {
                self.query.len(self.world)
            }
            
            pub fn at(&'a self, index: usize) -> Option<T> {
                self.query.at(self.world, index)
            }
            
            pub fn is_empty(&'a self) -> bool {
                self.query.len(self.world) == 0
            }
        }

        #[allow(dead_code)]
        impl<'a, T> WithQueryMut<'a, T>
            where World: QueryMutFrom<'a, T>,
                World: LenFrom<'a, T>,
                T: 'a + Send,
        {
            pub fn iter_mut(&'a mut self) -> impl Iterator<Item = T> + 'a {
                self.query.iter_mut(self.world)
            }
            
            pub fn par_iter_mut(&'a mut self) -> impl ParallelIterator<Item = T> + 'a {
                self.query.par_iter_mut(self.world)
            }
            
            pub fn get_mut(&'a mut self, id: Uuid) -> Option<T> {
                if let Some(&(index, entity_type)) = self.world.index_lookup.get(&id) {
                    self.query.get_mut(self.world, index, entity_type)
                } else {
                    None
                }
            }
            
            pub fn len(&'a mut self) -> usize {
                self.query.len(self.world)
            }
            
            pub fn at_mut(&'a mut self, index: usize) -> Option<T> {
                self.query.at_mut(self.world, index)
            }
            
            pub fn is_empty(&'a mut self) -> bool {
                self.query.len(self.world) == 0
            }
        }
        
        #[allow(dead_code)]
        impl World {
            pub fn with_query<'a, T: 'a + Send>(&'a self, query: Query<T>) -> WithQuery<'a, T>
            where
                World: QueryFrom<'a, T>,
            {
                WithQuery {
                    query,
                    world: self,
                }
            }
        }

        #[allow(dead_code)]
        impl World {
            pub fn with_query_mut<'a, T: 'a + Send>(&'a mut self, query: Query<T>) -> WithQueryMut<'a, T>
            where
                World: QueryMutFrom<'a, T>,
            {
                WithQueryMut {
                    query,
                    world: self,
                }
            }
        }
    };
    
    write_token_stream_to_file(out_dir, FILE_NAME, &code_rs.to_string())
}
