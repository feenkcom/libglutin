//#![feature(trace_macros)] trace_macros!(true);

extern crate glutin;
extern crate libc;
extern crate gleam;

use libc::{c_char};
use std::ffi::CStr;
use std::mem::transmute;

use glutin::dpi::*;
use glutin::ContextTrait;
use glutin::Event::*;

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
pub struct GlutinSizeU32 {
    pub x: u32,
    pub y: u32,
}

#[repr(C)]
pub struct GlutinSizeF64 {
    pub x: f64,
    pub y: f64,
}

///////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////// E V E N T S  L O O P /////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub fn glutin_create_events_loop() -> *mut glutin::EventsLoop {
    let _events_loop = for_create!(glutin::EventsLoop::new());
    _events_loop
}

#[no_mangle]
pub fn glutin_destroy_events_loop(_ptr: *mut glutin::EventsLoop) {
    let _events_loop: Box<glutin::EventsLoop> = for_delete!(_ptr);
    // Drop
}

///////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////// E V E N T S ////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[repr(C)]
pub struct GlutinEvent {
    event_type: GlutinEventType,
    touch: GlutinTouchEvent,
    mouse_wheel: GlutinMouseWheelEvent,
    window_resized: GlutinWindowResizedEvent,
    window_moved: GlutinWindowMovedEvent
}

#[repr(C)]
pub struct GlutinTouchEvent {
    device_id: i64,
    phase: GlutinEventTouchPhase,
    x: f64,
    y: f64,
    /// unique identifier of a finger.
    id: u64
}

#[repr(C)]
pub struct GlutinMouseWheelEvent {
    device_id: i64,
    phase: GlutinEventTouchPhase,
    delta: GlutinMouseScrollDelta,
    modifiers: GlutinEventModifiersState
}

#[repr(C)]
pub struct GlutinWindowResizedEvent {
    width: f64,
    height: f64
}

#[repr(C)]
pub struct GlutinWindowMovedEvent {
    x: f64,
    y: f64
}

///////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////// S T R U C T S  ////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[repr(C)]
pub struct GlutinMouseScrollDelta {
    delta_type: GlutinEventMouseScrollDeltaType,
    x: f64,
    y: f64
}

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub struct GlutinEventModifiersState {
    /// The "shift" key
    shift: bool,
    /// The "control" key
    ctrl: bool,
    /// The "alt" key
    alt: bool,
    /// The "logo" key
    ///
    /// This is the "windows" key on PC and "command" key on Mac.
    logo: bool
}

#[derive(Clone, Debug)]
#[repr(u32)]
pub enum GlutinEventType {
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

#[derive(Clone, Debug)]
#[repr(u32)]
pub enum GlutinEventTouchPhase {
    Started,
    Moved,
    Ended,
    Cancelled
}

#[derive(Clone, Debug)]
#[repr(u32)]
pub enum GlutinEventMouseScrollDeltaType {
    LineDelta,
    PixelDelta,
}

fn glutin_events_loop_process_event(global_event: glutin::Event, c_event: &mut GlutinEvent) -> bool {
    c_event.event_type = GlutinEventType::Unknown;
    let mut result = false;

    match global_event {
        WindowEvent { event, window_id } => {
            result = true;
            match event {
                glutin::WindowEvent::Resized(LogicalSize { width, height }) => {
                    c_event.event_type = GlutinEventType::WindowEventResized;
                    c_event.window_resized.width = width;
                    c_event.window_resized.height = height;
                },
                glutin::WindowEvent::Moved(LogicalPosition { x, y }) => {
                    c_event.event_type = GlutinEventType::WindowEventMoved;
                    c_event.window_moved.x = x;
                    c_event.window_moved.y = y;
                },
                glutin::WindowEvent::CloseRequested => {
                    c_event.event_type = GlutinEventType::WindowEventCloseRequested;
                },
                glutin::WindowEvent::Destroyed => {
                    c_event.event_type = GlutinEventType::WindowEventDestroyed;
                },
                glutin::WindowEvent::Refresh => {
                    c_event.event_type = GlutinEventType::WindowEventRefresh;
                },
                glutin::WindowEvent::Touch(glutin::Touch {device_id, phase, location, id}) => {
                    glutin_event_loop_process_touch(c_event, device_id, phase, location, id);
                },
                glutin::WindowEvent::MouseWheel { device_id, delta, phase, modifiers } => {
                    glutin_event_loop_process_mouse_wheel(c_event, device_id, delta, phase, modifiers);
                },
                _ => ({result = false})
            }
        },
        _ => ()
    }
    result
}

fn glutin_event_loop_process_mouse_wheel(c_event: &mut GlutinEvent, device_id: glutin::DeviceId, delta: glutin::MouseScrollDelta, phase: glutin::TouchPhase, modifiers: glutin::ModifiersState) {
    c_event.event_type = GlutinEventType::WindowEventMouseWheel;
    c_event.mouse_wheel.device_id = unsafe { transmute(&device_id)};

    match delta {
        glutin::MouseScrollDelta::LineDelta(x,y) => {
            c_event.mouse_wheel.delta.delta_type = GlutinEventMouseScrollDeltaType::LineDelta;
            c_event.mouse_wheel.delta.x = x as f64;
            c_event.mouse_wheel.delta.y = y as f64;
        },
        glutin::MouseScrollDelta::PixelDelta(LogicalPosition { x, y }) => {
            c_event.mouse_wheel.delta.delta_type = GlutinEventMouseScrollDeltaType::PixelDelta;
            c_event.mouse_wheel.delta.x = x;
            c_event.mouse_wheel.delta.y = y;
        }
    }

    c_event.mouse_wheel.modifiers.alt = modifiers.alt;
    c_event.mouse_wheel.modifiers.ctrl = modifiers.ctrl;
    c_event.mouse_wheel.modifiers.logo = modifiers.logo;
    c_event.mouse_wheel.modifiers.shift = modifiers.shift;

    match phase {
        glutin::TouchPhase::Started => {
            c_event.mouse_wheel.phase = GlutinEventTouchPhase::Started;
        },
        glutin::TouchPhase::Moved => {
            c_event.mouse_wheel.phase = GlutinEventTouchPhase::Moved;
        },
        glutin::TouchPhase::Ended => {
            c_event.mouse_wheel.phase = GlutinEventTouchPhase::Ended;
        },
        glutin::TouchPhase::Cancelled => {
            c_event.mouse_wheel.phase = GlutinEventTouchPhase::Cancelled;
        },
    }
}

fn glutin_event_loop_process_touch(c_event: &mut GlutinEvent, device_id: glutin::DeviceId, phase: glutin::TouchPhase, location: LogicalPosition, id: u64) {
    c_event.event_type = GlutinEventType::WindowEventTouch;
    c_event.touch.device_id = unsafe { transmute(&device_id)};
    c_event.touch.x = location.x;
    c_event.touch.y = location.y;
    c_event.touch.id = id;

    match phase {
        glutin::TouchPhase::Started => {
            c_event.touch.phase = GlutinEventTouchPhase::Started;
        },
        glutin::TouchPhase::Moved => {
            c_event.touch.phase = GlutinEventTouchPhase::Moved;

        },
        glutin::TouchPhase::Ended => {
            c_event.touch.phase = GlutinEventTouchPhase::Ended;

        },
        glutin::TouchPhase::Cancelled => {
            c_event.touch.phase = GlutinEventTouchPhase::Cancelled;

        },
    }
}

#[no_mangle]
pub fn glutin_events_loop_poll_events(_ptr_events_loop: *mut glutin::EventsLoop, _ptr_c_event: *mut GlutinEvent, callback: extern fn() -> bool) {
    assert_eq!(_ptr_events_loop.is_null(), false);

    let mut events_loop= (unsafe { &mut *_ptr_events_loop });
    let c_event = (unsafe { &mut *_ptr_c_event });

    events_loop.poll_events(|global_event: glutin::Event| {
        let processed = glutin_events_loop_process_event(global_event,c_event);
        if processed { callback(); }
    });
}

#[no_mangle]
pub fn glutin_events_loop_run_forever(_ptr_events_loop: *mut glutin::EventsLoop, _ptr_c_event: *mut GlutinEvent, callback: extern fn() -> bool) {
    let mut events_loop= (unsafe { &mut *_ptr_events_loop });
    let c_event = (unsafe { &mut *_ptr_c_event });

    events_loop.run_forever(|global_event: glutin::Event| {
        let processed = glutin_events_loop_process_event(global_event,c_event);

        if !processed {
            return glutin::ControlFlow::Continue;
        }

        let result: bool = callback();
        if result {
            return glutin::ControlFlow::Continue;
        }
        else {
            return glutin::ControlFlow::Break;
        }
    });
}

///////////////////////////////////////////////////////////////////////////////////////
//////////////////////////// W I N D O W    B U I L D E R /////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

macro_rules! builder_with {
    ($name:ident.$function:ident$variable:expr) => {
        {
            let mut window_builder_tmp = glutin::WindowBuilder::new();
            window_builder_tmp.clone_from($name);
            window_builder_tmp = window_builder_tmp.$function($variable);
            let mut _ptr_window_builder = for_create!(window_builder_tmp);
            _ptr_window_builder
        }
    };
}

#[no_mangle]
pub fn glutin_create_window_builder() -> *mut glutin::WindowBuilder {
    let _ptr_window_builder = for_create!(glutin::WindowBuilder::new());
    _ptr_window_builder
}

#[no_mangle]
pub fn glutin_destroy_window_builder(_ptr: *mut glutin::WindowBuilder) {
    let _window_builder: Box<glutin::WindowBuilder> = for_delete!(_ptr);
    // Drop
}

#[no_mangle]
pub fn glutin_window_builder_with_title(_ptr_window_builder: *mut glutin::WindowBuilder, _ptr_title: *const c_char) -> *mut glutin::WindowBuilder {
    let window_builder: &mut glutin::WindowBuilder = to_rust_reference!(_ptr_window_builder);
    let title = to_rust_string!(_ptr_title);
    return builder_with!(window_builder.with_title(title));
}

#[no_mangle]
pub fn glutin_window_builder_with_decorations(_ptr_window_builder: *mut glutin::WindowBuilder, with_decorations: bool) -> *mut glutin::WindowBuilder {
    let window_builder: &glutin::WindowBuilder = to_rust_reference!(_ptr_window_builder);
    return builder_with!(window_builder.with_decorations(with_decorations));
}

#[no_mangle]
pub fn glutin_window_builder_with_transparency(_ptr_window_builder: *mut glutin::WindowBuilder, with_transparency: bool) -> *mut glutin::WindowBuilder {
    let window_builder: &glutin::WindowBuilder = to_rust_reference!(_ptr_window_builder);
    return builder_with!(window_builder.with_transparency(with_transparency));
}

#[no_mangle]
pub fn glutin_window_builder_with_resizable(_ptr_window_builder: *mut glutin::WindowBuilder, with_resizable: bool) -> *mut glutin::WindowBuilder {
    let window_builder: &glutin::WindowBuilder = to_rust_reference!(_ptr_window_builder);
    return builder_with!(window_builder.with_resizable(with_resizable));
}

///////////////////////////////////////////////////////////////////////////////////////
//////////////////////////// C O N T E X T    B U I L D E R ////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub fn glutin_create_context_builder() -> *mut glutin::ContextBuilder<'static> {
    let _ptr_context_builder = for_create!(glutin::ContextBuilder::new().with_gl(glutin::GlRequest::GlThenGles {
            opengl_version: (3, 2),
            opengles_version: (3, 0),
        }));
    _ptr_context_builder
}

#[no_mangle]
pub fn glutin_destroy_context_builder(_ptr: *mut glutin::ContextBuilder) {
    let _context_builder: Box<glutin::ContextBuilder> = for_delete!(_ptr);
    // Drop
}

///////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////// G L   W I N D O W //////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub fn glutin_create_windowed_context(
        _ptr_events_loop: *mut glutin::EventsLoop,
        _ptr_window_builder: *mut glutin::WindowBuilder,
        _ptr_context_builder: *mut glutin::ContextBuilder) -> *mut glutin::WindowedContext {

    let events_loop = to_rust_reference!(_ptr_events_loop);
    let window_builder = to_rust_reference!(_ptr_window_builder);
    let context_builder = to_rust_reference!(_ptr_context_builder);

    let mut new_window_builder = glutin::WindowBuilder::new();
    new_window_builder.clone_from(window_builder);

    let mut new_context_builder = glutin::ContextBuilder::new();
    new_context_builder.gl_attr.clone_from(&context_builder.gl_attr);


    let _ptr_windowed_context =  for_create!(new_context_builder.build_windowed(new_window_builder, &events_loop).unwrap());
    _ptr_windowed_context
}

#[no_mangle]
pub fn glutin_destroy_windowed_context(_ptr: *mut glutin::WindowedContext) {
    let _window: Box<glutin::WindowedContext> = for_delete!(_ptr);
    // drop
}

#[no_mangle]
pub fn glutin_windowed_context_make_current(_ptr_window: *mut glutin::WindowedContext) {
    let window: &glutin::WindowedContext = to_rust_reference!(_ptr_window);
    unsafe { window.make_current().unwrap() };
}

#[no_mangle]
pub fn glutin_windowed_context_swap_buffers(_ptr_window: *mut glutin::WindowedContext) {
    let window: &glutin::WindowedContext = to_rust_reference!(_ptr_window);
    window.swap_buffers();
}

#[no_mangle]
pub fn glutin_gl_window_set_title(_ptr_window: *mut glutin::Window, _ptr_title: *const c_char) {
    let window: &glutin::Window = to_rust_reference!(_ptr_window);
    let title = to_rust_string!(_ptr_title);
    window.set_title(title);
}

#[no_mangle]
pub fn glutin_gl_window_get_framebuffer_size(_ptr_window: *mut glutin::Window, _ptr_size: *mut GlutinSizeU32) {
    let window: &glutin::Window = to_rust_reference!(_ptr_window);
    let size: &mut GlutinSizeU32 = to_rust_reference!(_ptr_size);
    let device_pixel_ratio = window.get_hidpi_factor() as f32;

    let window_size = window
        .get_inner_size()
        .unwrap()
        .to_physical(device_pixel_ratio as f64);

    size.x = (window_size.width as u32);
    size.y = (window_size.height as u32);
}

#[no_mangle]
pub fn glutin_gl_window_get_inner_size(_ptr_window: *mut glutin::Window, _ptr_size: *mut GlutinSizeF64) {
    let window: &glutin::Window = to_rust_reference!(_ptr_window);
    let size: &mut GlutinSizeF64 = to_rust_reference!(_ptr_size);

    let window_size = window
        .get_inner_size()
        .unwrap();

    size.x = window_size.width;
    size.y = window_size.height;
}

#[no_mangle]
pub fn glutin_gl_window_set_inner_size(_ptr_window: *mut glutin::Window, _width: f64, _height: f64) {
    let window: &glutin::Window = to_rust_reference!(_ptr_window);

    window.set_inner_size(LogicalSize::new(_width, _height));
}