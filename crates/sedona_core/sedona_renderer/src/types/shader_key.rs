use crate::types::ShaderFlags;
use std::borrow::Borrow;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ShaderKey {
    pub path: String,
    pub flags: ShaderFlags,
}

impl ShaderKey {
    pub fn new(path: &str, flags: ShaderFlags) -> Self {
        Self {
            path: path.to_string(),
            flags,
        }
    }
}
