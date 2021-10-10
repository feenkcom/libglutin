use boxer::number::BoxerUint128;
use boxer::point::BoxerPointI32;
use boxer::size::BoxerSizeU32;
use boxer::string::BoxerString;
use boxer::{ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use context_builder::GlutinContextBuilder;
use enums::GlutinCursorIcon;
use event_loop::GlutinEventLoop;
use glutin::dpi::{PhysicalPosition, PhysicalSize};
use glutin::window::Window;
use glutin::window::WindowBuilder;
use glutin::{Api, ContextError, NotCurrent, PixelFormat, PossiblyCurrent, WindowedContext};
use pixel_format::glutin_pixel_format_default;
use std::os::raw::c_void;
use {glutin_convert_window_id, ContextApi};

#[derive(Debug)]
pub enum GlutinWindowedContext {
    NotCurrent(WindowedContext<NotCurrent>),
    PossiblyCurrent(WindowedContext<PossiblyCurrent>),
}

impl GlutinWindowedContext {
    pub fn make_current(self) -> Result<GlutinWindowedContext, (Self, ContextError)> {
        (match self {
            GlutinWindowedContext::NotCurrent(context) => (unsafe { context.make_current() })
                .map_err(|error| (GlutinWindowedContext::NotCurrent(error.0), error.1)),
            GlutinWindowedContext::PossiblyCurrent(context) => (unsafe { context.make_current() })
                .map_err(|error| (GlutinWindowedContext::PossiblyCurrent(error.0), error.1)),
        })
        .map(|context| GlutinWindowedContext::PossiblyCurrent(context))
    }

    pub fn swap_buffers(&self) -> Result<(), ContextError> {
        match self {
            GlutinWindowedContext::NotCurrent(_) => Err(ContextError::FunctionUnavailable),
            GlutinWindowedContext::PossiblyCurrent(context) => context.swap_buffers(),
        }
    }

    pub fn get_proc_address(&self, addr: &str) -> *const c_void {
        match self {
            GlutinWindowedContext::NotCurrent(_) => {
                error!("Unable to get proc address of a not current context");
                std::ptr::null()
            }
            GlutinWindowedContext::PossiblyCurrent(context) => context.get_proc_address(addr),
        }
    }

    pub fn window(&self) -> &Window {
        match self {
            GlutinWindowedContext::NotCurrent(context) => context.window(),
            GlutinWindowedContext::PossiblyCurrent(context) => context.window(),
        }
    }

    pub fn get_api(&self) -> Api {
        match self {
            GlutinWindowedContext::NotCurrent(context) => context.get_api(),
            GlutinWindowedContext::PossiblyCurrent(context) => context.get_api(),
        }
    }

    pub fn is_current(&self) -> bool {
        match self {
            GlutinWindowedContext::NotCurrent(context) => context.is_current(),
            GlutinWindowedContext::PossiblyCurrent(context) => context.is_current(),
        }
    }

    pub fn get_pixel_format(&self) -> Option<PixelFormat> {
        match self {
            GlutinWindowedContext::NotCurrent(_) => {
                error!("Unable to get pixel format of the not current context");
                None
            }
            GlutinWindowedContext::PossiblyCurrent(context) => Some(context.get_pixel_format()),
        }
    }

    pub fn resize(&self, size: PhysicalSize<u32>) {
        match self {
            GlutinWindowedContext::NotCurrent(_) => error!("Unable to resize not current context"),
            GlutinWindowedContext::PossiblyCurrent(context) => context.resize(size),
        }
    }
}

#[no_mangle]
pub fn glutin_create_windowed_context(
    _ptr_events_loop: *mut ValueBox<GlutinEventLoop>,
    mut _ptr_window_builder: *mut ValueBox<WindowBuilder>,
    mut _ptr_context_builder: *mut ValueBox<GlutinContextBuilder>,
) -> *mut ValueBox<GlutinWindowedContext> {
    if _ptr_events_loop.is_null() {
        error!("Event loop is null");
        return std::ptr::null_mut();
    }

    if _ptr_window_builder.is_null() {
        error!("Window builder is null");
        return std::ptr::null_mut();
    }

    if _ptr_context_builder.is_null() {
        error!("Context builder is null");
        return std::ptr::null_mut();
    }

    _ptr_events_loop.with_not_null_return(std::ptr::null_mut(), |event_loop| {
        _ptr_context_builder.with_not_null_value_consumed_return(
            std::ptr::null_mut(),
            |context_builder| {
                _ptr_window_builder.with_not_null_value_consumed_return(
                    std::ptr::null_mut(),
                    |window_builder| {
                        debug!("Windowed context builder: {:?}", &context_builder);
                        debug!("Window builder: {:?}", &window_builder);

                        match context_builder.build_windowed(window_builder, event_loop) {
                            Ok(windowed_context) => ValueBox::new(windowed_context).into_raw(),
                            Err(err) => {
                                error!("Could not create windowed context {:?}", err);
                                std::ptr::null_mut()
                            }
                        }
                    },
                )
            },
        )
    })
}

///////////////////////////////////////////////////////////////////////////////////////
///////////////////////////// W I N D O W   A C C E S S O R S /////////////////////////
///////////////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub fn glutin_windowed_context_make_current(mut _ptr_window: *mut ValueBox<GlutinWindowedContext>) {
    if _ptr_window.is_null() {
        return;
    }

    _ptr_window.with_not_null_value_mutate(|window| {
        let context: GlutinWindowedContext;

        match window.make_current() {
            Ok(windowed_context) => context = windowed_context,
            Err((windowed_context, err)) => {
                context = windowed_context;
                match err {
                    ContextError::OsError(string) => error!("OS Error in make_current: {}", string),
                    ContextError::IoError(error) => error!("IO Error in make_current: {:?}", error),
                    ContextError::ContextLost => error!("ContextLost Error in make_current"),
                    ContextError::FunctionUnavailable => {
                        error!("FunctionUnavailable Error in make_current")
                    }
                }
            }
        }
        context
    });
}

#[no_mangle]
pub fn glutin_windowed_context_swap_buffers(_ptr_window: *mut ValueBox<GlutinWindowedContext>) {
    _ptr_window.with_not_null(|window| match window.swap_buffers() {
        Ok(_) => {}
        Err(err) => match err {
            ContextError::OsError(string) => error!("OS Error in swap_buffers: {}", string),
            ContextError::IoError(error) => error!("IO Error in swap_buffers: {:?}", error),
            ContextError::ContextLost => error!("ContextLost Error in swap_buffers"),
            ContextError::FunctionUnavailable => {
                error!("FunctionUnavailable Error in swap_buffers")
            }
        },
    });
}

#[no_mangle]
pub fn glutin_windowed_context_get_proc_address(
    _ptr_window: *mut ValueBox<GlutinWindowedContext>,
    _ptr_symbol: *mut ValueBox<BoxerString>,
) -> *const c_void {
    _ptr_window.with_not_null_return(std::ptr::null(), |window| {
        _ptr_symbol.with_not_null_return(std::ptr::null(), |symbol| {
            window.get_proc_address(symbol.to_string().as_str())
        })
    })
}

#[no_mangle]
pub fn glutin_windowed_context_get_api(
    _ptr_context: *mut ValueBox<GlutinWindowedContext>,
) -> ContextApi {
    _ptr_context.with_not_null_return(ContextApi::Unknown, |context| context.get_api().into())
}

#[no_mangle]
pub fn glutin_windowed_context_is_current(
    _ptr_window: *mut ValueBox<GlutinWindowedContext>,
) -> bool {
    _ptr_window.with_not_null_return(false, |window| window.is_current())
}

#[no_mangle]
pub fn glutin_windowed_context_get_pixel_format(
    _ptr_window: *mut ValueBox<GlutinWindowedContext>,
) -> *mut ValueBox<PixelFormat> {
    _ptr_window.with_not_null_return(std::ptr::null_mut(), |window| {
        match window.get_pixel_format() {
            None => glutin_pixel_format_default(),
            Some(format) => ValueBox::new(format).into_raw(),
        }
    })
}

#[no_mangle]
pub fn glutin_windowed_context_resize(
    _ptr_window: *mut ValueBox<GlutinWindowedContext>,
    _width: u32,
    _height: u32,
) {
    _ptr_window.with_not_null(|window| window.resize(PhysicalSize::new(_width, _height)));
}

#[no_mangle]
pub fn glutin_windowed_context_request_redraw(_ptr_window: *mut ValueBox<GlutinWindowedContext>) {
    _ptr_window.with_not_null(|window| window.window().request_redraw());
}

#[no_mangle]
pub fn glutin_windowed_context_get_scale_factor(
    _ptr_window: *mut ValueBox<GlutinWindowedContext>,
) -> f64 {
    _ptr_window.with_not_null_return(1.0, |window| window.window().scale_factor())
}

#[no_mangle]
pub fn glutin_windowed_context_get_inner_size(
    _ptr_window: *mut ValueBox<GlutinWindowedContext>,
    _ptr_size: *mut ValueBox<BoxerSizeU32>,
) {
    _ptr_window.with_not_null(|window| {
        _ptr_size.with_not_null(|size| {
            let window_size: PhysicalSize<u32> = window.window().inner_size();
            size.width = window_size.width;
            size.height = window_size.height;
        });
    });
}

#[no_mangle]
pub fn glutin_windowed_context_set_inner_size(
    _ptr_window: *mut ValueBox<GlutinWindowedContext>,
    _width: u32,
    _height: u32,
) {
    _ptr_window.with_not_null(|window| {
        window
            .window()
            .set_inner_size(PhysicalSize::new(_width, _height))
    });
}

#[no_mangle]
pub fn glutin_windowed_context_get_position(
    _ptr_window: *mut ValueBox<GlutinWindowedContext>,
    _ptr_position: *mut ValueBox<BoxerPointI32>,
) {
    _ptr_window.with_not_null(|window| {
        _ptr_position.with_not_null(|position| match window.window().outer_position() {
            Ok(physical_position) => {
                position.x = physical_position.x;
                position.y = physical_position.y;
            }
            Err(err) => {
                error!(
                    "[glutin_windowed_context_get_position] Error getting position: {:?}",
                    err
                );
                position.be_zero()
            }
        })
    });
}

#[no_mangle]
pub fn glutin_windowed_context_set_position(
    _ptr_window: *mut ValueBox<GlutinWindowedContext>,
    x: i32,
    y: i32,
) {
    _ptr_window.with_not_null(|window| {
        window
            .window()
            .set_outer_position(PhysicalPosition::new(x, y))
    });
}

#[no_mangle]
pub fn glutin_windowed_context_get_id(
    _ptr_window: *mut ValueBox<GlutinWindowedContext>,
    _ptr_number: *mut ValueBox<BoxerUint128>,
) {
    _ptr_window.with_not_null(|window| {
        _ptr_number.with_not_null(|number| {
            let id: BoxerUint128 = glutin_convert_window_id(window.window().id());
            number.low = id.low;
            number.high = id.high
        });
    });
}

#[no_mangle]
pub fn glutin_windowed_context_set_title(
    _ptr_window: *mut ValueBox<GlutinWindowedContext>,
    _ptr_boxer_string: *mut ValueBox<BoxerString>,
) {
    _ptr_window.with_not_null(|window| {
        _ptr_boxer_string
            .with_not_null(|string| window.window().set_title(string.to_string().as_ref()))
    });
}

#[no_mangle]
pub fn glutin_windowed_context_set_cursor_icon(
    _ptr_window: *mut ValueBox<GlutinWindowedContext>,
    cursor_icon: GlutinCursorIcon,
) {
    _ptr_window.with_not_null(|window| window.window().set_cursor_icon(cursor_icon.into()));
}

#[no_mangle]
pub fn glutin_windowed_context_set_maximized(
    _ptr_window: *mut ValueBox<GlutinWindowedContext>,
    maximized: bool,
) {
    _ptr_window.with_not_null(|window| {
        window.window().set_maximized(maximized);
    });
}

#[no_mangle]
pub fn glutin_destroy_windowed_context(_ptr: &mut *mut ValueBox<GlutinWindowedContext>) {
    _ptr.drop();
}
