use glutin::window::WindowBuilder;
use boxer::boxes::ValueBox;

#[no_mangle]
pub fn glutin_window_builder_with_full_size(_ptr_window_builder: *mut ValueBox<WindowBuilder>, _with_full_size: bool) {

}