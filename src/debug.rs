use std::ffi::{c_void, CString};
use std::{mem, slice};

use gl::types::*;

pub fn set_error_callback(callback: fn(&str)) {
    log::debug!("Error callback function set");
    unsafe { gl::DebugMessageCallback(Some(error_callback), callback as *const c_void) }
}

extern "system" fn error_callback(
    source: GLenum,
    gltype: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    msg_raw: *const GLchar,
    user_param: *mut c_void,
) {
    let buffer = unsafe { slice::from_raw_parts(msg_raw, length as usize).to_vec() };
    let msg = CString::new::<Vec<u8>>(buffer.into_iter().map(|c| c as u8).collect())
        .unwrap()
        .into_string()
        .unwrap();

    log::error!(
        "OpenGL error callback called: source = {}, type = {}, id = {}, severity = {} - {}",
        source,
        gltype,
        id,
        severity,
        msg
    );

    let ptr = user_param as *const ();
    let callback: fn(&str) = unsafe { mem::transmute(ptr) };
    callback(&msg);
}
