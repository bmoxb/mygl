#![allow(incomplete_features)]
#![feature(adt_const_params)]

pub mod debug;
pub mod error;
pub mod rendering;
pub mod shaders;
pub mod textures;
pub mod vao;

pub use error::Error;

use gl::*;

use debug::gl;

/**
 * Clears the screen in the color specified.
 *
 * Each red/green/blue/alpha channel value should be between 0.0 and 1.0
 * inclusive.
 */
pub fn clear(r: f32, g: f32, b: f32, a: f32) {
    gl!(ClearColor(r, g, b, a));
    gl!(Clear(gl::COLOR_BUFFER_BIT));
}

/**
 * Enable wireframe drawing (draw everything as lines instead of filling in
 * faces).
 *
 * Equivalent to: `glPolygonMode(GL_FRONT_AND_BACK, GL_LINE)`
 */
pub fn enable_wireframe_rendering() {
    gl!(PolygonMode(gl::FRONT_AND_BACK, gl::LINE));
}

/**
 * Disable wireframe drawing (draw everything with faces filled in).
 *
 * Equivalent to: `glPolygonMode(GL_FRONT_AND_BACK, GL_FILL)`
 */
pub fn disable_wireframe_rendering() {
    gl!(PolygonMode(gl::FRONT_AND_BACK, gl::FILL));
}

#[cfg(test)]
mod tests {}
