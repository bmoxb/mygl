use std::{convert, ptr};

use gl::types::*;

use crate::shaders::ShaderProgram;
use crate::textures::{Texture, TextureType};
use crate::vao::VertexArrayObject;

pub fn draw_arrays(
    prog: &ShaderProgram,
    vao: &VertexArrayObject,
    mode: DrawMode,
    first: i32,
    count: i32,
) {
    prog.use_program();
    vao.bind();

    log::trace!(
        "Using {} and {} to draw {} vertices as {} (arrays)",
        prog,
        vao,
        count,
        mode
    );

    unsafe {
        gl::DrawArrays(mode.into(), first, count);
    }
}

pub fn draw_arrays_with_textures<const T: TextureType>(
    prog: &ShaderProgram,
    vao: &VertexArrayObject,
    mode: DrawMode,
    first: i32,
    count: i32,
    textures: &[&Texture<{ T }>],
) {
    activate_textures(textures);
    draw_arrays(prog, vao, mode, first, count);
}

pub fn draw_elements(
    prog: &ShaderProgram,
    vao: &VertexArrayObject,
    index_type: IndexType,
    mode: DrawMode,
    count: i32,
) {
    prog.use_program();
    vao.bind();

    log::trace!(
        "Using {} and {} to draw {} vertices as {} (elements)",
        prog,
        vao,
        count,
        mode
    );

    unsafe {
        gl::DrawElements(mode.into(), count, index_type.into(), ptr::null());
    }
}

pub fn draw_elements_with_textures<const T: TextureType>(
    prog: &ShaderProgram,
    vao: &VertexArrayObject,
    index_type: IndexType,
    mode: DrawMode,
    count: i32,
    textures: &[&Texture<{ T }>],
) {
    activate_textures(textures);
    draw_elements(prog, vao, index_type, mode, count);
}

#[derive(Copy, Clone, Debug)]
pub enum IndexType {
    UnsignedByte,
    UnsignedShort,
    UnsignedInt,
}

impl convert::From<IndexType> for GLenum {
    fn from(i: IndexType) -> GLenum {
        match i {
            IndexType::UnsignedByte => gl::UNSIGNED_BYTE,
            IndexType::UnsignedShort => gl::UNSIGNED_SHORT,
            IndexType::UnsignedInt => gl::UNSIGNED_INT,
        }
    }
}

#[derive(Copy, Clone, Debug, strum_macros::Display)]
#[strum(serialize_all = "snake_case")]
pub enum DrawMode {
    Points,
    Lines,
    Triangles,
}

impl convert::From<DrawMode> for GLenum {
    fn from(d: DrawMode) -> GLenum {
        match d {
            DrawMode::Points => gl::POINTS,
            DrawMode::Lines => gl::LINES,
            DrawMode::Triangles => gl::TRIANGLES,
        }
    }
}

fn activate_textures<const T: TextureType>(textures: &[&Texture<{ T }>]) {
    for (index, texture) in textures.iter().enumerate() {
        texture.activate(index as u32);
    }
}
