extern crate glutin;
extern crate libc;
extern crate gl;
extern crate time;

mod draw_object;

fn clear_screen() {
    unsafe { 
        gl::ClearColor(1.0, 1.0, 1.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

fn get_msec_tick(t: time::Timespec) -> i64 {
    (t.sec * 1000 + (t.nsec / 1000).to_i64().unwrap())
}

fn get_tick() -> i64 {
    let time = time::get_time();
    return get_msec_tick(time);
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
    let mut previous = get_tick();
    let max_delta = 16;
    while !window.is_closed() {
        window.wait_events();
        let mut current = get_tick();
        loop {
            for _ in window.poll_events() {
                // process events
            }
            let delta = current - previous;
            if delta >= max_delta {
                break;
            }
            std::io::timer::sleep(std::time::duration::Duration::milliseconds(1));
            current = get_tick();
        }

        // draw
        clear_screen();
        obj.draw();
        unsafe { gl::Flush(); }
        window.swap_buffers();
        previous = current;
    }
    obj.close();
}