extern crate glutin;
extern crate libc;
extern crate gl;
extern crate time;

mod draw_object;
// mod timer;

fn clear_screen() {
    unsafe { 
        gl::ClearColor(1.0, 1.0, 1.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
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

    let dop = draw_object::DrawObjectParameter {
        vertex_shader_path: "src\\draw_object\\vertex.glsl",
        fragment_shader_path: "src\\draw_object\\fragment.glsl",
    };
    let mut obj = draw_object::DrawObject::new();
    obj.init(dop)
        .unwrap_or_else(|e| panic!("DrawObject init failed: {}", e));
    while !window.is_closed() {
        // process window evets
        for _ in window.poll_events() {
        }
        // free CPU.
        // We should be checking the elapsed time to see how long we can wait here.
        std::io::timer::sleep(std::time::duration::Duration::milliseconds(1));

        // draw
        clear_screen();
        obj.draw();
        unsafe { gl::Flush(); }
        window.swap_buffers();
    }
    obj.close();
}