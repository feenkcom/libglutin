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

pub struct CBox {}

impl CBox {
    pub fn into_raw<T> (object: T) -> *mut T {
        Box::into_raw(Box::new(object))
    }

    pub fn from_raw<T>(pointer: *mut T) -> Box<T> {
        assert_eq!(pointer.is_null(), false, "CBox::from_raw(): Pointer must not be null!");
        unsafe { Box::from_raw(pointer) }
    }

    pub fn drop<T>(pointer: *mut T) {
        if pointer.is_null() {
            return;
        }
        CBox::from_raw(pointer);
    }

    pub fn to_string(_ptr_chars: *const c_char) -> String {
        let title_string = unsafe {
            CStr::from_ptr(_ptr_chars).to_string_lossy().into_owned()
        };
        title_string
    }

    pub fn with_raw<F, R, T>(pointer: *mut T, block: F) -> R where F : FnOnce(&mut Box<T>) -> R {
        assert_eq!(pointer.is_null(), false, "CBox::with_raw(): Pointer must not be null!");

        let mut boxed_object: Box<T> = Self::from_raw(pointer);
        let result: R = block(&mut boxed_object);
        Self::into_raw(boxed_object);
        result
    }

    pub fn with_two_raw<F, R, T1, T2>(pointer_1: *mut T1, pointer_2: *mut T2, block: F) -> R where F : FnOnce(&mut Box<T1>, &mut Box<T2>) -> R {
        assert_eq!(pointer_1.is_null(), false, "CBox::with_raw(): Pointer #1 must not be null!");
        assert_eq!(pointer_2.is_null(), false, "CBox::with_raw(): Pointer #2 must not be null!");

        let mut boxed_object_1: Box<T1> = Self::from_raw(pointer_1);
        let mut boxed_object_2: Box<T2> = Self::from_raw(pointer_2);
        let result: R = block(&mut boxed_object_1, &mut boxed_object_2);
        Self::into_raw(boxed_object_1);
        Self::into_raw(boxed_object_2);
        result
    }

    pub fn with_three_raw<F, R, T1, T2, T3>(pointer_1: *mut T1, pointer_2: *mut T2, pointer_3: *mut T3, block: F)
                                            -> R where F : FnOnce(&mut Box<T1>, &mut Box<T2>, &mut Box<T3>) -> R {
        assert_eq!(pointer_1.is_null(), false, "CBox::with_raw(): Pointer #1 must not be null!");
        assert_eq!(pointer_2.is_null(), false, "CBox::with_raw(): Pointer #2 must not be null!");
        assert_eq!(pointer_3.is_null(), false, "CBox::with_raw(): Pointer #3 must not be null!");

        let mut boxed_object_1: Box<T1> = Self::from_raw(pointer_1);
        let mut boxed_object_2: Box<T2> = Self::from_raw(pointer_2);
        let mut boxed_object_3: Box<T3> = Self::from_raw(pointer_3);
        let result: R = block(&mut boxed_object_1, &mut boxed_object_2, &mut boxed_object_3);
        Self::into_raw(boxed_object_1);
        Self::into_raw(boxed_object_2);
        Self::into_raw(boxed_object_3);
        result
    }

    pub fn with_window_builder<F>(_ptr_window_builder: *mut WindowBuilder, block: F) -> *mut WindowBuilder where F : FnOnce(WindowBuilder) -> WindowBuilder {
        CBox::with_raw(_ptr_window_builder, |builder| {
            let mut window_builder_tmp = WindowBuilder::new();
            window_builder_tmp.clone_from(builder);
            window_builder_tmp = block(window_builder_tmp);
            let mut _ptr_window_builder = CBox::into_raw(window_builder_tmp);
            _ptr_window_builder
        })
    }
}