use glutin::dpi::LogicalSize;
use glutin::window::WindowBuilder;
use string_box::StringBox;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn glutin_create_window_builder() -> *mut ValueBox<WindowBuilder> {
    ValueBox::new(WindowBuilder::new()).into_raw()
}

#[no_mangle]
pub fn glutin_destroy_window_builder(_ptr: *mut ValueBox<WindowBuilder>) {
    _ptr.release();
}

#[no_mangle]
pub fn glutin_window_builder_with_title(
    mut _ptr_window_builder: *mut ValueBox<WindowBuilder>,
    _ptr_boxer_string: *mut ValueBox<StringBox>,
) {
    _ptr_boxer_string.with_not_null(|title| {
        _ptr_window_builder
            .replace_value(|builder| builder.with_title(title.to_string()))
            .log();
    });
}

#[no_mangle]
pub fn glutin_window_builder_with_decorations(
    mut _ptr_window_builder: *mut ValueBox<WindowBuilder>,
    with_decorations: bool,
) {
    _ptr_window_builder
        .replace_value(|builder| builder.with_decorations(with_decorations))
        .log();
}

#[no_mangle]
pub fn glutin_window_builder_with_transparency(
    mut _ptr_window_builder: *mut ValueBox<WindowBuilder>,
    with_transparency: bool,
) {
    _ptr_window_builder
        .replace_value(|builder| builder.with_transparent(with_transparency))
        .log();
}

#[no_mangle]
pub fn glutin_window_builder_with_resizable(
    mut _ptr_window_builder: *mut ValueBox<WindowBuilder>,
    with_resizable: bool,
) {
    _ptr_window_builder
        .replace_value(|builder| builder.with_resizable(with_resizable))
        .log();
}

#[no_mangle]
pub fn glutin_window_builder_with_dimensions(
    mut _ptr_window_builder: *mut ValueBox<WindowBuilder>,
    width: f64,
    height: f64,
) {
    _ptr_window_builder
        .replace_value(|builder| builder.with_inner_size(LogicalSize::new(width, height)))
        .log();
}

#[no_mangle]
pub fn glutin_window_builder_with_maximized(
    mut _ptr_window_builder: *mut ValueBox<WindowBuilder>,
    with_maximized: bool,
) {
    _ptr_window_builder
        .replace_value(|builder| builder.with_maximized(with_maximized))
        .log();
}

#[no_mangle]
pub fn glutin_window_builder_with_visibility(
    mut _ptr_window_builder: *mut ValueBox<WindowBuilder>,
    with_visibility: bool,
) {
    _ptr_window_builder
        .replace_value(|builder| builder.with_visible(with_visibility))
        .log();
}

#[no_mangle]
pub fn glutin_window_builder_with_always_on_top(
    mut _ptr_window_builder: *mut ValueBox<WindowBuilder>,
    with_always_on_top: bool,
) {
    _ptr_window_builder
        .replace_value(|builder| builder.with_always_on_top(with_always_on_top))
        .log();
}
