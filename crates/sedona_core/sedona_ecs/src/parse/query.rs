use quote::ToTokens;
use std::hash::{Hash, Hasher};
use syn::Type;

#[derive(Debug, Clone)]
pub struct QueryDef {
    pub mut_fields: Vec<String>,
    pub const_fields: Vec<String>,
}

impl Eq for QueryDef {}

impl PartialEq for QueryDef {
    fn eq(&self, other: &Self) -> bool {
        self.mut_fields == other.mut_fields && self.const_fields == other.const_fields
    }
}

impl Hash for QueryDef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.mut_fields.hash(state);
        self.const_fields.hash(state);
    }
}

/// Attempts to parse a query parameter type into a `QueryDef`.
///
/// This function supports queries in the form of:
/// - Single references: `&T`, `&mut T`
/// - Tuples of references: `(&T, &mut U)`
///
/// # Parameters
/// - `ty`: The type of the query generic (typically inside `Query<T>`).
///
/// # Returns
/// - `Some(QueryDef)` if any valid query fields were extracted.
/// - `None` if the type contains no extractable fields or isn't a supported form.
pub fn parse_query_def(ty: &Type) -> Option<QueryDef> {
    match ty {
        Type::Reference(type_reference) => {
            let elem = &type_reference.elem;
            if let Type::Path(type_path) = &**elem {
                let name = type_path.to_token_stream().to_string();
                let (mutable_fields, const_fields) = if type_reference.mutability.is_some() {
                    (vec![name], vec![])
                } else {
                    (vec![], vec![name])
                };
                Some(QueryDef {
                    mut_fields: mutable_fields,
                    const_fields,
                })
            } else {
                None
            }
        }
        Type::Tuple(type_tuple) => {
            let mut mutable_fields = Vec::new();
            let mut const_fields = Vec::new();

            for elem in &type_tuple.elems {
                if let Type::Reference(type_reference) = elem {
                    let name = type_reference.elem.to_token_stream().to_string();
                    if type_reference.mutability.is_some() {
                        mutable_fields.push(name);
                    } else {
                        const_fields.push(name);
                    }
                }
            }

            if mutable_fields.is_empty() && const_fields.is_empty() {
                None
            } else {
                Some(QueryDef {
                    mut_fields: mutable_fields,
                    const_fields,
                })
            }
        }
        _ => None,
    }
}
