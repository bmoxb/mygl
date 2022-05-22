use std::collections::HashMap;
use std::ffi::c_void;
use std::{convert, fmt};

use gl::types::*;

use super::{ElementBufferObject, VertexArrayObject, VertexBufferObject};

#[derive(Copy, Clone, Debug, strum_macros::Display)]
#[strum(serialize_all = "snake_case")]
pub enum VertexAttributeType {
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

impl convert::From<VertexAttributeType> for GLenum {
    fn from(a: VertexAttributeType) -> GLenum {
        match a {
            VertexAttributeType::Byte => gl::BYTE,
            VertexAttributeType::UnsignedByte => gl::UNSIGNED_BYTE,
            VertexAttributeType::Short => gl::SHORT,
            VertexAttributeType::UnsignedShort => gl::UNSIGNED_SHORT,
            VertexAttributeType::Int => gl::INT,
            VertexAttributeType::UnsignedInt => gl::UNSIGNED_INT,
            VertexAttributeType::HalfFloat => gl::HALF_FLOAT,
            VertexAttributeType::Float => gl::FLOAT,
            VertexAttributeType::Double => gl::DOUBLE,
        }
    }
}

#[derive(Debug)]
pub struct VertexAttribute {
    pub layout_index: u32,
    pub component_count: u32,
    pub component_type: VertexAttributeType,
    pub normalize: bool,
    pub stride: u32,
    pub offset: usize,
}

impl Default for VertexAttribute {
    fn default() -> Self {
        VertexAttribute {
            layout_index: 0,
            component_count: 1,
            component_type: VertexAttributeType::Float,
            normalize: false,
            stride: 4,
            offset: 0,
        }
    }
}

impl fmt::Display for VertexAttribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "vertex attribute {} ({} components of type {}, {}normalize, {} stride, {} offset",
            self.layout_index,
            self.component_count,
            self.component_type,
            if self.normalize { "" } else { "not " },
            self.stride,
            self.offset,
        )
    }
}

#[derive(Default)]
pub struct VertexArrayObjectBuilder {
    ebo: Option<ElementBufferObject>,
    vbo_attributes: HashMap<VertexBufferObject, Vec<VertexAttribute>>,
}

impl VertexArrayObjectBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build(self) -> VertexArrayObject {
        let mut vao = VertexArrayObject::new();
        vao.bind();

        for (vbo, attrib_pointers) in &self.vbo_attributes {
            vbo.bind();

            for a in attrib_pointers {
                unsafe {
                    gl::VertexAttribPointer(
                        a.layout_index,
                        a.component_count as GLint,
                        a.component_type.into(),
                        a.normalize as GLboolean,
                        a.stride as GLsizei,
                        a.offset as *const c_void,
                    );
                    gl::EnableVertexAttribArray(a.layout_index);
                }

                log::debug!("Set up and enabled {}", a);
            }
        }

        vao.vbos = self.vbo_attributes.into_keys().collect();

        if let Some(ebo) = &self.ebo {
            ebo.bind();
        }
        vao.ebo = self.ebo;

        vao
    }

    pub fn element_buffer_object(mut self, ebo: &ElementBufferObject) -> Self {
        self.ebo = Some(ebo.clone());
        self
    }

    pub fn attribute(mut self, vbo: &VertexBufferObject, attribute: VertexAttribute) -> Self {
        self.vbo_attributes
            .entry(vbo.clone())
            .or_insert_with(Vec::new)
            .push(attribute);

        self
    }
}
