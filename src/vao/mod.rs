pub mod builder;
pub mod data_source;

pub use builder::{AttribPointerType, VertexArrayObjectBuilder};
pub use data_source::BufferDataSource;

use std::rc::Rc;
use std::{convert, fmt};

use gl::types::*;

pub struct VertexArrayObject {
    id: GLuint,
    ebo: Option<ElementBufferObject>,
    vbos: Vec<VertexBufferObject>,
}

impl VertexArrayObject {
    fn new() -> Self {
        let mut id = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        let vao = VertexArrayObject {
            id,
            ebo: None,
            vbos: Vec::new(),
        };

        log::debug!("Created {}", vao);

        vao
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }

        log::trace!("Bound {}", self);
    }
}

impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        log::debug!("Deleting {}", self);

        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}

impl fmt::Display for VertexArrayObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "vertex array object {}", self.id)
    }
}

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct VertexBufferObject {
    inner: Rc<BufferObjectInner>,
}

impl VertexBufferObject {
    pub fn new(usage: BufferUsageHint, data: impl BufferDataSource) -> Self {
        VertexBufferObject {
            inner: Rc::new(BufferObjectInner::new(BufferType::Vertex, usage, data)),
        }
    }
}

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct ElementBufferObject {
    inner: Rc<BufferObjectInner>,
}

impl ElementBufferObject {
    pub fn new(usage: BufferUsageHint, data: impl BufferDataSource) -> Self {
        ElementBufferObject {
            inner: Rc::new(BufferObjectInner::new(BufferType::Element, usage, data)),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug, strum_macros::Display)]
#[strum(serialize_all = "snake_case")]
pub enum BufferUsageHint {
    Static,
    Dynamic,
}

impl convert::From<BufferUsageHint> for GLenum {
    fn from(b: BufferUsageHint) -> GLenum {
        match b {
            BufferUsageHint::Static => gl::STATIC_DRAW,
            BufferUsageHint::Dynamic => gl::DYNAMIC_DRAW,
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
struct BufferObjectInner {
    id: GLuint,
    buf_type: BufferType,
    usage: BufferUsageHint,
}

impl BufferObjectInner {
    fn new(buf_type: BufferType, usage: BufferUsageHint, data: impl BufferDataSource) -> Self {
        let mut id = 0;

        unsafe {
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(buf_type.into(), id);
            gl::BufferData(
                buf_type.into(),
                data.size() as GLsizeiptr,
                data.ptr(),
                usage.into(),
            );
        }

        let bo = BufferObjectInner {
            id,
            buf_type,
            usage,
        };

        log::debug!("Created {} for {} usage and initialised with {} bytes of data", bo, bo.usage, data.size());

        bo
    }

    fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.buf_type.into(), self.id);
        }

        log::debug!("Bound {}", self);
    }
}

impl Drop for BufferObjectInner {
    fn drop(&mut self) {
        log::debug!("Deleting {}", self);

        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

impl fmt::Display for BufferObjectInner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} buffer object {}", self.buf_type, self.id)
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug, strum_macros::Display)]
#[strum(serialize_all = "snake_case")]
enum BufferType {
    Vertex,
    Element,
}

impl convert::From<BufferType> for GLenum {
    fn from(b: BufferType) -> GLenum {
        match b {
            BufferType::Vertex => gl::ARRAY_BUFFER,
            BufferType::Element => gl::ELEMENT_ARRAY_BUFFER,
        }
    }
}
