pub mod error;
pub mod shaders;
pub mod vao;

pub use error::Error;

pub fn clear(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        gl::ClearColor(r, g, b, a);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

pub fn enable_wireframe_rendering() {
    unsafe {
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }
}

pub fn disable_wireframe_rendering() {
    unsafe {
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
    }
}

#[cfg(test)]
mod tests {}
