pub mod builder;
pub mod data_source;

pub use builder::{AttribPointerType, VertexArrayObjectBuilder};
pub use data_source::BufferDataSource;

use crate::shaders::ShaderProgram;

use std::rc::Rc;
use std::{convert, fmt};

use gl::types::*;

pub struct VertexArrayObject {
    id: GLuint,
    ebo: Option<ElementBufferObject>,
    vbos: Vec<VertexBufferObject>,
}

impl VertexArrayObject {
    pub fn draw_arrays(&self, mode: DrawMode, first: i32, count: i32) {
        self.bind();

        log::trace!(
            "Using vertex array object {} to draw {} vertices",
            self.id,
            count
        );

        unsafe {
            gl::DrawArrays(mode.into(), first, count);
        }
    }

    pub fn draw_elements() {
        unimplemented!()
    }

    fn new() -> Self {
        let mut id = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        log::debug!("Created vertex array object {}", id);

        VertexArrayObject {
            id,
            ebo: None,
            vbos: Vec::new(),
        }
    }

    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }

        log::trace!("Bound vertex array object {}", self.id);
    }
}

impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        log::debug!("Deleting vertex array object {}", self.id);

        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}

pub enum DrawMode {
    Points,
    Lines,
    Triangles,
}

impl convert::Into<GLenum> for DrawMode {
    fn into(self) -> GLenum {
        match self {
            DrawMode::Points => gl::POINTS,
            DrawMode::Lines => gl::LINES,
            DrawMode::Triangles => gl::TRIANGLES,
        }
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

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum BufferUsageHint {
    Static,
    Dynamic,
}

impl convert::Into<GLenum> for BufferUsageHint {
    fn into(self) -> GLenum {
        match self {
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

        log::debug!("Created {} buffer object {}", buf_type, id);

        BufferObjectInner {
            id,
            buf_type,
            usage,
        }
    }

    fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.buf_type.into(), self.id);
        }

        log::debug!("Bound {} buffer object {}", self.buf_type, self.id);
    }
}

impl Drop for BufferObjectInner {
    fn drop(&mut self) {
        log::debug!("Deleting {} buffer object {}", self.buf_type, self.id);

        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum BufferType {
    Vertex,
    Element,
}

impl convert::Into<GLenum> for BufferType {
    fn into(self) -> GLenum {
        match self {
            BufferType::Vertex => gl::ARRAY_BUFFER,
            BufferType::Element => gl::ELEMENT_ARRAY_BUFFER,
        }
    }
}

impl fmt::Display for BufferType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BufferType::Vertex => write!(f, "vertex"),
            BufferType::Element => write!(f, "element"),
        }
    }
}
