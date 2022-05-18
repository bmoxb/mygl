use gl::types::*;

use std::ffi::CString;
use std::path::Path;

use crate::error::{Error, ShaderError};

#[allow(drop_bounds)]
pub trait Shader: Drop {
    fn from_source(src: &str) -> Result<Self, Error>
    where
        Self: Sized;

    fn from_file(path: impl AsRef<Path>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let src = std::fs::read_to_string(path).map_err(|e| Error::Shader(ShaderError::from(e)))?;
        Self::from_source(&src)
    }

    fn get_id(&self) -> GLuint;
}

pub struct Vertex {
    id: GLuint,
}

impl Shader for Vertex {
    fn from_source(src: &str) -> Result<Self, Error> {
        make_shader(src, gl::VERTEX_SHADER).map(|id| Self { id })
    }

    fn get_id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Vertex {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

pub struct Fragment {
    id: GLuint,
}

impl Shader for Fragment {
    fn from_source(src: &str) -> Result<Self, Error> {
        make_shader(src, gl::FRAGMENT_SHADER).map(|id| Self { id })
    }

    fn get_id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Fragment {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

pub struct Program {
    id: GLuint,
}

impl Program {
    pub fn new(vert: Vertex, frag: Fragment) -> Result<Self, Error> {
        let id = unsafe { gl::CreateProgram() };

        let mut success = gl::TRUE as GLint;

        unsafe {
            gl::AttachShader(id, vert.get_id());
            gl::AttachShader(id, frag.get_id());
            gl::LinkProgram(id);

            let success_ptr: *mut i32 = &mut success;
            gl::GetProgramiv(id, gl::LINK_STATUS, success_ptr);
        }

        if success as GLboolean == gl::FALSE {
            return Err(Error::Shader(ShaderError::Linking("TODO".to_string())));
        }

        Ok(Program { id })
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

fn make_shader(src: &str, variety: GLenum) -> Result<GLuint, Error> {
    let id = unsafe { gl::CreateShader(variety) };

    let src_c_str = CString::new(src)?;
    let mut success = gl::TRUE as GLint;

    unsafe {
        let src_c_str_ptr: *const *const i8 = &src_c_str.as_ptr();
        let null: *const i32 = std::ptr::null();

        gl::ShaderSource(id, 1, src_c_str_ptr, null);
        gl::CompileShader(id);

        let success_ptr: *mut i32 = &mut success;
        gl::GetShaderiv(id, gl::COMPILE_STATUS, success_ptr);
    }

    if success as GLboolean == gl::FALSE {
        return Err(Error::Shader(ShaderError::Compilation("TODO".to_string())));
    }

    Ok(id)
}
