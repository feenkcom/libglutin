use crate::context_builder::GlutinContextBuilder;
use crate::event_loop::GlutinEventLoop;
use crate::ContextApi;
use glutin::dpi::PhysicalSize;
use glutin::{
    Api, Context, ContextBuilder, ContextCurrentState, ContextError, CreationError, NotCurrent,
    PossiblyCurrent,
};
use std::ffi::c_void;
use string_box::StringBox;
use value_box::{BoxerError, ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[derive(Debug)]
pub enum GlutinHeadlessContext {
    NotCurrent(Context<NotCurrent>),
    PossiblyCurrent(Context<PossiblyCurrent>),
}

impl GlutinHeadlessContext {
    pub fn make_current(self) -> Self {
        match self {
            GlutinHeadlessContext::NotCurrent(context) => match unsafe { context.make_current() } {
                Ok(new_context) => GlutinHeadlessContext::PossiblyCurrent(new_context),
                Err((old_context, err)) => {
                    match err {
                        ContextError::OsError(string) => {
                            error!("Failed to make context current: {}", string)
                        }
                        ContextError::IoError(error) => {
                            error!("Failed to make context current: {:?}", error)
                        }
                        ContextError::ContextLost => {
                            error!("Failed to make context current: ContextLost")
                        }
                        ContextError::FunctionUnavailable => {
                            error!("Failed to make context current: FunctionUnavailable")
                        }
                    }
                    GlutinHeadlessContext::NotCurrent(old_context)
                }
            },
            GlutinHeadlessContext::PossiblyCurrent(context) => {
                match unsafe { context.make_current() } {
                    Ok(new_context) => GlutinHeadlessContext::PossiblyCurrent(new_context),
                    Err((old_context, err)) => {
                        match err {
                            ContextError::OsError(string) => {
                                error!("Failed to make context current: {}", string)
                            }
                            ContextError::IoError(error) => {
                                error!("Failed to make context current: {:?}", error)
                            }
                            ContextError::ContextLost => {
                                error!("Failed to make context current: ContextLost")
                            }
                            ContextError::FunctionUnavailable => {
                                error!("Failed to make context current: FunctionUnavailable")
                            }
                        }
                        GlutinHeadlessContext::PossiblyCurrent(old_context)
                    }
                }
            }
        }
    }

    pub fn is_current(&self) -> bool {
        match self {
            GlutinHeadlessContext::NotCurrent(context) => context.is_current(),
            GlutinHeadlessContext::PossiblyCurrent(context) => context.is_current(),
        }
    }

    pub fn get_api(&self) -> Api {
        match self {
            GlutinHeadlessContext::NotCurrent(context) => context.get_api(),
            GlutinHeadlessContext::PossiblyCurrent(context) => context.get_api(),
        }
    }

    pub fn get_proc_address(&self, addr: &str) -> *const c_void {
        match self {
            GlutinHeadlessContext::NotCurrent(_) => {
                error!("Unable to get proc address of a not current context");
                std::ptr::null()
            }
            GlutinHeadlessContext::PossiblyCurrent(context) => context.get_proc_address(addr),
        }
    }
}

#[cfg(target_os = "linux")]
fn build_context_surfaceless<T1: ContextCurrentState>(
    cb: ContextBuilder<T1>,
    el: &GlutinEventLoop,
) -> Result<Context<NotCurrent>, CreationError> {
    use glutin::platform::unix::EventLoopWindowTargetExtUnix;
    use glutin::platform::unix::HeadlessContextExt;
    if el.is_x11() {
        return Err(CreationError::NotSupported(String::from(
            "Surfaceless context is not supported with X11",
        )));
    }
    cb.build_surfaceless(el)
}

fn build_context_headless<T1: ContextCurrentState>(
    cb: ContextBuilder<T1>,
    el: &GlutinEventLoop,
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
    el: &GlutinEventLoop,
    cb: ContextBuilder<T1>,
) -> Result<Context<NotCurrent>, [CreationError; 3]> {
    // On unix operating systems, you should always try for surfaceless first,
    // and if that does not work, headless (pbuffers), and if that too fails,
    // finally osmesa.
    //
    // If willing, you could attempt to use hidden windows instead of os mesa,
    // but note that you must handle events for the window that come on the
    // events loop.
    debug!("Trying surfaceless");
    let err1 = match build_context_surfaceless(cb.clone(), el) {
        Ok(ctx) => return Ok(ctx),
        Err(err) => err,
    };

    debug!("Trying headless");
    let err2 = match build_context_headless(cb.clone(), el) {
        Ok(ctx) => return Ok(ctx),
        Err(err) => err,
    };

    debug!("Trying osmesa");
    let err3 = match build_context_osmesa(cb) {
        Ok(ctx) => return Ok(ctx),
        Err(err) => err,
    };

    Err([err1, err2, err3])
}

#[cfg(not(target_os = "linux"))]
fn build_context<T1: ContextCurrentState>(
    el: &GlutinEventLoop,
    cb: ContextBuilder<T1>,
) -> Result<Context<NotCurrent>, CreationError> {
    debug!("Trying headless with {:?}", &cb);
    build_context_headless(cb.clone(), el)
}

#[no_mangle]
pub fn glutin_create_headless_context(
    _ptr_events_loop: *mut ValueBox<GlutinEventLoop>,
    mut _ptr_context_builder: *mut ValueBox<GlutinContextBuilder>,
) -> *mut ValueBox<GlutinHeadlessContext> {
    _ptr_events_loop
        .with_mut(|event_loop| {
            _ptr_context_builder
                .take_value()
                .and_then(|context_builder| {
                    let context = match context_builder {
                        GlutinContextBuilder::NotCurrent(builder) => {
                            build_context(event_loop, builder.clone())
                        }
                        GlutinContextBuilder::PossiblyCurrent(builder) => {
                            build_context(event_loop, builder.clone())
                        }
                    };
                    context
                        .map(|context| GlutinHeadlessContext::NotCurrent(context))
                        .map_err(|error| BoxerError::AnyError(error.into()))
                })
        })
        .into_raw()
}

// I *do not* consume the context builder
#[no_mangle]
pub fn glutin_try_headless_context(
    _ptr_events_loop: *mut ValueBox<GlutinEventLoop>,
    mut _ptr_context_builder: *mut ValueBox<GlutinContextBuilder>,
) -> bool {
    debug!("[glutin_try_headless_context] Trying if a context works");

    let context = glutin_create_headless_context(_ptr_events_loop, _ptr_context_builder);
    let is_valid = context.has_value();
    context.release();
    is_valid
}

#[no_mangle]
pub fn glutin_context_make_current(mut _ptr: *mut ValueBox<GlutinHeadlessContext>) {
    _ptr.replace_value(|context| context.make_current()).log();
}

#[no_mangle]
pub fn glutin_context_is_current(_ptr_context: *mut ValueBox<GlutinHeadlessContext>) -> bool {
    _ptr_context.with_not_null_return(false, |context| context.is_current())
}

#[no_mangle]
pub fn glutin_context_get_api(_ptr_context: *mut ValueBox<GlutinHeadlessContext>) -> ContextApi {
    _ptr_context.with_not_null_return(ContextApi::Unknown, |context| context.get_api().into())
}

#[no_mangle]
pub fn glutin_context_get_proc_address(
    _ptr_context: *mut ValueBox<GlutinHeadlessContext>,
    _ptr_symbol: *mut ValueBox<StringBox>,
) -> *const c_void {
    _ptr_context.with_not_null_return(std::ptr::null(), |context| {
        _ptr_symbol.with_not_null_return(std::ptr::null(), |symbol| {
            context.get_proc_address(symbol.to_string().as_str())
        })
    })
}

#[no_mangle]
pub fn glutin_destroy_context(_ptr: *mut ValueBox<GlutinHeadlessContext>) {
    _ptr.release();
}
