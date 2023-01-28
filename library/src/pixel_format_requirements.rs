use glutin::PixelFormatRequirements;
use value_box::{ValueBox, ValueBoxPointer};

#[no_mangle]
pub fn glutin_pixel_format_requirements_default() -> *mut ValueBox<PixelFormatRequirements> {
    ValueBox::new(PixelFormatRequirements::default()).into_raw()
}

#[no_mangle]
pub fn glutin_pixel_format_requirements_cares_hardware_accelerated(
    _ptr: *mut ValueBox<PixelFormatRequirements>,
) -> bool {
    _ptr.with_not_null_return(false, |requirements| {
        requirements.hardware_accelerated.is_some()
    })
}

#[no_mangle]
pub fn glutin_pixel_format_requirements_is_hardware_accelerated(
    _ptr: *mut ValueBox<PixelFormatRequirements>,
) -> bool {
    _ptr.with_not_null_return(false, |requirements| {
        requirements.hardware_accelerated.unwrap_or(false)
    })
}

#[no_mangle]
pub fn glutin_pixel_format_requirements_cares_color_bits(
    _ptr: *mut ValueBox<PixelFormatRequirements>,
) -> bool {
    _ptr.with_not_null_return(false, |requirements| requirements.color_bits.is_some())
}

#[no_mangle]
pub fn glutin_pixel_format_requirements_get_color_bits(
    _ptr: *mut ValueBox<PixelFormatRequirements>,
) -> u8 {
    _ptr.with_not_null_return(0, |requirements| requirements.color_bits.unwrap_or(0))
}

#[no_mangle]
pub fn glutin_pixel_format_requirements_is_float_color_buffer(
    _ptr: *mut ValueBox<PixelFormatRequirements>,
) -> bool {
    _ptr.with_not_null_return(false, |requirements| requirements.float_color_buffer)
}

#[no_mangle]
pub fn glutin_pixel_format_requirements_cares_alpha_bits(
    _ptr: *mut ValueBox<PixelFormatRequirements>,
) -> bool {
    _ptr.with_not_null_return(false, |requirements| requirements.alpha_bits.is_some())
}

#[no_mangle]
pub fn glutin_pixel_format_requirements_get_alpha_bits(
    _ptr: *mut ValueBox<PixelFormatRequirements>,
) -> u8 {
    _ptr.with_not_null_return(0, |requirements| requirements.alpha_bits.unwrap_or(0))
}

#[no_mangle]
pub fn glutin_pixel_format_requirements_cares_depth_bits(
    _ptr: *mut ValueBox<PixelFormatRequirements>,
) -> bool {
    _ptr.with_not_null_return(false, |requirements| requirements.depth_bits.is_some())
}

#[no_mangle]
pub fn glutin_pixel_format_requirements_get_depth_bits(
    _ptr: *mut ValueBox<PixelFormatRequirements>,
) -> u8 {
    _ptr.with_not_null_return(0, |requirements| requirements.depth_bits.unwrap_or(0))
}

#[no_mangle]
pub fn glutin_pixel_format_requirements_cares_stencil_bits(
    _ptr: *mut ValueBox<PixelFormatRequirements>,
) -> bool {
    _ptr.with_not_null_return(false, |requirements| requirements.stencil_bits.is_some())
}

#[no_mangle]
pub fn glutin_pixel_format_requirements_get_stencil_bits(
    _ptr: *mut ValueBox<PixelFormatRequirements>,
) -> u8 {
    _ptr.with_not_null_return(0, |requirements| requirements.stencil_bits.unwrap_or(0))
}

#[no_mangle]
pub fn glutin_pixel_format_requirements_cares_double_buffer(
    _ptr: *mut ValueBox<PixelFormatRequirements>,
) -> bool {
    _ptr.with_not_null_return(false, |requirements| requirements.double_buffer.is_some())
}

#[no_mangle]
pub fn glutin_pixel_format_requirements_is_double_buffer(
    _ptr: *mut ValueBox<PixelFormatRequirements>,
) -> bool {
    _ptr.with_not_null_return(false, |requirements| {
        requirements.double_buffer.unwrap_or(false)
    })
}

#[no_mangle]
pub fn glutin_pixel_format_requirements_cares_multisampling(
    _ptr: *mut ValueBox<PixelFormatRequirements>,
) -> bool {
    _ptr.with_not_null_return(false, |requirements| requirements.multisampling.is_some())
}

#[no_mangle]
pub fn glutin_pixel_format_requirements_get_multisampling(
    _ptr: *mut ValueBox<PixelFormatRequirements>,
) -> u16 {
    _ptr.with_not_null_return(0, |requirements| requirements.multisampling.unwrap_or(0))
}

#[no_mangle]
pub fn glutin_pixel_format_requirements_is_stereoscopy(
    _ptr: *mut ValueBox<PixelFormatRequirements>,
) -> bool {
    _ptr.with_not_null_return(false, |requirements| requirements.stereoscopy)
}

#[no_mangle]
pub fn glutin_pixel_format_requirements_is_srgb(
    _ptr: *mut ValueBox<PixelFormatRequirements>,
) -> bool {
    _ptr.with_not_null_return(false, |requirements| requirements.srgb)
}

#[no_mangle]
pub fn glutin_pixel_format_requirements_drop(_ptr: *mut ValueBox<PixelFormatRequirements>) {
    _ptr.release();
}
