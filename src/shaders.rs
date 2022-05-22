use std::ffi::CString;
use std::fmt;
use std::path::Path;

use gl::types::*;

use paste::paste;
use seq_macro::seq;

use crate::error::{Error, ShaderError};

pub struct Shader<const T: ShaderType> {
    id: GLuint,
}

impl<const T: ShaderType> Shader<T> {
    pub fn from_source(src: &str) -> Result<Self, Error> {
        let id = unsafe { gl::CreateShader(Into::into(T)) };

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
            let msg = get_error_msg(id, gl::GetShaderiv, gl::GetShaderInfoLog)?;
            return Err(Error::Shader(ShaderError::Compilation(msg)));
        }

        let shader = Self { id };

        log::debug!("Created and compiled {}", shader);

        Ok(shader)
    }

    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, Error> {
        let src = std::fs::read_to_string(path).map_err(|e| Error::Shader(ShaderError::from(e)))?;
        Self::from_source(&src)
    }
}

impl<const T: ShaderType> fmt::Display for Shader<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} shader {}", T, self.id)
    }
}

impl<const T: ShaderType> Drop for Shader<T> {
    fn drop(&mut self) {
        log::debug!("Flagged {} for deletion", self);

        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

pub type VertexShader = Shader<{ ShaderType::Vertex }>;
pub type FragmentShader = Shader<{ ShaderType::Fragment }>;

#[derive(Eq, PartialEq, Copy, Clone, Debug, strum_macros::Display)]
#[strum(serialize_all = "snake_case")]
pub enum ShaderType {
    Vertex,
    Fragment,
}

impl From<ShaderType> for GLenum {
    fn from(s: ShaderType) -> GLenum {
        match s {
            ShaderType::Vertex => gl::VERTEX_SHADER,
            ShaderType::Fragment => gl::FRAGMENT_SHADER,
        }
    }
}

pub struct ShaderProgram {
    id: GLuint,
}

impl ShaderProgram {
    pub fn new(vert: &VertexShader, frag: &FragmentShader) -> Result<Self, Error> {
        let id = unsafe { gl::CreateProgram() };

        let prog = ShaderProgram { id };

        log::debug!("Created {}", prog);

        let mut success = gl::TRUE as GLint;

        unsafe {
            gl::AttachShader(id, vert.id);
            gl::AttachShader(id, frag.id);
            gl::LinkProgram(id);

            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
        }

        if success as GLboolean == gl::FALSE {
            let msg = get_error_msg(id, gl::GetProgramiv, gl::GetProgramInfoLog)?;
            return Err(Error::Shader(ShaderError::Linking(msg)));
        }

        log::debug!("Attached and linked {} and {} to {}", vert, frag, prog);

        Ok(prog)
    }

    pub fn set_uniform(&self, key: &str, value: impl UniformValue) -> Result<(), Error> {
        self.use_program();

        let key_c_str = CString::new(key)?;
        let location = unsafe { gl::GetUniformLocation(self.id, key_c_str.as_ptr()) };

        if location == -1 {
            return Err(Error::Shader(ShaderError::UniformName(key.to_string())));
        }

        log::debug!(
            "Setting uniform '{}' (location = {}) for {} to value {:?} (type {})",
            key,
            location,
            self,
            value,
            value.ty(),
        );

        value.set(location);

        Ok(())
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }

        log::trace!("Using {}", self);
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        log::debug!("Deleting {}", self);

        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

impl fmt::Display for ShaderProgram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "shader program {}", self.id)
    }
}

pub trait UniformValue: fmt::Debug {
    fn set(self, location: GLint);
    fn ty(&self) -> &str;
}

macro_rules! uniform {
    ($impl_type:ty, $gl_type:ty, $fun:path) => {
        impl UniformValue for $impl_type {
            fn set(self, location: GLint) {
                unsafe {
                    $fun(location, self as $gl_type);
                }
            }
            fn ty(&self) -> &str {
                stringify!($impl_type)
            }
        }
    };

    ($base_type:ty, $len:expr, $fun_suffix:ident) => {
        impl UniformValue for &[$base_type; $len] {
            fn set(self, location: GLint) {
                unsafe {
                    paste! { gl::[< Uniform $len $fun_suffix >](location, 1, self.as_ptr()); }
                }
            }
            fn ty(&self) -> &str {
                stringify!([$base_type; $len])
            }
        }
    };
}

uniform!(f32, GLfloat, gl::Uniform1f);
uniform!(i32, GLint, gl::Uniform1i);
uniform!(u32, GLuint, gl::Uniform1ui);

seq!(N in 1..=4 {
    uniform!(f32, N, fv);
    uniform!(i32, N, iv);
    uniform!(u32, N, uiv);
});

uniform!(bool, GLint, gl::Uniform1i);

#[cfg(feature = "nalgebra")]
mod nalgebra_uniforms {
    use super::*;
    use nalgebra as na;

    macro_rules! gl_uniform_matrix {
        (2, 2) => {
            paste! { gl::[< UniformMatrix2 fv >] }
        };
        (3, 3) => {
            paste! { gl::[< UniformMatrix3 fv >] }
        };
        (4, 4) => {
            paste! { gl::[< UniformMatrix4 fv >] }
        };
        ($rows:expr, $columns:expr) => {
            paste! { gl::[< UniformMatrix $rows x $columns fv >] }
        };
    }

    macro_rules! uniform_matrix {
        ($rows:expr, $columns:expr) => {
            paste! {
                impl<T> UniformValue for na::Matrix<f32, na::[< U $rows >], na::[< U $columns >], T>
                where
                    T: fmt::Debug + na::Storage<f32, na::[< U $rows >], na::[< U $columns >]>
                {
                    fn set(self, location: GLint) {
                        unsafe {
                            gl_uniform_matrix!($rows, $columns)(location, 1, gl::FALSE, self.as_ptr());
                        }
                    }
                    fn ty(&self) -> &str { stringify!(Matrix<f32, $rows, $columns>) }
                }
            }
        }
    }

    seq!(R in 2..=4 {
        seq!(C in 2..=4 {
            uniform_matrix!(R, C);
        });
    });
}

fn get_error_msg(
    id: GLuint,
    get_iv: unsafe fn(GLuint, GLenum, *mut GLint),
    get_log: unsafe fn(GLuint, GLsizei, *mut GLsizei, *mut GLchar),
) -> Result<String, Error> {
    let mut buffer;

    unsafe {
        let mut length = 0;
        get_iv(id, gl::INFO_LOG_LENGTH, &mut length);

        buffer = vec![0i8; length as usize];
        get_log(id, length, &mut length, buffer.as_mut_ptr());

        buffer.truncate(length as usize);
    }

    Ok(CString::new::<Vec<u8>>(buffer.into_iter().map(|c| c as u8).collect())?.into_string()?)
}
