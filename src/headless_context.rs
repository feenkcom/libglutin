use boxer::boxes::{ValueBox, ValueBoxPointer};
use boxer::string::{BoxerString, BoxerStringPointer};
use glutin::dpi::PhysicalSize;
use glutin::event_loop::EventLoop;
use glutin::{
    Context, ContextBuilder, ContextCurrentState, ContextError, CreationError, NotCurrent,
    PossiblyCurrent,
};
use std::os::raw::c_void;
use ContextApi;

#[cfg(target_os = "linux")]
fn build_context_surfaceless<T1: ContextCurrentState>(
    cb: ContextBuilder<T1>,
    el: &EventLoop<()>,
) -> Result<Context<NotCurrent>, CreationError> {
    use glutin::platform::unix::HeadlessContextExt;
    if el.is_x11() {
        return Err(CreationError::NotSupported("Surfaceless context is not supported with X11".into_string()))
    }
    cb.build_surfaceless(el)
}

fn build_context_headless<T1: ContextCurrentState>(
    cb: ContextBuilder<T1>,
    el: &EventLoop<()>,
) -> Result<Context<NotCurrent>, CreationError> {
    let size_one = PhysicalSize::new(1, 1);
    cb.build_headless(el, size_one)
}

#[cfg(target_os = "linux")]
fn build_context_osmesa<T1: ContextCurrentState>(
    cb: ContextBuilder<T1>,
) -> Result<Context<NotCurrent>, CreationError> {
    use glutin::platform::unix::HeadlessContextExt;
    let size_one = PhysicalSize::new(1, 1);
    cb.build_osmesa(size_one)
}

#[cfg(target_os = "linux")]
fn build_context<T1: ContextCurrentState>(
    el: &EventLoop<()>,
    cb: ContextBuilder<T1>,
) -> Result<Context<NotCurrent>, [CreationError; 3]> {
    // On unix operating systems, you should always try for surfaceless first,
    // and if that does not work, headless (pbuffers), and if that too fails,
    // finally osmesa.
    //
    // If willing, you could attempt to use hidden windows instead of os mesa,
    // but note that you must handle events for the window that come on the
    // events loop.
    if cfg!(debug_assertions) {
        println!("[Glutin][build_context] Trying surfaceless");
    }
    let err1 = match build_context_surfaceless(cb.clone(), el) {
        Ok(ctx) => return Ok(ctx),
        Err(err) => err,
    };

    if cfg!(debug_assertions) {
        println!("[Glutin][build_context] Trying headless");
    }
    let err2 = match build_context_headless(cb.clone(), el) {
        Ok(ctx) => return Ok(ctx),
        Err(err) => err,
    };

    if cfg!(debug_assertions) {
        println!("[Glutin][build_context] Trying osmesa");
    }
    let err3 = match build_context_osmesa(cb) {
        Ok(ctx) => return Ok(ctx),
        Err(err) => err,
    };

    Err([err1, err2, err3])
}

#[cfg(not(target_os = "linux"))]
fn build_context<T1: ContextCurrentState>(
    el: &EventLoop<()>,
    cb: ContextBuilder<T1>,
) -> Result<Context<NotCurrent>, CreationError> {
    if cfg!(debug_assertions) {
        println!("[Glutin][build_context] Trying headless");
    }
    build_context_headless(cb.clone(), el)
}

#[no_mangle]
pub fn glutin_create_headless_context(
    _ptr_events_loop: *mut ValueBox<EventLoop<()>>,
    mut _ptr_context_builder: *mut ValueBox<ContextBuilder<NotCurrent>>,
) -> *mut ValueBox<Context<NotCurrent>> {
    if _ptr_events_loop.is_null() {
        eprintln!("[glutin_create_windowed_context] Event loop is null");
        return std::ptr::null_mut();
    }

    if _ptr_context_builder.is_null() {
        eprintln!("[glutin_create_windowed_context] Context builder is null");
        return std::ptr::null_mut();
    }

    _ptr_events_loop.with(|event_loop| {
        _ptr_context_builder.with_value_consumed(|context_builder| {
            if cfg!(debug_assertions) {
                println!("[Glutin] OpenGL Headless Context: {:?}", context_builder);
                println!(
                    "[Glutin] Primary monitor: {:?}",
                    event_loop.primary_monitor()
                );
            }

            match build_context(event_loop, context_builder) {
                Ok(context) => ValueBox::new(context).into_raw(),
                Err(err) => {
                    if cfg!(debug_assertions) {
                        eprintln!("[Glutin] Could not create headless context {:?}", err);
                    }
                    std::ptr::null_mut()
                }
            }
        })
    })
}

// I *do not* consume the context builder
#[no_mangle]
pub fn glutin_try_headless_context(
    _ptr_events_loop: *mut ValueBox<EventLoop<()>>,
    _ptr_context_builder: *mut ValueBox<ContextBuilder<NotCurrent>>,
) -> bool {
    let builder_copy = _ptr_context_builder
        .with_value(|context_builder| ValueBox::new(context_builder).into_raw());
    let context = glutin_create_headless_context(_ptr_events_loop, builder_copy);
    let is_valid = context.is_valid();
    context.drop();
    is_valid
}

#[no_mangle]
pub fn glutin_context_make_current(mut _ptr: *mut ValueBox<Context<PossiblyCurrent>>) {
    _ptr.with_value_and_box_consumed(|window, value_box| {
        let context: Context<PossiblyCurrent>;

        match unsafe { window.make_current() } {
            Ok(new_context) => context = new_context,
            Err((old_context, err)) => {
                context = old_context;
                match err {
                    ContextError::OsError(string) => {
                        eprintln!("OS Error in make_current: {}", string)
                    }
                    ContextError::IoError(error) => {
                        eprintln!("IO Error in make_current: {:?}", error)
                    }
                    ContextError::ContextLost => eprintln!("ContextLost Error in make_current"),
                    ContextError::FunctionUnavailable => {
                        eprintln!("FunctionUnavailable Error in make_current")
                    }
                }
            }
        }
        unsafe {
            value_box.mutate(context);
        };
    });
}

#[no_mangle]
pub fn glutin_context_is_current(_ptr_context: *mut ValueBox<Context<PossiblyCurrent>>) -> bool {
    _ptr_context.with_not_null_return(false, |context| context.is_current())
}

#[no_mangle]
pub fn glutin_context_get_api(_ptr_context: *mut ValueBox<Context<PossiblyCurrent>>) -> ContextApi {
    _ptr_context.with_not_null_return(ContextApi::Unknown, |context| context.get_api().into())
}

#[no_mangle]
pub fn glutin_context_get_proc_address(
    _ptr_context: *mut ValueBox<Context<PossiblyCurrent>>,
    _ptr_symbol: *mut BoxerString,
) -> *const c_void {
    _ptr_context.with_not_null_return(std::ptr::null(), |context| {
        _ptr_symbol.with(|symbol| context.get_proc_address(symbol.to_string().as_str()))
    })
}

#[no_mangle]
pub fn glutin_destroy_context(_ptr: *mut ValueBox<Context<PossiblyCurrent>>) {
    _ptr.drop();
}
