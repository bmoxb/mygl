use std::collections::HashMap;
use std::{convert, ptr};

use gl::types::*;

use super::{ElementBufferObject, VertexArrayObject, VertexBufferObject};

#[derive(Copy, Clone, Debug)]
pub enum AttribPointerType {
    Byte,
    UnsignedByte,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    HalfFloat,
    Float,
    Double,
}

impl convert::From<AttribPointerType> for GLenum {
    fn from(a: AttribPointerType) -> GLenum {
        match a {
            AttribPointerType::Byte => gl::BYTE,
            AttribPointerType::UnsignedByte => gl::UNSIGNED_BYTE,
            AttribPointerType::Short => gl::SHORT,
            AttribPointerType::UnsignedShort => gl::UNSIGNED_SHORT,
            AttribPointerType::Int => gl::INT,
            AttribPointerType::UnsignedInt => gl::UNSIGNED_INT,
            AttribPointerType::HalfFloat => gl::HALF_FLOAT,
            AttribPointerType::Float => gl::FLOAT,
            AttribPointerType::Double => gl::DOUBLE,
        }
    }
}

struct AttribPointer {
    index: u32,
    size: i32,
    ty: AttribPointerType,
    normalized: bool,
    stride: i32,
}

#[derive(Default)]
pub struct VertexArrayObjectBuilder {
    ebo: Option<ElementBufferObject>,
    vbo_attrib_pointers: HashMap<VertexBufferObject, Vec<AttribPointer>>,
}

impl VertexArrayObjectBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build(self) -> VertexArrayObject {
        let mut vao = VertexArrayObject::new();
        vao.bind();

        for (vbo, attrib_pointers) in &self.vbo_attrib_pointers {
            vbo.bind();

            for a in attrib_pointers {
                unsafe {
                    gl::VertexAttribPointer(
                        a.index,
                        a.size,
                        a.ty.into(),
                        a.normalized as GLboolean,
                        a.stride,
                        ptr::null(),
                    );
                    gl::EnableVertexAttribArray(a.index);
                }

                log::debug!("Set up and enabled attribute {}", a.index);
            }
        }

        vao.vbos = self.vbo_attrib_pointers.into_keys().collect();

        if let Some(ebo) = &self.ebo {
            ebo.bind();
        }
        vao.ebo = self.ebo;

        vao
    }

    pub fn element_buffer_object(mut self, ebo: ElementBufferObject) -> Self {
        self.ebo = Some(ebo);
        self
    }

    pub fn attrib_pointer(
        mut self,
        vbo: &VertexBufferObject,
        index: u32,
        size: i32,
        ty: AttribPointerType,
        normalized: bool,
        stride: i32,
    ) -> Self {
        let attrib = AttribPointer {
            index,
            size,
            ty,
            normalized,
            stride,
        };
        self.vbo_attrib_pointers
            .entry(vbo.clone())
            .or_insert_with(Vec::new)
            .push(attrib);
        self
    }
}
