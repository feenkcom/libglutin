use boxer::boxes::{ValueBox, ValueBoxPointer};
use boxer::number::BoxerUint128;
use boxer::point::BoxerPointI32;
use boxer::size::BoxerSizeU32;
use boxer::string::{BoxerString, BoxerStringPointer};
use boxer::CBox;
use enums::GlutinCursorIcon;
use glutin::dpi::{PhysicalPosition, PhysicalSize};
use glutin::event_loop::EventLoop;
use glutin::window::WindowBuilder;
use glutin::{
    ContextBuilder, ContextError, NotCurrent, PixelFormat, PossiblyCurrent, WindowedContext,
};
use std::os::raw::c_void;
use {glutin_convert_window_id, ContextApi};

#[no_mangle]
pub fn glutin_create_windowed_context(
    _ptr_events_loop: *mut ValueBox<EventLoop<()>>,
    mut _ptr_window_builder: *mut ValueBox<WindowBuilder>,
    mut _ptr_context_builder: *mut ValueBox<ContextBuilder<NotCurrent>>,
) -> *mut ValueBox<WindowedContext<PossiblyCurrent>> {
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

    _ptr_events_loop.with_not_null_return(std::ptr::null_mut(), |event_loop| {
        _ptr_context_builder.with_value_consumed(|context_builder| {
            _ptr_window_builder.with_value_consumed(|window_builder| {
                if cfg!(debug_assertions) {
                    println!("[Glutin] OpenGL Context: {:?}", context_builder);
                    println!("[Glutin] Window attributes: {:?}", window_builder);
                }

                match context_builder.build_windowed(window_builder, event_loop) {
                    Ok(windowed_context) => match unsafe { windowed_context.make_current() } {
                        Ok(windowed_context) => ValueBox::new(windowed_context).into_raw(),
                        Err(err) => {
                            if cfg!(debug_assertions) {
                                println!("[Glutin] Could not create context {:?}", err);
                            }
                            std::ptr::null_mut()
                        }
                    },
                    Err(err) => {
                        if cfg!(debug_assertions) {
                            println!("[Glutin] Could not create context {:?}", err);
                        }
                        std::ptr::null_mut()
                    }
                }
            })
        })
    })
}

///////////////////////////////////////////////////////////////////////////////////////
///////////////////////////// W I N D O W   A C C E S S O R S /////////////////////////
///////////////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub fn glutin_windowed_context_make_current(
    mut _ptr_window: *mut ValueBox<WindowedContext<PossiblyCurrent>>,
) {
    if _ptr_window.is_null() {
        return;
    }

    _ptr_window.with_value_and_box_consumed(|window, value_box| {
        let context: WindowedContext<PossiblyCurrent>;

        match unsafe { window.make_current() } {
            Ok(windowed_context) => context = windowed_context,
            Err((windowed_context, err)) => {
                context = windowed_context;
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
pub fn glutin_windowed_context_swap_buffers(
    _ptr_window: *mut ValueBox<WindowedContext<PossiblyCurrent>>,
) {
    _ptr_window.with_not_null(|window| match window.swap_buffers() {
        Ok(_) => {}
        Err(err) => match err {
            ContextError::OsError(string) => eprintln!("OS Error in swap_buffers: {}", string),
            ContextError::IoError(error) => eprintln!("IO Error in swap_buffers: {:?}", error),
            ContextError::ContextLost => eprintln!("ContextLost Error in swap_buffers"),
            ContextError::FunctionUnavailable => {
                eprintln!("FunctionUnavailable Error in swap_buffers")
            }
        },
    });
}

#[no_mangle]
pub fn glutin_windowed_context_get_proc_address(
    _ptr_window: *mut ValueBox<WindowedContext<PossiblyCurrent>>,
    _ptr_symbol: *mut BoxerString,
) -> *const c_void {
    _ptr_window.with_not_null_return(std::ptr::null(), |window| {
        _ptr_symbol.with(|symbol| window.get_proc_address(symbol.to_string().as_str()))
    })
}

#[no_mangle]
pub fn glutin_windowed_context_get_api(
    _ptr_context: *mut ValueBox<WindowedContext<PossiblyCurrent>>,
) -> ContextApi {
    _ptr_context.with_not_null_return(ContextApi::Unknown, |context| context.get_api().into())
}

#[no_mangle]
pub fn glutin_windowed_context_request_redraw(
    _ptr_window: *mut ValueBox<WindowedContext<PossiblyCurrent>>,
) {
    _ptr_window.with_not_null(|window| window.window().request_redraw());
}

#[no_mangle]
pub fn glutin_windowed_context_is_current(
    _ptr_window: *mut ValueBox<WindowedContext<PossiblyCurrent>>,
) -> bool {
    _ptr_window.with_not_null_return(false, |window| window.is_current())
}

#[no_mangle]
pub fn glutin_windowed_context_get_scale_factor(
    _ptr_window: *mut ValueBox<WindowedContext<PossiblyCurrent>>,
) -> f64 {
    _ptr_window.with_not_null_return(1.0, |window| window.window().scale_factor())
}

#[no_mangle]
pub fn glutin_windowed_context_get_pixel_format(
    _ptr_window: *mut ValueBox<WindowedContext<PossiblyCurrent>>,
) -> *mut ValueBox<PixelFormat> {
    _ptr_window.with_not_null_return(std::ptr::null_mut(), |window| {
        ValueBox::new(window.get_pixel_format()).into_raw()
    })
}

#[no_mangle]
pub fn glutin_windowed_context_get_inner_size(
    _ptr_window: *mut ValueBox<WindowedContext<PossiblyCurrent>>,
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
    _ptr_window: *mut ValueBox<WindowedContext<PossiblyCurrent>>,
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
    _ptr_window: *mut ValueBox<WindowedContext<PossiblyCurrent>>,
    _ptr_position: *mut ValueBox<BoxerPointI32>,
) {
    _ptr_window.with_not_null(|window| {
        _ptr_position.with_not_null(|position| match window.window().outer_position() {
            Ok(physical_position) => {
                position.x = physical_position.x;
                position.y = physical_position.y;
            }
            Err(err) => {
                eprintln!(
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
    _ptr_window: *mut ValueBox<WindowedContext<PossiblyCurrent>>,
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
    _ptr_window: *mut ValueBox<WindowedContext<PossiblyCurrent>>,
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
    _ptr_window: *mut ValueBox<WindowedContext<PossiblyCurrent>>,
    _ptr_boxer_string: *mut BoxerString,
) {
    _ptr_window.with_not_null(|window| {
        CBox::with_raw(_ptr_boxer_string, |string| {
            window.window().set_title(string.to_string().as_ref())
        })
    });
}

#[no_mangle]
pub fn glutin_windowed_context_resize(
    _ptr_window: *mut ValueBox<WindowedContext<PossiblyCurrent>>,
    _width: u32,
    _height: u32,
) {
    _ptr_window.with_not_null(|window| window.resize(PhysicalSize::new(_width, _height)));
}

#[no_mangle]
pub fn glutin_windowed_context_set_cursor_icon(
    _ptr_window: *mut ValueBox<WindowedContext<PossiblyCurrent>>,
    cursor_icon: GlutinCursorIcon,
) {
    _ptr_window.with_not_null(|window| window.window().set_cursor_icon(cursor_icon.into()));
}

#[no_mangle]
pub fn glutin_destroy_windowed_context(_ptr: *mut ValueBox<WindowedContext<PossiblyCurrent>>) {
    _ptr.drop();
}
