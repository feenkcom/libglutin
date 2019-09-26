#![allow(non_snake_case)]

extern crate glutin;
extern crate libc;
extern crate gleam;

use std::os::raw::c_char;
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

#[macro_use]
mod cmacro;

pub mod cstruct;
pub mod cgl;
pub mod cenum;

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
use std::collections::VecDeque;
use cenum::GlutinCursorIcon;
use glutin::platform::desktop::EventLoopExtDesktop;

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
    window_focused: GlutinWindowFocusedEvent
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

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct GlutinWindowFocusedEvent {
    is_focused: bool
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
    WindowEventRedrawRequested,
    WindowEventTouch,
    WindowEventHiDpiFactorChanged,
    NewEvents,
    EventsCleared,
    LoopDestroyed,
    Suspended,
    Resumed
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

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum GlutinControlFlow {
    /// When the current loop iteration finishes, immediately begin a new iteration regardless of
    /// whether or not new events are available to process.
    Poll,
    /// When the current loop iteration finishes, suspend the thread until another event arrives.
    Wait,
    /// Send a `LoopDestroyed` event and stop the event loop. This variant is *sticky* - once set,
    /// `control_flow` cannot be changed from `Exit`, and any future attempts to do so will result
    /// in the `control_flow` parameter being reset to `Exit`.
    Exit,
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
    let mut result = true;

    match global_event {
        glutin::event::Event::WindowEvent { event, window_id } => {
            let id: (u64, u64) = glutin_convert_window_id(window_id);
            c_event.window_id.x = id.0;
            c_event.window_id.y = id.1;

            match event {
                WindowEvent::Resized(LogicalSize { width, height }) => {
                    c_event.event_type = GlutinEventType::WindowEventResized;
                    c_event.window_resized.width = width;
                    c_event.window_resized.height = height;
                },
                WindowEvent::Focused(is_focused) => {
                    c_event.event_type = GlutinEventType::WindowEventFocused;
                    c_event.window_focused.is_focused = is_focused;
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
                    c_event.event_type = GlutinEventType::WindowEventRedrawRequested;
                },
                WindowEvent::Touch(Touch {device_id, phase, location, force: _, id}) => {
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
                },
                WindowEvent::ReceivedCharacter (character) => {
                    glutin_event_loop_process_received_character (c_event, character);
                },
                _ => ({result = false})
            }
        },
        glutin::event::Event::NewEvents(_start_cause) => {
            c_event.event_type = GlutinEventType::NewEvents;
        },
        glutin::event::Event::EventsCleared => {
            c_event.event_type = GlutinEventType::EventsCleared;
        },
        glutin::event::Event::LoopDestroyed => {
            c_event.event_type = GlutinEventType::LoopDestroyed;
        },
        glutin::event::Event::Suspended => {
            c_event.event_type = GlutinEventType::Suspended;
        },
        glutin::event::Event::Resumed => {
            c_event.event_type = GlutinEventType::Resumed;
        },
        _ => ({result = false})
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

#[derive(Debug)]
#[repr(C)]
pub struct GlutinEventLoopCallback {
    is_valid: bool,
    is_running: bool,
    callback: extern fn(*mut GlutinEvent, *const EventLoopWindowTarget<()>) -> GlutinControlFlow,
    window_commands: *mut VecDeque<GlutinWindowCommand>
}

#[derive(Debug)]
pub enum GlutinWindowCommand {
    Split {
        windowed_context: WindowedContext<PossiblyCurrent>
    },
    Drop {
        window: Window
    }
}

pub fn glutin_events_loop_process_window_commands(queue: &mut VecDeque<GlutinWindowCommand>, counter: u32) -> u32 {
    if queue.is_empty() {
        return 0;
    }

    if counter > 0 {
        return counter - 1;
    }

    let command = queue.pop_front().unwrap();

    match command {
        GlutinWindowCommand::Split { windowed_context} => {
            windowed_context.window().set_visible(true);
            let (context, window) = unsafe { windowed_context.split() };
            drop(context);
            queue.push_front(GlutinWindowCommand::Drop {window});
            return 16;
        },
        GlutinWindowCommand::Drop {window} => {
            window.set_visible(true);
            drop(window);
            return 16;
        }
    }
}

#[no_mangle]
pub fn glutin_events_loop_run_forever(_ptr_events_loop: *mut EventLoop<()>, _ptr_events_loop_callback: *mut GlutinEventLoopCallback) {
    if _ptr_events_loop.is_null() {
        eprintln!("[glutin_events_loop_run_forever] _ptr_events_loop is null");
        return;
    }

    if _ptr_events_loop_callback.is_null() {
        eprintln!("[glutin_events_loop_run_forever] _ptr_events_loop_callback is null");
        return;
    }

    let events_loop = CBox::from_raw(_ptr_events_loop);
    let events_loop_callback: &mut GlutinEventLoopCallback = to_rust_reference!(_ptr_events_loop_callback);
    let mut counter: u32 = 0;

    events_loop_callback.is_running = true;
    events_loop_callback.window_commands = CBox::into_raw(VecDeque::new());

    events_loop.run(move |event, _events_loop: &EventLoopWindowTarget<()>, control_flow: &mut ControlFlow| {
		*control_flow = ControlFlow::Poll;
        let mut c_event: GlutinEvent = Default::default();
        let processed = glutin_events_loop_process_event(event, &mut c_event);
        if processed {
            if events_loop_callback.is_valid {
                let callback = events_loop_callback.callback;
                let _ptr_event_events_loop: *const EventLoopWindowTarget<()> = _events_loop as *const EventLoopWindowTarget<()>;
                let c_control_flow = callback(&mut c_event, _ptr_event_events_loop);

                let mut must_be_poll = false;

                CBox::with_raw(events_loop_callback.window_commands, |commands| {
                     if c_event.event_type == GlutinEventType::NewEvents {
                         counter = glutin_events_loop_process_window_commands(commands, counter);
                     }
                    if counter > 0 || !commands.is_empty() {
                        must_be_poll = true;
                    }
                });

                match c_control_flow {
                    GlutinControlFlow::Poll => { *control_flow = ControlFlow::Poll },
                    GlutinControlFlow::Wait => { if !must_be_poll {
                        *control_flow = ControlFlow::WaitUntil(time::Instant::now() + time::Duration::new(0, 50 * 1000000))
                    } else {
                        *control_flow = ControlFlow::Poll
                    }},
                    GlutinControlFlow::Exit => { *control_flow = ControlFlow::Poll }
                }
            }
        };
	});
}

#[no_mangle]
pub fn glutin_events_loop_run_return(_ptr_events_loop: *mut EventLoop<()>, _ptr_events_loop_callback: *mut GlutinEventLoopCallback) {
    if _ptr_events_loop.is_null() {
        eprintln!("[glutin_events_loop_run_forever] _ptr_events_loop is null");
        return;
    }

    if _ptr_events_loop_callback.is_null() {
        eprintln!("[glutin_events_loop_run_forever] _ptr_events_loop_callback is null");
        return;
    }

    CBox::with_two_raw(_ptr_events_loop, _ptr_events_loop_callback, |events_loop, events_loop_callback| {
        events_loop_callback.is_running = true;

        events_loop.run_return(|event, _events_loop: &EventLoopWindowTarget<()>, control_flow: &mut ControlFlow| {
            *control_flow = ControlFlow::Poll;
            let mut c_event: GlutinEvent = Default::default();
            let processed = glutin_events_loop_process_event(event, &mut c_event);
            if processed {
                if events_loop_callback.is_valid {
                    let callback = events_loop_callback.callback;
                    let _ptr_event_events_loop: *const EventLoopWindowTarget<()> = _events_loop as *const EventLoopWindowTarget<()>;
                    let c_control_flow = callback(&mut c_event, _ptr_event_events_loop);
                    match c_control_flow {
                        GlutinControlFlow::Poll => { *control_flow = ControlFlow::Poll }
                        GlutinControlFlow::Wait => { *control_flow = ControlFlow::WaitUntil(time::Instant::now() + time::Duration::new(0, 50 * 1000000)) }
                        GlutinControlFlow::Exit => { *control_flow = ControlFlow::Exit }
                    }
                }
            }
        });
        events_loop_callback.is_running = false;
    });
}

#[no_mangle]
pub fn glutin_events_loop_run_forever_destroy_windowed_context(_ptr_windowed_context: *mut WindowedContext<PossiblyCurrent>, _ptr_events_loop_callback: *mut GlutinEventLoopCallback) {
    // the window is already destroyed, we do nothing
    if _ptr_windowed_context.is_null() {
        return;
    }

    if _ptr_events_loop_callback.is_null() {
        eprintln!("[glutin_events_loop_run_forever_destroy_windowed_context] Event loop is null");
        return;
    }
    CBox::drop(_ptr_windowed_context);
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
    CBox::with_window_builder(_ptr_window_builder, |builder| {
        builder.with_title(CBox::to_string(_ptr_title))
    })
}

#[no_mangle]
pub fn glutin_window_builder_with_decorations(_ptr_window_builder: *mut WindowBuilder, with_decorations: bool) -> *mut WindowBuilder {
    CBox::with_window_builder(_ptr_window_builder, |builder| builder.with_decorations(with_decorations))
}

#[no_mangle]
pub fn glutin_window_builder_with_transparency(_ptr_window_builder: *mut WindowBuilder, with_transparency: bool) -> *mut WindowBuilder {
    CBox::with_window_builder(_ptr_window_builder, |builder| builder.with_transparent(with_transparency))
}

#[no_mangle]
pub fn glutin_window_builder_with_resizable(_ptr_window_builder: *mut WindowBuilder, with_resizable: bool) -> *mut WindowBuilder {
    CBox::with_window_builder(_ptr_window_builder, |builder| builder.with_resizable(with_resizable))
}

#[no_mangle]
pub fn glutin_window_builder_with_dimensions(_ptr_window_builder: *mut WindowBuilder, width: f64, height: f64) -> *mut WindowBuilder {
    CBox::with_window_builder(_ptr_window_builder, |builder| builder.with_inner_size(LogicalSize::new(width, height)))
}

#[no_mangle]
pub fn glutin_window_builder_with_maximized(_ptr_window_builder: *mut WindowBuilder, with_maximized: bool) -> *mut WindowBuilder {
    CBox::with_window_builder(_ptr_window_builder, |builder| builder.with_maximized(with_maximized))
}

#[no_mangle]
pub fn glutin_window_builder_with_visibility(_ptr_window_builder: *mut WindowBuilder, with_visibility: bool) -> *mut WindowBuilder {
    CBox::with_window_builder(_ptr_window_builder, |builder| builder.with_visible(with_visibility))
}


#[no_mangle]
pub fn glutin_window_builder_with_always_on_top(_ptr_window_builder: *mut WindowBuilder, with_always_on_top: bool) -> *mut WindowBuilder {
    CBox::with_window_builder(_ptr_window_builder, |builder| builder.with_always_on_top(with_always_on_top))
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
    CBox::with_context_builder(_ptr_context_builder, |builder| {
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
    CBox::with_context_builder(_ptr_context_builder, |builder| builder.with_gl(GlRequest::Latest))
}

#[no_mangle]
pub fn glutin_context_builder_with_gl_profile_core(_ptr_context_builder: *mut ContextBuilder<'static,NotCurrent>) -> *mut ContextBuilder<'static, NotCurrent> {
    CBox::with_context_builder(_ptr_context_builder, |builder| builder.with_gl_profile(GlProfile::Core))
}

#[no_mangle]
pub fn glutin_context_builder_with_multisampling(_ptr_context_builder: *mut ContextBuilder<'static,NotCurrent>, samples: u16) -> *mut ContextBuilder<'static, NotCurrent> {
    CBox::with_context_builder(_ptr_context_builder, |builder| builder.with_multisampling(samples))
}

#[no_mangle]
pub fn glutin_context_builder_with_depth_buffer(_ptr_context_builder: *mut ContextBuilder<'static,NotCurrent>, bits: u8) -> *mut ContextBuilder<'static, NotCurrent> {
    CBox::with_context_builder(_ptr_context_builder, |builder| builder.with_depth_buffer(bits))
}

#[no_mangle]
pub fn glutin_context_builder_with_stencil_buffer(_ptr_context_builder: *mut ContextBuilder<'static,NotCurrent>, bits: u8) -> *mut ContextBuilder<'static, NotCurrent> {
    CBox::with_context_builder(_ptr_context_builder, |builder| builder.with_stencil_buffer(bits))
}

#[no_mangle]
pub fn glutin_context_builder_with_pixel_format(_ptr_context_builder: *mut ContextBuilder<'static,NotCurrent>, color_bits: u8, alpha_bits: u8) -> *mut ContextBuilder<'static, NotCurrent> {
    CBox::with_context_builder(_ptr_context_builder, |builder| builder.with_pixel_format(color_bits, alpha_bits))
}

#[no_mangle]
pub fn glutin_context_builder_with_vsync(_ptr_context_builder: *mut ContextBuilder<'static,NotCurrent>, vsync: bool) -> *mut ContextBuilder<'static, NotCurrent> {
    CBox::with_context_builder(_ptr_context_builder, |builder| builder.with_vsync(vsync))
}

#[no_mangle]
pub fn glutin_context_builder_with_srgb(_ptr_context_builder: *mut ContextBuilder<'static,NotCurrent>, srgb_enabled: bool) -> *mut ContextBuilder<'static, NotCurrent> {
    CBox::with_context_builder(_ptr_context_builder, |builder| builder.with_srgb(srgb_enabled))
}

#[no_mangle]
pub fn glutin_context_builder_with_double_buffer(_ptr_context_builder: *mut ContextBuilder<'static,NotCurrent>, double_buffer_enabled: bool) -> *mut ContextBuilder<'static, NotCurrent> {
    CBox::with_context_builder(_ptr_context_builder, |builder| builder.with_double_buffer(Some(double_buffer_enabled)))
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
        Ok(_windowed_context) => { },
        Err((_windowed_context, err)) => {
            match err {
                ContextError::OsError(string) => { eprintln!("OS Error in glutin_destroy_windowed_context: {}", string) },
                ContextError::IoError(error)=> { eprintln!("IO Error in glutin_destroy_windowed_context: {:?}", error) },
                ContextError::ContextLost => { eprintln!("ContextLost Error in glutin_destroy_windowed_context") }
            }
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
                ContextError::IoError(error)=> { eprintln!("IO Error in swap_buffers: {:?}", error) },
                ContextError::ContextLost => { eprintln!("ContextLost Error in swap_buffers") }
            }
        }
    }
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

    let window: &WindowedContext<PossiblyCurrent> = to_rust_reference!(_ptr_window);

    return window.is_current();
}

#[no_mangle]
pub fn glutin_windowed_context_get_proc_address(_ptr_window: *mut WindowedContext<PossiblyCurrent>, _ptr_proc_name: *const c_char) -> *const () {
    CBox::with_raw(_ptr_window, |window| {
        window.get_proc_address(&CBox::to_string(_ptr_proc_name))
    })
}

#[no_mangle]
pub fn glutin_windowed_context_get_framebuffer_size(_ptr_window: *mut WindowedContext<PossiblyCurrent>, _ptr_size: *mut GlutinSizeU32) {
    let size: &mut GlutinSizeU32 = to_rust_reference!(_ptr_size);

    if _ptr_window.is_null() {
        CBox::with_raw(_ptr_size, |size| {
            size.x = 0;
            size.y = 0;
        });
        return;
    }

    let window: &WindowedContext<PossiblyCurrent> = to_rust_reference!(_ptr_window);

    let device_pixel_ratio = window.window().hidpi_factor() as f32;

    let window_size = window.window()
        .inner_size()
        .to_physical(device_pixel_ratio as f64);

    size.x = window_size.width as u32;
    size.y = window_size.height as u32;
}

#[no_mangle]
pub fn glutin_windowed_context_get_inner_size(_ptr_window: *mut WindowedContext<PossiblyCurrent>, _ptr_size: *mut GlutinSizeF64) {
    let size: &mut GlutinSizeF64 = to_rust_reference!(_ptr_size);

    if _ptr_window.is_null() {
       CBox::with_raw(_ptr_size, |size| {
            size.x = 0.0;
            size.y = 0.0;
        });
        return;
    }

    let window: &WindowedContext<PossiblyCurrent> = to_rust_reference!(_ptr_window);

    let window_size = window.window()
        .inner_size();

    size.x = window_size.width;
    size.y = window_size.height;
}

#[no_mangle]
pub fn glutin_windowed_context_get_position(_ptr_window: *mut WindowedContext<PossiblyCurrent>, _ptr_position: *mut GlutinSizeF64) {
    if _ptr_window.is_null() {
        CBox::with_raw(_ptr_position, |size| {
            size.x = 0.0;
            size.y = 0.0;
        });
        return;
    }

    CBox::with_two_raw(_ptr_window, _ptr_position, |window, size | {
        match window.window().outer_position() {
            Ok(_logical_position) => {
                size.x = _logical_position.x;
                size.y = _logical_position.y;
            },
            Err(err) => {
                eprintln!("Error in glutin_windowed_context_get_position: {:?}", err);
                size.x = 0.0;
                size.y = 0.0;
            }
        }
    });
}

#[no_mangle]
pub fn glutin_windowed_context_set_position(_ptr_window: *mut WindowedContext<PossiblyCurrent>, x: f64, y: f64) {
    let window: &WindowedContext<PossiblyCurrent> = to_rust_reference!(_ptr_window);

    window.window().set_outer_position(LogicalPosition::new(x, y));
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
    let window: &WindowedContext<PossiblyCurrent> = to_rust_reference!(_ptr_window);

    window.window().set_inner_size(LogicalSize::new(_width, _height));
}

#[no_mangle]
pub fn glutin_windowed_context_resize_logical(_ptr_window: *mut WindowedContext<PossiblyCurrent>, _width: f64, _height: f64) {
    let window: &WindowedContext<PossiblyCurrent> = to_rust_reference!(_ptr_window);

    let dpi_factor = window.window().hidpi_factor();
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

#[no_mangle]
pub fn glutin_windowed_context_set_cursor_icon(_ptr_window: *mut WindowedContext<PossiblyCurrent>, cursor_icon: GlutinCursorIcon) {
    CBox::with_raw(_ptr_window, |window| {
        window.window().set_cursor_icon(cursor_icon.into());
    })
}