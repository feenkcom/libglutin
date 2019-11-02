use boxer::CBox;
use boxer::string::BoxerString;
use glutin::window::WindowBuilder;
use glutin::dpi::{LogicalSize};

#[no_mangle]
pub fn glutin_create_window_builder() -> *mut WindowBuilder {
    CBox::into_raw(WindowBuilder::new())
}

#[no_mangle]
pub fn glutin_destroy_window_builder(_ptr: *mut WindowBuilder) {
    CBox::drop(_ptr);
}

#[no_mangle]
pub fn glutin_window_builder_with_title(_ptr_window_builder: *mut WindowBuilder, _ptr_boxer_string: *mut BoxerString) -> *mut WindowBuilder {
    CBox::with_consumable_raw(_ptr_window_builder, |builder| {
        CBox::with_raw(_ptr_boxer_string, |string| {
          builder.with_title(string.to_string())
        })
    })
}

#[no_mangle]
pub fn glutin_window_builder_with_decorations(_ptr_window_builder: *mut WindowBuilder, with_decorations: bool) -> *mut WindowBuilder {
    CBox::with_consumable_raw(_ptr_window_builder, |builder| builder.with_decorations(with_decorations))
}

#[no_mangle]
pub fn glutin_window_builder_with_transparency(_ptr_window_builder: *mut WindowBuilder, with_transparency: bool) -> *mut WindowBuilder {
    CBox::with_consumable_raw(_ptr_window_builder, |builder| builder.with_transparent(with_transparency))
}

#[no_mangle]
pub fn glutin_window_builder_with_resizable(_ptr_window_builder: *mut WindowBuilder, with_resizable: bool) -> *mut WindowBuilder {
    CBox::with_consumable_raw(_ptr_window_builder, |builder| builder.with_resizable(with_resizable))
}

#[no_mangle]
pub fn glutin_window_builder_with_dimensions(_ptr_window_builder: *mut WindowBuilder, width: f64, height: f64) -> *mut WindowBuilder {
    CBox::with_consumable_raw(_ptr_window_builder, |builder| builder.with_inner_size(LogicalSize::new(width, height)))
}

#[no_mangle]
pub fn glutin_window_builder_with_maximized(_ptr_window_builder: *mut WindowBuilder, with_maximized: bool) -> *mut WindowBuilder {
    CBox::with_consumable_raw(_ptr_window_builder, |builder| builder.with_maximized(with_maximized))
}

#[no_mangle]
pub fn glutin_window_builder_with_visibility(_ptr_window_builder: *mut WindowBuilder, with_visibility: bool) -> *mut WindowBuilder {
    CBox::with_consumable_raw(_ptr_window_builder, |builder| builder.with_visible(with_visibility))
}

#[no_mangle]
pub fn glutin_window_builder_with_always_on_top(_ptr_window_builder: *mut WindowBuilder, with_always_on_top: bool) -> *mut WindowBuilder {
    CBox::with_consumable_raw(_ptr_window_builder, |builder| builder.with_always_on_top(with_always_on_top))
}