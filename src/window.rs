use boxer::boxes::{ValueBox, ValueBoxPointer};
use glutin::{PossiblyCurrent, WindowedContext, ContextError};
use glutin::dpi::{LogicalSize, LogicalPosition, PhysicalSize};
use boxer::CBox;
use boxer::size::BoxerSizeF64;
use boxer::number::BoxerUint128;
use glutin_convert_window_id;
use boxer::string::BoxerString;
use enums::GlutinCursorIcon;
use boxer::point::BoxerPointF64;


///////////////////////////////////////////////////////////////////////////////////////
///////////////////////////// W I N D O W   A C C E S S O R S /////////////////////////
///////////////////////////////////////////////////////////////////////////////////////
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

    let _ptr_windowed_context =  CBox::into_raw(context);
    _ptr_windowed_context
}

#[no_mangle]
pub fn glutin_windowed_context_swap_buffers(_ptr_window: *mut ValueBox<WindowedContext<PossiblyCurrent>>) {
    _ptr_window.with_not_null(|window|
        match window.swap_buffers() {
            Ok(_) => {}
            Err(err) => {
                match err {
                    ContextError::OsError(string) => { eprintln!("OS Error in swap_buffers: {}", string) }
                    ContextError::IoError(error) => { eprintln!("IO Error in swap_buffers: {:?}", error) }
                    ContextError::ContextLost => { eprintln!("ContextLost Error in swap_buffers") }
                }
            }
        });
}

#[no_mangle]
pub fn glutin_windowed_context_request_redraw(_ptr_window: *mut ValueBox<WindowedContext<PossiblyCurrent>>) {
    _ptr_window.with_not_null( |window| window.window().request_redraw());
}

#[no_mangle]
pub fn glutin_windowed_context_is_current(_ptr_window: *mut ValueBox<WindowedContext<PossiblyCurrent>>) -> bool {
    _ptr_window.with_not_null_return(false, |window | window.is_current())
}

#[no_mangle]
pub fn glutin_windowed_context_get_framebuffer_size(_ptr_window: *mut ValueBox<WindowedContext<PossiblyCurrent>>, _ptr_size: *mut ValueBox<BoxerSizeF64>) {
    _ptr_window.with_not_null(|window| {
        _ptr_size.with_not_null(|size| {
            let window_size: PhysicalSize = window.window()
                .inner_size()
                .to_physical(window.window().hidpi_factor());

            size.width = window_size.width;
            size.height = window_size.height;
        });
    });
}

#[no_mangle]
pub fn glutin_windowed_context_get_inner_size(_ptr_window: *mut ValueBox<WindowedContext<PossiblyCurrent>>, _ptr_size: *mut ValueBox<BoxerSizeF64>) {
    _ptr_window.with_not_null(|window| {
        _ptr_size.with_not_null(|size| {
            let window_size: LogicalSize = window.window().inner_size();
            size.width = window_size.width;
            size.height = window_size.height;
        });
    });
}

#[no_mangle]
pub fn glutin_windowed_context_get_position(_ptr_window: *mut ValueBox<WindowedContext<PossiblyCurrent>>, _ptr_position: *mut ValueBox<BoxerPointF64>) {
    _ptr_window.with_not_null(|window| {
        _ptr_position.with_not_null(|position| {
            match window.window().inner_position() {
                Ok(_logical_position) => {
                    position.x = _logical_position.x;
                    position.y = _logical_position.y;
                },
                Err(err) => {
                    eprintln!("[glutin_windowed_context_get_position] Error getting position: {:?}", err);
                    position.be_zero()
                }
            }
        })
    });
}

#[no_mangle]
pub fn glutin_windowed_context_get_id(_ptr_window: *mut ValueBox<WindowedContext<PossiblyCurrent>>, _ptr_number: *mut ValueBox<BoxerUint128>) {
    _ptr_window.with_not_null(|window| {
        _ptr_number.with_not_null(|number| {
            let id: BoxerUint128 = glutin_convert_window_id(window.window().id());
            number.low = id.low;
            number.high = id.high
        });
    });
}

#[no_mangle]
pub fn glutin_windowed_context_set_position(_ptr_window: *mut ValueBox<WindowedContext<PossiblyCurrent>>, x: f64, y: f64) {
    _ptr_window.with_not_null(|window| window.window().set_outer_position(LogicalPosition::new(x, y)));
}

#[no_mangle]
pub fn glutin_windowed_context_set_title(_ptr_window: *mut ValueBox<WindowedContext<PossiblyCurrent>>, _ptr_boxer_string: *mut BoxerString) {
    _ptr_window.with_not_null(|window| {
        CBox::with_raw(_ptr_boxer_string, |string| {
            window.window().set_title(string.to_string().as_ref())
        })
    });
}

#[no_mangle]
pub fn glutin_windowed_context_set_inner_size(_ptr_window: *mut ValueBox<WindowedContext<PossiblyCurrent>>, _width: f64, _height: f64) {
    _ptr_window.with_not_null(|window| window.window().set_inner_size(LogicalSize::new(_width, _height)));
}

#[no_mangle]
pub fn glutin_windowed_context_resize_logical(_ptr_window: *mut ValueBox<WindowedContext<PossiblyCurrent>>, _width: f64, _height: f64) {
    _ptr_window.with_not_null(|window| {
        window.resize(LogicalSize::new(_width, _height).to_physical(window.window().hidpi_factor()))
    });
}

#[no_mangle]
pub fn glutin_windowed_context_resize_physical(_ptr_window: *mut ValueBox<WindowedContext<PossiblyCurrent>>, _width: f64, _height: f64) {
    _ptr_window.with_not_null(|window| window.resize(PhysicalSize::new(_width, _height)));
}

#[no_mangle]
pub fn glutin_windowed_context_set_cursor_icon(_ptr_window: *mut ValueBox<WindowedContext<PossiblyCurrent>>, cursor_icon: GlutinCursorIcon) {
    _ptr_window.with_not_null(|window| window.window().set_cursor_icon(cursor_icon.into()));
}