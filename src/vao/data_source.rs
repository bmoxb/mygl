use std::ffi::c_void;
use std::mem;

pub trait BufferDataSource {
    fn ptr(&self) -> *const c_void;
    fn size(&self) -> usize;
}

impl<T, const N: usize> BufferDataSource for &[T; N] {
    fn ptr(&self) -> *const c_void {
        let ptr = *self as *const T;
        ptr as *const c_void
    }
    fn size(&self) -> usize {
        mem::size_of::<[T; N]>()
    }
}
