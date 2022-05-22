pub mod builder;
pub mod data_source;

pub use builder::*;
pub use data_source::BufferDataSource;

use std::cell::RefCell;
use std::cmp::PartialEq;
use std::convert::{From, Into};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use gl::types::*;

use crate::error::{BufferError, Error};

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

pub type VertexBufferObject = BufferObject<{ BufferType::Vertex }>;
pub type ElementBufferObject = BufferObject<{ BufferType::Element }>;

#[derive(Clone, Eq)]
pub struct BufferObject<const T: BufferType> {
    id: GLuint,
    inner: Rc<RefCell<BufferObjectInner>>,
}

impl<const T: BufferType> Hash for BufferObject<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<const T: BufferType> PartialEq for BufferObject<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<const T: BufferType> BufferObject<T> {
    pub fn new(data: impl BufferDataSource, usage: BufferUsageHint) -> Self {
        let mut id = 0;

        unsafe {
            gl::GenBuffers(1, &mut id);
        }

        let bo = BufferObject {
            id,
            inner: Rc::new(RefCell::new(BufferObjectInner {
                id,
                buf_type: T,
                usage,
                allocated_size: 0,
            })),
        };

        log::debug!("Created {} for {} usage", bo, bo.inner.borrow().usage);

        bo.allocate_data(data);

        bo
    }

    pub fn update_data(&self, data: impl BufferDataSource, offset: usize) -> Result<(), Error> {
        if offset + data.size() > self.inner.borrow().allocated_size {
            return Err(Error::Buffer(BufferError::DataUpdateExceedsBounds {
                allocated_size: self.inner.borrow().allocated_size,
                offset,
                size: data.size(),
            }));
        }

        self.bind();

        unsafe {
            gl::BufferSubData(
                Into::into(T),
                offset as GLintptr,
                data.size() as GLsizeiptr,
                data.ptr(),
            );
        }

        log::trace!(
            "Modified data in {} by setting {} bytes from offset {}",
            self,
            data.size(),
            offset
        );

        Ok(())
    }

    pub fn allocate_data(&self, data: impl BufferDataSource) {
        self.bind();

        unsafe {
            gl::BufferData(
                Into::into(T),
                data.size() as GLsizeiptr,
                data.ptr(),
                self.inner.borrow().usage.into(),
            );
        }

        self.inner.borrow_mut().allocated_size = data.size();

        log::debug!("Allocated {} bytes of data in {}", data.size(), self);
    }

    fn bind(&self) {
        unsafe {
            gl::BindBuffer(Into::into(T), self.id);
        }

        log::trace!("Bound {}", self);
    }
}

impl<const T: BufferType> fmt::Display for BufferObject<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} buffer object {}", T, self.id)
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug, strum_macros::Display)]
#[strum(serialize_all = "snake_case")]
pub enum BufferType {
    Vertex,
    Element,
}

impl From<BufferType> for GLenum {
    fn from(b: BufferType) -> GLenum {
        match b {
            BufferType::Vertex => gl::ARRAY_BUFFER,
            BufferType::Element => gl::ELEMENT_ARRAY_BUFFER,
        }
    }
}

#[derive(Hash, Eq, PartialEq)]
struct BufferObjectInner {
    id: GLuint,
    buf_type: BufferType,
    usage: BufferUsageHint,
    allocated_size: usize,
}

impl Drop for BufferObjectInner {
    fn drop(&mut self) {
        log::debug!("Deleting {} buffer object {}", self.buf_type, self.id);

        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug, strum_macros::Display)]
#[strum(serialize_all = "snake_case")]
pub enum BufferUsageHint {
    Static,
    Dynamic,
}

impl From<BufferUsageHint> for GLenum {
    fn from(b: BufferUsageHint) -> GLenum {
        match b {
            BufferUsageHint::Static => gl::STATIC_DRAW,
            BufferUsageHint::Dynamic => gl::DYNAMIC_DRAW,
        }
    }
}
