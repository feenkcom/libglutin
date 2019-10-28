#![allow(non_snake_case)]

extern crate glutin;
extern crate libc;
extern crate gleam;
extern crate boxer;

use std::os::raw::c_char;
use std::ffi::CStr;
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

#[macro_use]
mod cmacro;

pub mod cstruct;
pub mod cgl;
pub mod cenum;
pub mod events;

#[cfg(target_os = "macos")]
#[path = "platform/macos.rs"]
mod ext;

#[cfg(all(
    not(target_os = "macos")
))]
#[path = "platform/others.rs"]
mod ext;

use cstruct::*;
use std::time;
use cenum::GlutinCursorIcon;
use glutin::platform::desktop::EventLoopExtDesktop;
use events::*;

///////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////// L I B R A R Y /////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub fn glutin_test() -> bool {
    return true
}

#[no_mangle]
pub fn glutin_println(_ptr_message: *const c_char) {
    let message = to_rust_string!(_ptr_message);
    println!("{}", message);
}

#[no_mangle]
pub fn glutin_print(_ptr_message: *const c_char) {
    let message = to_rust_string!(_ptr_message);
    print!("{}", message);
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
pub fn glutin_window_builder_with_title(_ptr_window_builder: *mut WindowBuilder, _ptr_title: *const c_char) -> *mut WindowBuilder {
    CBox::with_consumable_raw(_ptr_window_builder, |builder| {
        builder.with_title(CBox::to_string(_ptr_title))
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

///////////////////////////////////////////////////////////////////////////////////////
/////////////////////////// W I N D O W E D    C O N T E X T //////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub fn glutin_create_windowed_context(
        _ptr_events_loop: *mut EventLoop<()>,
        _ptr_window_builder: *mut WindowBuilder,
        _ptr_context_builder: *mut ContextBuilder<NotCurrent>) -> *mut WindowedContext<NotCurrent> {

    if _ptr_events_loop.is_null() {
        eprintln!("[glutin_create_windowed_context] Event loop is null");
        return std::ptr::null_mut();
    }

    if _ptr_window_builder.is_null() {
        eprintln!("[glutin_create_windowed_context] Window builder is null");
        return std::ptr::null_mut();
    }

    if _ptr_context_builder.is_null() {
        eprintln!("[glutin_create_windowed_context] Context builder is null");
        return std::ptr::null_mut();
    }

    CBox::with_raw(_ptr_events_loop, |events_loop| {
        let window_builder = *CBox::from_raw(_ptr_window_builder);
        let context_builder= *CBox::from_raw(_ptr_context_builder);

        println!("[Glutin] OpenGL Context: {:?}", context_builder);
        println!("[Glutin] Primary monitor: {:?}", events_loop.primary_monitor());
        println!("[Glutin] Window attributes: {:?}", window_builder);

        match context_builder.build_windowed(window_builder, events_loop) {
            Ok(context) => CBox::into_raw(context),
            Err(err) => {
                println!("[Glutin] Could not create context {:?}", err);
                std::ptr::null_mut()
            }
        }
    })
}

#[no_mangle]
pub fn glutin_create_headless_context(
        _ptr_events_loop: *mut EventLoop<()>,
        _ptr_context_builder: *mut ContextBuilder<NotCurrent>) -> *mut Context<NotCurrent> {

     if _ptr_events_loop.is_null() {
        eprintln!("[glutin_create_windowed_context] Event loop is null");
        return std::ptr::null_mut();
    }

    if _ptr_context_builder.is_null() {
        eprintln!("[glutin_create_windowed_context] Context builder is null");
        return std::ptr::null_mut();
    }

    CBox::with_raw(_ptr_events_loop, |events_loop| {
        let context_builder= *CBox::from_raw(_ptr_context_builder);

        println!("[Glutin] OpenGL Headless Context: {:?}", context_builder);
        println!("[Glutin] Primary monitor: {:?}", events_loop.primary_monitor());

        match context_builder.build_headless(events_loop, PhysicalSize::new(1., 1.)) {
            Ok(context) => CBox::into_raw(context),
            Err(err) => {
                println!("[Glutin] Could not create headless context {:?}", err);
                std::ptr::null_mut()
            }
        }
    })
}

#[no_mangle]
pub fn glutin_try_headless_context(
        _ptr_events_loop: *mut EventLoop<()>,
        _ptr_context_builder: *mut ContextBuilder<NotCurrent>) -> bool {

    let builder_copy = CBox::with_raw(_ptr_context_builder, |builder| {
        Box::into_raw(builder.clone())
    });

    let context = glutin_create_headless_context(_ptr_events_loop, builder_copy);
    return if context.is_null() {
        false
    }
    else {
        CBox::drop(context);
        true
    }
}

#[no_mangle]
pub fn glutin_destroy_windowed_context(_ptr: *mut WindowedContext<PossiblyCurrent>) {
    let window = CBox::from_raw(_ptr);

    match unsafe { window.make_not_current() } {
        Ok(_windowed_context) => { std::mem::drop(_windowed_context) },
        Err((_windowed_context, err)) => {
            match err {
                ContextError::OsError(string) => { eprintln!("OS Error in glutin_destroy_windowed_context: {}", string) },
                ContextError::IoError(error) => { eprintln!("IO Error in glutin_destroy_windowed_context: {:?}", error) },
                ContextError::ContextLost => { eprintln!("ContextLost Error in glutin_destroy_windowed_context") }
            }
            std::mem::drop(_windowed_context)
        }
    }
}

#[no_mangle]
pub fn glutin_windowed_context_make_current(_ptr_window: *mut WindowedContext<PossiblyCurrent>) -> *mut WindowedContext<PossiblyCurrent> {
    if _ptr_window.is_null() { return _ptr_window }

    let window: Box<WindowedContext<PossiblyCurrent>> = unsafe { Box::from_raw(_ptr_window) };
    let context: WindowedContext<PossiblyCurrent>;

    match unsafe { window.make_current() } {
        Ok(windowed_context) => { context = windowed_context },
        Err((windowed_context, err)) => {
            context = windowed_context;
            match err {
                ContextError::OsError(string) => { eprintln!("OS Error in make_current: {}", string) },
                ContextError::IoError(error)=> { eprintln!("IO Error in make_current: {:?}", error) },
                ContextError::ContextLost => { eprintln!("ContextLost Error in make_current") }
            }
        }
    }

    let _ptr_windowed_context =  CBox::into_raw(context);
    _ptr_windowed_context
}

#[no_mangle]
pub fn glutin_windowed_context_swap_buffers(_ptr_window: *mut WindowedContext<PossiblyCurrent>) {
    if _ptr_window.is_null() { return; }

    CBox::with_raw(_ptr_window, |window| {
        match window.swap_buffers() {
            Ok(_) => {}
            Err(err) => {
                match err {
                    ContextError::OsError(string) => { eprintln!("OS Error in swap_buffers: {}", string) }
                    ContextError::IoError(error) => { eprintln!("IO Error in swap_buffers: {:?}", error) }
                    ContextError::ContextLost => { eprintln!("ContextLost Error in swap_buffers") }
                }
            }
        }
    });
}

#[no_mangle]
pub fn glutin_windowed_context_request_redraw(_ptr_window: *mut WindowedContext<PossiblyCurrent>) {
    if _ptr_window.is_null() { return }

    CBox::with_raw(_ptr_window, | window | {
         window.window().request_redraw();
    } );
}

#[no_mangle]
pub fn glutin_windowed_context_is_current(_ptr_window: *mut WindowedContext<PossiblyCurrent>) -> bool {
    if _ptr_window.is_null() { return false };

    CBox::with_raw(_ptr_window, |window| {
        window.is_current()
    })
}

#[no_mangle]
pub fn glutin_windowed_context_get_proc_address(_ptr_window: *mut WindowedContext<PossiblyCurrent>, _ptr_proc_name: *const c_char) -> *const () {
    CBox::with_raw(_ptr_window, |window| {
        window.get_proc_address(&CBox::to_string(_ptr_proc_name))
    })
}

#[no_mangle]
pub fn glutin_windowed_context_get_framebuffer_size(_ptr_window: *mut WindowedContext<PossiblyCurrent>, _ptr_size: *mut BoxerSizeF64) {
    if _ptr_window.is_null() {
        CBox::with_raw(_ptr_size, |size| size.be_zero());
        return;
    }

    CBox::with_two_raw(_ptr_window, _ptr_size, |window, size | {
        let device_pixel_ratio = window.window().hidpi_factor() as f32;

    let window_size: PhysicalSize = window.window()
        .inner_size()
        .to_physical(device_pixel_ratio as f64);

        size.width = window_size.width;
        size.height = window_size.height
    });
}

#[no_mangle]
pub fn glutin_windowed_context_get_inner_size(_ptr_window: *mut WindowedContext<PossiblyCurrent>, _ptr_size: *mut BoxerSizeF64) {
    if _ptr_window.is_null() {
        CBox::with_raw(_ptr_size, |size| size.be_zero());
        return;
    }

    CBox::with_two_raw(_ptr_window, _ptr_size, |window, size | {
        let window_size: LogicalSize = window.window().inner_size();

        size.width = window_size.width;
        size.height = window_size.height
    });
}

#[no_mangle]
pub fn glutin_windowed_context_get_position(_ptr_window: *mut WindowedContext<PossiblyCurrent>, _ptr_position: *mut BoxerPointF64) {
    if _ptr_window.is_null() {
        CBox::with_raw(_ptr_position, |point| point.be_zero());
        return;
    }

    CBox::with_two_raw(_ptr_window, _ptr_position, |window, point | {
        match window.window().inner_position() {
            Ok(_logical_position) => {
                point.x = _logical_position.x;
                point.y = _logical_position.y;
            },
            Err(err) => {
                eprintln!("Error in glutin_windowed_context_get_position: {:?}", err);
               point.be_zero()
            }
        }
    });
}

#[no_mangle]
pub fn glutin_windowed_context_get_id(_ptr_window: *mut WindowedContext<PossiblyCurrent>, _ptr_number: *mut BoxerUint128) {
    if _ptr_window.is_null() {
        CBox::with_raw(_ptr_number, |number| number.be_zero() );
        return;
    }

    CBox::with_two_raw(_ptr_window, _ptr_number, |window, number | {
        let id: BoxerUint128 = glutin_convert_window_id(window.window().id());
        number.low = id.low;
        number.high = id.high
    });
}

#[no_mangle]
pub fn glutin_windowed_context_set_position(_ptr_window: *mut WindowedContext<PossiblyCurrent>, x: f64, y: f64) {
    CBox::with_raw(_ptr_window, |window| {
        window.window().set_outer_position(LogicalPosition::new(x, y));
    });
}

#[no_mangle]
pub fn glutin_windowed_context_set_title(_ptr_window: *mut WindowedContext<PossiblyCurrent>, _ptr_title: *const c_char) {
    if _ptr_window.is_null() {
        return;
    }
    CBox::with_raw(_ptr_window, |window| {
        window.window().set_title(&CBox::to_string(_ptr_title))
    });
}

#[no_mangle]
pub fn glutin_windowed_context_set_inner_size(_ptr_window: *mut WindowedContext<PossiblyCurrent>, _width: f64, _height: f64) {
    CBox::with_raw(_ptr_window, |window| {
        window.window().set_inner_size(LogicalSize::new(_width, _height));
    });
}

#[no_mangle]
pub fn glutin_windowed_context_resize_logical(_ptr_window: *mut WindowedContext<PossiblyCurrent>, _width: f64, _height: f64) {
    CBox::with_raw(_ptr_window, |window| {
        let dpi_factor = window.window().hidpi_factor();
        window.resize(LogicalSize::new(_width, _height).to_physical(dpi_factor));
    });
}

#[no_mangle]
pub fn glutin_windowed_context_resize_physical(_ptr_window: *mut WindowedContext<PossiblyCurrent>, _width: f64, _height: f64) {
    CBox::with_raw(_ptr_window, |window| {
        window.resize(PhysicalSize::new(_width, _height));
    });
}

#[no_mangle]
pub fn glutin_windowed_context_set_cursor_icon(_ptr_window: *mut WindowedContext<PossiblyCurrent>, cursor_icon: GlutinCursorIcon) {
    CBox::with_raw(_ptr_window, |window| {
        window.window().set_cursor_icon(cursor_icon.into());
    })
}