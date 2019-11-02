use glutin::{ContextBuilder, NotCurrent, Robustness, GlProfile, GlRequest, PossiblyCurrent};
use boxer::CBox;

#[no_mangle]
pub fn glutin_create_context_builder() -> *mut ContextBuilder<'static, NotCurrent> {
    let context_builder = ContextBuilder::new()
        .with_gl_robustness(Robustness::TryRobustNoResetNotification)
        .with_gl_profile(GlProfile::Core);
    CBox::into_raw(context_builder)
}

#[no_mangle]
pub fn glutin_context_builder_with_gl_then_gles(_ptr_context_builder: *mut ContextBuilder<'static, NotCurrent>, gl_major: u8, gl_minor: u8, gles_major: u8, gles_minor: u8) -> *mut ContextBuilder<'static, NotCurrent> {
    CBox::with_consumable_raw(_ptr_context_builder, |builder| {
        builder.with_gl(GlRequest::GlThenGles {
            /// The version to use for OpenGL.
            opengl_version: (gl_major, gl_minor),
            /// The version to use for OpenGL ES.
            opengles_version: (gles_major, gles_minor),
        })
    })
}

#[no_mangle]
pub fn glutin_context_builder_with_gl_latest(_ptr_context_builder: *mut ContextBuilder<'static,NotCurrent>) -> *mut ContextBuilder<'static, NotCurrent> {
    CBox::with_consumable_raw(_ptr_context_builder, |builder| builder.with_gl(GlRequest::Latest))
}

#[no_mangle]
pub fn glutin_context_builder_with_gl_profile_core(_ptr_context_builder: *mut ContextBuilder<'static,NotCurrent>) -> *mut ContextBuilder<'static, NotCurrent> {
    CBox::with_consumable_raw(_ptr_context_builder, |builder| builder.with_gl_profile(GlProfile::Core))
}

#[no_mangle]
pub fn glutin_context_builder_with_multisampling(_ptr_context_builder: *mut ContextBuilder<'static,NotCurrent>, samples: u16) -> *mut ContextBuilder<'static, NotCurrent> {
    CBox::with_consumable_raw(_ptr_context_builder, |builder| builder.with_multisampling(samples))
}

#[no_mangle]
pub fn glutin_context_builder_with_depth_buffer(_ptr_context_builder: *mut ContextBuilder<'static,NotCurrent>, bits: u8) -> *mut ContextBuilder<'static, NotCurrent> {
    CBox::with_consumable_raw(_ptr_context_builder, |builder| builder.with_depth_buffer(bits))
}

#[no_mangle]
pub fn glutin_context_builder_with_stencil_buffer(_ptr_context_builder: *mut ContextBuilder<'static,NotCurrent>, bits: u8) -> *mut ContextBuilder<'static, NotCurrent> {
    CBox::with_consumable_raw(_ptr_context_builder, |builder| builder.with_stencil_buffer(bits))
}

#[no_mangle]
pub fn glutin_context_builder_with_pixel_format(_ptr_context_builder: *mut ContextBuilder<'static,NotCurrent>, color_bits: u8, alpha_bits: u8) -> *mut ContextBuilder<'static, NotCurrent> {
    CBox::with_consumable_raw(_ptr_context_builder, |builder| builder.with_pixel_format(color_bits, alpha_bits))
}

#[no_mangle]
pub fn glutin_context_builder_with_vsync(_ptr_context_builder: *mut ContextBuilder<'static,NotCurrent>, vsync: bool) -> *mut ContextBuilder<'static, NotCurrent> {
    CBox::with_consumable_raw(_ptr_context_builder, |builder| builder.with_vsync(vsync))
}

#[no_mangle]
pub fn glutin_context_builder_with_srgb(_ptr_context_builder: *mut ContextBuilder<'static,NotCurrent>, srgb_enabled: bool) -> *mut ContextBuilder<'static, NotCurrent> {
    CBox::with_consumable_raw(_ptr_context_builder, |builder| builder.with_srgb(srgb_enabled))
}

#[no_mangle]
pub fn glutin_context_builder_with_double_buffer(_ptr_context_builder: *mut ContextBuilder<'static,NotCurrent>, double_buffer_enabled: bool) -> *mut ContextBuilder<'static, NotCurrent> {
    CBox::with_consumable_raw(_ptr_context_builder, |builder| builder.with_double_buffer(Some(double_buffer_enabled)))
}

#[no_mangle]
pub fn glutin_destroy_context_builder(_ptr: *mut ContextBuilder<PossiblyCurrent>) {
    CBox::drop(_ptr);
}