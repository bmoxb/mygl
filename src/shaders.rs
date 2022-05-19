use std::ffi::CString;
use std::path::Path;

use gl::types::*;

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

pub struct VertexShader {
    id: GLuint,
}

impl Shader for VertexShader {
    fn from_source(src: &str) -> Result<Self, Error> {
        make_shader(src, gl::VERTEX_SHADER).map(|id| Self { id })
    }

    fn get_id(&self) -> GLuint {
        self.id
    }
}

impl Drop for VertexShader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

pub struct FragmentShader {
    id: GLuint,
}

impl Shader for FragmentShader {
    fn from_source(src: &str) -> Result<Self, Error> {
        make_shader(src, gl::FRAGMENT_SHADER).map(|id| Self { id })
    }

    fn get_id(&self) -> GLuint {
        self.id
    }
}

impl Drop for FragmentShader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

pub struct ShaderProgram {
    id: GLuint,
}

impl ShaderProgram {
    pub fn new(vert: VertexShader, frag: FragmentShader) -> Result<Self, Error> {
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

        Ok(ShaderProgram { id })
    }

    pub fn set_uniform(&self, key: &str, value: impl UniformValue) -> Result<(), Error> {
        self.use_program();

        let key_c_str = CString::new(key)?;
        let location = unsafe { gl::GetUniformLocation(self.id, key_c_str.as_ptr()) };

        if location == -1 {
            return Err(Error::Shader(ShaderError::UniformName(key.to_string())));
        }

        value.set(location);

        Ok(())
    }

    fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

pub trait UniformValue {
    fn set(self, location: GLint);
}

macro_rules! uniform {
    ($impl_type:ty, $gl_type:ty, $fun:path) => {
        impl UniformValue for $impl_type {
            fn set(self, location: GLint) {
                unsafe {
                    $fun(location, self as $gl_type);
                }
            }
        }
    };

    ($base_type:ty, $len:expr, $gl_type:ty, $fun:path) => {
        impl UniformValue for &[$base_type; $len] {
            fn set(self, location: GLint) {
                unsafe {
                    $fun(location, $len, self.as_ptr());
                }
            }
        }
    };
}

uniform!(f32, GLfloat, gl::Uniform1f);
// seq!(N in 1..=4 { uniform!(f32, N, GLfloat, gl::Uniform~N~fv); });
uniform!(f32, 1, GLfloat, gl::Uniform1fv);
uniform!(f32, 2, GLfloat, gl::Uniform2fv);
uniform!(f32, 3, GLfloat, gl::Uniform3fv);
uniform!(f32, 4, GLfloat, gl::Uniform4fv);

uniform!(i32, GLint, gl::Uniform1i);
uniform!(i32, 1, GLint, gl::Uniform1iv);
uniform!(i32, 2, GLint, gl::Uniform2iv);
uniform!(i32, 3, GLint, gl::Uniform3iv);
uniform!(i32, 4, GLint, gl::Uniform4iv);

uniform!(u32, GLuint, gl::Uniform1ui);
uniform!(u32, 1, GLuint, gl::Uniform1uiv);
uniform!(u32, 2, GLuint, gl::Uniform2uiv);
uniform!(u32, 3, GLuint, gl::Uniform3uiv);
uniform!(u32, 4, GLuint, gl::Uniform4uiv);

uniform!(bool, GLint, gl::Uniform1i);

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
