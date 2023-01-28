use glutin::event_loop::EventLoopWindowTarget;
use glutin::window::WindowBuilder;
use glutin::{
    ContextBuilder, CreationError, GlProfile, GlRequest, NotCurrent, PixelFormatRequirements,
    PossiblyCurrent,
};
use std::mem::transmute;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

use crate::headless_context::GlutinHeadlessContext;
use crate::windowed_context::GlutinWindowedContext;

#[derive(Debug)]
pub enum GlutinContextBuilder {
    NotCurrent(ContextBuilder<'static, NotCurrent>),
    PossiblyCurrent(ContextBuilder<'static, PossiblyCurrent>),
}

impl GlutinContextBuilder {
    pub fn build_windowed<TE>(
        self,
        wb: WindowBuilder,
        el: &EventLoopWindowTarget<TE>,
    ) -> Result<GlutinWindowedContext, CreationError> {
        (match self {
            GlutinContextBuilder::NotCurrent(builder) => builder.build_windowed(wb, el),
            GlutinContextBuilder::PossiblyCurrent(builder) => builder.build_windowed(wb, el),
        })
        .map(|context| GlutinWindowedContext::NotCurrent(context))
    }

    pub fn with_shared_lists_headless(self, context: &GlutinHeadlessContext) -> Self {
        match self {
            GlutinContextBuilder::NotCurrent(builder) => match context {
                GlutinHeadlessContext::NotCurrent(context) => {
                    GlutinContextBuilder::NotCurrent(unsafe {
                        transmute(builder.with_shared_lists(context))
                    })
                }
                GlutinHeadlessContext::PossiblyCurrent(context) => {
                    GlutinContextBuilder::PossiblyCurrent(unsafe {
                        transmute(builder.with_shared_lists(context))
                    })
                }
            },
            GlutinContextBuilder::PossiblyCurrent(builder) => match context {
                GlutinHeadlessContext::NotCurrent(context) => {
                    GlutinContextBuilder::NotCurrent(unsafe {
                        transmute(builder.with_shared_lists(context))
                    })
                }
                GlutinHeadlessContext::PossiblyCurrent(context) => {
                    GlutinContextBuilder::PossiblyCurrent(unsafe {
                        transmute(builder.with_shared_lists(context))
                    })
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
pub fn glutin_context_builder_default() -> *mut ValueBox<GlutinContextBuilder> {
    ValueBox::new(GlutinContextBuilder::NotCurrent(ContextBuilder::new())).into_raw()
}

#[no_mangle]
pub fn glutin_context_builder_with_shared_headless_context(
    context_builder: *mut ValueBox<GlutinContextBuilder>,
    another_context: *mut ValueBox<GlutinHeadlessContext>,
) {
    another_context
        .with_ref(|another_context| {
            context_builder.replace_value(|context_builder| {
                context_builder.with_shared_lists_headless(another_context)
            })
        })
        .log();
}

#[no_mangle]
pub fn glutin_context_builder_with_gl_then_gles(
    context_builder: *mut ValueBox<GlutinContextBuilder>,
    gl_major: u8,
    gl_minor: u8,
    gles_major: u8,
    gles_minor: u8,
) {
    context_builder
        .replace_value(|builder| {
            with_builder!(
                builder,
                builder.with_gl(GlRequest::GlThenGles {
                    opengl_version: (gl_major, gl_minor),
                    opengles_version: (gles_major, gles_minor),
                })
            )
        })
        .log()
}

#[no_mangle]
pub fn glutin_context_builder_with_gl_latest(context_builder: *mut ValueBox<GlutinContextBuilder>) {
    context_builder
        .replace_value(|builder| with_builder!(builder, builder.with_gl(GlRequest::Latest)))
        .log();
}

#[no_mangle]
pub fn glutin_context_builder_with_gl_profile_core(
    context_builder: *mut ValueBox<GlutinContextBuilder>,
) {
    context_builder
        .replace_value(|builder| with_builder!(builder, builder.with_gl_profile(GlProfile::Core)))
        .log();
}

#[no_mangle]
pub fn glutin_context_builder_with_gl_profile_compatibility(
    context_builder: *mut ValueBox<GlutinContextBuilder>,
) {
    context_builder
        .replace_value(|builder| {
            with_builder!(builder, builder.with_gl_profile(GlProfile::Compatibility))
        })
        .log();
}

#[no_mangle]
pub fn glutin_context_builder_with_multisampling(
    context_builder: *mut ValueBox<GlutinContextBuilder>,
    samples: u16,
) {
    context_builder
        .replace_value(|builder| with_builder!(builder, builder.with_multisampling(samples)))
        .log();
}

#[no_mangle]
pub fn glutin_context_builder_with_depth_buffer(
    context_builder: *mut ValueBox<GlutinContextBuilder>,
    bits: u8,
) {
    context_builder
        .replace_value(|builder| with_builder!(builder, builder.with_depth_buffer(bits)))
        .log();
}

#[no_mangle]
pub fn glutin_context_builder_with_stencil_buffer(
    context_builder: *mut ValueBox<GlutinContextBuilder>,
    bits: u8,
) {
    context_builder
        .replace_value(|builder| with_builder!(builder, builder.with_stencil_buffer(bits)))
        .log();
}

#[no_mangle]
pub fn glutin_context_builder_with_pixel_format(
    context_builder: *mut ValueBox<GlutinContextBuilder>,
    color_bits: u8,
    alpha_bits: u8,
) {
    context_builder
        .replace_value(|builder| {
            with_builder!(builder, builder.with_pixel_format(color_bits, alpha_bits))
        })
        .log();
}

#[no_mangle]
pub fn glutin_context_builder_with_vsync(
    context_builder: *mut ValueBox<GlutinContextBuilder>,
    vsync: bool,
) {
    context_builder
        .replace_value(|builder| with_builder!(builder, builder.with_vsync(vsync)))
        .log();
}

#[no_mangle]
pub fn glutin_context_builder_with_srgb(
    context_builder: *mut ValueBox<GlutinContextBuilder>,
    srgb_enabled: bool,
) {
    context_builder
        .replace_value(|builder| with_builder!(builder, builder.with_srgb(srgb_enabled)))
        .log();
}

#[no_mangle]
pub fn glutin_context_builder_with_double_buffer(
    context_builder: *mut ValueBox<GlutinContextBuilder>,
    double_buffer_enabled: bool,
) {
    context_builder
        .replace_value(|builder| {
            with_builder!(
                builder,
                builder.with_double_buffer(Some(double_buffer_enabled))
            )
        })
        .log();
}

#[no_mangle]
pub fn glutin_context_builder_with_hardware_acceleration(
    context_builder: *mut ValueBox<GlutinContextBuilder>,
    hardware_acceleration_enabled: bool,
) {
    context_builder
        .replace_value(|builder| {
            with_builder!(
                builder,
                builder.with_hardware_acceleration(Some(hardware_acceleration_enabled))
            )
        })
        .log();
}

#[no_mangle]
pub fn glutin_context_builder_with_any_hardware_acceleration(
    context_builder: *mut ValueBox<GlutinContextBuilder>,
) {
    context_builder
        .replace_value(|builder| with_builder!(builder, builder.with_hardware_acceleration(None)))
        .log();
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
pub fn glutin_destroy_context_builder(_ptr: *mut ValueBox<GlutinContextBuilder>) {
    _ptr.release();
}
