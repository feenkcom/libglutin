use boxer::boxes::{ValueBox, ValueBoxPointer};
use boxer::CBox;
use glutin::dpi::PhysicalSize;
use glutin::event_loop::EventLoop;
use glutin::window::WindowBuilder;
use glutin::{Context, ContextBuilder, NotCurrent, PossiblyCurrent, WindowedContext, ContextError};

#[no_mangle]
pub fn glutin_create_windowed_context(
    _ptr_events_loop: *mut ValueBox<EventLoop<()>>,
    _ptr_window_builder: *mut WindowBuilder,
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
            // we consume both window_builder and context_builder
            let window_builder = unsafe { *CBox::from_raw(_ptr_window_builder) };

            if cfg!(debug_assertions) {
                println!("[Glutin] OpenGL Context: {:?}", context_builder);
                println!(
                    "[Glutin] Primary monitor: {:?}",
                    event_loop.primary_monitor()
                );
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

            match context_builder.build_headless(event_loop, PhysicalSize::new(1., 1.)) {
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

#[no_mangle]
pub fn glutin_try_headless_context(
    _ptr_events_loop: *mut ValueBox<EventLoop<()>>,
    _ptr_context_builder: *mut ValueBox<ContextBuilder<NotCurrent>>,
) -> bool {
    let builder_copy = _ptr_context_builder.with_value(|context_builder| ValueBox::new(context_builder.clone()).into_raw());
    let context = glutin_create_headless_context(_ptr_events_loop, builder_copy);
    return if context.is_null() {
        false
    } else {
        context.drop();
        true
    };
}

#[no_mangle]
pub fn glutin_context_make_current(mut _ptr: *mut ValueBox<Context<PossiblyCurrent>>) {
    _ptr.with_value_and_box_consumed(|window, value_box| {
        let context: Context<PossiblyCurrent>;

        match unsafe { window.make_current() } {
            Ok(new_context) => { context = new_context },
            Err((old_context, err)) => {
                context = old_context;
                match err {
                    ContextError::OsError(string) => { eprintln!("OS Error in make_current: {}", string) },
                    ContextError::IoError(error)=> { eprintln!("IO Error in make_current: {:?}", error) },
                    ContextError::ContextLost => { eprintln!("ContextLost Error in make_current") }
                    ContextError::FunctionUnavailable => { eprintln!("FunctionUnavailable Error in make_current") }
                }
            }
        }
        unsafe { value_box.mutate(context); };
    });
}

#[no_mangle]
pub fn glutin_context_is_current(_ptr_context: *mut ValueBox<Context<PossiblyCurrent>>) -> bool {
    _ptr_context.with_not_null_return(false, |context | context.is_current())
}

#[no_mangle]
pub fn glutin_destroy_context(_ptr: *mut ValueBox<Context<PossiblyCurrent>>) {
    _ptr.drop();
}

#[no_mangle]
pub fn glutin_destroy_windowed_context(_ptr: *mut ValueBox<WindowedContext<PossiblyCurrent>>) {
    _ptr.drop();
}
