extern crate gl;
extern crate cgmath;

use gl::types::{GLuint,GLfloat,GLsizeiptr,GLboolean};
use std::ptr;
use std::mem;
use cgmath::{Vector3};

use glutil;
use draw;

pub struct ChristmasStar {
    geometry: Geometry,
    resource: GlResource
}

pub struct Parameter<'a> {
    pub fragment_shader_path: &'a str,
    pub vertex_shader_path: &'a str,
}

struct GlResource {
    shader_program: GLuint,
    vao: GLuint,
    vbo: GLuint,
    triangle_num : i32,
}

struct Geometry {
    center : cgmath::Vector3<f32>,
    left_canyon_offset : cgmath::Vector3<f32>,
    right_canyon_offset : cgmath::Vector3<f32>,
    long_spike_length  : f32,
    short_spike_length : f32,
    thickness : f32, 
}

impl ChristmasStar {
    pub fn new() -> ChristmasStar {
        ChristmasStar{
            geometry : Geometry {
                center : cgmath::Vector3::new(0.0,0.0,0.0),
                left_canyon_offset : cgmath::Vector3::new(0.05, 0.1, 0.0),
                right_canyon_offset : cgmath::Vector3::new(0.1, 0.05, 0.0),
                long_spike_length : 0.8,
                short_spike_length : 0.3,
                thickness : 0.1,
            },
            resource : GlResource {
                shader_program : 0,
                vao: 0,
                vbo: 0,
                triangle_num: 0,
            }
        }
    }

    pub fn init(&mut self, param: Parameter) -> Result<(), String> {
        let vss = match glutil::read_shader(param.vertex_shader_path) {
            Ok(s) => s,
            Err(e) => return Err(format!("Failed reading vertex shader: {}", e)),
        };
        let fss = match glutil::read_shader(param.fragment_shader_path) {
            Ok(s) => s,
            Err(e) => return Err(format!("Failed reading fragment shader: {}", e)),
        };
        let vs = try!(glutil::compile_shader(vss.as_slice(), gl::VERTEX_SHADER));
        let fs = try!(glutil::compile_shader(fss.as_slice(), gl::FRAGMENT_SHADER));
        let prog = try!(glutil::link_program(vs, fs));

        // remove shaders since we've finished linking it
        glutil::remove_shader(prog, vs);
        glutil::remove_shader(prog, fs);
 
        let (vao, vbo, tri_num) = try!(init_buffers(&self.geometry));
        let r = &mut self.resource;
        r.shader_program = prog;
        r.vao = vao;
        r.vbo = vbo;
        r.triangle_num = tri_num;

        Ok(())
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

impl draw::Draw for ChristmasStar {
    fn draw(&self) -> Result<(),String> {
        let r = &self.resource;
        unsafe {
            gl::UseProgram(r.shader_program);
            try!(glutil::check_error());
            gl::BindVertexArray(r.vao);
            try!(glutil::check_error());
            gl::DrawArrays(gl::TRIANGLES, 0, r.triangle_num);
            try!(glutil::check_error());
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }
        Ok(())
    }
}

fn generate_partial_vertices(
    center: cgmath::Vector3<f32>,
    left_canyon_offset: cgmath::Vector3<f32>,
    right_canyon_offset: cgmath::Vector3<f32>,
    left_long_spike: cgmath::Vector3<f32>,
    right_long_spike: cgmath::Vector3<f32>,
    short_spike: cgmath::Vector3<f32>,
    depth: f32) -> Vec<GLfloat> {
    let lcox = left_canyon_offset.x;
    let lcoy = left_canyon_offset.y;
    let rcox = right_canyon_offset.x;
    let rcoy = right_canyon_offset.y;
    let cx = center.x;
    let cy = center.y;
    let cz = center.z;
    let llsx = left_long_spike.x;
    let llsy = left_long_spike.y;
    let rlsx = right_long_spike.x;
    let rlsy = right_long_spike.y;
    let ssx = short_spike.x;
    let ssy = short_spike.y;
    let top = cz+depth;
    let vertices : Vec<GLfloat> = vec![
        cx, cy, top, 1.0, 
        cx+llsx, cy+llsy, cz, 1.0, 
        cx+lcox, cy+lcoy, cz, 1.0, 

        cx, cy, top, 1.0, 
        cx+lcox, cy+lcoy, cz, 1.0, 
        cx+ssx, cy+ssy, cz, 1.0, 

        cx, cy, top, 1.0, 
        cx+ssx, cy+ssy, cz, 1.0, 
        cx+rcox, cy+rcoy, cz, 1.0, 

        cx, cy, top, 1.0, 
        cx+rcox, cy+rcoy, cz, 1.0, 
        cx+rlsx, cy+rlsy, cz, 1.0, 
    ];
    vertices
}

fn generate_vertices(geom: &Geometry) -> Vec<GLfloat> {
    let c = geom.center;
    let ls = geom.long_spike_length;
    let ss = geom.short_spike_length;
    let lco = geom.left_canyon_offset;
    let rco = geom.right_canyon_offset;
    let depth = geom.thickness * 0.5;

    // Create a quarter of a star per each generation
    // and merge at the end.
    let mut top_right = generate_partial_vertices(c,
        lco,
        rco, 
        cgmath::Vector3::new(0.0,ls,0.0),
        cgmath::Vector3::new(ls,0.0,0.0),
        cgmath::Vector3::new(ss,ss,0.0),
        depth);
    let btm_right = generate_partial_vertices(c,
        cgmath::Vector3::new(rco.x, -rco.y, lco.z),
        cgmath::Vector3::new(lco.x, -lco.y, rco.z),
        cgmath::Vector3::new(ls,0.0,0.0),
        cgmath::Vector3::new(0.0,-ls,0.0),
        cgmath::Vector3::new(ss,-ss,0.0),
        depth);
    let btm_left = generate_partial_vertices(c,
        cgmath::Vector3::new(-lco.x, -lco.y, lco.z),
        cgmath::Vector3::new(-rco.x, -rco.y, rco.z),
        cgmath::Vector3::new(0.0,-ls,0.0),
        cgmath::Vector3::new(-ls,0.0,0.0),
        cgmath::Vector3::new(-ss,-ss,0.0),
        depth);
    let top_left = generate_partial_vertices(c,
        cgmath::Vector3::new(-rco.x, rco.y, lco.z),
        cgmath::Vector3::new(-lco.x, lco.y, rco.z),
        cgmath::Vector3::new(-ls, 0.0, 0.0),
        cgmath::Vector3::new(0.0, ls, 0.0),
        cgmath::Vector3::new(-ss, ss, 0.0),
        depth);
    top_right.push_all(btm_right.as_slice());
    top_right.push_all(btm_left.as_slice());
    top_right.push_all(top_left.as_slice());
    top_right
}

fn init_buffers(geom : &Geometry) -> Result<(GLuint, GLuint, i32), String> {
    let mut vao = 0;
    let mut vbo = 0;
    let mut triangle_num = 0;
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
        let vertices = generate_vertices(geom);
        gl::BufferData(gl::ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            mem::transmute(&vertices[0]), gl::STATIC_DRAW);
        try!(glutil::check_error());

        triangle_num = (vertices.len() / 3).to_i32().unwrap();

        let pos_location = 0; // value taken from layout location in vertex shader
        gl::EnableVertexAttribArray(pos_location);
        try!(glutil::check_error());
        gl::VertexAttribPointer(pos_location, 4, gl::FLOAT, gl::FALSE as GLboolean, 0, ptr::null());
        try!(glutil::check_error());
        gl::BindVertexArray(0);
    }
    Ok((vao, vbo, triangle_num))
}

