#![allow(non_snake_case)]

#[macro_use]
extern crate log;

pub mod context_builder;
pub mod enums;
pub mod event_loop;
pub mod events;
pub mod headless_context;
pub mod pixel_format;
pub mod pixel_format_requirements;
pub mod window_builder;
pub mod windowed_context;

#[cfg(target_os = "macos")]
#[path = "platform/macos.rs"]
mod ext;

#[cfg(all(not(target_os = "macos")))]
#[path = "platform/others.rs"]
mod ext;

use geometry_box::U128Box;
use glutin::window::WindowId;
use glutin::Api;
use std::mem::transmute_copy;
use string_box::StringBox;
use value_box::{ValueBox, ValueBoxPointer};

pub use value_box_ffi::*;

///////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////// L I B R A R Y /////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

/// All APIs related to OpenGL that you can possibly get while using glutin.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ContextApi {
    /// The classical OpenGL. Available on Windows, Unix operating systems,
    /// OS/X.
    OpenGl,
    /// OpenGL embedded system. Available on Unix operating systems, Android.
    OpenGlEs,
    /// OpenGL for the web. Very similar to OpenGL ES.
    WebGl,
    Unknown,
}

impl From<Api> for ContextApi {
    fn from(api: Api) -> Self {
        match api {
            Api::OpenGl => ContextApi::OpenGl,
            Api::OpenGlEs => ContextApi::OpenGlEs,
            Api::WebGl => ContextApi::WebGl,
        }
    }
}

#[no_mangle]
pub fn glutin_test() -> bool {
    return true;
}

#[no_mangle]
pub fn glutin_init_logger() {
    env_logger::init();
}

#[no_mangle]
pub fn glutin_println(_ptr_message: *mut ValueBox<StringBox>) {
    _ptr_message.with_not_null(|message| println!("{}", message.to_string()));
}

#[no_mangle]
pub fn glutin_print(_ptr_message: *mut ValueBox<StringBox>) {
    _ptr_message.with_not_null(|message| print!("{}", message.to_string()));
}

pub fn glutin_convert_window_id(window_id: WindowId) -> U128Box {
    let size = std::mem::size_of::<WindowId>();

    let id_128: u128 = match size {
        4 => {
            // u32
            let id: u32 = unsafe { transmute_copy::<WindowId, u32>(&window_id) };
            id as u128
        }
        8 => {
            // u64
            let id: u64 = unsafe { transmute_copy::<WindowId, u64>(&window_id) };
            id as u128
        }
        16 => {
            //u128
            let id: u128 = unsafe { transmute_copy::<WindowId, u128>(&window_id) };
            id
        }
        _ => {
            eprintln!("Unknown size of window id ({:?})", window_id);
            0 as u128
        }
    };

    id_128.into()
}
