use thiserror::Error;

#[derive(Debug, Error)]
pub enum RendererError {
    #[error("Invalid object index: {index}")]
    InvalidObjectIndex { index: usize },

    #[error("Invalid node index: {index}")]
    InvalidNodeIndex { index: usize },

    #[error("Invalid material index: {index}")]
    InvalidMaterialIndex { index: usize },

    #[error("No shader loaded from path: {path}")]
    InvalidShaderPath { path: String },
}
