//#![feature(trace_macros)] trace_macros!(true);

extern crate glutin;
extern crate winit;
extern crate libc;
extern crate gleam;

use libc::{c_char};
use std::ffi::CStr;
use std::mem::transmute;
use winit::{DeviceId, WindowId, Event, WindowEvent};
use winit::dpi::{LogicalPosition, LogicalSize};

macro_rules! to_rust_reference {
    ($name:ident) => { unsafe { &mut *$name } };
}

macro_rules! to_rust_structure {
    ($name:ident) => ($name);
}

macro_rules! to_c_string {
    ($name:ident) => {
        unsafe {
            assert!(!$name.is_null());
            CStr::from_ptr($name)
        };
    };
}

macro_rules! to_rust_string {
    ($name:ident) => { to_c_string!($name).to_str().unwrap() };
}

macro_rules! for_delete {
    ($name:ident) => (unsafe { transmute($name) });
}

macro_rules! for_create {
    ($expression:expr) => (unsafe { transmute(Box::new($expression)) });
}

#[repr(C)]
pub struct WinitSizeU32 {
    pub x: u32,
    pub y: u32,
}

///////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////// E V E N T S  L O O P /////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub fn winit_create_events_loop() -> *mut winit::EventsLoop {
    let _events_loop = for_create!(winit::EventsLoop::new());
    _events_loop
}

#[no_mangle]
pub fn winit_destroy_events_loop(_ptr: *mut winit::EventsLoop) {
    let _events_loop: Box<winit::EventsLoop> = for_delete!(_ptr);
    // Drop
}

/// `EventsLoopProxy` allows you to wakeup an `EventsLoop` from an other thread.
pub struct WinitEvent {
    event_type: WinitEventType
}

#[derive(Clone, Debug)]
#[repr(u32)]
pub enum WinitEventType {
    Unknown,
    WindowEventResized,
    WindowEventMoved,
    WindowEventCloseRequested,
    WindowEventDestroyed,
    WindowEventDroppedFile,
    WindowEventHoveredFile,
    WindowEventHoveredFileCancelled,
    WindowEventReceivedCharacter,
    WindowEventFocused,
    WindowEventKeyboardInput,
    WindowEventCursorMoved,
    WindowEventCursorEntered,
    WindowEventCursorLeft,
    WindowEventMouseWheel,
    WindowEventMouseInput,
    WindowEventTouchpadPressure,
    WindowEventAxisMotion,
    WindowEventRefresh,
    WindowEventTouch,
    WindowEventHiDpiFactorChanged,
}

fn winit_events_loop_process_event(global_event: Event, c_event: &mut WinitEvent) -> bool {
    c_event.event_type = WinitEventType::Unknown;
    let mut result = false;

    match global_event {
        Event::WindowEvent { event, window_id } => {
            result = true;
            match event {
                WindowEvent::Resized(LogicalSize { width, height }) => {
                    c_event.event_type = WinitEventType::WindowEventResized;
                },
                WindowEvent::Moved(LogicalPosition { x, y }) => {
                    c_event.event_type = WinitEventType::WindowEventMoved;
                },
                WindowEvent::CloseRequested => {
                    c_event.event_type = WinitEventType::WindowEventCloseRequested;
                },
                WindowEvent::Destroyed => {
                    c_event.event_type = WinitEventType::WindowEventDestroyed;
                },
                WindowEvent::Refresh => {
                    c_event.event_type = WinitEventType::WindowEventRefresh;
                },
                _ => ({result = false})
            }
        },
        _ => ()
    }
    result
}

#[no_mangle]
pub fn winit_events_loop_poll_events(_ptr_events_loop: *mut winit::EventsLoop, _ptr_c_event: *mut WinitEvent, callback: extern fn() -> bool) {
    let mut events_loop= (unsafe { &mut *_ptr_events_loop });
    let c_event = (unsafe { &mut *_ptr_c_event });

    events_loop.poll_events(|global_event: winit::Event| {
        let processed =winit_events_loop_process_event(global_event,c_event);
        if processed { callback(); }
    });
}

#[no_mangle]
pub fn winit_events_loop_run_forever(_ptr_events_loop: *mut winit::EventsLoop, _ptr_c_event: *mut WinitEvent, callback: extern fn() -> bool) {
    let mut events_loop= (unsafe { &mut *_ptr_events_loop });
    let c_event = (unsafe { &mut *_ptr_c_event });

    events_loop.run_forever(|global_event: winit::Event| {
        let processed = winit_events_loop_process_event(global_event,c_event);

        if !processed {
            return winit::ControlFlow::Continue;
        }

        let result: bool = callback();
        if result {
            return winit::ControlFlow::Continue;
        }
        else {
            return winit::ControlFlow::Break;
        }
    });
}

///////////////////////////////////////////////////////////////////////////////////////
////////////////////////////// S I M P L E   W I N D O W //////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub fn winit_create_window(_ptr_events_loop: *mut winit::EventsLoop) -> *mut winit::Window {
    let events_loop = to_rust_reference!(_ptr_events_loop);
    let _ptr_window =  for_create!(winit::Window::new(events_loop).unwrap());
    _ptr_window
}

#[no_mangle]
pub fn winit_destroy_window(_ptr: *mut winit::Window) {
    let _window: Box<winit::Window> = for_delete!(_ptr);
    // drop
}

#[no_mangle]
pub fn winit_window_set_title(_ptr_window: *mut winit::Window, _ptr_title: *const c_char) {
    let window = to_rust_reference!(_ptr_window);
    let title = to_rust_string!(_ptr_title);
    window.set_title(title);
}


///////////////////////////////////////////////////////////////////////////////////////
//////////////////////////// W I N D O W    B U I L D E R /////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

macro_rules! builder_with {
    ($name:ident.$function:ident$variable:expr) => {
        {
            let mut window_builder_tmp = winit::WindowBuilder::new();
            window_builder_tmp.clone_from($name);
            window_builder_tmp = window_builder_tmp.$function($variable);
            let mut _ptr_window_builder = for_create!(window_builder_tmp);
            _ptr_window_builder
        }
    };
}

#[no_mangle]
pub fn winit_create_window_builder() -> *mut winit::WindowBuilder {
    let _ptr_window_builder = for_create!(winit::WindowBuilder::new());
    _ptr_window_builder
}

#[no_mangle]
pub fn winit_destroy_window_builder(_ptr: *mut winit::WindowBuilder) {
    let _window_builder: Box<winit::WindowBuilder> = for_delete!(_ptr);
    // Drop
}

#[no_mangle]
pub fn winit_window_builder_with_title(_ptr_window_builder: *mut winit::WindowBuilder, _ptr_title: *const c_char) -> *mut winit::WindowBuilder {
    let window_builder: &mut winit::WindowBuilder = to_rust_reference!(_ptr_window_builder);
    let title = to_rust_string!(_ptr_title);
    return builder_with!(window_builder.with_title(title));
}

#[no_mangle]
pub fn winit_window_builder_with_decorations(_ptr_window_builder: *mut winit::WindowBuilder, with_decorations: bool) -> *mut winit::WindowBuilder {
    let window_builder: &winit::WindowBuilder = to_rust_reference!(_ptr_window_builder);
    return builder_with!(window_builder.with_decorations(with_decorations));
}

#[no_mangle]
pub fn winit_window_builder_with_transparency(_ptr_window_builder: *mut winit::WindowBuilder, with_transparency: bool) -> *mut winit::WindowBuilder {
    let window_builder: &winit::WindowBuilder = to_rust_reference!(_ptr_window_builder);
    return builder_with!(window_builder.with_transparency(with_transparency));
}

#[no_mangle]
pub fn winit_window_builder_with_resizable(_ptr_window_builder: *mut winit::WindowBuilder, with_resizable: bool) -> *mut winit::WindowBuilder {
    let window_builder: &winit::WindowBuilder = to_rust_reference!(_ptr_window_builder);
    return builder_with!(window_builder.with_resizable(with_resizable));
}

///////////////////////////////////////////////////////////////////////////////////////
//////////////////////////// C O N T E X T    B U I L D E R ////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub fn winit_create_context_builder() -> *mut glutin::ContextBuilder<'static> {
    let _ptr_context_builder = for_create!(glutin::ContextBuilder::new());
    _ptr_context_builder
}

#[no_mangle]
pub fn winit_destroy_context_builder(_ptr: *mut glutin::ContextBuilder) {
    let _context_builder: Box<glutin::ContextBuilder> = for_delete!(_ptr);
    // Drop
}

///////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////// G L   W I N D O W //////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////


#[no_mangle]
pub fn winit_create_gl_window_with_context(
        _ptr_events_loop: *mut winit::EventsLoop,
        _ptr_window_builder: *mut winit::WindowBuilder,
        _ptr_context_builder: *mut glutin::ContextBuilder) -> *mut glutin::GlWindow {

    let events_loop = to_rust_reference!(_ptr_events_loop);
    let window_builder = to_rust_reference!(_ptr_window_builder);
    let context_builder = to_rust_reference!(_ptr_context_builder);

    let mut new_window_builder = winit::WindowBuilder::new();
    new_window_builder.clone_from(window_builder);

    let mut new_context_builder = glutin::ContextBuilder::new();
    new_context_builder.gl_attr.clone_from(&context_builder.gl_attr);


    let _ptr_window =  for_create!(glutin::GlWindow::new(new_window_builder, new_context_builder, events_loop).unwrap());
    _ptr_window
}

#[no_mangle]
pub fn winit_destroy_gl_window(_ptr: *mut glutin::GlWindow) {
    let _window: Box<glutin::GlWindow> = for_delete!(_ptr);
    // drop
}

#[no_mangle]
pub fn winit_gl_window_set_title(_ptr_window: *mut glutin::GlWindow, _ptr_title: *const c_char) {
    let window: &glutin::GlWindow = to_rust_reference!(_ptr_window);
    let title = to_rust_string!(_ptr_title);
    window.set_title(title);
}

#[no_mangle]
pub fn winit_gl_window_get_framebuffer_size(_ptr_window: *mut glutin::GlWindow, _ptr_size: *mut WinitSizeU32) {
    let window: &glutin::GlWindow = to_rust_reference!(_ptr_window);
    let size: &mut WinitSizeU32 = to_rust_reference!(_ptr_size);
    let device_pixel_ratio = window.get_hidpi_factor() as f32;

    let window_size = window
        .get_inner_size()
        .unwrap()
        .to_physical(device_pixel_ratio as f64);


    size.x = (window_size.width as u32);
    size.y = (window_size.height as u32);
}