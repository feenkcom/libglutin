use boxer::number::BoxerUint128;

use glutin::dpi::{PhysicalPosition, PhysicalSize};
use glutin::event::*;

use crate::event_loop::GlutinCustomEvent;
use crate::glutin_convert_window_id;
use boxer::{ValueBox, ValueBoxPointerReference};
use std::collections::HashMap;
use std::mem::transmute;

#[derive(Debug, Default)]
#[repr(C)]
pub struct GlutinEvent {
    pub window_id: BoxerUint128,
    pub event_type: GlutinEventType,
    pub touch: GlutinTouchEvent,
    pub mouse_wheel: GlutinMouseWheelEvent,
    pub mouse_input: GlutinMouseInputEvent,
    pub cursor_moved: GlutinCursorMovedEvent,
    pub keyboard_input: GlutinEventKeyboardInput,
    pub received_character: GlutinEventReceivedCharacter,
    pub window_resized: GlutinWindowResizedEvent,
    pub scale_factor: GlutinWindowScaleFactorChangedEvent,
    pub window_moved: GlutinWindowMovedEvent,
    pub window_focused: GlutinWindowFocusedEvent,
    pub modifiers: GlutinEventModifiersState,
    pub user_event: GlutinEventUserEvent,
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct GlutinTouchEvent {
    device_id: i64,
    phase: GlutinEventTouchPhase,
    x: f64,
    y: f64,
    /// unique identifier of a finger.
    id: u64,
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct GlutinMouseWheelEvent {
    device_id: i64,
    phase: GlutinEventTouchPhase,
    delta: GlutinMouseScrollDelta,
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct GlutinMouseInputEvent {
    device_id: i64,
    state: GlutinEventInputElementState,
    button: GlutinEventMouseButton,
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct GlutinCursorMovedEvent {
    device_id: i64,
    x: f64,
    y: f64,
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct GlutinWindowResizedEvent {
    width: u32,
    height: u32,
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct GlutinWindowScaleFactorChangedEvent {
    scale_factor: f64,
    width: u32,
    height: u32,
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct GlutinWindowMovedEvent {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct GlutinWindowFocusedEvent {
    is_focused: bool,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct GlutinEventKeyboardInput {
    device_id: i64,
    scan_code: u32,
    state: GlutinEventInputElementState,
    has_virtual_keycode: bool,
    virtual_keycode: VirtualKeyCode,
    is_synthetic: bool,
}

impl Default for GlutinEventKeyboardInput {
    fn default() -> Self {
        GlutinEventKeyboardInput {
            device_id: Default::default(),
            scan_code: Default::default(),
            state: Default::default(),
            has_virtual_keycode: Default::default(),
            virtual_keycode: VirtualKeyCode::Unlabeled,
            is_synthetic: false,
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct GlutinEventReceivedCharacter {
    length: usize,
    byte_1: u8,
    byte_2: u8,
    byte_3: u8,
    byte_4: u8,
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct GlutinMouseScrollDelta {
    delta_type: GlutinEventMouseScrollDeltaType,
    x: f64,
    y: f64,
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
    logo: bool,
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct GlutinEventMouseButton {
    button_type: GlutinEventMouseButtonType,
    button_code: u16,
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct GlutinEventUserEvent {
    event: GlutinCustomEvent,
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
    fn default() -> Self {
        GlutinEventMouseButtonType::Unknown
    }
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
    WindowEventTouch,
    WindowEventScaleFactorChanged,
    NewEvents,
    MainEventsCleared,
    LoopDestroyed,
    Suspended,
    Resumed,
    RedrawRequested,
    RedrawEventsCleared,
    ModifiersChanged,
    UserEvent,
}

impl Default for GlutinEventType {
    fn default() -> Self {
        GlutinEventType::Unknown
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum GlutinEventTouchPhase {
    Unknown,
    Started,
    Moved,
    Ended,
    Cancelled,
}

impl Default for GlutinEventTouchPhase {
    fn default() -> Self {
        GlutinEventTouchPhase::Unknown
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum GlutinEventMouseScrollDeltaType {
    Unknown,
    LineDelta,
    PixelDelta,
}

impl Default for GlutinEventMouseScrollDeltaType {
    fn default() -> Self {
        GlutinEventMouseScrollDeltaType::Unknown
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum GlutinEventInputElementState {
    Unknown,
    Pressed,
    Released,
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
    fn default() -> Self {
        GlutinEventInputElementState::Unknown
    }
}

///////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////// E V E N T S ////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

pub struct EventProcessor {
    pub key_buffer: HashMap<ScanCode, VirtualKeyCode>,
}

impl EventProcessor {
    pub fn new() -> Self {
        Self {
            key_buffer: HashMap::new(),
        }
    }

    pub fn process(
        &mut self,
        global_event: glutin::event::Event<GlutinCustomEvent>,
        c_event: &mut GlutinEvent,
    ) -> bool {
        c_event.event_type = GlutinEventType::Unknown;
        let mut result = true;

        match global_event {
            glutin::event::Event::WindowEvent { event, window_id } => {
                let id: BoxerUint128 = glutin_convert_window_id(window_id);
                c_event.window_id.clone_from(&id);

                match event {
                    WindowEvent::Resized(PhysicalSize { width, height }) => {
                        c_event.event_type = GlutinEventType::WindowEventResized;
                        c_event.window_resized.width = width;
                        c_event.window_resized.height = height;
                    }
                    WindowEvent::ScaleFactorChanged {
                        scale_factor,
                        new_inner_size,
                    } => {
                        c_event.event_type = GlutinEventType::WindowEventScaleFactorChanged;
                        c_event.scale_factor.scale_factor = scale_factor;
                        c_event.scale_factor.width = new_inner_size.width;
                        c_event.scale_factor.height = new_inner_size.height;
                    }
                    WindowEvent::Focused(is_focused) => {
                        c_event.event_type = GlutinEventType::WindowEventFocused;
                        c_event.window_focused.is_focused = is_focused;
                    }
                    WindowEvent::Moved(PhysicalPosition { x, y }) => {
                        c_event.event_type = GlutinEventType::WindowEventMoved;
                        c_event.window_moved.x = x as i32;
                        c_event.window_moved.y = y as i32;
                    }
                    WindowEvent::CloseRequested => {
                        c_event.event_type = GlutinEventType::WindowEventCloseRequested;
                    }
                    WindowEvent::Destroyed => {
                        c_event.event_type = GlutinEventType::WindowEventDestroyed;
                    }
                    WindowEvent::Touch(Touch {
                        device_id,
                        phase,
                        location,
                        force: _,
                        id,
                    }) => {
                        glutin_event_loop_process_touch(c_event, device_id, phase, location, id);
                    }
                    WindowEvent::MouseInput {
                        device_id,
                        state,
                        button,
                        ..
                    } => {
                        glutin_event_loop_process_mouse_input(c_event, device_id, state, button);
                    }
                    WindowEvent::CursorMoved {
                        device_id,
                        position,
                        ..
                    } => {
                        glutin_event_loop_process_cursor_moved(c_event, device_id, position);
                    }
                    WindowEvent::CursorEntered { device_id } => {
                        glutin_event_loop_process_cursor_entered(c_event, device_id);
                    }
                    WindowEvent::CursorLeft { device_id } => {
                        glutin_event_loop_process_cursor_left(c_event, device_id);
                    }
                    WindowEvent::MouseWheel {
                        device_id,
                        delta,
                        phase,
                        ..
                    } => {
                        glutin_event_loop_process_mouse_wheel(c_event, device_id, delta, phase);
                    }
                    WindowEvent::KeyboardInput {
                        device_id,
                        input,
                        is_synthetic,
                    } => {
                        self.process_keyboard_input(c_event, device_id, input, is_synthetic);
                    }
                    WindowEvent::ReceivedCharacter(character) => {
                        glutin_event_loop_process_received_character(c_event, character);
                    }
                    WindowEvent::ModifiersChanged(modifiers) => {
                        c_event.event_type = GlutinEventType::ModifiersChanged;
                        c_event.modifiers.alt = modifiers.alt();
                        c_event.modifiers.ctrl = modifiers.ctrl();
                        c_event.modifiers.logo = modifiers.logo();
                        c_event.modifiers.shift = modifiers.shift();
                    }
                    _ => ({ result = false }),
                }
            }

            glutin::event::Event::NewEvents(_start_cause) => {
                c_event.event_type = GlutinEventType::NewEvents;
            }
            glutin::event::Event::MainEventsCleared => {
                c_event.event_type = GlutinEventType::MainEventsCleared;
            }
            glutin::event::Event::RedrawEventsCleared => {
                c_event.event_type = GlutinEventType::RedrawEventsCleared;
            }
            glutin::event::Event::LoopDestroyed => {
                c_event.event_type = GlutinEventType::LoopDestroyed;
            }
            glutin::event::Event::RedrawRequested(window_id) => {
                c_event.event_type = GlutinEventType::RedrawRequested;
                let id: BoxerUint128 = glutin_convert_window_id(window_id);
                c_event.window_id.clone_from(&id);
            }
            glutin::event::Event::Suspended => {
                c_event.event_type = GlutinEventType::Suspended;
            }
            glutin::event::Event::Resumed => {
                c_event.event_type = GlutinEventType::Resumed;
            }
            glutin::event::Event::UserEvent(custom_event) => {
                c_event.event_type = GlutinEventType::UserEvent;
                c_event.user_event.event = custom_event;
            }
            Event::DeviceEvent {
                device_id: _,
                event: _,
            } => result = false,
        }
        result
    }

    fn process_keyboard_input(
        &mut self,
        c_event: &mut GlutinEvent,
        device_id: DeviceId,
        input: KeyboardInput,
        is_synthetic: bool,
    ) {
        c_event.event_type = GlutinEventType::WindowEventKeyboardInput;
        c_event.keyboard_input.device_id = unsafe { transmute(&device_id) };
        c_event.keyboard_input.is_synthetic = is_synthetic;
        c_event.keyboard_input.scan_code = input.scancode;

        match input.state {
            ElementState::Pressed => {
                c_event.keyboard_input.state = GlutinEventInputElementState::Pressed;
            }
            ElementState::Released => {
                c_event.keyboard_input.state = GlutinEventInputElementState::Released;
            }
        }

        let key_code = match input.state {
            ElementState::Pressed => match input.virtual_keycode {
                None => None,
                Some(code) => match self.key_buffer.get(&input.scancode) {
                    None => {
                        (&mut self.key_buffer).insert(input.scancode, code);
                        Some(code)
                    }
                    Some(code) => Some(code.clone()),
                },
            },
            ElementState::Released => match self.key_buffer.remove(&input.scancode) {
                None => input.virtual_keycode,
                Some(code) => Some(code),
            },
        };

        match key_code {
            Some(code) => {
                c_event.keyboard_input.has_virtual_keycode = true;
                c_event.keyboard_input.virtual_keycode = code;
            }
            None => {
                c_event.keyboard_input.has_virtual_keycode = false;
            }
        }
    }
}

fn glutin_event_loop_process_mouse_wheel(
    c_event: &mut GlutinEvent,
    device_id: DeviceId,
    delta: MouseScrollDelta,
    phase: TouchPhase,
) {
    c_event.event_type = GlutinEventType::WindowEventMouseWheel;
    c_event.mouse_wheel.device_id = unsafe { transmute(&device_id) };

    match delta {
        MouseScrollDelta::LineDelta(x, y) => {
            c_event.mouse_wheel.delta.delta_type = GlutinEventMouseScrollDeltaType::LineDelta;
            c_event.mouse_wheel.delta.x = x as f64;
            c_event.mouse_wheel.delta.y = y as f64;
        }
        MouseScrollDelta::PixelDelta(PhysicalPosition { x, y }) => {
            c_event.mouse_wheel.delta.delta_type = GlutinEventMouseScrollDeltaType::PixelDelta;
            c_event.mouse_wheel.delta.x = x;
            c_event.mouse_wheel.delta.y = y;
        }
    }

    match phase {
        TouchPhase::Started => {
            c_event.mouse_wheel.phase = GlutinEventTouchPhase::Started;
        }
        TouchPhase::Moved => {
            c_event.mouse_wheel.phase = GlutinEventTouchPhase::Moved;
        }
        TouchPhase::Ended => {
            c_event.mouse_wheel.phase = GlutinEventTouchPhase::Ended;
        }
        TouchPhase::Cancelled => {
            c_event.mouse_wheel.phase = GlutinEventTouchPhase::Cancelled;
        }
    }
}

fn glutin_event_loop_process_touch(
    c_event: &mut GlutinEvent,
    device_id: DeviceId,
    phase: TouchPhase,
    location: PhysicalPosition<f64>,
    id: u64,
) {
    c_event.event_type = GlutinEventType::WindowEventTouch;
    c_event.touch.device_id = unsafe { transmute(&device_id) };
    c_event.touch.x = location.x;
    c_event.touch.y = location.y;
    c_event.touch.id = id;

    match phase {
        TouchPhase::Started => {
            c_event.touch.phase = GlutinEventTouchPhase::Started;
        }
        TouchPhase::Moved => {
            c_event.touch.phase = GlutinEventTouchPhase::Moved;
        }
        TouchPhase::Ended => {
            c_event.touch.phase = GlutinEventTouchPhase::Ended;
        }
        TouchPhase::Cancelled => {
            c_event.touch.phase = GlutinEventTouchPhase::Cancelled;
        }
    }
}

fn glutin_event_loop_process_mouse_input(
    c_event: &mut GlutinEvent,
    device_id: DeviceId,
    state: ElementState,
    button: MouseButton,
) {
    c_event.event_type = GlutinEventType::WindowEventMouseInput;
    c_event.mouse_input.device_id = unsafe { transmute(&device_id) };

    match state {
        ElementState::Released => {
            c_event.mouse_input.state = GlutinEventInputElementState::Released;
        }
        ElementState::Pressed => {
            c_event.mouse_input.state = GlutinEventInputElementState::Pressed;
        }
    }

    match button {
        MouseButton::Left => {
            c_event.mouse_input.button.button_type = GlutinEventMouseButtonType::Left;
            c_event.mouse_input.button.button_code = 0;
        }
        MouseButton::Right => {
            c_event.mouse_input.button.button_type = GlutinEventMouseButtonType::Right;
            c_event.mouse_input.button.button_code = 1;
        }
        MouseButton::Middle => {
            c_event.mouse_input.button.button_type = GlutinEventMouseButtonType::Middle;
            c_event.mouse_input.button.button_code = 2;
        }
        MouseButton::Other(code) => {
            c_event.mouse_input.button.button_type = GlutinEventMouseButtonType::Other;
            c_event.mouse_input.button.button_code = code;
        }
    }
}

fn glutin_event_loop_process_cursor_moved<T: Into<f64>>(
    c_event: &mut GlutinEvent,
    device_id: DeviceId,
    position: PhysicalPosition<T>,
) {
    c_event.event_type = GlutinEventType::WindowEventCursorMoved;
    c_event.cursor_moved.device_id = unsafe { transmute(&device_id) };

    c_event.cursor_moved.x = position.x.into();
    c_event.cursor_moved.y = position.y.into();
}

fn glutin_event_loop_process_cursor_entered(c_event: &mut GlutinEvent, _: DeviceId) {
    c_event.event_type = GlutinEventType::WindowEventCursorEntered;
}

fn glutin_event_loop_process_cursor_left(c_event: &mut GlutinEvent, _: DeviceId) {
    c_event.event_type = GlutinEventType::WindowEventCursorLeft;
}

fn glutin_event_loop_process_received_character(c_event: &mut GlutinEvent, character: char) {
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

#[no_mangle]
pub fn glutin_event_drop(_ptr: &mut *mut ValueBox<GlutinEvent>) {
    _ptr.drop();
}
