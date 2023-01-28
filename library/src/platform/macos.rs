#![cfg(target_os = "macos")]

use glutin::platform::macos::WindowBuilderExtMacOS;
use glutin::window::WindowBuilder;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn glutin_window_builder_with_full_size(
    _ptr_window_builder: *mut ValueBox<WindowBuilder>,
    _with_full_size: bool,
) {
    _ptr_window_builder
        .replace_value(|builder| {
            builder
                .with_titlebar_transparent(_with_full_size)
                .with_fullsize_content_view(_with_full_size)
                .with_title_hidden(_with_full_size)
        })
        .log();
}
