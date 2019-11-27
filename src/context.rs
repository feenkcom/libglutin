use glutin::{ContextBuilder, NotCurrent, PossiblyCurrent, WindowedContext, Context};
use glutin::dpi::PhysicalSize;
use glutin::event_loop::EventLoop;
use glutin::window::WindowBuilder;
use boxer::boxes::{ValueBox, ValueBoxPointer};
use boxer::CBox;

#[no_mangle]
pub fn glutin_create_windowed_context(
        _ptr_events_loop: *mut EventLoop<()>,
        _ptr_window_builder: *mut WindowBuilder,
        _ptr_context_builder: *mut ContextBuilder<NotCurrent>) -> *mut ValueBox<WindowedContext<PossiblyCurrent>> {

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

        if cfg!(debug_assertions) {
            println!("[Glutin] OpenGL Context: {:?}", context_builder);
            println!("[Glutin] Primary monitor: {:?}", events_loop.primary_monitor());
            println!("[Glutin] Window attributes: {:?}", window_builder);
        }

        match context_builder.build_windowed(window_builder, events_loop) {
            Ok(windowed_context) => {
                match unsafe { windowed_context.make_current() } {
                    Ok(windowed_context) => { ValueBox::new(windowed_context).into_raw() },
                    Err(err) => {
                        if cfg!(debug_assertions) {
                            println!("[Glutin] Could not create context {:?}", err);
                        }
                        std::ptr::null_mut() }
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
}

#[no_mangle]
pub fn glutin_create_headless_context(
        _ptr_events_loop: *mut EventLoop<()>,
        _ptr_context_builder: *mut ContextBuilder<NotCurrent>) -> *mut ValueBox<Context<NotCurrent>> {

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

        if cfg!(debug_assertions) {
            println!("[Glutin] OpenGL Headless Context: {:?}", context_builder);
            println!("[Glutin] Primary monitor: {:?}", events_loop.primary_monitor());
        }

        match context_builder.build_headless(events_loop, PhysicalSize::new(1., 1.)) {
            Ok(context) => ValueBox::new(context).into_raw(),
            Err(err) => {
                if cfg!(debug_assertions) {
                    eprintln!("[Glutin] Could not create headless context {:?}", err);
                }
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
pub fn glutin_destroy_windowed_context(_ptr: *mut ValueBox<WindowedContext<PossiblyCurrent>>) {
    _ptr.drop();
}