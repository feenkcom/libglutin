#![allow(non_snake_case)]

extern crate glutin;
extern crate libc;
extern crate gleam;
extern crate winit;

use libc::{c_char};
use std::ffi::CStr;
use std::mem::transmute;
use std::mem::transmute_copy;

use glutin::*;
use glutin::dpi::*;
use glutin::event::*;
use glutin::window::*;
use glutin::event_loop::*;
use glutin::monitor::*;

use gleam::gl;

use winit::platform::desktop::EventLoopExtDesktop;

#[macro_use]
mod cmacro;

pub mod cstruct;
pub mod cgl;

use cstruct::*;

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
    virtual_keycode: VirtualKeyCode,
    modifiers: GlutinEventModifiersState
}

impl Default for GlutinEventKeyboardInput {
    fn default() -> Self { GlutinEventKeyboardInput {
        device_id: Default::default(),
        scan_code: Default::default(),
        state: Default::default(),
        has_virtual_keycode: Default::default(),
        virtual_keycode: VirtualKeyCode::Unlabeled,
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

fn glutin_convert_window_id(window_id: WindowId) -> (u64, u64) {
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

    let lo = id_128 as u64 ;
    let hi = (id_128 >> 64) as u64;
    (lo, hi)
}

fn glutin_events_loop_process_event(global_event: glutin::event::Event<()>, c_event: &mut GlutinEvent) -> bool {
    c_event.event_type = GlutinEventType::Unknown;
    let mut result = false;

    match global_event {
        glutin::event::Event::WindowEvent { event, window_id } => {
            result = true;

            let id: (u64, u64) = glutin_convert_window_id(window_id);
            c_event.window_id.x = id.0;
            c_event.window_id.y = id.1;

            match event {
                WindowEvent::Resized(LogicalSize { width, height }) => {
                    c_event.event_type = GlutinEventType::WindowEventResized;
                    c_event.window_resized.width = width;
                    c_event.window_resized.height = height;
                },
                WindowEvent::Moved(LogicalPosition { x, y }) => {
                    c_event.event_type = GlutinEventType::WindowEventMoved;
                    c_event.window_moved.x = x;
                    c_event.window_moved.y = y;
                },
                WindowEvent::CloseRequested => {
                    c_event.event_type = GlutinEventType::WindowEventCloseRequested;
                },
                WindowEvent::Destroyed => {
                    c_event.event_type = GlutinEventType::WindowEventDestroyed;
                },
                WindowEvent::RedrawRequested => {
                    c_event.event_type = GlutinEventType::WindowEventRefresh;
                },
                WindowEvent::Touch(Touch {device_id, phase, location, id}) => {
                    glutin_event_loop_process_touch(c_event, device_id, phase, location, id);
                },
                WindowEvent::MouseInput{ device_id, state, button, modifiers } => {
                    glutin_event_loop_process_mouse_input(c_event, device_id, state, button, modifiers);
                },
                WindowEvent::CursorMoved { device_id, position, modifiers } => {
                    glutin_event_loop_process_cursor_moved(c_event, device_id, position, modifiers);
                },
                WindowEvent::MouseWheel { device_id, delta, phase, modifiers } => {
                    glutin_event_loop_process_mouse_wheel(c_event, device_id, delta, phase, modifiers);
                },
                WindowEvent::KeyboardInput { device_id, input } => {
                    glutin_event_loop_process_keyboard_input (c_event, device_id, input);
                }
                WindowEvent::ReceivedCharacter (character) => {
                    glutin_event_loop_process_received_character (c_event, character);
                }
                _ => ({result = false})
            }
        },
        _ => ()
    }
    result
}

fn glutin_event_loop_process_mouse_wheel(c_event: &mut GlutinEvent, device_id: DeviceId, delta: MouseScrollDelta, phase: TouchPhase, modifiers: ModifiersState) {
    c_event.event_type = GlutinEventType::WindowEventMouseWheel;
    c_event.mouse_wheel.device_id = unsafe { transmute(&device_id)};

    match delta {
        MouseScrollDelta::LineDelta(x,y) => {
            c_event.mouse_wheel.delta.delta_type = GlutinEventMouseScrollDeltaType::LineDelta;
            c_event.mouse_wheel.delta.x = x as f64;
            c_event.mouse_wheel.delta.y = y as f64;
        },
        MouseScrollDelta::PixelDelta(LogicalPosition { x, y }) => {
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
        TouchPhase::Started => {
            c_event.mouse_wheel.phase = GlutinEventTouchPhase::Started;
        },
        TouchPhase::Moved => {
            c_event.mouse_wheel.phase = GlutinEventTouchPhase::Moved;
        },
        TouchPhase::Ended => {
            c_event.mouse_wheel.phase = GlutinEventTouchPhase::Ended;
        },
        TouchPhase::Cancelled => {
            c_event.mouse_wheel.phase = GlutinEventTouchPhase::Cancelled;
        },
    }
}

fn glutin_event_loop_process_touch(c_event: &mut GlutinEvent, device_id: DeviceId, phase: TouchPhase, location: LogicalPosition, id: u64) {
    c_event.event_type = GlutinEventType::WindowEventTouch;
    c_event.touch.device_id = unsafe { transmute(&device_id)};
    c_event.touch.x = location.x;
    c_event.touch.y = location.y;
    c_event.touch.id = id;

    match phase {
        TouchPhase::Started => {
            c_event.touch.phase = GlutinEventTouchPhase::Started;
        },
        TouchPhase::Moved => {
            c_event.touch.phase = GlutinEventTouchPhase::Moved;

        },
        TouchPhase::Ended => {
            c_event.touch.phase = GlutinEventTouchPhase::Ended;

        },
        TouchPhase::Cancelled => {
            c_event.touch.phase = GlutinEventTouchPhase::Cancelled;

        },
    }
}

fn glutin_event_loop_process_mouse_input(c_event: &mut GlutinEvent, device_id: DeviceId, state: ElementState, button: MouseButton, modifiers: ModifiersState) {
    c_event.event_type = GlutinEventType::WindowEventMouseInput;
    c_event.mouse_input.device_id = unsafe { transmute(&device_id)};

    match state {
        ElementState::Released => {
            c_event.mouse_input.state = GlutinEventInputElementState::Released;
        },
        ElementState::Pressed => {
            c_event.mouse_input.state = GlutinEventInputElementState::Pressed;

        }
    }

    match button {
        MouseButton::Left => {
            c_event.mouse_input.button.button_type = GlutinEventMouseButtonType::Left;
            c_event.mouse_input.button.button_code = 0;
        },
        MouseButton::Right => {
            c_event.mouse_input.button.button_type = GlutinEventMouseButtonType::Right;
            c_event.mouse_input.button.button_code = 1;
        },
        MouseButton::Middle => {
            c_event.mouse_input.button.button_type = GlutinEventMouseButtonType::Middle;
            c_event.mouse_input.button.button_code = 2;
        },
        MouseButton::Other(code) => {
            c_event.mouse_input.button.button_type = GlutinEventMouseButtonType::Other;
            c_event.mouse_input.button.button_code = code;
        },
    }

    c_event.mouse_input.modifiers.alt = modifiers.alt;
    c_event.mouse_input.modifiers.ctrl = modifiers.ctrl;
    c_event.mouse_input.modifiers.logo = modifiers.logo;
    c_event.mouse_input.modifiers.shift = modifiers.shift;
}

fn glutin_event_loop_process_cursor_moved(c_event: &mut GlutinEvent, device_id: DeviceId, position: LogicalPosition, modifiers: ModifiersState) {
    c_event.event_type = GlutinEventType::WindowEventCursorMoved;
    c_event.cursor_moved.device_id = unsafe { transmute(&device_id)};

    c_event.cursor_moved.x = position.x;
    c_event.cursor_moved.y = position.y;

    c_event.cursor_moved.modifiers.alt = modifiers.alt;
    c_event.cursor_moved.modifiers.ctrl = modifiers.ctrl;
    c_event.cursor_moved.modifiers.logo = modifiers.logo;
    c_event.cursor_moved.modifiers.shift = modifiers.shift;
}

fn glutin_event_loop_process_keyboard_input(c_event: &mut GlutinEvent, device_id: DeviceId, input: KeyboardInput) {
    c_event.event_type = GlutinEventType::WindowEventKeyboardInput;
    c_event.keyboard_input.device_id = unsafe { transmute(&device_id)};

    c_event.keyboard_input.scan_code = input.scancode;

    match input.state {
        ElementState::Released => {
            c_event.keyboard_input.state = GlutinEventInputElementState::Released;
        },
        ElementState::Pressed => {
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

#[no_mangle]
pub fn glutin_print(_ptr_message: *const c_char) {
    let message = to_rust_string!(_ptr_message);
    print!("{}", message);
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
    CBox::from_raw(_ptr);
}

#[no_mangle]
pub fn glutin_create_fetched_events() -> *mut GlutinFetchedEvents {
    let fetched_events = GlutinFetchedEvents { data: std::ptr::null_mut(), length: 0 };
    let _fetched_events_ptr: *mut GlutinFetchedEvents = Box::into_raw(Box::new(fetched_events));
    _fetched_events_ptr
}


#[no_mangle]
pub fn glutin_events_loop_fetch_events(_ptr_event_loop: *mut EventLoop<()>, _ptr_fetched_events: *mut GlutinFetchedEvents) {
    CBox::with_two_raw(_ptr_event_loop, _ptr_fetched_events, | event_loop, fetched_events| {
        let mut events: Vec<GlutinEvent> = Vec::new();

        // turned out, poll_events doesn't poll *ALL* events, therefore we should loop until there are no more events to poll
        let mut had_event = true;
        while had_event {
            had_event = false;
            event_loop.run_return(|global_event: Event<()>, event_loop_window_target: &EventLoopWindowTarget<()>, control_flow: &mut ControlFlow| {
                had_event = true;
                let mut c_event: GlutinEvent = Default::default();
                let processed = glutin_events_loop_process_event(global_event, &mut c_event);
                if processed { events.push(c_event) };
            });
        }

        let mut buf = events.into_boxed_slice();
        let data = buf.as_mut_ptr();
        let len = buf.len();
        std::mem::forget(buf);

        fetched_events.data = data;
        fetched_events.length = len;
    });
}

#[no_mangle]
fn glutin_events_loop_free_events(_ptr_fetched_events: *mut GlutinFetchedEvents) {
    let mut buf: Box<GlutinFetchedEvents> = unsafe { Box::from_raw(_ptr_fetched_events) };

    if !buf.data.is_null() && buf.length > 0 {
        let s = unsafe { std::slice::from_raw_parts_mut(buf.data, buf.length) };
        let s = s.as_mut_ptr();
        unsafe {
            Box::from_raw(s);
        }
    }

    buf.data = std::ptr::null_mut();
    buf.length = 0;
}


///////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////// M O N I T O R    I D /////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[no_mangle]
fn glutin_events_loop_get_primary_monitor(_ptr_event_loop: *mut EventLoop<()>) -> *mut MonitorHandle {
    CBox::with_raw(_ptr_event_loop, |event_loop| {
        CBox::into_raw(event_loop.get_primary_monitor())
    })
}

#[no_mangle]
fn glutin_primary_monitor_free (_ptr_monitor_id: *mut MonitorHandle) {
    CBox::from_raw(_ptr_monitor_id);
}

#[no_mangle]
fn glutin_primary_monitor_get_hidpi_factor (_ptr_monitor_id: *mut MonitorHandle) -> f64 {
    CBox::with_raw(_ptr_monitor_id, |monitor_id| monitor_id.get_hidpi_factor() )
}

///////////////////////////////////////////////////////////////////////////////////////
//////////////////////////// W I N D O W    B U I L D E R /////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

macro_rules! window_builder_with {
   ($name:ident.$function:ident($($variable:expr),+)) => {
        {
            let mut window_builder_tmp = WindowBuilder::new();
            window_builder_tmp.clone_from($name);
            window_builder_tmp = window_builder_tmp.$function($($variable),+);
            let mut _ptr_window_builder = for_create!(window_builder_tmp);
            _ptr_window_builder
        }
    };
}

#[no_mangle]
pub fn glutin_create_window_builder() -> *mut WindowBuilder {
    let _ptr_window_builder = for_create!(WindowBuilder::new());
    _ptr_window_builder
}

#[no_mangle]
pub fn glutin_destroy_window_builder(_ptr: *mut WindowBuilder) {
    let _window_builder: Box<WindowBuilder> = for_delete!(_ptr);
    // Drop
}

#[no_mangle]
pub fn glutin_window_builder_with_title(_ptr_window_builder: *mut WindowBuilder, _ptr_title: *const c_char) -> *mut WindowBuilder {
    let window_builder: &mut WindowBuilder = to_rust_reference!(_ptr_window_builder);
    let title = to_rust_string!(_ptr_title);
    return window_builder_with!(window_builder.with_title(title));
}

#[no_mangle]
pub fn glutin_window_builder_with_decorations(_ptr_window_builder: *mut WindowBuilder, with_decorations: bool) -> *mut WindowBuilder {
    let window_builder: &WindowBuilder = to_rust_reference!(_ptr_window_builder);
    return window_builder_with!(window_builder.with_decorations(with_decorations));
}

#[no_mangle]
pub fn glutin_window_builder_with_transparency(_ptr_window_builder: *mut WindowBuilder, with_transparency: bool) -> *mut WindowBuilder {
    let window_builder: &WindowBuilder = to_rust_reference!(_ptr_window_builder);
    return window_builder_with!(window_builder.with_transparency(with_transparency));
}

#[no_mangle]
pub fn glutin_window_builder_with_resizable(_ptr_window_builder: *mut WindowBuilder, with_resizable: bool) -> *mut WindowBuilder {
    let window_builder: &WindowBuilder = to_rust_reference!(_ptr_window_builder);
    return window_builder_with!(window_builder.with_resizable(with_resizable));
}

#[no_mangle]
pub fn glutin_window_builder_with_dimensions(_ptr_window_builder: *mut WindowBuilder, width: f64, height: f64) -> *mut WindowBuilder {
    let window_builder: &WindowBuilder = to_rust_reference!(_ptr_window_builder);
    return window_builder_with!(window_builder.with_dimensions(LogicalSize::new(width, height)));
}

#[no_mangle]
pub fn glutin_window_builder_with_maximized(_ptr_window_builder: *mut WindowBuilder, with_maximized: bool) -> *mut WindowBuilder {
    let window_builder: &WindowBuilder = to_rust_reference!(_ptr_window_builder);
    return window_builder_with!(window_builder.with_maximized(with_maximized));
}

#[no_mangle]
pub fn glutin_window_builder_with_visibility(_ptr_window_builder: *mut WindowBuilder, with_visibility: bool) -> *mut WindowBuilder {
    let window_builder: &WindowBuilder = to_rust_reference!(_ptr_window_builder);
    return window_builder_with!(window_builder.with_visibility(with_visibility));
}


#[no_mangle]
pub fn glutin_window_builder_with_always_on_top(_ptr_window_builder: *mut WindowBuilder, with_always_on_top: bool) -> *mut WindowBuilder {
    let window_builder: &WindowBuilder = to_rust_reference!(_ptr_window_builder);
    return window_builder_with!(window_builder.with_always_on_top(with_always_on_top));
}

///////////////////////////////////////////////////////////////////////////////////////
//////////////////////////// C O N T E X T    B U I L D E R ////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

macro_rules! context_builder_with {
    ($name:ident.$function:ident($($variable:expr),+)) => {
        {
            let mut context_builder_tmp = ContextBuilder::new();
            context_builder_tmp.gl_attr.clone_from(&$name.gl_attr);
            context_builder_tmp.pf_reqs.clone_from(&$name.pf_reqs);
            context_builder_tmp = context_builder_tmp.$function($($variable),+);
            let mut _ptr_context_builder = for_create!(context_builder_tmp);
            _ptr_context_builder
        }
    };
}

#[no_mangle]
pub fn glutin_create_context_builder() -> *mut ContextBuilder<'static, NotCurrent> {
    let context_builder = ContextBuilder::new()
        .with_gl_robustness(Robustness::TryRobustNoResetNotification)
        .with_gl_profile(GlProfile::Core);

    let _ptr_context_builder = for_create!(context_builder);
    _ptr_context_builder
}

#[no_mangle]
pub fn glutin_context_builder_with_gl_then_gles(_ptr_context_builder: *mut ContextBuilder<NotCurrent>, gl_major: u8, gl_minor: u8, gles_major: u8, gles_minor: u8) -> *mut ContextBuilder<'static, NotCurrent> {
    let context_builder: &ContextBuilder<NotCurrent> = to_rust_reference!(_ptr_context_builder);
    return context_builder_with!(context_builder.with_gl(GlRequest::GlThenGles {
        /// The version to use for OpenGL.
        opengl_version: (gl_major, gl_minor),
        /// The version to use for OpenGL ES.
        opengles_version: (gles_major, gles_minor),
    }));
}

#[no_mangle]
pub fn glutin_context_builder_with_gl_latest(_ptr_context_builder: *mut ContextBuilder<NotCurrent>) -> *mut ContextBuilder<'static, NotCurrent> {
    let context_builder: &ContextBuilder<NotCurrent> = to_rust_reference!(_ptr_context_builder);
    return context_builder_with!(context_builder.with_gl(GlRequest::Latest));
}

#[no_mangle]
pub fn glutin_context_builder_with_gl_profile_core(_ptr_context_builder: *mut ContextBuilder<NotCurrent>) -> *mut ContextBuilder<'static, NotCurrent> {
    let context_builder: &ContextBuilder<NotCurrent> = to_rust_reference!(_ptr_context_builder);
    return context_builder_with!(context_builder.with_gl_profile(GlProfile::Core));
}

#[no_mangle]
pub fn glutin_context_builder_with_multisampling(_ptr_context_builder: *mut ContextBuilder<NotCurrent>, samples: u16) -> *mut ContextBuilder<'static, NotCurrent> {
    let context_builder: &ContextBuilder<NotCurrent> = to_rust_reference!(_ptr_context_builder);
    return context_builder_with!(context_builder.with_multisampling(samples));
}

#[no_mangle]
pub fn glutin_context_builder_with_depth_buffer(_ptr_context_builder: *mut ContextBuilder<NotCurrent>, bits: u8) -> *mut ContextBuilder<'static, NotCurrent> {
    let context_builder: &ContextBuilder<NotCurrent> = to_rust_reference!(_ptr_context_builder);
    return context_builder_with!(context_builder.with_depth_buffer(bits));
}

#[no_mangle]
pub fn glutin_context_builder_with_stencil_buffer(_ptr_context_builder: *mut ContextBuilder<NotCurrent>, bits: u8) -> *mut ContextBuilder<'static, NotCurrent> {
    let context_builder: &ContextBuilder<NotCurrent> = to_rust_reference!(_ptr_context_builder);
    return context_builder_with!(context_builder.with_stencil_buffer(bits));
}

#[no_mangle]
pub fn glutin_context_builder_with_pixel_format(_ptr_context_builder: *mut ContextBuilder<NotCurrent>, color_bits: u8, alpha_bits: u8) -> *mut ContextBuilder<'static, NotCurrent> {
    let context_builder: &ContextBuilder<NotCurrent> = to_rust_reference!(_ptr_context_builder);
    return context_builder_with!(context_builder.with_pixel_format(color_bits, alpha_bits));
}

#[no_mangle]
pub fn glutin_context_builder_with_vsync(_ptr_context_builder: *mut ContextBuilder<NotCurrent>, vsync: bool) -> *mut ContextBuilder<'static, NotCurrent> {
    let context_builder: &ContextBuilder<NotCurrent> = to_rust_reference!(_ptr_context_builder);
    return context_builder_with!(context_builder.with_vsync(vsync));
}

#[no_mangle]
pub fn glutin_context_builder_with_srgb(_ptr_context_builder: *mut ContextBuilder<NotCurrent>, srgb_enabled: bool) -> *mut ContextBuilder<'static, NotCurrent> {
    let context_builder: &ContextBuilder<NotCurrent> = to_rust_reference!(_ptr_context_builder);
    return context_builder_with!(context_builder.with_srgb(srgb_enabled));
}

#[no_mangle]
pub fn glutin_context_builder_with_double_buffer(_ptr_context_builder: *mut ContextBuilder<NotCurrent>, double_buffer_enabled: bool) -> *mut ContextBuilder<'static, NotCurrent> {
    let context_builder: &ContextBuilder<NotCurrent> = to_rust_reference!(_ptr_context_builder);
    return context_builder_with!(context_builder.with_double_buffer(Some(double_buffer_enabled)));
}

#[no_mangle]
pub fn glutin_destroy_context_builder(_ptr: *mut ContextBuilder<PossiblyCurrent>) {
    let _context_builder: Box<ContextBuilder<PossiblyCurrent>> = for_delete!(_ptr);
    // Drop
}

///////////////////////////////////////////////////////////////////////////////////////
/////////////////////////// W I N D O W E D    C O N T E X T //////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub fn glutin_create_windowed_context(
        _ptr_events_loop: *mut EventLoop<()>,
        _ptr_window_builder: *mut WindowBuilder,
        _ptr_context_builder: *mut ContextBuilder<NotCurrent>) -> *mut WindowedContext<NotCurrent> {

    let events_loop: &mut EventLoop<()> = to_rust_reference!(_ptr_events_loop);
    let window_builder = to_rust_reference!(_ptr_window_builder);
    let context_builder = to_rust_reference!(_ptr_context_builder);

    let mut new_window_builder = WindowBuilder::new();
    new_window_builder.clone_from(window_builder);

    let mut new_context_builder = ContextBuilder::new();
    new_context_builder.gl_attr.clone_from(&context_builder.gl_attr);
    new_context_builder.pf_reqs.clone_from(&context_builder.pf_reqs);

    println!("{:?}", new_context_builder);

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
pub fn glutin_destroy_windowed_context(_ptr: *mut WindowedContext<PossiblyCurrent>) {
    let _window: Box<WindowedContext<PossiblyCurrent>> = for_delete!(_ptr);
    // drop
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

    let _ptr_windowed_context =  for_create!(context);
    _ptr_windowed_context
}

#[no_mangle]
pub fn glutin_windowed_context_swap_buffers(_ptr_window: *mut WindowedContext<PossiblyCurrent>) {
    if _ptr_window.is_null() { return }

    let window: &WindowedContext<PossiblyCurrent> = to_rust_reference!(_ptr_window);

    match window.swap_buffers() {
        Ok(_) => {},
        Err(err) => {
            match err {
                ContextError::OsError(string) => { eprintln!("OS Error in swap_buffers: {}", string) },
                ContextError::IoError(error)=> { eprintln!("IO Error in swap_bufferst: {:?}", error) },
                ContextError::ContextLost => { eprintln!("ContextLost Error in swap_buffers") }
            }
        }
    }
}

#[no_mangle]
pub fn glutin_windowed_context_is_current(_ptr_window: *mut WindowedContext<PossiblyCurrent>) -> bool {
    if _ptr_window.is_null() { return false };

    let window: &WindowedContext<PossiblyCurrent> = to_rust_reference!(_ptr_window);

    return window.is_current();
}

#[no_mangle]
pub fn glutin_windowed_context_get_proc_address(_ptr_window: *mut WindowedContext<PossiblyCurrent>, _ptr_proc_name: *const c_char) -> *const () {
    let window: &WindowedContext<PossiblyCurrent> = to_rust_reference!(_ptr_window);
    let proc_name = to_rust_string!(_ptr_proc_name);
    let address = window.get_proc_address(proc_name);
    address
}

#[no_mangle]
pub fn glutin_windowed_context_get_framebuffer_size(_ptr_window: *mut WindowedContext<PossiblyCurrent>, _ptr_size: *mut GlutinSizeU32) {
    let window: &WindowedContext<PossiblyCurrent> = to_rust_reference!(_ptr_window);

    let size: &mut GlutinSizeU32 = to_rust_reference!(_ptr_size);
    let device_pixel_ratio = window.window().get_hidpi_factor() as f32;

    let window_size = window.window()
        .get_inner_size()
        .unwrap()
        .to_physical(device_pixel_ratio as f64);

    size.x = window_size.width as u32;
    size.y = window_size.height as u32;
}

#[no_mangle]
pub fn glutin_windowed_context_get_inner_size(_ptr_window: *mut WindowedContext<PossiblyCurrent>, _ptr_size: *mut GlutinSizeF64) {
    let window: &WindowedContext<PossiblyCurrent> = to_rust_reference!(_ptr_window);
    let size: &mut GlutinSizeF64 = to_rust_reference!(_ptr_size);

    let window_size = window.window()
        .get_inner_size()
        .unwrap();

    size.x = window_size.width;
    size.y = window_size.height;
}

#[no_mangle]
pub fn glutin_windowed_context_get_position(_ptr_window: *mut WindowedContext<PossiblyCurrent>, _ptr_position: *mut GlutinSizeF64) {
    let window: &WindowedContext<PossiblyCurrent> = to_rust_reference!(_ptr_window);
    let size: &mut GlutinSizeF64 = to_rust_reference!(_ptr_position);

    let window_position = window.window()
        .get_position()
        .unwrap();

    size.x = window_position.x;
    size.y = window_position.y;
}

#[no_mangle]
pub fn glutin_windowed_context_set_position(_ptr_window: *mut WindowedContext<PossiblyCurrent>, x: f64, y: f64) {
    let window: &WindowedContext<PossiblyCurrent> = to_rust_reference!(_ptr_window);

    window.window().set_position(LogicalPosition::new(x, y));
}

#[no_mangle]
pub fn glutin_windowed_context_set_title(_ptr_window: *mut WindowedContext<PossiblyCurrent>, _ptr_title: *const c_char) {
    let window: &WindowedContext<PossiblyCurrent> = to_rust_reference!(_ptr_window);
    let title = to_rust_string!(_ptr_title);
    window.window().set_title(title);
}

#[no_mangle]
pub fn glutin_windowed_context_set_inner_size(_ptr_window: *mut WindowedContext<PossiblyCurrent>, _width: f64, _height: f64) {
    let window: &WindowedContext<PossiblyCurrent> = to_rust_reference!(_ptr_window);

    window.window().set_inner_size(LogicalSize::new(_width, _height));
}

#[no_mangle]
pub fn glutin_windowed_context_resize_logical(_ptr_window: *mut WindowedContext<PossiblyCurrent>, _width: f64, _height: f64) {
    let window: &WindowedContext<PossiblyCurrent> = to_rust_reference!(_ptr_window);

    let dpi_factor = window.window().get_hidpi_factor();
    window.resize(LogicalSize::new(_width, _height).to_physical(dpi_factor));
}

#[no_mangle]
pub fn glutin_windowed_context_resize_physical(_ptr_window: *mut WindowedContext<PossiblyCurrent>, _width: f64, _height: f64) {
    let window: &WindowedContext<PossiblyCurrent> = to_rust_reference!(_ptr_window);

    window.resize(PhysicalSize::new(_width, _height));
}

#[no_mangle]
pub fn glutin_windowed_context_get_id(_ptr_window: *mut WindowedContext<PossiblyCurrent>, _ptr_size: *mut GlutinSizeU64) {
    let window: &WindowedContext<PossiblyCurrent> = to_rust_reference!(_ptr_window);
    let size: &mut GlutinSizeU64 = to_rust_reference!(_ptr_size);

    let id: (u64, u64) = glutin_convert_window_id(window.window().id());
    size.x = id.0;
    size.y = id.1;
}