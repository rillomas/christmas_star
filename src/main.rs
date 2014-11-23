extern crate glutin;
extern crate libc;
extern crate gl;

use gl::types::{GLenum,GLuint,GLchar,GLint};
use std::ptr;
use std::io::{File,IoResult};

fn compile_shader(src: &str, ty: GLenum) -> Result<GLuint, String> {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        // Attempt to compile the shader
        src.with_c_str(|ptr| gl::ShaderSource(shader, 1, &ptr, ptr::null()));
        gl::CompileShader(shader);
        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::from_elem(len as uint - 1, 0u8); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            match String::from_utf8(buf) {
                Ok(s) => return Err(s),
                Err(_) => return Err("Shader compile failed. Since the ShaderInfoLog is not valid utf8, the error log could not be retrived".to_string())
            }
        }
    }
    Ok(shader)
}

fn link_program(vs: GLuint, fs: GLuint) -> Result<GLuint, String> {
    let program;
    unsafe {
        program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);
        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::from_elem(len as uint - 1, 0u8); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            match String::from_utf8(buf) {
                Ok(s) => return Err(s),
                Err(_) => return Err("Program link failed. Since the ProgramInfoLog is not valid utf8, the error log could not be retrived".to_string())
            }
        }
    }
    Ok(program)
}

fn read_shader(path: &str) -> IoResult<String> {
    let mut sf = try!(File::open(&Path::new(path)));
    let ss = try!(sf.read_to_string());
    Ok(ss)
}

fn remove_shader(program: GLuint, s: GLuint) {
    unsafe {
        gl::DetachShader(program, s);
        gl::DeleteShader(s);
    }
}

fn remove_program(program: GLuint) {
    unsafe {
        gl::DeleteProgram(program);
    }
}

struct DrawObject {
    shader_program: GLuint,
    vao: GLuint,
    vbo: GLuint,
}
struct DrawObjectParameter<'a> {
    fragment_shader_path: &'a str,
    vertex_shader_path: &'a str,
}

impl DrawObject {
    fn init(&mut self, param: DrawObjectParameter) -> Result<(), String> {
        let vss = match read_shader(param.vertex_shader_path) {
            Ok(s) => s,
            Err(e) => return Err(format!("Failed reading vertex shader: {}", e)),
        };
        let fss = match read_shader(param.fragment_shader_path) {
            Ok(s) => s,
            Err(e) => return Err(format!("Failed reading fragment shader: {}", e)),
        };
        let vs = try!(compile_shader(vss.as_slice(), gl::VERTEX_SHADER));
        let fs = try!(compile_shader(fss.as_slice(), gl::FRAGMENT_SHADER));
        let prog = try!(link_program(vs, fs));

        // remove shaders since we've finished linking it
        remove_shader(prog, vs);
        remove_shader(prog, fs);
        self.shader_program = prog;
        Ok(())
    }

    fn close(&mut self) {
        remove_program(self.shader_program);
        self.shader_program = 0;
    }
}

fn main() {
    let builder = glutin::WindowBuilder::new();
    let r = builder.with_dimensions(300, 300)
        .with_title("rust glsl sample".to_string())
        .build();
    let window = r.unwrap_or_else(|e| panic!("Error while building window: {}", e));
    unsafe { window.make_current() };
    gl::load_with(|symbol| window.get_proc_address(symbol));

    let dop = DrawObjectParameter {
        vertex_shader_path: "src\\vertex.glsl",
        fragment_shader_path: "src\\fragment.glsl",
    };
    let mut obj = DrawObject{
        shader_program : 0,
        vao: 0,
        vbo: 0,
    };
    obj.init(dop)
        .unwrap_or_else(|e| panic!("DrawObject init failed: {}", e));

    unsafe { gl::ClearColor(0.0, 0.0, 1.0, 1.0); }
    while !window.is_closed() {
        window.wait_events();
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }
        unsafe { gl::Flush(); }
        window.swap_buffers();
    }
    obj.close();
}