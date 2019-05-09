use super::*;

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct GlutinSizeU32 {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct GlutinSizeU64 {
    pub x: u64,
    pub y: u64,
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct GlutinSizeF64 {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct GlutinFetchedEvents {
    pub data: *mut GlutinEvent,
    pub length: usize
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct GlutinCString {
    pub data: *mut c_char,
    pub length: usize,
}

#[repr(C)]
pub struct GlutinGL {
    pub gl: *const gleam::gl::Gl,
}

impl GlutinGL {
    pub fn with_raw<F, R>(this: *mut GlutinGL, block: F) -> R where F : Fn(&std::rc::Rc<gleam::gl::Gl>) -> R {
        // does not consume
        let _glutin_gl: &GlutinGL = { unsafe { &mut *this } };
        let _gl: std::rc::Rc<gleam::gl::Gl> = unsafe { std::rc::Rc::from_raw(_glutin_gl.gl) };

        let result: R = block(&_gl);

        std::rc::Rc::into_raw(_gl);
        result
    }
}