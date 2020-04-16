use boxer::boxes::{ValueBox, ValueBoxPointer};
use boxer::CBox;
use events::{glutin_events_loop_process_event, GlutinControlFlow, GlutinEvent};
use glutin::event_loop::{ControlFlow, EventLoop, EventLoopProxy, EventLoopWindowTarget};
use glutin::monitor::MonitorHandle;
use glutin::platform::desktop::EventLoopExtDesktop;
use std::time;

#[no_mangle]
pub fn glutin_create_events_loop() -> *mut ValueBox<EventLoop<()>> {
    ValueBox::new(EventLoop::new()).into_raw()
}

#[no_mangle]
pub fn glutin_destroy_events_loop(_ptr: *mut ValueBox<EventLoop<()>>) {
    _ptr.drop()
}

#[no_mangle]
pub fn glutin_events_loop_run_return(
    _ptr_events_loop: *mut ValueBox<EventLoop<()>>,
    callback: extern "C" fn(*mut GlutinEvent) -> GlutinControlFlow,
) {
    if _ptr_events_loop.is_null() {
        eprintln!("[glutin_events_loop_run_forever] _ptr_events_loop is null");
        return;
    }

    _ptr_events_loop.with_not_null(|event_loop| {
        event_loop.run_return(
            |event, _events_loop: &EventLoopWindowTarget<()>, control_flow: &mut ControlFlow| {
                *control_flow = ControlFlow::Poll;
                let mut c_event: GlutinEvent = Default::default();
                let processed = glutin_events_loop_process_event(event, &mut c_event);
                if processed {
                    let c_event_ptr = CBox::into_raw(c_event);
                    let c_control_flow = callback(c_event_ptr);
                    CBox::drop(c_event_ptr);
                    match c_control_flow {
                        GlutinControlFlow::Poll => *control_flow = ControlFlow::Poll,
                        GlutinControlFlow::Wait => {
                            *control_flow = ControlFlow::WaitUntil(
                                time::Instant::now() + time::Duration::new(0, 50 * 1000000),
                            )
                        }
                        GlutinControlFlow::Exit => *control_flow = ControlFlow::Exit,
                    }
                }
            },
        );
    });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GlutinEventLoopType {
    Windows,
    MacOS,
    X11,
    Wayland,
    Unknown,
}

#[cfg(target_os = "linux")]
fn get_event_loop_type(_event_loop: &EventLoop<()>) -> GlutinEventLoopType {
    use glutin::platform::unix::EventLoopWindowTargetExtUnix;
    if _event_loop.is_wayland() {
        return GlutinEventLoopType::Wayland;
    }
    if _event_loop.is_x11() {
        return GlutinEventLoopType::X11;
    }
    return GlutinEventLoopType::Unknown;
}

#[cfg(target_os = "windows")]
fn get_event_loop_type(_event_loop: &EventLoop<()>) -> GlutinEventLoopType {
    GlutinEventLoopType::Windows
}

#[cfg(target_os = "macos")]
fn get_event_loop_type(_event_loop: &EventLoop<()>) -> GlutinEventLoopType {
    GlutinEventLoopType::MacOS
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
fn get_event_loop_type(_event_loop: &EventLoop<()>) -> GlutinEventLoopType {
    GlutinEventLoopType::Unknown
}

#[no_mangle]
fn glutin_events_loop_get_type(
    _ptr_event_loop: *mut ValueBox<EventLoop<()>>,
) -> GlutinEventLoopType {
    _ptr_event_loop.with_not_null_return(GlutinEventLoopType::Unknown, |event_loop| {
        get_event_loop_type(event_loop)
    })
}

#[no_mangle]
fn glutin_events_loop_create_proxy(
    _ptr_event_loop: *mut ValueBox<EventLoop<()>>,
) -> *mut ValueBox<EventLoopProxy<()>> {
    _ptr_event_loop.with(|event_loop| ValueBox::new(event_loop.create_proxy()).into_raw())
}

#[no_mangle]
fn glutin_events_loop_drop_proxy(_ptr: *mut ValueBox<EventLoopProxy<()>>) {
    _ptr.drop();
}

///////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////// M O N I T O R    I D /////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[no_mangle]
fn glutin_events_loop_get_primary_monitor(
    _ptr_event_loop: *mut ValueBox<EventLoop<()>>,
) -> *mut ValueBox<MonitorHandle> {
    _ptr_event_loop.with(|event_loop| ValueBox::new(event_loop.primary_monitor()).into_raw())
}

#[no_mangle]
fn glutin_primary_monitor_free(_ptr_monitor_id: *mut ValueBox<MonitorHandle>) {
    _ptr_monitor_id.drop()
}

#[no_mangle]
fn glutin_primary_monitor_get_hidpi_factor(_ptr_monitor_id: *mut ValueBox<MonitorHandle>) -> f64 {
    _ptr_monitor_id.with_not_null_return(1.0, |monitor_id| monitor_id.scale_factor())
}
