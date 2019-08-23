use super::*;

#[no_mangle]
pub fn glutin_window_builder_with_full_size(_ptr_window_builder: *mut WindowBuilder, _with_full_size: bool) -> *mut WindowBuilder {
    return _ptr_window_builder;
}