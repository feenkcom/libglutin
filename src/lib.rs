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
use gleam::gl;

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
////////////////////////////////////// E V E N T S ////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[repr(C)]
pub struct GlutinEvent {
    event_type: GlutinEventType,
    touch: GlutinTouchEvent,
    mouse_wheel: GlutinMouseWheelEvent,
    mouse_input: GlutinMouseInputEvent,
    cursor_moved: GlutinCursorMovedEvent,
    keyboard_input: GlutinEventKeyboardInput,
    received_character: GlutinEventReceivedCharacter,
    window_resized: GlutinWindowResizedEvent,
    window_moved: GlutinWindowMovedEvent,
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
pub struct GlutinMouseInputEvent {
    device_id: i64,
    state: GlutinEventInputElementState,
    button: GlutinEventMouseButton,
    modifiers: GlutinEventModifiersState
}

#[repr(C)]
pub struct GlutinCursorMovedEvent {
    device_id: i64,
    x: f64,
    y: f64,
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

#[repr(C)]
pub struct GlutinEventKeyboardInput {
    device_id: i64,
    scan_code: u32,
    state: GlutinEventInputElementState,
    has_virtual_keycode: bool,
    virtual_keycode: glutin::VirtualKeyCode,
    modifiers: GlutinEventModifiersState
}

#[repr(C)]
pub struct GlutinEventReceivedCharacter {
    length: usize,
    byte_1: u8,
    byte_2: u8,
    byte_3: u8,
    byte_4: u8
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

#[repr(C)]
pub struct GlutinEventMouseButton {
    button_type: GlutinEventMouseButtonType,
    button_code: u8
}

#[derive(Clone, Debug)]
#[repr(u32)]
pub enum GlutinEventMouseButtonType {
    Left,
    Right,
    Middle,
    Other,
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

#[derive(Clone, Debug)]
#[repr(u32)]
pub enum GlutinEventInputElementState {
    Pressed,
    Released
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
                glutin::WindowEvent::MouseInput{ device_id, state, button, modifiers } => {
                    glutin_event_loop_process_mouse_input(c_event, device_id, state, button, modifiers);
                },
                glutin::WindowEvent::CursorMoved { device_id, position, modifiers } => {
                    glutin_event_loop_process_cursor_moved(c_event, device_id, position, modifiers);
                },
                glutin::WindowEvent::MouseWheel { device_id, delta, phase, modifiers } => {
                    glutin_event_loop_process_mouse_wheel(c_event, device_id, delta, phase, modifiers);
                },
                glutin::WindowEvent::KeyboardInput { device_id, input } => {
                    glutin_event_loop_process_keyboard_input (c_event, device_id, input);
                }
                glutin::WindowEvent::ReceivedCharacter (character) => {
                    glutin_event_loop_process_received_character (c_event, character);
                }
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

fn glutin_event_loop_process_mouse_input(c_event: &mut GlutinEvent, device_id: glutin::DeviceId, state: glutin::ElementState, button: glutin::MouseButton, modifiers: glutin::ModifiersState) {
    c_event.event_type = GlutinEventType::WindowEventMouseInput;
    c_event.mouse_input.device_id = unsafe { transmute(&device_id)};

    match state {
        glutin::ElementState::Released => {
            c_event.mouse_input.state = GlutinEventInputElementState::Released;
        },
        glutin::ElementState::Pressed => {
            c_event.mouse_input.state = GlutinEventInputElementState::Pressed;

        }
    }

    match button {
        glutin::MouseButton::Left => {
            c_event.mouse_input.button.button_type = GlutinEventMouseButtonType::Left;
            c_event.mouse_input.button.button_code = 0;
        },
        glutin::MouseButton::Right => {
            c_event.mouse_input.button.button_type = GlutinEventMouseButtonType::Right;
            c_event.mouse_input.button.button_code = 1;
        },
        glutin::MouseButton::Middle => {
            c_event.mouse_input.button.button_type = GlutinEventMouseButtonType::Middle;
            c_event.mouse_input.button.button_code = 2;
        },
        glutin::MouseButton::Other(code) => {
            c_event.mouse_input.button.button_type = GlutinEventMouseButtonType::Other;
            c_event.mouse_input.button.button_code = code;
        },
    }

    c_event.mouse_input.modifiers.alt = modifiers.alt;
    c_event.mouse_input.modifiers.ctrl = modifiers.ctrl;
    c_event.mouse_input.modifiers.logo = modifiers.logo;
    c_event.mouse_input.modifiers.shift = modifiers.shift;
}

fn glutin_event_loop_process_cursor_moved(c_event: &mut GlutinEvent, device_id: glutin::DeviceId, position: LogicalPosition, modifiers: glutin::ModifiersState) {
    c_event.event_type = GlutinEventType::WindowEventCursorMoved;
    c_event.cursor_moved.device_id = unsafe { transmute(&device_id)};

    c_event.cursor_moved.x = position.x;
    c_event.cursor_moved.y = position.y;

    c_event.cursor_moved.modifiers.alt = modifiers.alt;
    c_event.cursor_moved.modifiers.ctrl = modifiers.ctrl;
    c_event.cursor_moved.modifiers.logo = modifiers.logo;
    c_event.cursor_moved.modifiers.shift = modifiers.shift;
}

fn glutin_event_loop_process_keyboard_input(c_event: &mut GlutinEvent, device_id: glutin::DeviceId, input: glutin::KeyboardInput) {
    c_event.event_type = GlutinEventType::WindowEventKeyboardInput;
    c_event.keyboard_input.device_id = unsafe { transmute(&device_id)};

    c_event.keyboard_input.scan_code = input.scancode;

    match input.state {
        glutin::ElementState::Released => {
            c_event.keyboard_input.state = GlutinEventInputElementState::Released;
        },
        glutin::ElementState::Pressed => {
            c_event.keyboard_input.state = GlutinEventInputElementState::Pressed;
        }
    }

    match input.virtual_keycode {
        Some(keycode) => {
            c_event.keyboard_input.has_virtual_keycode = true;
            c_event.keyboard_input.virtual_keycode = keycode;
        }
        None => {
            c_event.keyboard_input.has_virtual_keycode = false;
        }
    }

    c_event.keyboard_input.modifiers.alt = input.modifiers.alt;
    c_event.keyboard_input.modifiers.ctrl = input.modifiers.ctrl;
    c_event.keyboard_input.modifiers.logo = input.modifiers.logo;
    c_event.keyboard_input.modifiers.shift = input.modifiers.shift;
}

fn glutin_event_loop_process_received_character (c_event: &mut GlutinEvent, character: char) {
    c_event.event_type = GlutinEventType::WindowEventReceivedCharacter;

    let mut buffer = [0; 4];
    let result = character.encode_utf8(&mut buffer);
    let length = result.len();

    c_event.received_character.length = length;

    let bytes = result.as_bytes();

    if length >= 1 {
        c_event.received_character.byte_1 = bytes[0];
    }
    if length >= 2 {
        c_event.received_character.byte_2 = bytes[1];
    }
    if length >= 3 {
        c_event.received_character.byte_3 = bytes[2];
    }
    if length >= 4 {
        c_event.received_character.byte_4 = bytes[3];
    }
}

///////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////// L I B R A R Y /////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub fn glutin_test() -> bool {
    return true
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

#[no_mangle]
pub fn glutin_events_loop_poll_events(_ptr_events_loop: *mut glutin::EventsLoop, _ptr_c_event: *mut GlutinEvent, callback: extern fn() -> bool) {
    assert_eq!(_ptr_events_loop.is_null(), false);

    let events_loop= (unsafe { &mut *_ptr_events_loop });
    let c_event = (unsafe { &mut *_ptr_c_event });

    events_loop.poll_events(|global_event: glutin::Event| {
        let processed = glutin_events_loop_process_event(global_event,c_event);
        if processed { callback(); }
    });
}

#[no_mangle]
pub fn glutin_events_loop_run_forever(_ptr_events_loop: *mut glutin::EventsLoop, _ptr_c_event: *mut GlutinEvent, callback: extern fn() -> bool) {
    let events_loop= (unsafe { &mut *_ptr_events_loop });
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
    let _ptr_context_builder = for_create!(glutin::ContextBuilder::new().with_double_buffer(Some(true)));
    _ptr_context_builder
}

#[no_mangle]
pub fn glutin_destroy_context_builder(_ptr: *mut glutin::ContextBuilder) {
    let _context_builder: Box<glutin::ContextBuilder> = for_delete!(_ptr);
    // Drop
}

///////////////////////////////////////////////////////////////////////////////////////
/////////////////////////// W I N D O W E D    C O N T E X T //////////////////////////
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
    new_context_builder.pf_reqs.clone_from(&context_builder.pf_reqs);

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
pub fn glutin_windowed_context_get_proc_address(_ptr_window: *mut glutin::WindowedContext, _ptr_proc_name: *const c_char) -> *const () {
    let window: &glutin::WindowedContext = to_rust_reference!(_ptr_window);
    let proc_name = to_rust_string!(_ptr_proc_name);
    let address = window.get_proc_address(proc_name);
    address
}

#[no_mangle]
pub fn glutin_windowed_context_get_framebuffer_size(_ptr_window: *mut glutin::WindowedContext, _ptr_size: *mut GlutinSizeU32) {
    let window: &glutin::WindowedContext = to_rust_reference!(_ptr_window);
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
pub fn glutin_windowed_context_get_inner_size(_ptr_window: *mut glutin::WindowedContext, _ptr_size: *mut GlutinSizeF64) {
    let window: &glutin::WindowedContext = to_rust_reference!(_ptr_window);
    let size: &mut GlutinSizeF64 = to_rust_reference!(_ptr_size);

    let window_size = window
        .get_inner_size()
        .unwrap();

    size.x = window_size.width;
    size.y = window_size.height;
}

#[no_mangle]
pub fn glutin_windowed_context_get_position(_ptr_window: *mut glutin::WindowedContext, _ptr_position: *mut GlutinSizeF64) {
    let window: &glutin::WindowedContext = to_rust_reference!(_ptr_window);
    let size: &mut GlutinSizeF64 = to_rust_reference!(_ptr_position);

    let window_position = window
        .get_position()
        .unwrap();

    size.x = window_position.x;
    size.y = window_position.y;
}

#[no_mangle]
pub fn glutin_windowed_context_set_position(_ptr_window: *mut glutin::WindowedContext, x: f64, y: f64) {
    let window: &glutin::WindowedContext = to_rust_reference!(_ptr_window);

    window.set_position(LogicalPosition::new(x, y));
}

#[no_mangle]
pub fn glutin_windowed_context_set_title(_ptr_window: *mut glutin::WindowedContext, _ptr_title: *const c_char) {
    let window: &glutin::WindowedContext = to_rust_reference!(_ptr_window);
    let title = to_rust_string!(_ptr_title);
    window.set_title(title);
}

#[no_mangle]
pub fn glutin_windowed_context_set_inner_size(_ptr_window: *mut glutin::WindowedContext, _width: f64, _height: f64) {
    let window: &glutin::WindowedContext = to_rust_reference!(_ptr_window);

    window.set_inner_size(LogicalSize::new(_width, _height));
}

#[no_mangle]
pub fn glutin_windowed_context_resize_logical(_ptr_window: *mut glutin::WindowedContext, _width: f64, _height: f64) {
    let window: &glutin::WindowedContext = to_rust_reference!(_ptr_window);

    let dpi_factor = window.get_hidpi_factor();
    window.resize(LogicalSize::new(_width, _height).to_physical(dpi_factor));
}

#[no_mangle]
pub fn glutin_windowed_context_resize_physical(_ptr_window: *mut glutin::WindowedContext, _width: f64, _height: f64) {
    let window: &glutin::WindowedContext = to_rust_reference!(_ptr_window);

    window.resize(PhysicalSize::new(_width, _height));
}

///////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////  G L ////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[repr(C)]
pub struct GlutinGL {
    pub gl: *const gleam::gl::Gl,
}

fn error_callback(_gl: &gleam::gl::Gl, message: &str, error: gl::GLenum) {
    println!("[GL] error: {} code: {}", message, error);
}

#[no_mangle]
pub fn glutin_windowed_context_load_gl(_ptr_window: *mut glutin::WindowedContext) -> *mut GlutinGL {
    let window: &glutin::WindowedContext = to_rust_reference!(_ptr_window);

    let mut gl: std::rc::Rc<(dyn gleam::gl::Gl + 'static)> = (match window.get_api() {
        glutin::Api::OpenGl => unsafe {
            gl::GlFns::load_with(|symbol| window.get_proc_address(symbol) as *const _)
        },
        glutin::Api::OpenGlEs => unsafe {
            gl::GlesFns::load_with(|symbol| window.get_proc_address(symbol) as *const _)
        },
        glutin::Api::WebGl => unimplemented!(),
    });

    gl = gl::ErrorReactingGl::wrap(gl, error_callback);

    let _mut_gl: *const gleam::gl::Gl = std::rc::Rc::into_raw(gl);

    let hack = GlutinGL { gl: _mut_gl };
    let _hack_ptr = for_create!(hack);

    _hack_ptr
}

#[no_mangle]
pub fn glutin_gl_release(_ptr_gl: *mut GlutinGL) {
    let hack: &GlutinGL = for_delete!(_ptr_gl);
    let gl: std::rc::Rc<dyn gleam::gl::Gl> = unsafe { std::rc::Rc::from_raw(hack.gl) };
    //drop
}

#[no_mangle]
pub fn glutin_gl_gen_texture(_ptr_gl: *mut GlutinGL) -> gl::GLuint {
    let hack: &GlutinGL = to_rust_reference!(_ptr_gl);
    let gl: std::rc::Rc<gleam::gl::Gl> = unsafe { std::rc::Rc::from_raw(hack.gl) };

    let textures: Vec<gl::GLuint>;
    textures = gl.gen_textures(1);

    std::rc::Rc::into_raw(gl);

    return textures[0];
}

#[no_mangle]
pub fn glutin_gl_bind_texture_2d(_ptr_gl: *mut GlutinGL, texture: gl::GLuint) {
    let hack: &GlutinGL = to_rust_reference!(_ptr_gl);
    let gl: std::rc::Rc<gleam::gl::Gl> = unsafe { std::rc::Rc::from_raw(hack.gl) };

    gl.bind_texture(gl::TEXTURE_2D, texture);

    std::rc::Rc::into_raw(gl);
}

#[no_mangle]
pub fn glutin_gl_enable_texture_2d(_ptr_gl: *mut GlutinGL) {
    let hack: &GlutinGL = to_rust_reference!(_ptr_gl);
    let gl: std::rc::Rc<gleam::gl::Gl> = unsafe { std::rc::Rc::from_raw(hack.gl) };

    gl.enable(gl::TEXTURE_2D);

    std::rc::Rc::into_raw(gl);
}

#[no_mangle]
pub fn glutin_gl_disable_texture_2d(_ptr_gl: *mut GlutinGL) {
    let hack: &GlutinGL = to_rust_reference!(_ptr_gl);
    let gl: std::rc::Rc<gleam::gl::Gl> = unsafe { std::rc::Rc::from_raw(hack.gl) };

    gl.disable(gl::TEXTURE_2D);

    std::rc::Rc::into_raw(gl);
}

#[no_mangle]
pub fn glutin_gl_create_vertex_shader(_ptr_gl: *mut GlutinGL)-> gl::GLuint {
    let hack: &GlutinGL = to_rust_reference!(_ptr_gl);
    let gl: std::rc::Rc<gleam::gl::Gl> = unsafe { std::rc::Rc::from_raw(hack.gl) };

    let shader = gl.create_shader(gl::VERTEX_SHADER);

    std::rc::Rc::into_raw(gl);

    shader
}

#[no_mangle]
pub fn glutin_gl_create_fragment_shader(_ptr_gl: *mut GlutinGL) -> gl::GLuint {
    let hack: &GlutinGL = to_rust_reference!(_ptr_gl);
    let gl: std::rc::Rc<gleam::gl::Gl> = unsafe { std::rc::Rc::from_raw(hack.gl) };

    let shader = gl.create_shader(gl::FRAGMENT_SHADER);

    std::rc::Rc::into_raw(gl);

    shader
}

#[no_mangle]
pub fn glutin_gl_compile_shader(_ptr_gl: *mut GlutinGL, _shader: gl::GLuint) {
    let hack: &GlutinGL = to_rust_reference!(_ptr_gl);
    let gl: std::rc::Rc<gleam::gl::Gl> = unsafe { std::rc::Rc::from_raw(hack.gl) };

    gl.compile_shader(_shader);

    std::rc::Rc::into_raw(gl);
}

#[no_mangle]
pub fn glutin_gl_shader_source(_ptr_gl: *mut GlutinGL, _shader: gl::GLuint, _ptr_title: *const c_char) {
    let hack: &GlutinGL = to_rust_reference!(_ptr_gl);
    let gl: std::rc::Rc<gleam::gl::Gl> = unsafe { std::rc::Rc::from_raw(hack.gl) };

    let source: &str = to_rust_string!(_ptr_title);

    gl.shader_source(_shader, &[source.as_bytes()]);

    std::rc::Rc::into_raw(gl);
}

#[no_mangle]
pub fn glutin_gl_create_program(_ptr_gl: *mut GlutinGL) -> gl::GLuint {
    let hack: &GlutinGL = to_rust_reference!(_ptr_gl);
    let gl: std::rc::Rc<gleam::gl::Gl> = unsafe { std::rc::Rc::from_raw(hack.gl) };

    let program = gl.create_program();

    std::rc::Rc::into_raw(gl);

    program
}

#[no_mangle]
pub fn glutin_gl_attach_shader(_ptr_gl: *mut GlutinGL, _program: gl::GLuint, _shader: gl::GLuint) {
    let hack: &GlutinGL = to_rust_reference!(_ptr_gl);
    let gl: std::rc::Rc<gleam::gl::Gl> = unsafe { std::rc::Rc::from_raw(hack.gl) };

    gl.attach_shader(_program, _shader);

    std::rc::Rc::into_raw(gl);
}

#[no_mangle]
pub fn glutin_gl_link_program(_ptr_gl: *mut GlutinGL, _program: gl::GLuint) {
    let hack: &GlutinGL = to_rust_reference!(_ptr_gl);
    let gl: std::rc::Rc<gleam::gl::Gl> = unsafe { std::rc::Rc::from_raw(hack.gl) };

    gl.link_program(_program);

    std::rc::Rc::into_raw(gl);
}

#[no_mangle]
pub fn glutin_gl_use_program(_ptr_gl: *mut GlutinGL, _program: gl::GLuint) {
    let hack: &GlutinGL = to_rust_reference!(_ptr_gl);
    let gl: std::rc::Rc<gleam::gl::Gl> = unsafe { std::rc::Rc::from_raw(hack.gl) };

    gl.use_program(_program);

    std::rc::Rc::into_raw(gl);
}

#[no_mangle]
pub fn glutin_gl_viewport(_ptr_gl: *mut GlutinGL, x: gl::GLint, y: gl::GLint, width: gl::GLsizei, height: gl::GLsizei) {
    let hack: &GlutinGL = to_rust_reference!(_ptr_gl);
    let gl: std::rc::Rc<gleam::gl::Gl> = unsafe { std::rc::Rc::from_raw(hack.gl) };

    gl.viewport(x, y, width, height);

    std::rc::Rc::into_raw(gl);
}

#[no_mangle]
pub fn glutin_gl_create_buffer(_ptr_gl: *mut GlutinGL) -> gl::GLuint {
    let hack: &GlutinGL = to_rust_reference!(_ptr_gl);
    let gl: std::rc::Rc<gleam::gl::Gl> = unsafe { std::rc::Rc::from_raw(hack.gl) };

    let buffers: Vec<gl::GLuint>;
    buffers = gl.gen_buffers(1);

    std::rc::Rc::into_raw(gl);

    return buffers[0];
}

#[no_mangle]
pub fn glutin_gl_bind_array_buffer(_ptr_gl: *mut GlutinGL, buffer: gl::GLuint) {
    let hack: &GlutinGL = to_rust_reference!(_ptr_gl);
    let gl: std::rc::Rc<gleam::gl::Gl> = unsafe { std::rc::Rc::from_raw(hack.gl) };

    gl.bind_buffer(gl::ARRAY_BUFFER, buffer);

    std::rc::Rc::into_raw(gl);
}

#[no_mangle]
pub fn glutin_gl_array_buffer_data_static_draw(_ptr_gl: *mut GlutinGL, array: *const f32, length: u32) {
    let hack: &GlutinGL = to_rust_reference!(_ptr_gl);
    let gl: std::rc::Rc<gleam::gl::Gl> = unsafe { std::rc::Rc::from_raw(hack.gl) };

    let data: &[f32] = unsafe { std::slice::from_raw_parts(array, length as usize) };

    gl.buffer_data_untyped(
        gl::ARRAY_BUFFER,
        (data.len() * std::mem::size_of::<f32>()) as gl::GLsizeiptr,
        data.as_ptr() as *const gl::GLvoid,
        gl::STATIC_DRAW,
    );

    std::rc::Rc::into_raw(gl);
}

#[no_mangle]
pub fn glutin_gl_get_attribute_location(_ptr_gl: *mut GlutinGL, program: gl::GLuint, _ptr_name: *const c_char) -> i32 {
    let hack: &GlutinGL = to_rust_reference!(_ptr_gl);
    let gl: std::rc::Rc<gleam::gl::Gl> = unsafe { std::rc::Rc::from_raw(hack.gl) };
    let name = to_rust_string!(_ptr_name);

    let location = gl.get_attrib_location(program, name);

    std::rc::Rc::into_raw(gl);

    location
}

#[no_mangle]
pub fn glutin_gl_get_uniform_location(_ptr_gl: *mut GlutinGL, program: gl::GLuint, _ptr_name: *const c_char) -> i32 {
    let hack: &GlutinGL = to_rust_reference!(_ptr_gl);
    let gl: std::rc::Rc<gleam::gl::Gl> = unsafe { std::rc::Rc::from_raw(hack.gl) };
    let name = to_rust_string!(_ptr_name);

    let location = gl.get_uniform_location(program, name);

    std::rc::Rc::into_raw(gl);

    location
}

#[no_mangle]
pub fn glutin_gl_tex_parameter_i(_ptr_gl: *mut GlutinGL, target: gl::GLenum, pname: gl::GLenum, param: gl::GLint) {
    let hack: &GlutinGL = to_rust_reference!(_ptr_gl);
    let gl: std::rc::Rc<gleam::gl::Gl> = unsafe { std::rc::Rc::from_raw(hack.gl) };

    gl.tex_parameter_i(target, pname, param);

    std::rc::Rc::into_raw(gl);
}

#[no_mangle]
pub fn glutin_gl_tex_image_2d(_ptr_gl: *mut GlutinGL, level: gl::GLint, internal_format: gl::GLint, width: gl::GLsizei, height: gl::GLsizei, border: gl::GLint, format: gl::GLenum, ty: gl::GLenum, array: *const u8, length: u32) {
    let hack: &GlutinGL = to_rust_reference!(_ptr_gl);
    let gl: std::rc::Rc<gleam::gl::Gl> = unsafe { std::rc::Rc::from_raw(hack.gl) };

    let data: &[u8] = unsafe { std::slice::from_raw_parts(array, length as usize) };

    gl.tex_image_2d(
        gl::TEXTURE_2D,
        level,
        internal_format,
        width,
        height,
        border,
        format,
        ty,
        Some(data));

    std::rc::Rc::into_raw(gl);
}

#[no_mangle]
pub fn glutin_gl_gen_vertex_array(_ptr_gl: *mut GlutinGL) -> gl::GLuint {
    let hack: &GlutinGL = to_rust_reference!(_ptr_gl);
    let gl: std::rc::Rc<gleam::gl::Gl> = unsafe { std::rc::Rc::from_raw(hack.gl) };

    let arrays: Vec<gl::GLuint>;
    arrays = gl.gen_vertex_arrays(1);

    std::rc::Rc::into_raw(gl);

    return arrays[0];
}

#[no_mangle]
pub fn glutin_gl_bind_vertex_array (_ptr_gl: *mut GlutinGL, vao: gl::GLuint) {
    let hack: &GlutinGL = to_rust_reference!(_ptr_gl);
    let gl: std::rc::Rc<gleam::gl::Gl> = unsafe { std::rc::Rc::from_raw(hack.gl) };

    gl.bind_vertex_array(vao);

    std::rc::Rc::into_raw(gl);
}

#[no_mangle]
pub fn glutin_gl_enable_vertex_attrib_array(_ptr_gl: *mut GlutinGL, index: gl::GLuint) {
    let hack: &GlutinGL = to_rust_reference!(_ptr_gl);
    let gl: std::rc::Rc<gleam::gl::Gl> = unsafe { std::rc::Rc::from_raw(hack.gl) };

    gl.enable_vertex_attrib_array(index);

    std::rc::Rc::into_raw(gl);
}

#[no_mangle]
pub fn glutin_gl_vertex_attrib_pointer(_ptr_gl: *mut GlutinGL, index: gl::GLuint, size: gl::GLint, type_: gl::GLenum, normalized: bool, stride: gl::GLsizei, offset: gl::GLuint) {
    let hack: &GlutinGL = to_rust_reference!(_ptr_gl);
    let gl: std::rc::Rc<gleam::gl::Gl> = unsafe { std::rc::Rc::from_raw(hack.gl) };

    gl.vertex_attrib_pointer(
        index,
        size,
        type_,
        normalized,
        stride,
        offset);

    std::rc::Rc::into_raw(gl);
}

#[no_mangle]
pub fn glutin_gl_draw_arrays(_ptr_gl: *mut GlutinGL, mode: gl::GLenum, first: gl::GLint, count: gl::GLsizei) {
    let hack: &GlutinGL = to_rust_reference!(_ptr_gl);
    let gl: std::rc::Rc<gleam::gl::Gl> = unsafe { std::rc::Rc::from_raw(hack.gl) };

    gl.draw_arrays(mode, first, count);

    std::rc::Rc::into_raw(gl);
}