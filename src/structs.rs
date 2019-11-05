use boxer::CBox;

#[repr(C)]
pub struct GlutinGL {
    pub gl: *const dyn gleam::gl::Gl,
}

impl GlutinGL {
    pub fn with_raw<F, R>(this: *mut GlutinGL, block: F) -> R where F : Fn(&std::rc::Rc<dyn gleam::gl::Gl>) -> R {
        CBox::with_raw(this, |glutin_gl| {
            let _gl: std::rc::Rc<dyn gleam::gl::Gl> = unsafe { std::rc::Rc::from_raw(glutin_gl.gl) };
            let result: R = block(&_gl);
            std::rc::Rc::into_raw(_gl);
            result
        })
    }
}