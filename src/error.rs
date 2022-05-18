use std::ffi::NulError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to convert from Rust string type to C string: {0}")]
    CString(#[from] NulError),
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
