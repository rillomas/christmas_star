extern crate gl;
extern crate cgmath;

use gl::types::{GLuint,GLfloat,GLsizeiptr,GLboolean};
use std::ptr;
use std::mem;
use std::f32::consts;
use std::num::FloatMath;
use cgmath::{Vector3,Vector4};
use glutil;
use game;
use control;

pub struct Light {
    position: cgmath::Vector3<f32>,
    resource : GlResource,
}

struct Vertex {
    position: cgmath::Vector3<f32>,
    diffuse_color: cgmath::Vector4<f32>,
}

impl Vertex {
    fn new(pos: cgmath::Vector3<f32>, diffuse: cgmath::Vector4<f32>) -> Vertex {
        Vertex {
            position : pos,
            diffuse_color : diffuse,
        }
    }
}
struct GlResource {
    shader_program: GLuint,
    vao: GLuint,
    vbo: GLuint,
    indice_num : i32,
    mvp_name : String,
}

impl Light {
    pub fn new(position: cgmath::Vector3<f32>) -> Light {
        Light {
            position: position,
            resource : GlResource {
                shader_program : 0,
                vao: 0,
                vbo: 0,
                indice_num: 0,
                mvp_name : "mvp".to_string(),
            },
        }
    }


    pub fn init(&mut self) -> Result<(), String> {
        let vss = include_str!("vertex.glsl");
        let fss = include_str!("fragment.glsl");
        let vs = try!(glutil::compile_shader(vss.as_slice(), gl::VERTEX_SHADER));
        let fs = try!(glutil::compile_shader(fss.as_slice(), gl::FRAGMENT_SHADER));
        let prog = try!(glutil::link_program(vs, fs));

        // remove shaders since we've finished linking it
        glutil::remove_shader(prog, vs);
        glutil::remove_shader(prog, fs);
 
        let (vao, vbo, ind_num) = try!(init_buffers());

        let r = &mut self.resource;
        r.shader_program = prog;
        r.vao = vao;
        r.vbo = vbo;
        r.indice_num = ind_num;

        Ok(())
    }

    pub fn vector_from(&self, target : &cgmath::Vector3<f32>) -> cgmath::Vector3<f32> {
        self.position.sub(target)
    }

    pub fn close(&mut self) {
        let r = &mut self.resource;
        unsafe {
            gl::DeleteBuffers(1, &r.vbo);
            gl::DeleteVertexArrays(1, &r.vao);
        }
        glutil::remove_program(r.shader_program);
        r.shader_program = 0;
        r.vbo = 0;
        r.vao = 0;
    }
}

impl game::Object for Light {
    fn update(&mut self, cs: &control::State) -> Result<(),String> {
        let delta = 0.01;
        if cs.move_up {
            self.position.y += delta;
        }
        if cs.move_down {
            self.position.y += -delta;
        }
        if cs.move_left {
            self.position.x += -delta;
        }
        if cs.move_right {
            self.position.x += delta;
        }
        Ok(())
    }

    fn draw(&self) -> Result<(),String> {
        let r = &self.resource;
        unsafe {
            gl::UseProgram(r.shader_program);
            try!(glutil::check_error());

            let cstr = r.mvp_name.to_c_str();
            let mvp = gl::GetUniformLocation(r.shader_program, cstr.as_ptr());
            try!(glutil::check_error());
            let p = &self.position;
            let mvp_mat : Vec<f32> = vec![
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                p.x, p.y, p.z, 1.0, // apply translation
            ];
            gl::UniformMatrix4fv(mvp, 1, gl::FALSE, mem::transmute(&mvp_mat[0]));
            try!(glutil::check_error());

            gl::BindVertexArray(r.vao);
            try!(glutil::check_error());
            gl::DrawArrays(gl::LINE_LOOP, 0, r.indice_num);
            try!(glutil::check_error());
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }
        Ok(())
    }
}

fn calculate_vertices(vertices: &mut Vec<Vertex>) {
    let diffuse = cgmath::Vector4::new(1.0,0.0,0.0,1.0);
    // calculate circle coordinates
    let div = 8i;
    let radius = 0.03;
    let rad_per_div = consts::PI_2 / div.to_f32().unwrap();
    for i in range(0,div) {
        let cur_rad = rad_per_div * i.to_f32().unwrap();
        let x = cur_rad.cos() * radius;
        let y = cur_rad.sin() * radius;
        let v = Vertex::new(cgmath::Vector3::new(x,y,0.0), diffuse);
        vertices.push(v);
    }
}

fn init_buffers() -> Result<(GLuint, GLuint, i32), String> {
    let mut vertices : Vec<Vertex> = Vec::new();
    calculate_vertices(&mut vertices);
    let mut vao = 0;
    let mut vbo = 0;
    let mut indice_num;
    unsafe {
        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        try!(glutil::check_error());
        gl::BindVertexArray(vao);
        try!(glutil::check_error());
        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        try!(glutil::check_error());
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        try!(glutil::check_error());
        let vertice_size = mem::size_of::<Vertex>();
        let vertice_num = vertices.len();
        let float_size = mem::size_of::<GLfloat>();
        // println!("Vertex size: {}", vertice_size);
        // println!("Vertex num: {}", vertice_num);
        // println!("float size: {}", float_size);
        gl::BufferData(gl::ARRAY_BUFFER,
            (vertice_num * vertice_size) as GLsizeiptr,
            mem::transmute(&vertices[0]), gl::STATIC_DRAW);
        try!(glutil::check_error());

        indice_num = vertice_num as i32;

        // values taken from layout location in vertex shader
        let pos_location = 0;
        let diffuse_location = 1;
        let stride = vertice_size as i32;
        gl::EnableVertexAttribArray(pos_location);
        try!(glutil::check_error());
        gl::EnableVertexAttribArray(diffuse_location);
        try!(glutil::check_error());
        gl::VertexAttribPointer(pos_location, 3, gl::FLOAT, gl::FALSE as GLboolean, stride, ptr::null());
        try!(glutil::check_error());
        let diffuse_offset = mem::transmute(float_size * 3); // diffuse comes after position
        gl::VertexAttribPointer(diffuse_location, 4, gl::FLOAT, gl::FALSE as GLboolean, stride, diffuse_offset);
        try!(glutil::check_error());
        gl::BindVertexArray(0);
    }
    Ok((vao, vbo, indice_num))
}
