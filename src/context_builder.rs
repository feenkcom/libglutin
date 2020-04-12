use boxer::boxes::{ValueBox, ValueBoxPointer};
use glutin::{
    ContextBuilder, GlProfile, GlRequest, NotCurrent, PixelFormatRequirements, PossiblyCurrent,
    Robustness,
};

#[no_mangle]
pub fn glutin_context_builder_default<'a>() -> *mut ValueBox<ContextBuilder<'a, NotCurrent>> {
    ValueBox::new(
        ContextBuilder::new().with_gl_robustness(Robustness::TryRobustNoResetNotification),
    )
    .into_raw()
}

#[no_mangle]
pub fn glutin_context_builder_with_gl_then_gles(
    mut _ptr_context_builder: *mut ValueBox<ContextBuilder<NotCurrent>>,
    gl_major: u8,
    gl_minor: u8,
    gles_major: u8,
    gles_minor: u8,
) {
    _ptr_context_builder.with_not_null_value_mutate_consumed(|builder| {
        builder.with_gl(GlRequest::GlThenGles {
            /// The version to use for OpenGL.
            opengl_version: (gl_major, gl_minor),
            /// The version to use for OpenGL ES.
            opengles_version: (gles_major, gles_minor),
        })
    });
}

#[no_mangle]
pub fn glutin_context_builder_with_gl_latest(
    mut _ptr_context_builder: *mut ValueBox<ContextBuilder<NotCurrent>>,
) {
    _ptr_context_builder
        .with_not_null_value_mutate_consumed(|builder| builder.with_gl(GlRequest::Latest))
}

#[no_mangle]
pub fn glutin_context_builder_with_gl_profile_core(
    mut _ptr_context_builder: *mut ValueBox<ContextBuilder<NotCurrent>>,
) {
    _ptr_context_builder
        .with_not_null_value_mutate_consumed(|builder| builder.with_gl_profile(GlProfile::Core))
}

#[no_mangle]
pub fn glutin_context_builder_with_gl_profile_compatibility(
    mut _ptr_context_builder: *mut ValueBox<ContextBuilder<NotCurrent>>,
) {
    _ptr_context_builder
        .with_not_null_value_mutate_consumed(|builder| builder.with_gl_profile(GlProfile::Compatibility))
}

#[no_mangle]
pub fn glutin_context_builder_with_multisampling(
    mut _ptr_context_builder: *mut ValueBox<ContextBuilder<NotCurrent>>,
    samples: u16,
) {
    _ptr_context_builder
        .with_not_null_value_mutate_consumed(|builder| builder.with_multisampling(samples))
}

#[no_mangle]
pub fn glutin_context_builder_with_depth_buffer(
    mut _ptr_context_builder: *mut ValueBox<ContextBuilder<NotCurrent>>,
    bits: u8,
) {
    _ptr_context_builder
        .with_not_null_value_mutate_consumed(|builder| builder.with_depth_buffer(bits))
}

#[no_mangle]
pub fn glutin_context_builder_with_stencil_buffer(
    mut _ptr_context_builder: *mut ValueBox<ContextBuilder<NotCurrent>>,
    bits: u8,
) {
    _ptr_context_builder
        .with_not_null_value_mutate_consumed(|builder| builder.with_stencil_buffer(bits))
}

#[no_mangle]
pub fn glutin_context_builder_with_pixel_format(
    mut _ptr_context_builder: *mut ValueBox<ContextBuilder<NotCurrent>>,
    color_bits: u8,
    alpha_bits: u8,
) {
    _ptr_context_builder.with_not_null_value_mutate_consumed(|builder| {
        builder.with_pixel_format(color_bits, alpha_bits)
    })
}

#[no_mangle]
pub fn glutin_context_builder_with_vsync(
    mut _ptr_context_builder: *mut ValueBox<ContextBuilder<NotCurrent>>,
    vsync: bool,
) {
    _ptr_context_builder.with_not_null_value_mutate_consumed(|builder| builder.with_vsync(vsync))
}

#[no_mangle]
pub fn glutin_context_builder_with_srgb(
    mut _ptr_context_builder: *mut ValueBox<ContextBuilder<NotCurrent>>,
    srgb_enabled: bool,
) {
    _ptr_context_builder
        .with_not_null_value_mutate_consumed(|builder| builder.with_srgb(srgb_enabled))
}

#[no_mangle]
pub fn glutin_context_builder_with_double_buffer(
    mut _ptr_context_builder: *mut ValueBox<ContextBuilder<NotCurrent>>,
    double_buffer_enabled: bool,
) {
    _ptr_context_builder.with_not_null_value_mutate_consumed(|builder| {
        builder.with_double_buffer(Some(double_buffer_enabled))
    })
}

#[no_mangle]
pub fn glutin_context_builder_with_hardware_acceleration(
    mut _ptr_context_builder: *mut ValueBox<ContextBuilder<NotCurrent>>,
    hardware_acceleration_enabled: bool,
) {
    _ptr_context_builder.with_not_null_value_mutate_consumed(|builder| {
        builder.with_hardware_acceleration(Some(hardware_acceleration_enabled))
    })
}

#[no_mangle]
pub fn glutin_context_builder_with_any_hardware_acceleration(
    mut _ptr_context_builder: *mut ValueBox<ContextBuilder<NotCurrent>>,
) {
    _ptr_context_builder.with_not_null_value_mutate_consumed(|builder| {
        builder.with_hardware_acceleration(None)
    })
}

#[no_mangle]
pub fn glutin_context_builder_get_pixel_format_requirements(
    _ptr_context_builder: *mut ValueBox<ContextBuilder<NotCurrent>>,
) -> *mut ValueBox<PixelFormatRequirements> {
    _ptr_context_builder.with_not_null_return(std::ptr::null_mut(), |builder| {
        ValueBox::new(builder.pf_reqs.clone()).into_raw()
    })
}


#[no_mangle]
pub fn glutin_destroy_context_builder(_ptr: *mut ValueBox<ContextBuilder<PossiblyCurrent>>) {
    _ptr.drop()
}
