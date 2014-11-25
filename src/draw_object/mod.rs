extern crate gl;

use gl::types::{GLuint,GLfloat,GLsizeiptr,GLboolean};
use std::ptr;
use std::mem;

mod util;

pub struct DrawObject {
    shader_program: GLuint,
    vao: GLuint,
    vbo: GLuint,
}

pub struct DrawObjectParameter<'a> {
    pub fragment_shader_path: &'a str,
    pub vertex_shader_path: &'a str,
}

impl DrawObject {
    pub fn new() -> DrawObject {
        DrawObject{
            shader_program : 0,
            vao: 0,
            vbo: 0,
        }
    }

    pub fn init(&mut self, param: DrawObjectParameter) -> Result<(), String> {
        let vss = match util::read_shader(param.vertex_shader_path) {
            Ok(s) => s,
            Err(e) => return Err(format!("Failed reading vertex shader: {}", e)),
        };
        let fss = match util::read_shader(param.fragment_shader_path) {
            Ok(s) => s,
            Err(e) => return Err(format!("Failed reading fragment shader: {}", e)),
        };
        let vs = try!(util::compile_shader(vss.as_slice(), gl::VERTEX_SHADER));
        let fs = try!(util::compile_shader(fss.as_slice(), gl::FRAGMENT_SHADER));
        let prog = try!(util::link_program(vs, fs));

        // remove shaders since we've finished linking it
        util::remove_shader(prog, vs);
        util::remove_shader(prog, fs);
        self.shader_program = prog;

        // initialize buffers
        let (vao, vbo) = self.init_buffers();
        self.vao = vao;
        self.vbo = vbo;

        Ok(())
    }

    fn init_buffers(&self) -> (GLuint, GLuint) {
        let mut vao = 0;
        let mut vbo = 0;
        // Vertex data
        static VERTEX_DATA: [GLfloat, ..12] = [
            0.75, 0.75, 0.0, 1.0, 
            0.75, -0.75, 0.0, 1.0, 
            -0.75, -0.75, 0.0, 1.0, 
        ];
        unsafe {
            // Create Vertex Array Object
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            // Create a Vertex Buffer Object and copy the vertex data to it
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER,
                (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                mem::transmute(&VERTEX_DATA[0]), gl::STATIC_DRAW);

            let pos_location = 0; // value taken from layout location in vertex shader
            gl::EnableVertexAttribArray(pos_location);
            gl::VertexAttribPointer(pos_location, 4, gl::FLOAT, gl::FALSE as GLboolean, 0, ptr::null());
            gl::BindVertexArray(0);
        }
        (vao, vbo)
    }

    pub fn draw(&self) {
        unsafe {
            gl::UseProgram(self.shader_program);
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }
    }

    pub fn close(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
        util::remove_program(self.shader_program);
        self.shader_program = 0;
        self.vbo = 0;
        self.vao = 0;
    }
}
