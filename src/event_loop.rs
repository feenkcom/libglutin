use boxer::{ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use events::{EventProcessor, GlutinControlFlow, GlutinEvent, GlutinEventType};
use glutin::event_loop::EventLoopClosed;
use glutin::event_loop::{ControlFlow, EventLoop, EventLoopProxy, EventLoopWindowTarget};
use glutin::monitor::MonitorHandle;
use glutin::platform::desktop::EventLoopExtDesktop;
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use std::collections::VecDeque;
use std::ffi::c_void;
use std::intrinsics::transmute;
use std::sync::{Arc, Mutex};
use std::time;

pub type GlutinCustomEvent = u32;
pub type GlutinEventLoop = EventLoop<GlutinCustomEvent>;
pub type GlutinEventLoopProxy = EventLoopProxy<GlutinCustomEvent>;

pub struct SemaphoreSignaller {
    semaphore_callback: unsafe extern "C" fn(usize, *const c_void),
    semaphore_index: usize,
    semaphore_thunk: *const c_void,
}

impl SemaphoreSignaller {
    pub fn new(
        semaphore_callback: unsafe extern "C" fn(usize, *const c_void),
        semaphore_index: usize,
        semaphore_thunk: *const c_void,
    ) -> Self {
        Self {
            semaphore_callback,
            semaphore_index,
            semaphore_thunk,
        }
    }

    pub fn signal(&self) {
        let callback = self.semaphore_callback;
        unsafe { callback(self.semaphore_index, self.semaphore_thunk) };
    }
}

pub struct MainEventClearedSignaller {
    callback: unsafe extern "C" fn(*const c_void),
    thunk: *const c_void,
}

impl MainEventClearedSignaller {
    pub fn new(callback: unsafe extern "C" fn(*const c_void), thunk: *const c_void) -> Self {
        Self { callback, thunk }
    }

    pub fn signal(&self) {
        let callback = self.callback;
        unsafe { callback(self.thunk) };
    }
}

pub struct PollingEventLoop {
    events: Mutex<VecDeque<GlutinEvent>>,
    semaphore_signaller: Option<SemaphoreSignaller>,
    main_events_cleared_signaller: Option<MainEventClearedSignaller>,
}

impl PollingEventLoop {
    pub fn new() -> Self {
        Self {
            events: Mutex::new(VecDeque::new()),
            semaphore_signaller: None,
            main_events_cleared_signaller: None,
        }
    }

    pub fn with_semaphore_signaller(
        mut self,
        semaphore_callback: extern "C" fn(usize, *const c_void),
        semaphore_index: usize,
        semaphore_thunk: *const c_void,
    ) -> Self {
        self.semaphore_signaller = Some(SemaphoreSignaller::new(
            semaphore_callback,
            semaphore_index,
            semaphore_thunk,
        ));
        self
    }

    pub fn with_main_events_signaller(
        mut self,
        callback: extern "C" fn(*const c_void),
        thunk: *const c_void,
    ) -> Self {
        self.main_events_cleared_signaller = Some(MainEventClearedSignaller::new(callback, thunk));
        self
    }

    pub fn poll(&mut self) -> Option<GlutinEvent> {
        match self.events.lock() {
            Ok(mut guard) => guard.pop_front(),
            Err(err) => {
                println!(
                    "[PollingEventLoop::poll] Error locking the guard: {:?}",
                    err
                );
                None
            }
        }
    }

    pub fn push(&mut self, event: GlutinEvent) {
        Self::push_event(&mut self.events, event);
    }

    pub fn push_event(events: &mut Mutex<VecDeque<GlutinEvent>>, event: GlutinEvent) {
        match events.lock() {
            Ok(mut guard) => {
                guard.push_back(event);
            }
            Err(err) => println!(
                "[PollingEventLoop::push] Error locking the guard: {:?}",
                err
            ),
        }
    }

    pub fn signal_semaphore(&self) {
        if self.semaphore_signaller.is_some() {
            self.semaphore_signaller.as_ref().unwrap().signal();
        }
    }

    pub fn signal_main_events_cleared(&self) {
        if self.main_events_cleared_signaller.is_some() {
            self.main_events_cleared_signaller
                .as_ref()
                .unwrap()
                .signal();
        }
    }

    pub fn run(&'static mut self) {
        let mut event_processor = EventProcessor::new();
        let event_loop = GlutinEventLoop::with_user_event();

        let window = ContextBuilder::new().build_windowed(WindowBuilder::default(), &event_loop);

        println!("new window: {:?}", window);

        event_loop.run(move |event, _, control_flow: &mut ControlFlow| {
            *control_flow = ControlFlow::Wait;

            let mut c_event = GlutinEvent::default();
            let processed = event_processor.process(event, &mut c_event);
            if processed {
                let event_type = c_event.event_type;
                if event_type != GlutinEventType::MainEventsCleared
                    && event_type != GlutinEventType::RedrawEventsCleared
                    && event_type != GlutinEventType::NewEvents
                {
                    self.push(c_event);
                    self.signal_semaphore();
                }

                if event_type == GlutinEventType::MainEventsCleared {
                    self.signal_main_events_cleared();
                }
            }
        })
    }
}

#[repr(C)]
pub struct GlutinEventLoopWaker {
    proxy: Arc<GlutinEventLoopProxy>,
}

impl GlutinEventLoopWaker {
    pub fn new(event_loop: &GlutinEventLoop) -> Self {
        Self {
            proxy: Arc::new(event_loop.create_proxy()),
        }
    }

    pub fn wake(&self, event: GlutinCustomEvent) -> Result<(), EventLoopClosed<GlutinCustomEvent>> {
        self.proxy.send_event(event)
    }
}

extern "C" fn glutin_waker_wake(waker_ptr: *const c_void, event: GlutinCustomEvent) -> bool {
    let ptr = waker_ptr as *mut ValueBox<GlutinEventLoopWaker>;
    ptr.with_not_null_return(false, |waker| match waker.wake(event) {
        Ok(_) => true,
        Err(_) => false,
    })
}

#[no_mangle]
pub fn glutin_event_loop_waker_create(
    event_loop_ptr: *mut ValueBox<GlutinEventLoop>,
) -> *mut ValueBox<GlutinEventLoopWaker> {
    event_loop_ptr.with_not_null_return(std::ptr::null_mut(), |event_loop| {
        ValueBox::new(GlutinEventLoopWaker::new(event_loop)).into_raw()
    })
}

#[no_mangle]
pub fn glutin_event_loop_waker_function() -> extern "C" fn(*const c_void, u32) -> bool {
    glutin_waker_wake
}

#[no_mangle]
pub fn glutin_event_loop_waker_drop(_ptr: &mut *mut ValueBox<GlutinEventLoopWaker>) {
    _ptr.drop();
}

#[no_mangle]
pub fn glutin_polling_event_loop_new() -> *mut ValueBox<PollingEventLoop> {
    ValueBox::new(PollingEventLoop::new()).into_raw()
}

#[no_mangle]
pub fn glutin_polling_event_loop_new_with_semaphore_and_main_events_signaller(
    semaphore_callback: extern "C" fn(usize, *const c_void),
    semaphore_index: usize,
    semaphore_thunk: *const c_void,
    main_events_callback: extern "C" fn(*const c_void),
    main_events_thunk: *const c_void,
) -> *mut ValueBox<PollingEventLoop> {
    ValueBox::new(
        PollingEventLoop::new()
            .with_semaphore_signaller(semaphore_callback, semaphore_index, semaphore_thunk)
            .with_main_events_signaller(main_events_callback, main_events_thunk),
    )
    .into_raw()
}

#[no_mangle]
pub fn glutin_polling_event_loop_poll(
    _ptr: *mut ValueBox<PollingEventLoop>,
) -> *mut ValueBox<GlutinEvent> {
    _ptr.with_not_null_return(std::ptr::null_mut(), |event_loop| match event_loop.poll() {
        None => std::ptr::null_mut(),
        Some(event) => ValueBox::new(event).into_raw(),
    })
}

#[no_mangle]
pub fn glutin_polling_event_loop_run(_ptr_event_loop: *mut ValueBox<PollingEventLoop>) {
    if _ptr_event_loop.is_null() {
        eprintln!("[glutin_polling_event_loop_run_return] _ptr_event_loop is null");
        return;
    }

    _ptr_event_loop.with_not_null(|polling_event_loop| {
        let event_loop: &'static mut PollingEventLoop = unsafe { transmute(polling_event_loop) };
        event_loop.run();
    });
}

#[no_mangle]
pub fn glutin_polling_event_loop_drop(_ptr: &mut *mut ValueBox<PollingEventLoop>) {
    _ptr.drop();
}

#[no_mangle]
pub fn glutin_create_events_loop() -> *mut ValueBox<GlutinEventLoop> {
    #[cfg(target_os = "linux")]
    {
        // respect the winit backend if it is set
        if (std::env::var("WINIT_UNIX_BACKEND").is_err()) {
            std::env::set_var("WINIT_UNIX_BACKEND", "x11");
        }
    }
    ValueBox::new(GlutinEventLoop::with_user_event()).into_raw()
}

#[no_mangle]
pub fn glutin_destroy_events_loop(_ptr: &mut *mut ValueBox<GlutinEventLoop>) {
    _ptr.drop();
}

#[no_mangle]
pub fn glutin_events_loop_run_return(
    _ptr_events_loop: *mut ValueBox<GlutinEventLoop>,
    callback: extern "C" fn(*mut GlutinEvent) -> GlutinControlFlow,
) {
    if _ptr_events_loop.is_null() {
        eprintln!("[glutin_events_loop_run_return] _ptr_events_loop is null");
        return;
    }

    let mut event_processor = EventProcessor::new();

    _ptr_events_loop.with_not_null(|event_loop| {
        event_loop.run_return(
            |event,
             _events_loop: &EventLoopWindowTarget<GlutinCustomEvent>,
             control_flow: &mut ControlFlow| {
                *control_flow = ControlFlow::Poll;
                let mut c_event: GlutinEvent = Default::default();
                let processed = event_processor.process(event, &mut c_event);
                if processed {
                    let c_event_ptr = Box::into_raw(Box::new(c_event));
                    let c_control_flow = callback(c_event_ptr);
                    unsafe { Box::from_raw(c_event_ptr) };
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
fn get_event_loop_type(_event_loop: &GlutinEventLoop) -> GlutinEventLoopType {
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
fn get_event_loop_type(_event_loop: &GlutinEventLoop) -> GlutinEventLoopType {
    GlutinEventLoopType::Windows
}

#[cfg(target_os = "macos")]
fn get_event_loop_type(_event_loop: &GlutinEventLoop) -> GlutinEventLoopType {
    GlutinEventLoopType::MacOS
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
fn get_event_loop_type(_event_loop: &GlutinEventLoop) -> GlutinEventLoopType {
    GlutinEventLoopType::Unknown
}

#[no_mangle]
fn glutin_events_loop_get_type(
    _ptr_event_loop: *mut ValueBox<GlutinEventLoop>,
) -> GlutinEventLoopType {
    _ptr_event_loop.with_not_null_return(GlutinEventLoopType::Unknown, |event_loop| {
        get_event_loop_type(event_loop)
    })
}

#[no_mangle]
fn glutin_events_loop_create_proxy(
    _ptr_event_loop: *mut ValueBox<GlutinEventLoop>,
) -> *mut ValueBox<GlutinEventLoopProxy> {
    _ptr_event_loop.with_not_null_return(std::ptr::null_mut(), |event_loop| {
        ValueBox::new(event_loop.create_proxy()).into_raw()
    })
}

#[no_mangle]
fn glutin_events_loop_drop_proxy(_ptr: &mut *mut ValueBox<GlutinEventLoopProxy>) {
    _ptr.drop();
}

///////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////// M O N I T O R    I D /////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[no_mangle]
fn glutin_events_loop_get_primary_monitor(
    _ptr_event_loop: *mut ValueBox<GlutinEventLoop>,
) -> *mut ValueBox<MonitorHandle> {
    _ptr_event_loop.with_not_null_return(std::ptr::null_mut(), |event_loop| {
        match event_loop.primary_monitor() {
            None => std::ptr::null_mut(),
            Some(monitor) => ValueBox::new(monitor).into_raw(),
        }
    })
}

#[no_mangle]
fn glutin_primary_monitor_free(_ptr_monitor_id: &mut *mut ValueBox<MonitorHandle>) {
    _ptr_monitor_id.drop();
}

#[no_mangle]
fn glutin_primary_monitor_get_hidpi_factor(_ptr_monitor_id: *mut ValueBox<MonitorHandle>) -> f64 {
    _ptr_monitor_id.with_not_null_return(1.0, |monitor_id| monitor_id.scale_factor())
}
