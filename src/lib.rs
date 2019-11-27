#![allow(non_snake_case)]

extern crate boxer;
extern crate gleam;
extern crate glutin;
extern crate libc;

pub mod context;
pub mod context_builder;
pub mod enums;
pub mod event_loop;
pub mod events;
pub mod opengl;
pub mod pixel_format;
pub mod pixel_format_requirements;
pub mod structs;
pub mod window;
pub mod window_builder;

#[cfg(target_os = "macos")]
#[path = "platform/macos.rs"]
mod ext;

#[cfg(all(not(target_os = "macos")))]
#[path = "platform/others.rs"]
mod ext;

use boxer::number::BoxerUint128;
use boxer::string::BoxerString;
use boxer::CBox;

use glutin::window::WindowId;

use std::mem::transmute_copy;

///////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////// L I B R A R Y /////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub fn glutin_test() -> bool {
    return true;
}

#[no_mangle]
pub fn glutin_println(_ptr_message: *mut BoxerString) {
    CBox::with_optional_raw(_ptr_message, |option| match option {
        None => {}
        Some(message) => println!("{}", message.to_string()),
    });
}

#[no_mangle]
pub fn glutin_print(_ptr_message: *mut BoxerString) {
    CBox::with_optional_raw(_ptr_message, |option| match option {
        None => {}
        Some(message) => print!("{}", message.to_string()),
    });
}

pub fn glutin_convert_window_id(window_id: WindowId) -> BoxerUint128 {
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
