use quote::ToTokens;
use syn::{Attribute, Fields, ItemStruct, Meta, MetaList};

#[derive(Debug)]
pub struct EntityDef {
    pub name: String,
    pub fields: Vec<EntityDefField>,
    pub serialize: bool,
}

#[derive(Debug)]
pub struct EntityDefField {
    pub name: String,
    pub data_type: String,
    pub serialize: bool,
}

/// Parses an `ItemStruct` and adds an `EntityDef` to the list of collected entities
/// if the struct is marked with an `#[entity]` attribute.
///
/// # Parameters
/// - `item_struct`: The struct item being inspected.
/// - `entities`: The list to which `EntityDef`s are appended if applicable.
pub fn parse_entity_def(item_struct: ItemStruct, entities: &mut Vec<EntityDef>) {
    for attribute in &item_struct.attrs {
        if let Some(serialize) = parse_entity_attributes(&attribute.meta) {
            let fields = parse_entity_def_fields(&item_struct.fields);
            entities.push(EntityDef {
                name: item_struct.ident.to_string(),
                fields,
                serialize,
            });
            break;
        }
    }
}

/// Parses the `#[entity(...)]` attribute metadata and returns the `serialize` flag.
///
/// Returns `Some(true)` if the attribute is `#[entity]` or `#[entity(serialize = true)]`.
/// Returns `Some(false)` if `#[entity(serialize = false)]`.
/// Returns `None` if the attribute is not `#[entity]`.
fn parse_entity_attributes(meta: &Meta) -> Option<bool> {
    match meta {
        // #[entity]
        Meta::Path(path) => {
            if path.is_ident("entity") {
                return Some(true);
            }
        }

        // #[entity(serialize = false)]
        Meta::List(list) => {
            if list.path.is_ident("entity") {
                let tokens_string = list.tokens.to_string();
                let mut key_value_parts = tokens_string.split('=');

                if let Some(key) = key_value_parts.next() {
                    if let Some(value) = key_value_parts.next() {
                        let key = key.trim();
                        let value = value.trim();

                        if key == "serialize" {
                            return match value {
                                "false" => Some(false),
                                "true" => Some(true),
                                _ => Some(true), // default to true if malformed
                            };
                        }
                    }
                }

                return Some(true); // no explicit value, default to true
            }
        }

        _ => {}
    }

    None
}

/// Parses the named fields (components) of an entity struct into a list of `EntityDefField`s.
///
/// # Parameters
/// - `fields`: The `Fields` (components) of the entity struct.
///
/// # Returns
/// - A vector of `EntityDefField` entries, empty if not `Fields::Named`.

fn parse_entity_def_fields(fields: &Fields) -> Vec<EntityDefField> {
    match fields {
        Fields::Named(named_fields) => named_fields
            .named
            .iter()
            .map(|field| {
                let serialize = is_field_serializable(&field.attrs);
                EntityDefField {
                    name: field.ident.as_ref().unwrap().to_string(),
                    data_type: field.ty.to_token_stream().to_string(),
                    serialize,
                }
            })
            .collect(),
        _ => vec![],
    }
}

/// Returns `false` if `#[serde(skip)]` is present in the attribute list.
fn is_field_serializable(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if attr.path().is_ident("serde") {
            let tokens = attr.into_token_stream().to_string();

            // Match for common cases like #[serde(skip)]
            if tokens.contains("skip") {
                return false;
            }
        }
    }
    true
}
