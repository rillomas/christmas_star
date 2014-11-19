extern crate glutin;
extern crate libc;
extern crate gl;

use gl::types::{GLenum,GLuint,GLchar,GLint};
use std::ptr;
use std::str;
use std::io::{File,IoResult};

fn compile_shader(src: &str, ty: GLenum) -> GLuint {
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
            panic!("{}", str::from_utf8(buf.as_slice()).expect("ShaderInfoLog not valid utf8"));
        }
        shader
    }
}

fn read_shader(path: &str) -> IoResult<String> {
    let mut sf = try!(File::open(&Path::new(path)));
    let ss = try!(sf.read_to_string());
    Ok(ss)
}

fn main() {
    let vss = match read_shader("src\\vertex.glsl") {
        Ok(s) => s,
        Err(e) => panic!("Failed reading vertex shader: {}", e),
    };
    println!("vs:\n{}",vss);
    let fss = match read_shader("src\\fragment.glsl") {
        Ok(s) => s,
        Err(e) => panic!("Failed reading fragment shader: {}", e),
    };
    println!("fs:\n{}",fss);

    let builder = glutin::WindowBuilder::new();
    let r = builder.with_dimensions(300, 300)
        .with_title("rust sample".to_string())
        .build();
    match r {
        Ok(window) => {
            unsafe { window.make_current() };
            gl::load_with(|symbol| window.get_proc_address(symbol));

            // call gl functions after the symbols are loaded
            let vs = compile_shader(vss.as_slice(), gl::VERTEX_SHADER);
            let fs = compile_shader(fss.as_slice(), gl::FRAGMENT_SHADER);
            unsafe { gl::ClearColor(0.0, 0.0, 1.0, 1.0); }

            while !window.is_closed() {
                window.wait_events();
                unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }
                unsafe { gl::Flush(); }
                window.swap_buffers();
            }
        },
        Err(e) => {
            println!("Error while building window: {}", e);
        }
    }
}