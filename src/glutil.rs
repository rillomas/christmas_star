extern crate gl;

use gl::types::{GLenum,GLuint,GLchar,GLint};
use std::ptr;
use std::ffi::CString;
use std::error::Error;

pub fn compile_shader(src: &str, ty: GLenum) -> Result<GLuint, String> {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        // Attempt to compile the shader
        let cstr;
        match CString::new(src.as_bytes()) {
            Ok(s) => cstr = s,
            Err(e) => return Err(e.description().to_string())
        }
        gl::ShaderSource(shader, 1, &cstr.as_ptr(), ptr::null());
        gl::CompileShader(shader);
        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let cap = len as usize - 1;// subtract 1 to skip the trailing null character
            let mut buf = Vec::with_capacity(cap); 
            let mut logLength = 0;
            let vp = buf.as_mut_ptr();
            gl::GetShaderInfoLog(shader, len, &mut logLength, vp as *mut GLchar);
            let rebuilt = Vec::from_raw_parts(vp, logLength as usize, cap);
            match String::from_utf8(rebuilt) {
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
            let mut buf = Vec::with_capacity(len as usize - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            match String::from_utf8(buf) {
                Ok(s) => return Err(s),
                Err(_) => return Err("Program link failed. Since the ProgramInfoLog is not valid utf8, the error log could not be retrived".to_string())
            }
        }
    }
    Ok(program)
}

// pub fn read_shader(path: &str) -> IoResult<String> {
//     let mut sf = try!(File::open(&Path::new(path)));
//     let ss = try!(sf.read_to_string());
//     Ok(ss)
// }

pub fn remove_shader(program: GLuint, s: GLuint) {
    unsafe {
        gl::DetachShader(program, s);
        gl::DeleteShader(s);
    }
}

pub fn check_error() -> Result<(), String> {
    let mut err;
    unsafe {
        err = gl::GetError();
    }
    match err {
        gl::NO_ERROR => return Ok(()),
        gl::INVALID_ENUM => return Err("Invalid enum".to_string()),
        gl::INVALID_OPERATION => return Err("Invalid operation".to_string()),
        // gl::STACK_OVERFLOW => return Err("Stack overflow".to_string()),
        // gl::STACK_UNDERFLOW => return Err("Stack underflow".to_string()),
        gl::OUT_OF_MEMORY => return Err("Out of memory".to_string()),
        // gl::TABLE_TOO_LARGE => return Err("Table too large".to_string()),
        _ => return Err("Unknown error".to_string()),
    }
}

pub fn remove_program(program: GLuint) {
    unsafe {
        gl::DeleteProgram(program);
    }
}
