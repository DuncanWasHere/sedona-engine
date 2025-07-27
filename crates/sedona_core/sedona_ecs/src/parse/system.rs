use crate::parse::{parse_query_def, QueryDef};
use quote::ToTokens;
use syn::{FnArg, GenericArgument, ItemFn, Meta, PatType, PathArguments, PathSegment, Type};

#[derive(Debug, Default)]
pub struct SystemDef {
    pub name: String,
    pub group: String,
    pub params: Vec<SystemDefParam>,
}

#[derive(Debug)]
pub enum SystemDefParam {
    Query(String),
    Reference(SystemDefParamReference),
    Value(SystemDefParamValue),
}

#[derive(Debug, Default, Clone)]
pub struct SystemDefParamReference {
    pub name: String,
    pub ty: String,
    pub mutable: bool,
}

#[derive(Debug, Default, Clone)]
pub struct SystemDefParamValue {
    pub name: String,
    pub ty: String,
}

/// Attempts to parse a function item as a system definition. If the function is marked
/// with a `#[system]` attribute, a `SystemDef` is constructed and pushed into the given
/// `systems` list. Any query parameters are parsed and collected into `queries`.
///
/// # Parameters
/// - `item_fn`: A reference to the function item (`syn::ItemFn`) being processed.
/// - `systems`: The list to which valid `SystemDef` entries are appended.
/// - `queries`: The list to which any discovered `QueryDef` entries are appended.
pub fn parse_system_def(
    item_fn: ItemFn,
    systems: &mut Vec<SystemDef>,
    queries: &mut Vec<QueryDef>,
) {
    let function_name = item_fn.sig.ident.to_string();
    if let Some(system_group) = parse_system_attributes(&item_fn) {
        let mut system_def = SystemDef {
            group: system_group,
            name: function_name.clone(),
            params: vec![],
        };

        for input in &item_fn.sig.inputs {
            if let FnArg::Typed(pat_type) = input {
                if let Some(param) = parse_system_def_param(pat_type, queries) {
                    system_def.params.push(param);
                }
            }
        }

        systems.push(system_def);
    }
}

/// Extracts the system group name from the `#[system]` attribute of a function.
///
/// This function looks for the `#[system]` attribute in the given `ItemFn`.
/// If a `group` is explicitly specified (e.g., `#[system(group = foo)]`), it returns that.
/// Otherwise, if the attribute is present without a group, it defaults to `main`.
/// Returns `None` if the `#[system]` attribute is not found.
///
/// # Parameters
/// - `item_fn`: A reference to a syn `ItemFn`, representing a function in the AST.
///
/// # Returns
/// - `Some(group_name)` if the function is a system and its group can be determined.
/// - `None` if the function does not have a `#[system]` attribute.
fn parse_system_attributes(item_fn: &ItemFn) -> Option<String> {
    for attribute in &item_fn.attrs {
        match &attribute.meta {
            // #[system]
            Meta::Path(path) => {
                if path.is_ident("system") {
                    return Some("main".into());
                }
            }
            // #[system(group=foo)]
            Meta::List(list) => {
                if list.path.is_ident("system") {
                    let tokens_string = list.tokens.to_string();
                    let mut key_value_parts = tokens_string.split('=');

                    if let Some(key) = key_value_parts.next() {
                        if let Some(value) = key_value_parts.next() {
                            if key.trim() == "group" {
                                return Some(value.trim().into());
                            }
                        }
                    }

                    return Some("main".into());
                }
            }
            // #[system = foo]
            Meta::NameValue(name_value) => {
                if name_value.path.is_ident("system") {
                    return Some("main".into()); // Unsupported syntax.
                }
            }
        }
    }

    None
}

/// Parses a single function parameter and returns the corresponding `SystemDefParam`,
/// pushing to `queries` if it is a `Query` type.
///
/// # Parameters
/// - `pat_type`: The typed parameter to inspect.
/// - `queries`: The list of collected `QueryDef` entries, modified in-place if needed.
///
/// # Returns
/// - `Some(SystemDefParam)` if the parameter is a recognized type.
/// - `None` for receiver/self arguments or unrecognized/unsupported types.
fn parse_system_def_param(
    pat_type: &PatType,
    queries: &mut Vec<QueryDef>,
) -> Option<SystemDefParam> {
    let param_name = pat_type.pat.to_token_stream().to_string();

    match *pat_type.ty.clone() {
        Type::Path(typed_path) => {
            for segment in &typed_path.path.segments {
                let name = segment.ident.to_string();
                if name == "Query" {
                    if let Some(query_param) = parse_query_param(segment, &param_name, queries) {
                        return Some(query_param);
                    }
                } else {
                    let ty_str = typed_path.to_token_stream().to_string();
                    return Some(SystemDefParam::Value(SystemDefParamValue {
                        name: param_name,
                        ty: ty_str,
                    }));
                }
            }
            None
        }
        Type::Reference(type_ref) => {
            let mutable = type_ref.mutability.is_some();
            let ty_str = type_ref.elem.to_token_stream().to_string();

            Some(SystemDefParam::Reference(SystemDefParamReference {
                name: param_name,
                ty: ty_str,
                mutable,
            }))
        }
        _ => {
            panic!("Unsupported param type in system: {param_name}");
        }
    }
}

/// Parses a `Query<T>` parameter and adds the extracted query to `queries` if valid.
///
/// # Parameters
/// - `segment`: The `PathSegment` expected to be the `Query`.
/// - `param_name`: The name of the function parameter (used in the system param def).
/// - `queries`: The list to which parsed `QueryDef` entries are appended.
///
/// # Returns
/// - `Some(SystemDefParam::Query)` if parsing succeeded.
/// - `None` otherwise.
pub fn parse_query_param(
    segment: &PathSegment,
    param_name: &str,
    queries: &mut Vec<QueryDef>,
) -> Option<SystemDefParam> {
    if let PathArguments::AngleBracketed(arguments) = &segment.arguments {
        if let Some(GenericArgument::Type(query_ty)) = arguments.args.iter().next() {
            if let Some(query) = parse_query_def(query_ty) {
                queries.push(query);
                return Some(SystemDefParam::Query(param_name.to_string()));
            }
        }
    }
    None
}
