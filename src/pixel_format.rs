use boxer::{ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use glutin::PixelFormat;

#[no_mangle]
pub fn glutin_pixel_format_default() -> *mut ValueBox<PixelFormat> {
    ValueBox::new(PixelFormat {
        hardware_accelerated: false,
        color_bits: 0,
        alpha_bits: 0,
        depth_bits: 0,
        stencil_bits: 0,
        stereoscopy: false,
        double_buffer: false,
        multisampling: None,
        srgb: false,
    })
    .into_raw()
}

#[no_mangle]
pub fn glutin_pixel_format_is_hardware_accelerated(_ptr: *mut ValueBox<PixelFormat>) -> bool {
    _ptr.with_not_null_return(false, |pixel_format| pixel_format.hardware_accelerated)
}

#[no_mangle]
pub fn glutin_pixel_format_get_color_bits(_ptr: *mut ValueBox<PixelFormat>) -> u8 {
    _ptr.with_not_null_return(0, |pixel_format| pixel_format.color_bits)
}

#[no_mangle]
pub fn glutin_pixel_format_get_alpha_bits(_ptr: *mut ValueBox<PixelFormat>) -> u8 {
    _ptr.with_not_null_return(0, |pixel_format| pixel_format.alpha_bits)
}

#[no_mangle]
pub fn glutin_pixel_format_get_depth_bits(_ptr: *mut ValueBox<PixelFormat>) -> u8 {
    _ptr.with_not_null_return(0, |pixel_format| pixel_format.depth_bits)
}

#[no_mangle]
pub fn glutin_pixel_format_get_stencil_bits(_ptr: *mut ValueBox<PixelFormat>) -> u8 {
    _ptr.with_not_null_return(0, |pixel_format| pixel_format.stencil_bits)
}

#[no_mangle]
pub fn glutin_pixel_format_is_stereoscopy(_ptr: *mut ValueBox<PixelFormat>) -> bool {
    _ptr.with_not_null_return(false, |pixel_format| pixel_format.stereoscopy)
}

#[no_mangle]
pub fn glutin_pixel_format_is_double_buffer(_ptr: *mut ValueBox<PixelFormat>) -> bool {
    _ptr.with_not_null_return(false, |pixel_format| pixel_format.double_buffer)
}

#[no_mangle]
pub fn glutin_pixel_format_has_multisampling(_ptr: *mut ValueBox<PixelFormat>) -> bool {
    _ptr.with_not_null_return(false, |pixel_format| pixel_format.multisampling.is_some())
}

#[no_mangle]
pub fn glutin_pixel_format_get_multisampling(_ptr: *mut ValueBox<PixelFormat>) -> u16 {
    _ptr.with_not_null_return(0, |pixel_format| pixel_format.multisampling.unwrap_or(0))
}

#[no_mangle]
pub fn glutin_pixel_format_is_srgb(_ptr: *mut ValueBox<PixelFormat>) -> bool {
    _ptr.with_not_null_return(false, |pixel_format| pixel_format.srgb)
}

#[no_mangle]
pub fn glutin_pixel_format_drop(_ptr: &mut *mut ValueBox<PixelFormat>) {
    _ptr.drop();
}
