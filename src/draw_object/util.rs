extern crate gl;

use gl::types::{GLenum,GLuint,GLchar,GLint};
use std::ptr;
use std::io::{File,IoResult};

pub fn compile_shader(src: &str, ty: GLenum) -> Result<GLuint, String> {
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

pub fn link_program(vs: GLuint, fs: GLuint) -> Result<GLuint, String> {
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

pub fn read_shader(path: &str) -> IoResult<String> {
    let mut sf = try!(File::open(&Path::new(path)));
    let ss = try!(sf.read_to_string());
    Ok(ss)
}

pub fn remove_shader(program: GLuint, s: GLuint) {
    unsafe {
        gl::DetachShader(program, s);
        gl::DeleteShader(s);
    }
}

pub fn remove_program(program: GLuint) {
    unsafe {
        gl::DeleteProgram(program);
    }
}
