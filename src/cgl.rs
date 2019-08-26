use super::*;

///////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////  G L ////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

fn error_callback(_gl: &dyn gleam::gl::Gl, message: &str, error: gl::GLenum) {
    println!("[GL] error: {} code: {}", message, error);
}

#[no_mangle]
pub fn glutin_windowed_context_load_gl(_ptr_window: *mut glutin::WindowedContext<glutin::PossiblyCurrent>) -> *mut GlutinGL {
    let window: &glutin::WindowedContext<glutin::PossiblyCurrent> = to_rust_reference!(_ptr_window);

    let mut gl: std::rc::Rc<(dyn gleam::gl::Gl + 'static)> = match window.get_api() {
        glutin::Api::OpenGl => unsafe {
            gl::GlFns::load_with(|symbol| window.get_proc_address(symbol) as *const _)
        },
        glutin::Api::OpenGlEs => unsafe {
            gl::GlesFns::load_with(|symbol| window.get_proc_address(symbol) as *const _)
        },
        glutin::Api::WebGl => unimplemented!(),
    };

    gl = gl::ErrorReactingGl::wrap(gl, error_callback);

    let _mut_gl: *const dyn gleam::gl::Gl = std::rc::Rc::into_raw(gl);

    let hack = GlutinGL { gl: _mut_gl };
    let _hack_ptr = for_create!(hack);

    _hack_ptr
}

#[no_mangle]
pub fn glutin_gl_release(_ptr_gl: *mut GlutinGL) {
    let hack: &GlutinGL = for_delete!(_ptr_gl);
    let _: std::rc::Rc<dyn gleam::gl::Gl> = unsafe { std::rc::Rc::from_raw(hack.gl) };
    //drop
}

#[no_mangle]
pub fn glutin_gl_clear_color(_ptr_gl: *mut GlutinGL, r: f32, g: f32, b: f32, a: f32) {
    GlutinGL::with_raw(_ptr_gl,|gl| gl.clear_color(r, g, b, a));
}

#[no_mangle]
pub fn glutin_gl_clear(_ptr_gl: *mut GlutinGL, buffer_mask: gl::GLbitfield) {
    GlutinGL::with_raw(_ptr_gl,|gl|  gl.clear(buffer_mask));
}

#[no_mangle]
pub fn glutin_gl_get_string(_ptr_gl: *mut GlutinGL, which: gl::GLenum, _ptr_string: *mut GlutinCString) {
    GlutinGL::with_raw(_ptr_gl,|gl| {
        let c_string: &mut GlutinCString = to_rust_reference!(_ptr_string);
        let version = gl.get_string(which);

        let length = version.len();

        let s: std::ffi::CString = std::ffi::CString::new(version).unwrap();
        let p: *mut c_char = s.into_raw();

        c_string.data = p;
        c_string.length = length;
    });
}

#[no_mangle]
pub fn glutin_gl_free_cstring(_ptr_string: *mut GlutinCString) {
    let c_string: &mut GlutinCString = to_rust_reference!(_ptr_string);
    unsafe {
        std::ffi::CString::from_raw(c_string.data);
    }
    c_string.length = 0;
}

#[no_mangle]
pub fn glutin_gl_get_shader_version(_ptr_gl: *mut GlutinGL) -> u32 {
    GlutinGL::with_raw(_ptr_gl,|gl| {
        let version = gl.get_string(gl::SHADING_LANGUAGE_VERSION);

        let split = version.split_whitespace();
        let vec: Vec<&str> = split.collect();

        let number = vec[0].parse::<f32>();
        (number.unwrap() * 100.0) as u32
    })
}

#[no_mangle]
pub fn glutin_gl_gen_texture(_ptr_gl: *mut GlutinGL) -> gl::GLuint {
    GlutinGL::with_raw(_ptr_gl,|gl| gl.gen_textures(1)[0])
}

#[no_mangle]
pub fn glutin_gl_bind_texture_2d(_ptr_gl: *mut GlutinGL, texture: gl::GLuint) {
    GlutinGL::with_raw(_ptr_gl,|gl| gl.bind_texture(gl::TEXTURE_2D, texture));
}

#[no_mangle]
pub fn glutin_gl_enable_texture_2d(_ptr_gl: *mut GlutinGL) {
    GlutinGL::with_raw(_ptr_gl,|gl| gl.enable(gl::TEXTURE_2D));
}

#[no_mangle]
pub fn glutin_gl_disable_texture_2d(_ptr_gl: *mut GlutinGL) {
    GlutinGL::with_raw(_ptr_gl,|gl| gl.disable(gl::TEXTURE_2D));
}

#[no_mangle]
pub fn glutin_gl_create_vertex_shader(_ptr_gl: *mut GlutinGL)-> gl::GLuint {
    GlutinGL::with_raw(_ptr_gl,|gl| gl.create_shader(gl::VERTEX_SHADER))
}

#[no_mangle]
pub fn glutin_gl_create_fragment_shader(_ptr_gl: *mut GlutinGL) -> gl::GLuint {
    GlutinGL::with_raw(_ptr_gl,|gl| gl.create_shader(gl::FRAGMENT_SHADER))
}

#[no_mangle]
pub fn glutin_gl_compile_shader(_ptr_gl: *mut GlutinGL, _shader: gl::GLuint) {
    GlutinGL::with_raw(_ptr_gl,|gl| {
        gl.compile_shader(_shader);
        let log = gl.get_shader_info_log(_shader);
        if !log.is_empty() {
            println!("shader log: {}", log);
        }
    });
}

#[no_mangle]
pub fn glutin_gl_shader_source(_ptr_gl: *mut GlutinGL, _shader: gl::GLuint, _ptr_title: *const c_char) {
    GlutinGL::with_raw(_ptr_gl,|gl| {
        let source: &str = to_rust_string!(_ptr_title);
        gl.shader_source(_shader, &[source.as_bytes()]);
    });
}

#[no_mangle]
pub fn glutin_gl_create_program(_ptr_gl: *mut GlutinGL) -> gl::GLuint {
    GlutinGL::with_raw(_ptr_gl,|gl| gl.create_program())
}

#[no_mangle]
pub fn glutin_gl_attach_shader(_ptr_gl: *mut GlutinGL, _program: gl::GLuint, _shader: gl::GLuint) {
    GlutinGL::with_raw(_ptr_gl,|gl| gl.attach_shader(_program, _shader));
}

#[no_mangle]
pub fn glutin_gl_link_program(_ptr_gl: *mut GlutinGL, _program: gl::GLuint) {
    GlutinGL::with_raw(_ptr_gl,|gl| gl.link_program(_program));
}

#[no_mangle]
pub fn glutin_gl_use_program(_ptr_gl: *mut GlutinGL, _program: gl::GLuint) {
    GlutinGL::with_raw(_ptr_gl,|gl| gl.use_program(_program));
}

#[no_mangle]
pub fn glutin_gl_viewport(_ptr_gl: *mut GlutinGL, x: gl::GLint, y: gl::GLint, width: gl::GLsizei, height: gl::GLsizei) {
    GlutinGL::with_raw(_ptr_gl,|gl| gl.viewport(x, y, width, height));
}

#[no_mangle]
pub fn glutin_gl_create_buffer(_ptr_gl: *mut GlutinGL) -> gl::GLuint {
    GlutinGL::with_raw(_ptr_gl,|gl| gl.gen_buffers(1)[0])
}

#[no_mangle]
pub fn glutin_gl_bind_array_buffer(_ptr_gl: *mut GlutinGL, buffer: gl::GLuint) {
    GlutinGL::with_raw(_ptr_gl,|gl| gl.bind_buffer(gl::ARRAY_BUFFER, buffer));
}

#[no_mangle]
pub fn glutin_gl_array_buffer_data_static_draw(_ptr_gl: *mut GlutinGL, array: *const f32, length: u32) {
    GlutinGL::with_raw(_ptr_gl,|gl| {
        let data: &[f32] = unsafe { std::slice::from_raw_parts(array, length as usize) };

        gl.buffer_data_untyped(
            gl::ARRAY_BUFFER,
            (data.len() * std::mem::size_of::<f32>()) as gl::GLsizeiptr,
            data.as_ptr() as *const gl::GLvoid,
            gl::STATIC_DRAW,
        );
    });
}

#[no_mangle]
pub fn glutin_gl_get_attribute_location(_ptr_gl: *mut GlutinGL, program: gl::GLuint, _ptr_name: *const c_char) -> i32 {
    GlutinGL::with_raw(_ptr_gl,|gl| gl.get_attrib_location(program, to_rust_string!(_ptr_name)))
}

#[no_mangle]
pub fn glutin_gl_get_uniform_location(_ptr_gl: *mut GlutinGL, program: gl::GLuint, _ptr_name: *const c_char) -> i32 {
    GlutinGL::with_raw(_ptr_gl,|gl| gl.get_uniform_location(program, to_rust_string!(_ptr_name)))
}

#[no_mangle]
pub fn glutin_gl_tex_parameter_i(_ptr_gl: *mut GlutinGL, target: gl::GLenum, pname: gl::GLenum, param: gl::GLint) {
    GlutinGL::with_raw(_ptr_gl,|gl| gl.tex_parameter_i(target, pname, param));
}

#[no_mangle]
pub fn glutin_gl_tex_image_2d(_ptr_gl: *mut GlutinGL, level: gl::GLint, internal_format: gl::GLint, width: gl::GLsizei, height: gl::GLsizei, border: gl::GLint, format: gl::GLenum, ty: gl::GLenum, array: *const u8, length: u32) {
    GlutinGL::with_raw(_ptr_gl,|gl| {
        let data: &[u8] = unsafe { std::slice::from_raw_parts(array, length as usize) };
        gl.tex_image_2d(
            gl::TEXTURE_2D,
            level,
            internal_format,
            width,
            height,
            border,
            format,
            ty,
            Some(data));
    });
}

#[no_mangle]
pub fn glutin_gl_tex_sub_image_2d(_ptr_gl: *mut GlutinGL, level: gl::GLint, xoffset: gl::GLint, yoffset: gl::GLint, width: gl::GLsizei, height: gl::GLsizei, format: gl::GLenum, ty: gl::GLenum, array: *const u8, length: u32) {
    GlutinGL::with_raw(_ptr_gl,|gl| {
        let data: &[u8] = unsafe { std::slice::from_raw_parts(array, length as usize) };

        gl.tex_sub_image_2d(
            gl::TEXTURE_2D,
            level,
            xoffset,
            yoffset,
            width,
            height,
            format,
            ty,
            data);
    });
}

#[no_mangle]
pub fn glutin_gl_gen_vertex_array(_ptr_gl: *mut GlutinGL) -> gl::GLuint {
    GlutinGL::with_raw(_ptr_gl,|gl| gl.gen_vertex_arrays(1)[0])
}

#[no_mangle]
pub fn glutin_gl_bind_vertex_array (_ptr_gl: *mut GlutinGL, vao: gl::GLuint) {
    GlutinGL::with_raw(_ptr_gl,|gl| gl.bind_vertex_array(vao));
}

#[no_mangle]
pub fn glutin_gl_enable_vertex_attrib_array(_ptr_gl: *mut GlutinGL, index: gl::GLuint) {
    GlutinGL::with_raw(_ptr_gl,|gl| gl.enable_vertex_attrib_array(index));
}

#[no_mangle]
pub fn glutin_gl_vertex_attrib_pointer(_ptr_gl: *mut GlutinGL, index: gl::GLuint, size: gl::GLint, type_: gl::GLenum, normalized: bool, stride: gl::GLsizei, offset: gl::GLuint) {
    GlutinGL::with_raw(_ptr_gl,|gl| gl.vertex_attrib_pointer(
        index,
        size,
        type_,
        normalized,
        stride,
        offset));
}

#[no_mangle]
pub fn glutin_gl_draw_arrays(_ptr_gl: *mut GlutinGL, mode: gl::GLenum, first: gl::GLint, count: gl::GLsizei) {
    GlutinGL::with_raw(_ptr_gl,|gl| gl.draw_arrays(mode, first, count));
}