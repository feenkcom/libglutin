use boxer::CBox;
use boxer::boxes::{ValueBox, ValueBoxPointer};
use gleam::gl;
use structs::GlutinGL;
use boxer::string::BoxerString;

///////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////  G L ////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////
fn error_callback(_gl: &dyn gleam::gl::Gl, message: &str, error: gl::GLenum) {
    println!("[GL] error: {} code: {}", message, error);
}

#[no_mangle]
pub fn glutin_windowed_context_load_gl(_ptr_window: *mut ValueBox<glutin::WindowedContext<glutin::PossiblyCurrent>>) -> *mut GlutinGL {
    _ptr_window.with(|window| {
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
        CBox::into_raw(GlutinGL { gl: _mut_gl })
    })
}

#[no_mangle]
pub fn glutin_gl_release(_ptr_gl: *mut GlutinGL) {
    // it will be dropped so give Rust control back
    let hack = unsafe { CBox::from_raw(_ptr_gl) };
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
pub fn glutin_gl_get_string(_ptr_gl: *mut GlutinGL, which: gl::GLenum, _ptr_string: *mut BoxerString) {
    GlutinGL::with_raw(_ptr_gl, |gl| {
        CBox::with_raw(_ptr_string, |string| {
            string.set_string(gl.get_string(which))
        })
    });
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
pub fn glutin_gl_shader_source(_ptr_gl: *mut GlutinGL, _shader: gl::GLuint, _ptr_source: *mut BoxerString) {
    GlutinGL::with_raw(_ptr_gl,|gl| {
        CBox::with_raw(_ptr_source, |source| {
            let source_string = source.to_string();
            gl.shader_source(_shader, &[source_string.as_bytes()]);
        });
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
pub fn glutin_gl_get_attribute_location(_ptr_gl: *mut GlutinGL, program: gl::GLuint, _ptr_location: *mut BoxerString) -> i32 {
    GlutinGL::with_raw(_ptr_gl,|gl| {
        CBox::with_raw(_ptr_location, |location| gl.get_attrib_location(program, location.to_string().as_ref()))
    })
}

#[no_mangle]
pub fn glutin_gl_get_uniform_location(_ptr_gl: *mut GlutinGL, program: gl::GLuint, _ptr_location: *mut BoxerString) -> i32 {
    GlutinGL::with_raw(_ptr_gl,|gl| {
        CBox::with_raw(_ptr_location, |location| gl.get_uniform_location(program, location.to_string().as_ref()))
    })
}

#[no_mangle]
pub fn glutin_gl_tex_parameter_i(_ptr_gl: *mut GlutinGL, target: gl::GLenum, pname: gl::GLenum, param: gl::GLint) {
    GlutinGL::with_raw(_ptr_gl,|gl| gl.tex_parameter_i(target, pname, param));
}

#[no_mangle]
pub fn glutin_gl_tex_image_2d(_ptr_gl: *mut GlutinGL, level: gl::GLint, internal_format: gl::GLint, width: gl::GLsizei, height: gl::GLsizei, border: gl::GLint, format: gl::GLenum, ty: gl::GLenum, array: *const u8, length: u32) {
    GlutinGL::with_raw(_ptr_gl,|gl| {
        // data is a reference, dropping it does nothing to the original source
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

#[no_mangle]
pub fn glutin_gl_get_integer(_ptr_gl: *mut GlutinGL, name: gl::GLenum) -> gl::GLint {
    GlutinGL::with_raw(_ptr_gl,|gl| {
        let mut result: [gl::GLint; 1] = [0;1];
        unsafe { gl.get_integer_v( name, &mut result) };
        result[0]
    })
}