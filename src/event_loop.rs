use boxer::CBox;
use events::{GlutinEvent, GlutinControlFlow, glutin_events_loop_process_event};
use std::time;
use glutin::platform::desktop::EventLoopExtDesktop;
use glutin::event_loop::{EventLoop, ControlFlow, EventLoopWindowTarget};
use glutin::monitor::MonitorHandle;

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