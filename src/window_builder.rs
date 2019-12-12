use boxer::string::{BoxerString, BoxerStringPointer};
use glutin::window::WindowBuilder;
use glutin::dpi::{LogicalSize};
use boxer::boxes::{ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn glutin_create_window_builder() -> *mut ValueBox<WindowBuilder> {
    ValueBox::new(WindowBuilder::new()).into_raw()
}

#[no_mangle]
pub fn glutin_destroy_window_builder(_ptr: *mut ValueBox<WindowBuilder>) {
    _ptr.drop();
}

#[no_mangle]
pub fn glutin_window_builder_with_title(mut _ptr_window_builder: *mut ValueBox<WindowBuilder>, _ptr_boxer_string: *mut BoxerString) {
    _ptr_window_builder.with_not_null_value_mutate_consumed(|builder| {
        _ptr_boxer_string.with(|string| {
            builder.with_title(string.to_string())
        })
    });
}

#[no_mangle]
pub fn glutin_window_builder_with_decorations(mut _ptr_window_builder: *mut ValueBox<WindowBuilder>, with_decorations: bool) {
    _ptr_window_builder.with_not_null_value_mutate_consumed(|builder| builder.with_decorations(with_decorations))
}

#[no_mangle]
pub fn glutin_window_builder_with_transparency(mut _ptr_window_builder: *mut ValueBox<WindowBuilder>, with_transparency: bool) {
    _ptr_window_builder.with_not_null_value_mutate_consumed(|builder| builder.with_transparent(with_transparency))
}

#[no_mangle]
pub fn glutin_window_builder_with_resizable(mut _ptr_window_builder: *mut ValueBox<WindowBuilder>, with_resizable: bool) {
    _ptr_window_builder.with_not_null_value_mutate_consumed(|builder| builder.with_resizable(with_resizable))
}

#[no_mangle]
pub fn glutin_window_builder_with_dimensions(mut _ptr_window_builder: *mut ValueBox<WindowBuilder>, width: f64, height: f64) {
    _ptr_window_builder.with_not_null_value_mutate_consumed(|builder| builder.with_inner_size(LogicalSize::new(width, height)))
}

#[no_mangle]
pub fn glutin_window_builder_with_maximized(mut _ptr_window_builder: *mut ValueBox<WindowBuilder>, with_maximized: bool) {
    _ptr_window_builder.with_not_null_value_mutate_consumed(|builder| builder.with_maximized(with_maximized))
}

#[no_mangle]
pub fn glutin_window_builder_with_visibility(mut _ptr_window_builder: *mut ValueBox<WindowBuilder>, with_visibility: bool) {
    _ptr_window_builder.with_not_null_value_mutate_consumed(|builder| builder.with_visible(with_visibility))
}

#[no_mangle]
pub fn glutin_window_builder_with_always_on_top(mut _ptr_window_builder: *mut ValueBox<WindowBuilder>, with_always_on_top: bool) {
    _ptr_window_builder.with_not_null_value_mutate_consumed(|builder| builder.with_always_on_top(with_always_on_top))
}