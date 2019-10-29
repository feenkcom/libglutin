use super::*;

///////////////////////////////////////////////////////////////////////////////////////
////////////////////////////// W I N D O W   C R E A T I O N //////////////////////////
///////////////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub fn glutin_create_windowed_context(
        _ptr_events_loop: *mut EventLoop<()>,
        _ptr_window_builder: *mut WindowBuilder,
        _ptr_context_builder: *mut ContextBuilder<NotCurrent>) -> *mut WindowedContext<PossiblyCurrent> {

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
        // we consume both window_builder and context_builder
        let window_builder = unsafe { *CBox::from_raw(_ptr_window_builder) };
        let context_builder= unsafe { *CBox::from_raw(_ptr_context_builder) };

        println!("[Glutin] OpenGL Context: {:?}", context_builder);
        println!("[Glutin] Primary monitor: {:?}", events_loop.primary_monitor());
        println!("[Glutin] Window attributes: {:?}", window_builder);

        match context_builder.build_windowed(window_builder, events_loop) {
            Ok(windowed_context) => {
                match unsafe { windowed_context.make_current() } {
                    Ok(windowed_context) => { CBox::into_raw(windowed_context)},
                    Err(err) => { println!("[Glutin] Could not create context {:?}", err);
                std::ptr::null_mut() },
                }
            },
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
        // we consume context_builder here
        let context_builder= unsafe { *CBox::from_raw(_ptr_context_builder) };

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

    // the window will be deleted to we let rust take control back
    let window = unsafe { CBox::from_raw(_ptr) };

    match unsafe { window.make_not_current() } {
        Ok(_windowed_context) => { std::mem::drop(_windowed_context) },
        Err((_windowed_context, err)) => {
            match err {
                ContextError::OsError(string) => { eprintln!("OS Error in glutin_destroy_windowed_context: {}", string) },
                ContextError::IoError(error) => { eprintln!("IO Error in glutin_destroy_windowed_context: {:?}", error) },
                ContextError::ContextLost => { eprintln!("ContextLost Error in glutin_destroy_windowed_context") }
            }
            std::mem::drop(_windowed_context)
        }
    }
}

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
pub fn glutin_windowed_context_swap_buffers(_ptr_window: *mut WindowedContext<PossiblyCurrent>) {
    CBox::with_optional_raw(_ptr_window, |option| match option {
        None => {},
        Some(window) => {
            match window.swap_buffers() {
                Ok(_) => {}
                Err(err) => {
                    match err {
                        ContextError::OsError(string) => { eprintln!("OS Error in swap_buffers: {}", string) }
                        ContextError::IoError(error) => { eprintln!("IO Error in swap_buffers: {:?}", error) }
                        ContextError::ContextLost => { eprintln!("ContextLost Error in swap_buffers") }
                    }
                }
            }
        },
    });
}

#[no_mangle]
pub fn glutin_windowed_context_request_redraw(_ptr_window: *mut WindowedContext<PossiblyCurrent>) {
    CBox::with_optional_raw(_ptr_window, |option| match option {
        None => {},
        Some(window) => { window.window().request_redraw() },
    });
}

#[no_mangle]
pub fn glutin_windowed_context_is_current(_ptr_window: *mut WindowedContext<PossiblyCurrent>) -> bool {
    CBox::with_optional_raw(_ptr_window, |option| match option {
        None => { false },
        Some(window) => { window.is_current() },
    })
}

#[no_mangle]
pub fn glutin_windowed_context_get_framebuffer_size(_ptr_window: *mut WindowedContext<PossiblyCurrent>, _ptr_size: *mut BoxerSizeF64) {
    CBox::with_optional_raw(_ptr_size, |size_option| {
        match size_option {
            None => { eprintln!("[glutin_windowed_context_get_framebuffer_size] size pointer is null"); },
            Some(size) => {
                CBox::with_optional_raw(_ptr_window, |window_option| {
                    match window_option {
                        None => {
                            eprintln!("[glutin_windowed_context_get_framebuffer_size] window pointer is null");
                            size.be_zero() },
                        Some(window) => {
                            let window_size: PhysicalSize = window.window()
                                .inner_size()
                                .to_physical(window.window().hidpi_factor());

                            size.width = window_size.width;
                            size.height = window_size.height
                        }
                    }
                });
            },
        }
    });
}

#[no_mangle]
pub fn glutin_windowed_context_get_inner_size(_ptr_window: *mut WindowedContext<PossiblyCurrent>, _ptr_size: *mut BoxerSizeF64) {
    CBox::with_optional_raw(_ptr_size, |size_option| {
        match size_option {
            None => { eprintln!("[glutin_windowed_context_get_inner_size] size pointer is null"); },
            Some(size) => {
                CBox::with_optional_raw(_ptr_window, |window_option| {
                    match window_option {
                        None => {
                            eprintln!("[glutin_windowed_context_get_inner_size] window pointer is null");
                            size.be_zero() },
                        Some(window) => {
                            let window_size: LogicalSize = window.window().inner_size();
                            size.width = window_size.width;
                            size.height = window_size.height;
                        }
                    }
                });
            },
        }
    });
}

#[no_mangle]
pub fn glutin_windowed_context_get_position(_ptr_window: *mut WindowedContext<PossiblyCurrent>, _ptr_position: *mut BoxerPointF64) {
    CBox::with_optional_raw(_ptr_position, |position_option| {
        match position_option {
            None => { eprintln!("[glutin_windowed_context_get_position] position pointer is null"); },
            Some(position) => {
                CBox::with_optional_raw(_ptr_window, |window_option| {
                    match window_option {
                        None => {
                            eprintln!("[glutin_windowed_context_get_inner_size] window pointer is null");
                            position.be_zero() },
                        Some(window) => {
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
                        }
                    }
                });
            },
        }
    });
}

#[no_mangle]
pub fn glutin_windowed_context_get_id(_ptr_window: *mut WindowedContext<PossiblyCurrent>, _ptr_number: *mut BoxerUint128) {
    CBox::with_optional_raw(_ptr_number, |number_option| {
        match number_option {
            None => { eprintln!("[glutin_windowed_context_get_id] number pointer is null"); },
            Some(number) => {
                CBox::with_optional_raw(_ptr_window, |window_option| {
                    match window_option {
                        None => {
                            eprintln!("[glutin_windowed_context_get_id] window pointer is null");
                            number.be_zero() },
                        Some(window) => {
                            let id: BoxerUint128 = glutin_convert_window_id(window.window().id());
                            number.low = id.low;
                            number.high = id.high
                        }
                    }
                });
            },
        }
    });
}

#[no_mangle]
pub fn glutin_windowed_context_set_position(_ptr_window: *mut WindowedContext<PossiblyCurrent>, x: f64, y: f64) {
    CBox::with_optional_raw(_ptr_window, |window_option| {
        match window_option {
            None => {},
            Some(window) => { window.window().set_outer_position(LogicalPosition::new(x, y)); }
        }
    });
}

#[no_mangle]
pub fn glutin_windowed_context_set_title(_ptr_window: *mut WindowedContext<PossiblyCurrent>, _ptr_boxer_string: *mut BoxerString) {
    CBox::with_optional_raw(_ptr_window, |window_option| {
        match window_option {
            None => {},
            Some(window) => {
                CBox::with_raw(_ptr_boxer_string, |string| {
                    window.window().set_title(string.to_string().as_ref())
                })
            }
        }
    });
}

#[no_mangle]
pub fn glutin_windowed_context_set_inner_size(_ptr_window: *mut WindowedContext<PossiblyCurrent>, _width: f64, _height: f64) {
    CBox::with_optional_raw(_ptr_window, |window_option| {
        match window_option {
            None => {},
            Some(window) => { window.window().set_inner_size(LogicalSize::new(_width, _height)); }
        }
    });
}

#[no_mangle]
pub fn glutin_windowed_context_resize_logical(_ptr_window: *mut WindowedContext<PossiblyCurrent>, _width: f64, _height: f64) {
    CBox::with_optional_raw(_ptr_window, |window_option| {
        match window_option {
            None => {},
            Some(window) => {
                let dpi_factor = window.window().hidpi_factor();
                window.resize(LogicalSize::new(_width, _height).to_physical(dpi_factor));
            }
        }
    });
}

#[no_mangle]
pub fn glutin_windowed_context_resize_physical(_ptr_window: *mut WindowedContext<PossiblyCurrent>, _width: f64, _height: f64) {
    CBox::with_optional_raw(_ptr_window, |window_option| {
        match window_option {
            None => {},
            Some(window) => { window.resize(PhysicalSize::new(_width, _height)); }
        }
    });
}

#[no_mangle]
pub fn glutin_windowed_context_set_cursor_icon(_ptr_window: *mut WindowedContext<PossiblyCurrent>, cursor_icon: GlutinCursorIcon) {
    CBox::with_optional_raw(_ptr_window, |window_option| {
        match window_option {
            None => {},
            Some(window) => { window.window().set_cursor_icon(cursor_icon.into()); }
        }
    });
}