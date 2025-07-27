use crate::parse::{parse_entity_def, parse_system_def, EntityDef, QueryDef, SystemDef};
use std::collections::HashSet;
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use syn::Item;

#[derive(Debug, Default)]
pub struct CollectedData {
    pub entities: Vec<EntityDef>,
    pub queries: Vec<QueryDef>,
    pub systems: Vec<SystemDef>,
}

impl CollectedData {
    pub fn retain_unique_queries(&mut self) {
        let mut seen = HashSet::new();
        self.queries.retain(|query| {
            let mut hasher = DefaultHasher::new();
            query.hash(&mut hasher);
            let hash = hasher.finish();
            seen.insert(hash)
        });
    }
}

/// Parses a Rust source file and collects ECS definitions (`EntityDef`, `SystemDef`, and `QueryDef`).
///
/// Reads and parses a file at the specified path, extracting ECS-related declarations
/// from structs and functions annotated with relevant attributes.
pub fn collect_ecs_defs(path: &str) -> CollectedData {
    let mut entities = Vec::new();
    let mut queries = Vec::new();
    let mut systems = Vec::new();

    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => {
            panic!("Failed to read file '{path}': {error}");
        }
    };

    let parsed_file = match syn::parse_file(&content) {
        Ok(file) => file,
        Err(error) => {
            panic!("Failed to parse file '{path}': {error}");
        }
    };

    for item in parsed_file.items {
        match item {
            Item::Struct(item_struct) => parse_entity_def(item_struct, &mut entities),
            Item::Fn(item_fn) => parse_system_def(item_fn, &mut systems, &mut queries),
            _ => {}
        }
    }

    CollectedData {
        entities,
        queries,
        systems,
    }
}
