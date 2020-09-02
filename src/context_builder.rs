use boxer::{ValueBox, ValueBoxPointer, ValueBoxPointerReference};

use glutin::event_loop::EventLoop;
use glutin::window::WindowBuilder;
use glutin::{
    ContextBuilder, CreationError, GlProfile, GlRequest, NotCurrent, PixelFormatRequirements,
    PossiblyCurrent, WindowedContext,
};
use headless_context::GlutinHeadlessContext;
use windowed_context::GlutinWindowedContext;

#[derive(Debug)]
pub enum GlutinContextBuilder<'a> {
    NotCurrent(ContextBuilder<'a, NotCurrent>),
    PossiblyCurrent(ContextBuilder<'a, PossiblyCurrent>),
}

impl<'a> GlutinContextBuilder<'a> {
    pub fn build_windowed<TE>(
        self,
        wb: WindowBuilder,
        el: &EventLoop<TE>,
    ) -> Result<GlutinWindowedContext, CreationError> {
        (match self {
            GlutinContextBuilder::NotCurrent(builder) => builder.build_windowed(wb, el),
            GlutinContextBuilder::PossiblyCurrent(builder) => builder.build_windowed(wb, el),
        })
        .map(|context| GlutinWindowedContext::NotCurrent(context))
    }

    pub fn with_shared_lists_headless(self, context: &'a GlutinHeadlessContext) -> Self {
        match self {
            GlutinContextBuilder::NotCurrent(builder) => match context {
                GlutinHeadlessContext::NotCurrent(context) => {
                    GlutinContextBuilder::NotCurrent(builder.with_shared_lists(context))
                }
                GlutinHeadlessContext::PossiblyCurrent(context) => {
                    GlutinContextBuilder::PossiblyCurrent(builder.with_shared_lists(context))
                }
            },
            GlutinContextBuilder::PossiblyCurrent(builder) => match context {
                GlutinHeadlessContext::NotCurrent(context) => {
                    GlutinContextBuilder::NotCurrent(builder.with_shared_lists(context))
                }
                GlutinHeadlessContext::PossiblyCurrent(context) => {
                    GlutinContextBuilder::PossiblyCurrent(builder.with_shared_lists(context))
                }
            },
        }
    }
}

macro_rules! with_builder {
    ($builder_name:ident, $expression:expr) => {
        match $builder_name {
            GlutinContextBuilder::NotCurrent($builder_name) => {
                GlutinContextBuilder::NotCurrent($expression)
            }
            GlutinContextBuilder::PossiblyCurrent($builder_name) => {
                GlutinContextBuilder::PossiblyCurrent($expression)
            }
        }
    };
}

#[no_mangle]
pub fn glutin_context_builder_default<'a>() -> *mut ValueBox<GlutinContextBuilder<'a>> {
    ValueBox::new(GlutinContextBuilder::NotCurrent(ContextBuilder::new())).into_raw()
}

#[no_mangle]
pub fn glutin_context_builder_with_shared_windowed_context(
    mut _ptr_context_builder: *mut ValueBox<ContextBuilder<NotCurrent>>,
    mut _ptr_another_context: *mut ValueBox<WindowedContext<PossiblyCurrent>>,
) {
}

#[no_mangle]
pub fn glutin_context_builder_with_shared_headless_context(
    mut _ptr_context_builder: *mut ValueBox<GlutinContextBuilder>,
    mut _ptr_another_context: *mut ValueBox<GlutinHeadlessContext>,
) {
    _ptr_context_builder.with_box(
        || (),
        |context_box, _| {
            _ptr_another_context.with_box(
                || (),
                |another_box, _| {
                    let new_builder = match context_box.take_value() {
                        None => None,
                        Some(context_builder) => match another_box.as_ref_mut() {
                            None => None,
                            Some(another_context) => {
                                Some(context_builder.with_shared_lists_headless(another_context))
                            }
                        },
                    };

                    match new_builder {
                        None => {}
                        Some(builder) => {
                            let new_builder_cleared: GlutinContextBuilder =
                                unsafe { std::mem::transmute(builder) };
                            context_box.set_value(new_builder_cleared)
                        }
                    }
                },
            )
        },
    );
    //
    //
    //
    //
    //    let mut context_builder_box = ManuallyDrop::new(unsafe { from_raw(_ptr_context_builder) });
    //    let context_builder = *unsafe { from_raw(context_builder_box.boxed()) };
    //
    //    let another_context_value_box = ManuallyDrop::new(unsafe { from_raw(_ptr_another_context) });
    //    let another_context = ManuallyDrop::new(unsafe { from_raw(another_context_value_box.boxed()) });
    //
    //    let new_builder = context_builder.with_shared_lists_headless(another_context.as_ref());
    //
    //    let new_builder_ptr = Box::into_raw(Box::new(new_builder));
    //    // transmute clears lifetime :)
    //    let new_builder_ptr: *mut GlutinContextBuilder =
    //        unsafe { std::mem::transmute(new_builder_ptr) };
    //
    //    unsafe { context_builder_box.mutate_ptr(new_builder_ptr) };
}

#[no_mangle]
pub fn glutin_context_builder_with_gl_then_gles(
    mut _ptr_context_builder: *mut ValueBox<GlutinContextBuilder>,
    gl_major: u8,
    gl_minor: u8,
    gles_major: u8,
    gles_minor: u8,
) {
    _ptr_context_builder.with_not_null_value_mutate(|builder| {
        with_builder!(
            builder,
            builder.with_gl(GlRequest::GlThenGles {
                opengl_version: (gl_major, gl_minor),
                opengles_version: (gles_major, gles_minor),
            })
        )
    });
}

#[no_mangle]
pub fn glutin_context_builder_with_gl_latest(
    mut _ptr_context_builder: *mut ValueBox<GlutinContextBuilder>,
) {
    _ptr_context_builder.with_not_null_value_mutate(|builder| {
        with_builder!(builder, builder.with_gl(GlRequest::Latest))
    })
}

#[no_mangle]
pub fn glutin_context_builder_with_gl_profile_core(
    mut _ptr_context_builder: *mut ValueBox<GlutinContextBuilder>,
) {
    _ptr_context_builder.with_not_null_value_mutate(|builder| {
        with_builder!(builder, builder.with_gl_profile(GlProfile::Core))
    })
}

#[no_mangle]
pub fn glutin_context_builder_with_gl_profile_compatibility(
    mut _ptr_context_builder: *mut ValueBox<GlutinContextBuilder>,
) {
    _ptr_context_builder.with_not_null_value_mutate(|builder| {
        with_builder!(builder, builder.with_gl_profile(GlProfile::Compatibility))
    })
}

#[no_mangle]
pub fn glutin_context_builder_with_multisampling(
    mut _ptr_context_builder: *mut ValueBox<GlutinContextBuilder>,
    samples: u16,
) {
    _ptr_context_builder.with_not_null_value_mutate(|builder| {
        with_builder!(builder, builder.with_multisampling(samples))
    })
}

#[no_mangle]
pub fn glutin_context_builder_with_depth_buffer(
    mut _ptr_context_builder: *mut ValueBox<GlutinContextBuilder>,
    bits: u8,
) {
    _ptr_context_builder.with_not_null_value_mutate(|builder| {
        with_builder!(builder, builder.with_depth_buffer(bits))
    })
}

#[no_mangle]
pub fn glutin_context_builder_with_stencil_buffer(
    mut _ptr_context_builder: *mut ValueBox<GlutinContextBuilder>,
    bits: u8,
) {
    _ptr_context_builder.with_not_null_value_mutate(|builder| {
        with_builder!(builder, builder.with_stencil_buffer(bits))
    })
}

#[no_mangle]
pub fn glutin_context_builder_with_pixel_format(
    mut _ptr_context_builder: *mut ValueBox<GlutinContextBuilder>,
    color_bits: u8,
    alpha_bits: u8,
) {
    _ptr_context_builder.with_not_null_value_mutate(|builder| {
        with_builder!(builder, builder.with_pixel_format(color_bits, alpha_bits))
    })
}

#[no_mangle]
pub fn glutin_context_builder_with_vsync(
    mut _ptr_context_builder: *mut ValueBox<GlutinContextBuilder>,
    vsync: bool,
) {
    _ptr_context_builder
        .with_not_null_value_mutate(|builder| with_builder!(builder, builder.with_vsync(vsync)))
}

#[no_mangle]
pub fn glutin_context_builder_with_srgb(
    mut _ptr_context_builder: *mut ValueBox<GlutinContextBuilder>,
    srgb_enabled: bool,
) {
    _ptr_context_builder.with_not_null_value_mutate(|builder| {
        with_builder!(builder, builder.with_srgb(srgb_enabled))
    })
}

#[no_mangle]
pub fn glutin_context_builder_with_double_buffer(
    mut _ptr_context_builder: *mut ValueBox<GlutinContextBuilder>,
    double_buffer_enabled: bool,
) {
    _ptr_context_builder.with_not_null_value_mutate(|builder| {
        with_builder!(
            builder,
            builder.with_double_buffer(Some(double_buffer_enabled))
        )
    })
}

#[no_mangle]
pub fn glutin_context_builder_with_hardware_acceleration(
    mut _ptr_context_builder: *mut ValueBox<GlutinContextBuilder>,
    hardware_acceleration_enabled: bool,
) {
    _ptr_context_builder.with_not_null_value_mutate(|builder| {
        with_builder!(
            builder,
            builder.with_hardware_acceleration(Some(hardware_acceleration_enabled))
        )
    })
}

#[no_mangle]
pub fn glutin_context_builder_with_any_hardware_acceleration(
    mut _ptr_context_builder: *mut ValueBox<GlutinContextBuilder>,
) {
    _ptr_context_builder.with_not_null_value_mutate(|builder| {
        with_builder!(builder, builder.with_hardware_acceleration(None))
    })
}

#[no_mangle]
pub fn glutin_context_builder_get_pixel_format_requirements(
    _ptr_context_builder: *mut ValueBox<GlutinContextBuilder>,
) -> *mut ValueBox<PixelFormatRequirements> {
    _ptr_context_builder.with_not_null_return(std::ptr::null_mut(), |builder| {
        ValueBox::new(match builder {
            GlutinContextBuilder::NotCurrent(builder) => builder.pf_reqs.clone(),
            GlutinContextBuilder::PossiblyCurrent(builder) => builder.pf_reqs.clone(),
        })
        .into_raw()
    })
}

#[no_mangle]
pub fn glutin_context_builder_print_it(_ptr: *mut ValueBox<GlutinContextBuilder>) {
    _ptr.with_not_null(|builder| println!("{:?}", builder))
}

#[no_mangle]
pub fn glutin_destroy_context_builder(_ptr: &mut *mut ValueBox<GlutinContextBuilder>) {
    _ptr.drop();
}
