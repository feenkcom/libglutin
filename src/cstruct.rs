use super::*;

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct GlutinSizeU64 {
    pub x: u64,
    pub y: u64,
}

#[no_mangle]
pub fn glutin_create_size_u64() -> *mut GlutinSizeU64 {
    CBox::into_raw(GlutinSizeU64::default())
}

#[no_mangle]
pub fn glutin_destroy_size_u64(_ptr: *mut GlutinSizeU64) {
    CBox::drop(_ptr)
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct GlutinSizeF64 {
    pub x: f64,
    pub y: f64,
}

#[no_mangle]
pub fn glutin_create_size_f64() -> *mut GlutinSizeF64 {
    CBox::into_raw(GlutinSizeF64::default())
}

#[no_mangle]
pub fn glutin_destroy_size_f64(_ptr: *mut GlutinSizeF64) {
    CBox::drop(_ptr)
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct GlutinCString {
    pub data: *mut c_char,
    pub length: usize,
}

#[repr(C)]
pub struct GlutinGL {
    pub gl: *const dyn gleam::gl::Gl,
}

impl GlutinGL {
    pub fn with_raw<F, R>(this: *mut GlutinGL, block: F) -> R where F : Fn(&std::rc::Rc<dyn gleam::gl::Gl>) -> R {
        // does not consume
        let _glutin_gl: &GlutinGL = { unsafe { &mut *this } };
        let _gl: std::rc::Rc<dyn gleam::gl::Gl> = unsafe { std::rc::Rc::from_raw(_glutin_gl.gl) };

        let result: R = block(&_gl);

        std::rc::Rc::into_raw(_gl);
        result
    }
}