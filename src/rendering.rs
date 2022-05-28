//! Module containing functions that facilitate rendering to the OpenGL context.
//!
//! All rendering functions assume nothing about the current OpenGL state in the
//! sense that all shader programs, vertex array objects, etc. that passed into
//! a rendering function are activated, bound, etc. in the function body as
//! necessary - prior calls to [`VertexArrayObject::bind`],
//! [`ShaderProgram::use_program`], and similar are not necessary prior to
//! calling any rendering functions.

use std::{convert, ptr};

use gl::types::*;

use crate::shaders::ShaderProgram;
use crate::textures::{Texture, TextureType};
use crate::vao::VertexArrayObject;

/**
 * Draw a vertex array.
 *
 * Works by using the shader program, binding the vertex array object (VAO),
 * and then calling the unsafe OpenGL function `glDrawArrays`.
 *
 * The passed VAO should have at least one vertex array object and at least one
 * attribute pointer set.
 *
 * The `fist` and `count` parameters indicate the vertex to begin drawing from and
 * the total number of vertices to draw respectively.
 *
 * # Examples
 *
 * ```
 * let vert = VertexShader::from_file("example.vert").unwrap();
 * let frag = FragmentShader::from_file("example.frag").unwrap();
 * let prog = ShaderProgram::new(&vert, &frag).unwrap();
 *
 * let data: [f32; 9] = [
 *     -0.5, -0.5, 0.0,
 *      0.5, -0.5, 0.0,
 *      0.0,  0.5, 0.0,
 * ];
 * let vbo = VertexBufferObject::new(&data, BufferUsageHint::Static);
 *
 *
 * let vao = VertexArrayObjectBuilder::new()
 *     .attribute(
 *         &vbo,
 *         VertexAttribute {
 *             layout_index: 0,
 *             component_count: 3,
 *             component_type: VertexAttributeType::Float,
 *             normalize: false,
 *             stride: 3 * 4,
 *             offset: 0,
 *         },
 *     )
 *     .build();
 *
 * while window.is_open() { // render loop
 *     rendering::draw_arrays(&prog, &vao, DrawMode::Triangles, 0, 3);
 *     window.swap_buffers();
 * }
 * ```
 */
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

/**
 * Draw a vertex array with some set of textures.
 *
 * Each texture is activated according to its index in the array passed (i.e.,
 * the first texture is activated at `GL_TEXTURE0`, the second at `GL_TEXTURE1`,
 * and so on).
 *
 * The passed VAO should have at least one vertex array object and at least one
 * attribute pointer set.
 *
 * The `fist` and `count` parameters indicate the vertex to begin drawing from and
 * the total number of vertices to draw respectively.
 */
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

/**
 * Draw a vertex array using indices.
 *
 * The passed vertex array object is expected to have an element buffer object
 * set containing indices of the type `index_type`.
 *
 * The `count` parameter specifies the number of elements/indices to draw (not
 * vertices).
 */
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

/**
 * Draw a vertex array using indices and some set of textures.
 *
 * The passed vertex array object is expected to have an element buffer object
 * set containing indices of the type `index_type`.
 *
 * Each texture is activated according to its index in the array passed (i.e.,
 * the first texture is activated at `GL_TEXTURE0`, the second at `GL_TEXTURE1`,
 * and so on).
 *
 * The `count` parameter specifies the number of elements/indices to draw (not
 * vertices).
 */
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

/**
 * Represents the type of indices contained within an element buffer object.
 */
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

/**
 * Represents the draw mode to use. Each variant is mapped to an OpenGL enum of
 * the same name (e.g., `DrawMap::Triangles` becomes `GL_TRIANGLES`) so consult
 * the OpenGL documentation for an explanation of each of the different draw
 * modes.
 */
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
