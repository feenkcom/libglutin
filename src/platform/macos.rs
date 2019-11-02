#![cfg(target_os = "macos")]

use glutin::platform::macos::WindowBuilderExtMacOS;
use boxer::CBox;
use glutin::window::WindowBuilder;

#[no_mangle]
pub fn glutin_window_builder_with_full_size(_ptr_window_builder: *mut WindowBuilder, _with_full_size: bool) -> *mut WindowBuilder {
    CBox::with_consumable_raw(_ptr_window_builder, |builder| {
        builder
            .with_titlebar_transparent(_with_full_size)
            .with_fullsize_content_view(_with_full_size)
            .with_title_hidden(_with_full_size)
    })
}