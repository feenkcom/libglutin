#![cfg(target_os = "macos")]

use super::*;

use platform::macos::WindowBuilderExtMacOS;

#[no_mangle]
pub fn glutin_window_builder_with_full_size(_ptr_window_builder: *mut WindowBuilder, _with_full_size: bool) -> *mut WindowBuilder {
    CBox::with_window_builder(_ptr_window_builder, |builder| {
        builder
            .with_titlebar_transparent(_with_full_size)
            .with_fullsize_content_view(_with_full_size)
            .with_title_hidden(_with_full_size)
    })
}