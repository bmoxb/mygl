use std::ffi::{IntoStringError, NulError};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unexpected null byte found in C string: {0}")]
    CStringNull(#[from] NulError),
    #[error("Failed to convert C string into Rust string: {0}")]
    CStringUTF8(#[from] IntoStringError),
    #[error("Shader error: {0}")]
    Shader(#[from] ShaderError),
}

#[derive(thiserror::Error, Debug)]
pub enum ShaderError {
    #[error("failed to load shader - {0}")]
    Loading(#[from] std::io::Error),
    #[error("failed to compile shader - {0}")]
    Compilation(String),
    #[error("failed to link shader - {0}")]
    Linking(String),
    #[error("could not find uniform with name '{0}'")]
    UniformName(String),
}
