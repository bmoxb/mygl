use std::convert;

use gl::types::*;

use crate::shaders::ShaderProgram;
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
        "Using {} and {} to draw {} vertices ({:?})",
        prog,
        vao,
        count,
        mode
    );

    unsafe {
        gl::DrawArrays(mode.into(), first, count);
    }
}

pub fn draw_elements() {
    unimplemented!()
}

#[derive(Copy, Clone, Debug)]
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
