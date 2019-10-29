#![allow(non_snake_case)]

extern crate glutin;
extern crate libc;
extern crate gleam;
extern crate boxer;

use std::mem::transmute;
use std::mem::transmute_copy;

use glutin::*;
use glutin::dpi::*;
use glutin::window::*;
use glutin::event_loop::*;
use glutin::monitor::*;

use gleam::gl;

use boxer::CBox;
use boxer::number::BoxerUint128;
use boxer::size::{BoxerSizeF64};
use boxer::point::{BoxerPointF64};

pub mod structs;
pub mod opengl;
pub mod enums;
pub mod events;
pub mod window;

#[cfg(target_os = "macos")]
#[path = "platform/macos.rs"]
mod ext;

#[cfg(all(
    not(target_os = "macos")
))]
#[path = "platform/others.rs"]
mod ext;

use structs::*;
use std::time;
use enums::GlutinCursorIcon;
use glutin::platform::desktop::EventLoopExtDesktop;
use events::*;
use boxer::string::BoxerString;

///////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////// L I B R A R Y /////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub fn glutin_test() -> bool {
    return true
}

#[no_mangle]
pub fn glutin_println(_ptr_message: *mut BoxerString) {
    CBox::with_optional_raw(_ptr_message, |option| match option {
        None => {},
        Some(message) => println!("{}", message.to_string())
    });
}

#[no_mangle]
pub fn glutin_print(_ptr_message: *mut BoxerString) {
    CBox::with_optional_raw(_ptr_message, |option| match option {
        None => {},
        Some(message) => print!("{}", message.to_string())
    });
}

pub fn glutin_convert_window_id(window_id: WindowId) -> BoxerUint128 {
    let size = std::mem::size_of::<WindowId>();

    let id_128: u128 = match size {
        4 => { // u32
            let id: u32 = unsafe { transmute_copy::<WindowId, u32>(&window_id) };
            id as u128
        },
        8 => { // u64
            let id: u64 = unsafe { transmute_copy::<WindowId, u64>(&window_id) };
            id as u128
        },
        16 => { //u128
            let id: u128 = unsafe { transmute_copy::<WindowId, u128>(&window_id) };
            id
        },
        _ => {
            eprintln!("Unknown size of window id ({:?})", window_id);
            0 as u128 }
    };

    id_128.into()
}

///////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////// E V E N T S  L O O P /////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub fn glutin_create_events_loop() -> *mut EventLoop<()> {
    CBox::into_raw(EventLoop::new())
}

#[no_mangle]
pub fn glutin_destroy_events_loop(_ptr: *mut EventLoop<()>) {
    CBox::drop(_ptr);
}

#[no_mangle]
pub fn glutin_events_loop_run_return(_ptr_events_loop: *mut EventLoop<()>, callback: extern fn(*mut GlutinEvent) -> GlutinControlFlow) {
    if _ptr_events_loop.is_null() {
        eprintln!("[glutin_events_loop_run_forever] _ptr_events_loop is null");
        return;
    }

    CBox::with_raw(_ptr_events_loop, |events_loop| {
        events_loop.run_return(|event, _events_loop: &EventLoopWindowTarget<()>, control_flow: &mut ControlFlow| {
            *control_flow = ControlFlow::Poll;
            let mut c_event: GlutinEvent = Default::default();
            let processed = glutin_events_loop_process_event(event, &mut c_event);
            if processed {
                let c_event_ptr = CBox::into_raw( c_event);
                let c_control_flow = callback(c_event_ptr);
                CBox::drop(c_event_ptr);
                match c_control_flow {
                    GlutinControlFlow::Poll => { *control_flow = ControlFlow::Poll }
                    GlutinControlFlow::Wait => { *control_flow = ControlFlow::WaitUntil(time::Instant::now() + time::Duration::new(0, 50 * 1000000)) }
                    GlutinControlFlow::Exit => { *control_flow = ControlFlow::Exit }
                }
            }
        });
    });
}

///////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////// M O N I T O R    I D /////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[no_mangle]
fn glutin_events_loop_get_primary_monitor(_ptr_event_loop: *mut EventLoop<()>) -> *mut MonitorHandle {
    CBox::with_raw(_ptr_event_loop, |event_loop| {
        CBox::into_raw(event_loop.primary_monitor())
    })
}

#[no_mangle]
fn glutin_primary_monitor_free (_ptr_monitor_id: *mut MonitorHandle) {
    CBox::drop(_ptr_monitor_id);
}

#[no_mangle]
fn glutin_primary_monitor_get_hidpi_factor (_ptr_monitor_id: *mut MonitorHandle) -> f64 {
    CBox::with_raw(_ptr_monitor_id, |monitor_id| monitor_id.hidpi_factor() )
}

///////////////////////////////////////////////////////////////////////////////////////
//////////////////////////// W I N D O W    B U I L D E R /////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

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

///////////////////////////////////////////////////////////////////////////////////////
//////////////////////////// C O N T E X T    B U I L D E R ////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub fn glutin_create_context_builder() -> *mut ContextBuilder<'static, NotCurrent> {
    let context_builder = ContextBuilder::new()
        .with_gl_robustness(Robustness::TryRobustNoResetNotification)
        .with_gl_profile(GlProfile::Core);
    CBox::into_raw(context_builder)
}

#[no_mangle]
pub fn glutin_context_builder_with_gl_then_gles(_ptr_context_builder: *mut ContextBuilder<'static, NotCurrent>, gl_major: u8, gl_minor: u8, gles_major: u8, gles_minor: u8) -> *mut ContextBuilder<'static, NotCurrent> {
    CBox::with_consumable_raw(_ptr_context_builder, |builder| {
        builder.with_gl(GlRequest::GlThenGles {
            /// The version to use for OpenGL.
            opengl_version: (gl_major, gl_minor),
            /// The version to use for OpenGL ES.
            opengles_version: (gles_major, gles_minor),
        })
    })
}

#[no_mangle]
pub fn glutin_context_builder_with_gl_latest(_ptr_context_builder: *mut ContextBuilder<'static,NotCurrent>) -> *mut ContextBuilder<'static, NotCurrent> {
    CBox::with_consumable_raw(_ptr_context_builder, |builder| builder.with_gl(GlRequest::Latest))
}

#[no_mangle]
pub fn glutin_context_builder_with_gl_profile_core(_ptr_context_builder: *mut ContextBuilder<'static,NotCurrent>) -> *mut ContextBuilder<'static, NotCurrent> {
    CBox::with_consumable_raw(_ptr_context_builder, |builder| builder.with_gl_profile(GlProfile::Core))
}

#[no_mangle]
pub fn glutin_context_builder_with_multisampling(_ptr_context_builder: *mut ContextBuilder<'static,NotCurrent>, samples: u16) -> *mut ContextBuilder<'static, NotCurrent> {
    CBox::with_consumable_raw(_ptr_context_builder, |builder| builder.with_multisampling(samples))
}

#[no_mangle]
pub fn glutin_context_builder_with_depth_buffer(_ptr_context_builder: *mut ContextBuilder<'static,NotCurrent>, bits: u8) -> *mut ContextBuilder<'static, NotCurrent> {
    CBox::with_consumable_raw(_ptr_context_builder, |builder| builder.with_depth_buffer(bits))
}

#[no_mangle]
pub fn glutin_context_builder_with_stencil_buffer(_ptr_context_builder: *mut ContextBuilder<'static,NotCurrent>, bits: u8) -> *mut ContextBuilder<'static, NotCurrent> {
    CBox::with_consumable_raw(_ptr_context_builder, |builder| builder.with_stencil_buffer(bits))
}

#[no_mangle]
pub fn glutin_context_builder_with_pixel_format(_ptr_context_builder: *mut ContextBuilder<'static,NotCurrent>, color_bits: u8, alpha_bits: u8) -> *mut ContextBuilder<'static, NotCurrent> {
    CBox::with_consumable_raw(_ptr_context_builder, |builder| builder.with_pixel_format(color_bits, alpha_bits))
}

#[no_mangle]
pub fn glutin_context_builder_with_vsync(_ptr_context_builder: *mut ContextBuilder<'static,NotCurrent>, vsync: bool) -> *mut ContextBuilder<'static, NotCurrent> {
    CBox::with_consumable_raw(_ptr_context_builder, |builder| builder.with_vsync(vsync))
}

#[no_mangle]
pub fn glutin_context_builder_with_srgb(_ptr_context_builder: *mut ContextBuilder<'static,NotCurrent>, srgb_enabled: bool) -> *mut ContextBuilder<'static, NotCurrent> {
    CBox::with_consumable_raw(_ptr_context_builder, |builder| builder.with_srgb(srgb_enabled))
}

#[no_mangle]
pub fn glutin_context_builder_with_double_buffer(_ptr_context_builder: *mut ContextBuilder<'static,NotCurrent>, double_buffer_enabled: bool) -> *mut ContextBuilder<'static, NotCurrent> {
    CBox::with_consumable_raw(_ptr_context_builder, |builder| builder.with_double_buffer(Some(double_buffer_enabled)))
}

#[no_mangle]
pub fn glutin_destroy_context_builder(_ptr: *mut ContextBuilder<PossiblyCurrent>) {
    CBox::drop(_ptr);
}