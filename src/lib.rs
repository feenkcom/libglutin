//#![feature(trace_macros)] trace_macros!(true);

extern crate glutin;
extern crate libc;
extern crate gleam;

use libc::{c_char};
use std::ffi::CStr;
use std::mem::transmute;
use std::mem::transmute_copy;

use glutin::dpi::*;
use glutin::Event::*;
use gleam::gl;
use glutin::EventsLoop;

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

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct GlutinSizeU32 {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct GlutinSizeU64 {
    pub x: u64,
    pub y: u64,
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct GlutinSizeF64 {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct GlutinFetchedEvents {
    pub data: *mut GlutinEvent,
    pub length: usize,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct GlutinCString {
    data: *mut c_char,
    length: usize,
}

///////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////// E V E N T S ////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct GlutinEvent {
    window_id: GlutinSizeU64,
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

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct GlutinTouchEvent {
    device_id: i64,
    phase: GlutinEventTouchPhase,
    x: f64,
    y: f64,
    /// unique identifier of a finger.
    id: u64
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct GlutinMouseWheelEvent {
    device_id: i64,
    phase: GlutinEventTouchPhase,
    delta: GlutinMouseScrollDelta,
    modifiers: GlutinEventModifiersState
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct GlutinMouseInputEvent {
    device_id: i64,
    state: GlutinEventInputElementState,
    button: GlutinEventMouseButton,
    modifiers: GlutinEventModifiersState
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct GlutinCursorMovedEvent {
    device_id: i64,
    x: f64,
    y: f64,
    modifiers: GlutinEventModifiersState
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct GlutinWindowResizedEvent {
    width: f64,
    height: f64
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct GlutinWindowMovedEvent {
    x: f64,
    y: f64
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct GlutinEventKeyboardInput {
    device_id: i64,
    scan_code: u32,
    state: GlutinEventInputElementState,
    has_virtual_keycode: bool,
    virtual_keycode: glutin::VirtualKeyCode,
    modifiers: GlutinEventModifiersState
}

impl Default for GlutinEventKeyboardInput {
    fn default() -> Self { GlutinEventKeyboardInput {
        device_id: Default::default(),
        scan_code: Default::default(),
        state: Default::default(),
        has_virtual_keycode: Default::default(),
        virtual_keycode: glutin::VirtualKeyCode::Unlabeled,
        modifiers: Default::default() } }
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct GlutinEventReceivedCharacter {
    length: usize,
    byte_1: u8,
    byte_2: u8,
    byte_3: u8,
    byte_4: u8
}

#[derive(Debug, Copy, Clone, Default)]
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

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct GlutinEventMouseButton {
    button_type: GlutinEventMouseButtonType,
    button_code: u8
}

///////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////// S T R U C T S  ////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////



#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum GlutinEventMouseButtonType {
    Unknown,
    Left,
    Right,
    Middle,
    Other,
}

impl Default for GlutinEventMouseButtonType {
    fn default() -> Self { GlutinEventMouseButtonType::Unknown }
}

#[derive(Debug, Copy, Clone, PartialEq)]
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

impl Default for GlutinEventType {
    fn default() -> Self { GlutinEventType::Unknown }
}

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum GlutinEventTouchPhase {
    Unknown,
    Started,
    Moved,
    Ended,
    Cancelled
}

impl Default for GlutinEventTouchPhase {
    fn default() -> Self { GlutinEventTouchPhase::Unknown }
}

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum GlutinEventMouseScrollDeltaType {
    Unknown,
    LineDelta,
    PixelDelta,
}

impl Default for GlutinEventMouseScrollDeltaType {
    fn default() -> Self { GlutinEventMouseScrollDeltaType::Unknown }
}

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum GlutinEventInputElementState {
    Unknown,
    Pressed,
    Released
}

impl Default for GlutinEventInputElementState {
    fn default() -> Self { GlutinEventInputElementState::Unknown }
}

fn glutin_convert_window_id(window_id: glutin::WindowId) -> (u64, u64) {
    let size = std::mem::size_of::<glutin::WindowId>();

    let id_128: u128 = match size {
        4 => { // u32
            let id: u32 = unsafe { transmute_copy::<glutin::WindowId, u32>(&window_id) };
            id as u128
        },
        8 => { // u64
            let id: u64 = unsafe { transmute_copy::<glutin::WindowId, u64>(&window_id) };
            id as u128
        },
        16 => { //u128
            let id: u128 = unsafe { transmute_copy::<glutin::WindowId, u128>(&window_id) };
            id
        },
        _ => {
            eprintln!("Unknown size of window id ({:?})", window_id);
            0 as u128 }
    };

    let lo = id_128 as u64 ;
    let hi = (id_128 >> 64) as u64;
    (lo, hi)
}

fn glutin_events_loop_process_event(global_event: glutin::Event, c_event: &mut GlutinEvent) -> bool {
    c_event.event_type = GlutinEventType::Unknown;
    let mut result = false;

    match global_event {
        WindowEvent { event, window_id } => {
            result = true;

            let id: (u64, u64) = glutin_convert_window_id(window_id);
            c_event.window_id.x = id.0;
            c_event.window_id.y = id.1;

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

#[no_mangle]
pub fn glutin_println(_ptr_message: *const c_char) {
    let message = to_rust_string!(_ptr_message);
    println!("{}", message);
}

///////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////// E V E N T S  L O O P /////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub fn glutin_create_events_loop() -> *mut glutin::EventsLoop {
    let mut events_loop = glutin::EventsLoop::new();
    let _events_loop_ptr: *mut glutin::EventsLoop = for_create!(events_loop);
    _events_loop_ptr
}

#[no_mangle]
pub fn glutin_destroy_events_loop(_ptr: *mut glutin::EventsLoop) {
    let _events_loop: Box<glutin::EventsLoop> = for_delete!(_ptr);
    // Drop
}

#[no_mangle]
pub fn glutin_events_loop_poll_events(_ptr_events_loop: *mut glutin::EventsLoop, _ptr_c_event: *mut GlutinEvent, callback: extern fn() -> bool) {
    assert_eq!(_ptr_events_loop.is_null(), false);

    let events_loop = (unsafe { &mut *_ptr_events_loop });
    let c_event = (unsafe { &mut *_ptr_c_event });

    let mut resize_event: GlutinWindowResizedEvent = GlutinWindowResizedEvent { width: 0.0, height: 0.0};
    let mut resize_window_id: (u64, u64) = (0, 0);
    let mut had_resize_event = false;

    let mut events: Vec<GlutinEvent> = Vec::new();

    events_loop.poll_events(|global_event: glutin::Event| {
        let processed = glutin_events_loop_process_event(global_event, c_event);
        if processed { events.push(c_event.clone()) };
    });

    for event in &events {
        c_event.clone_from(event);
        callback();
    }
}

#[no_mangle]
pub fn glutin_events_loop_fetch_events(_ptr_events_loop: *mut glutin::EventsLoop, _ptr_fetched_events: *mut GlutinFetchedEvents) {
    let events_loop: &mut glutin::EventsLoop = to_rust_reference!(_ptr_events_loop);
    let mut fetched_events: &mut GlutinFetchedEvents = to_rust_reference!(_ptr_fetched_events);

    let mut events: Vec<GlutinEvent> = Vec::new();

    events_loop.poll_events(|global_event: glutin::Event| {
        let mut c_event: GlutinEvent = Default::default();

        let processed = glutin_events_loop_process_event(global_event, &mut c_event);
        if processed { events.push(c_event) };
    });

    let mut buf = events.into_boxed_slice();
    let data = buf.as_mut_ptr();
    let len = buf.len();
    std::mem::forget(buf);

    fetched_events.data = data;
    fetched_events.length = len;
}

#[no_mangle]
fn glutin_events_loop_free_events(_ptr_fetched_events: *mut GlutinFetchedEvents) {
    let mut buf: &mut GlutinFetchedEvents = to_rust_reference!(_ptr_fetched_events);

    let s = unsafe { std::slice::from_raw_parts_mut(buf.data, buf.length) };
    let s = s.as_mut_ptr();
    unsafe {
        Box::from_raw(s);
    }

    buf.data = std::ptr::null_mut();
    buf.length = 0;
}


///////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////// M O N I T O R    I D /////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[no_mangle]
fn glutin_events_loop_get_primary_monitor(_ptr_events_loop: *mut glutin::EventsLoop) -> *mut glutin::MonitorId {
    assert_eq!(_ptr_events_loop.is_null(), false);

    let events_loop: &glutin::EventsLoop = to_rust_reference!(_ptr_events_loop);

    let _ptr_monitor_id = for_create!(events_loop.get_primary_monitor());
    _ptr_monitor_id
}

#[no_mangle]
fn glutin_primary_monitor_free (_ptr_monitor_id: *mut glutin::MonitorId) {
    let _monitor_id: Box<glutin::MonitorId> = for_delete!(_ptr_monitor_id);
    // Drop
}

#[no_mangle]
fn glutin_primary_monitor_get_hidpi_factor (_ptr_monitor_id: *mut glutin::MonitorId) -> f64 {
    let monitor_id: &glutin::MonitorId = to_rust_reference!(_ptr_monitor_id);

    println!("monitorId: {:?}", monitor_id);
    monitor_id.get_hidpi_factor()
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

#[no_mangle]
pub fn glutin_window_builder_with_dimensions(_ptr_window_builder: *mut glutin::WindowBuilder, width: f64, height: f64) -> *mut glutin::WindowBuilder {
    let window_builder: &glutin::WindowBuilder = to_rust_reference!(_ptr_window_builder);
    return builder_with!(window_builder.with_dimensions(LogicalSize::new(width, height)));
}

#[no_mangle]
pub fn glutin_window_builder_with_maximized(_ptr_window_builder: *mut glutin::WindowBuilder, with_maximized: bool) -> *mut glutin::WindowBuilder {
    let window_builder: &glutin::WindowBuilder = to_rust_reference!(_ptr_window_builder);
    return builder_with!(window_builder.with_maximized(with_maximized));
}

#[no_mangle]
pub fn glutin_window_builder_with_visibility(_ptr_window_builder: *mut glutin::WindowBuilder, with_visibility: bool) -> *mut glutin::WindowBuilder {
    let window_builder: &glutin::WindowBuilder = to_rust_reference!(_ptr_window_builder);
    return builder_with!(window_builder.with_visibility(with_visibility));
}


#[no_mangle]
pub fn glutin_window_builder_with_always_on_top(_ptr_window_builder: *mut glutin::WindowBuilder, with_always_on_top: bool) -> *mut glutin::WindowBuilder {
    let window_builder: &glutin::WindowBuilder = to_rust_reference!(_ptr_window_builder);
    return builder_with!(window_builder.with_always_on_top(with_always_on_top));
}

///////////////////////////////////////////////////////////////////////////////////////
//////////////////////////// C O N T E X T    B U I L D E R ////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub fn glutin_create_context_builder() -> *mut glutin::ContextBuilder<'static, glutin::NotCurrent> {
    let context_builder = glutin::ContextBuilder::new()
        //.with_double_buffer(Some(false))
        .with_gl(glutin::GlRequest::GlThenGles {
            /// The version to use for OpenGL.
            opengl_version: (3, 1),
            /// The version to use for OpenGL ES.
            opengles_version: (3, 1),
        })
        .with_gl_robustness(glutin::Robustness::TryRobustNoResetNotification)
        .with_gl_profile(glutin::GlProfile::Core)
        .with_multisampling(0)
        .with_depth_buffer(24u8)
        .with_stencil_buffer(8u8)
        .with_pixel_format(24u8, 0u8)
        .with_srgb(false)
        .with_vsync(false);

    let _ptr_context_builder = for_create!(context_builder);
    _ptr_context_builder
}

#[no_mangle]
pub fn glutin_destroy_context_builder(_ptr: *mut glutin::ContextBuilder<glutin::PossiblyCurrent>) {
    let _context_builder: Box<glutin::ContextBuilder<glutin::PossiblyCurrent>> = for_delete!(_ptr);
    // Drop
}

///////////////////////////////////////////////////////////////////////////////////////
/////////////////////////// W I N D O W E D    C O N T E X T //////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub fn glutin_create_windowed_context(
        _ptr_events_loop: *mut glutin::EventsLoop,
        _ptr_window_builder: *mut glutin::WindowBuilder,
        _ptr_context_builder: *mut glutin::ContextBuilder<glutin::NotCurrent>) -> *mut glutin::WindowedContext<glutin::NotCurrent> {

    let events_loop: &mut glutin::EventsLoop = to_rust_reference!(_ptr_events_loop);
    let window_builder = to_rust_reference!(_ptr_window_builder);
    let context_builder = to_rust_reference!(_ptr_context_builder);

    let mut new_window_builder = glutin::WindowBuilder::new();
    new_window_builder.clone_from(window_builder);

    let mut new_context_builder = glutin::ContextBuilder::new();
    new_context_builder.gl_attr.clone_from(&context_builder.gl_attr);
    new_context_builder.pf_reqs.clone_from(&context_builder.pf_reqs);

    match new_context_builder.build_windowed(new_window_builder, events_loop) {
        Ok(context) => {
            let _ptr_windowed_context = for_create!(context);
            _ptr_windowed_context },
        Err(err) => {
            eprintln!("Error in create_windowed_context: {:?}", err);
            std::ptr::null_mut()
        }
    }
}

#[no_mangle]
pub fn glutin_destroy_windowed_context(_ptr: *mut glutin::WindowedContext<glutin::PossiblyCurrent>) {
    let _window: Box<glutin::WindowedContext<glutin::PossiblyCurrent>> = for_delete!(_ptr);
    // drop
}

#[no_mangle]
pub fn glutin_windowed_context_make_current(_ptr_window: *mut glutin::WindowedContext<glutin::PossiblyCurrent>) -> *mut glutin::WindowedContext<glutin::PossiblyCurrent> {
    let window: Box<glutin::WindowedContext<glutin::PossiblyCurrent>> = unsafe { Box::from_raw(_ptr_window) };

    let context: glutin::WindowedContext<glutin::PossiblyCurrent>;

    match unsafe { window.make_current() } {
        Ok(windowed_context) => { context = windowed_context },
        Err((windowed_context, err)) => {
            context = windowed_context;
            match err {
                glutin::ContextError::OsError(string) => { eprintln!("OS Error in make_current: {}", string) },
                glutin::ContextError::IoError(error)=> { eprintln!("IO Error in make_current: {:?}", error) },
                glutin::ContextError::ContextLost => { eprintln!("ContextLost Error in make_current") }
            }
        }
    }

    let _ptr_windowed_context =  for_create!(context);
    _ptr_windowed_context
}

#[no_mangle]
pub fn glutin_windowed_context_swap_buffers(_ptr_window: *mut glutin::WindowedContext<glutin::PossiblyCurrent>) {
    let window: &glutin::WindowedContext<glutin::PossiblyCurrent> = to_rust_reference!(_ptr_window);

    match window.swap_buffers() {
        Ok(windowed_context) => {},
        Err(err) => {
            match err {
                glutin::ContextError::OsError(string) => { eprintln!("OS Error in swap_buffers: {}", string) },
                glutin::ContextError::IoError(error)=> { eprintln!("IO Error in swap_bufferst: {:?}", error) },
                glutin::ContextError::ContextLost => { eprintln!("ContextLost Error in swap_buffers") }
            }
        }
    }
}

#[no_mangle]
pub fn glutin_windowed_context_is_current(_ptr_window: *mut glutin::WindowedContext<glutin::PossiblyCurrent>) -> bool {
    let window: &glutin::WindowedContext<glutin::PossiblyCurrent> = to_rust_reference!(_ptr_window);

    return window.is_current();
}

#[no_mangle]
pub fn glutin_windowed_context_get_proc_address(_ptr_window: *mut glutin::WindowedContext<glutin::PossiblyCurrent>, _ptr_proc_name: *const c_char) -> *const () {
    let window: &glutin::WindowedContext<glutin::PossiblyCurrent> = to_rust_reference!(_ptr_window);
    let proc_name = to_rust_string!(_ptr_proc_name);
    let address = window.get_proc_address(proc_name);
    address
}

#[no_mangle]
pub fn glutin_windowed_context_get_framebuffer_size(_ptr_window: *mut glutin::WindowedContext<glutin::PossiblyCurrent>, _ptr_size: *mut GlutinSizeU32) {
    let window: &glutin::WindowedContext<glutin::PossiblyCurrent> = to_rust_reference!(_ptr_window);
    let size: &mut GlutinSizeU32 = to_rust_reference!(_ptr_size);
    let device_pixel_ratio = window.window().get_hidpi_factor() as f32;

    let window_size = window.window()
        .get_inner_size()
        .unwrap()
        .to_physical(device_pixel_ratio as f64);

    size.x = (window_size.width as u32);
    size.y = (window_size.height as u32);
}

#[no_mangle]
pub fn glutin_windowed_context_get_inner_size(_ptr_window: *mut glutin::WindowedContext<glutin::PossiblyCurrent>, _ptr_size: *mut GlutinSizeF64) {
    let window: &glutin::WindowedContext<glutin::PossiblyCurrent> = to_rust_reference!(_ptr_window);
    let size: &mut GlutinSizeF64 = to_rust_reference!(_ptr_size);

    let window_size = window.window()
        .get_inner_size()
        .unwrap();

    size.x = window_size.width;
    size.y = window_size.height;
}

#[no_mangle]
pub fn glutin_windowed_context_get_position(_ptr_window: *mut glutin::WindowedContext<glutin::PossiblyCurrent>, _ptr_position: *mut GlutinSizeF64) {
    let window: &glutin::WindowedContext<glutin::PossiblyCurrent> = to_rust_reference!(_ptr_window);
    let size: &mut GlutinSizeF64 = to_rust_reference!(_ptr_position);

    let window_position = window.window()
        .get_position()
        .unwrap();

    size.x = window_position.x;
    size.y = window_position.y;
}

#[no_mangle]
pub fn glutin_windowed_context_set_position(_ptr_window: *mut glutin::WindowedContext<glutin::PossiblyCurrent>, x: f64, y: f64) {
    let window: &glutin::WindowedContext<glutin::PossiblyCurrent> = to_rust_reference!(_ptr_window);

    window.window().set_position(LogicalPosition::new(x, y));
}

#[no_mangle]
pub fn glutin_windowed_context_set_title(_ptr_window: *mut glutin::WindowedContext<glutin::PossiblyCurrent>, _ptr_title: *const c_char) {
    let window: &glutin::WindowedContext<glutin::PossiblyCurrent> = to_rust_reference!(_ptr_window);
    let title = to_rust_string!(_ptr_title);
    window.window().set_title(title);
}

#[no_mangle]
pub fn glutin_windowed_context_set_inner_size(_ptr_window: *mut glutin::WindowedContext<glutin::PossiblyCurrent>, _width: f64, _height: f64) {
    let window: &glutin::WindowedContext<glutin::PossiblyCurrent> = to_rust_reference!(_ptr_window);

    window.window().set_inner_size(LogicalSize::new(_width, _height));
}

#[no_mangle]
pub fn glutin_windowed_context_resize_logical(_ptr_window: *mut glutin::WindowedContext<glutin::PossiblyCurrent>, _width: f64, _height: f64) {
    let window: &glutin::WindowedContext<glutin::PossiblyCurrent> = to_rust_reference!(_ptr_window);

    let dpi_factor = window.window().get_hidpi_factor();
    window.resize(LogicalSize::new(_width, _height).to_physical(dpi_factor));
}

#[no_mangle]
pub fn glutin_windowed_context_resize_physical(_ptr_window: *mut glutin::WindowedContext<glutin::PossiblyCurrent>, _width: f64, _height: f64) {
    let window: &glutin::WindowedContext<glutin::PossiblyCurrent> = to_rust_reference!(_ptr_window);

    window.resize(PhysicalSize::new(_width, _height));
}

#[no_mangle]
pub fn glutin_windowed_context_get_id(_ptr_window: *mut glutin::WindowedContext<glutin::PossiblyCurrent>, _ptr_size: *mut GlutinSizeU64) {
    let window: &glutin::WindowedContext<glutin::PossiblyCurrent> = to_rust_reference!(_ptr_window);
    let size: &mut GlutinSizeU64 = to_rust_reference!(_ptr_size);

    let id: (u64, u64) = glutin_convert_window_id(window.window().id());
    size.x = id.0;
    size.y = id.1;
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
pub fn glutin_windowed_context_load_gl(_ptr_window: *mut glutin::WindowedContext<glutin::PossiblyCurrent>) -> *mut GlutinGL {
    let window: &glutin::WindowedContext<glutin::PossiblyCurrent> = to_rust_reference!(_ptr_window);

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
pub fn glutin_gl_get_string(_ptr_gl: *mut GlutinGL, which: gl::GLenum, _ptr_string: *mut GlutinCString) {
    let hack: &GlutinGL = to_rust_reference!(_ptr_gl);
    let c_string: &mut GlutinCString = to_rust_reference!(_ptr_string);
    let gl: std::rc::Rc<gleam::gl::Gl> = unsafe { std::rc::Rc::from_raw(hack.gl) };

    let mut version = gl.get_string(which);

    std::rc::Rc::into_raw(gl);

    let length = version.len();

    let mut s: std::ffi::CString = std::ffi::CString::new(version).unwrap();
    let p: *mut c_char = s.into_raw();

    c_string.data = p;
    c_string.length = length;
}

#[no_mangle]
pub fn glutin_gl_free_cstring(_ptr_string: *mut GlutinCString) {
    let c_string: &mut GlutinCString = to_rust_reference!(_ptr_string);
    unsafe {
        std::ffi::CString::from_raw(c_string.data);
    }
    c_string.length = 0;
}

#[no_mangle]
pub fn glutin_gl_get_shader_version(_ptr_gl: *mut GlutinGL) -> u32 {
    let hack: &GlutinGL = to_rust_reference!(_ptr_gl);
    let gl: std::rc::Rc<gleam::gl::Gl> = unsafe { std::rc::Rc::from_raw(hack.gl) };

    let mut version = gl.get_string(gl::SHADING_LANGUAGE_VERSION);

    std::rc::Rc::into_raw(gl);

    let split = version.split_whitespace();
    let vec: Vec<&str> = split.collect();

    let number = vec[0].parse::<f32>();
    (number.unwrap() * 100.0) as u32
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

    let log = gl.get_shader_info_log(_shader);

    if !log.is_empty() {
        println!("shader log: {}", log);
    }

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