use glutin::window::WindowBuilder;
use value_box::ValueBox;

#[no_mangle]
pub fn glutin_window_builder_with_full_size(
    _ptr_window_builder: *mut ValueBox<WindowBuilder>,
    _with_full_size: bool,
) {
}
