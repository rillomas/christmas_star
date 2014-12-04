extern crate glutin;
extern crate libc;
extern crate gl;
extern crate time;
extern crate cgmath;

mod game;
mod glutil;
mod christmas_star;
mod light;
mod control;

fn clear_screen() {
    unsafe { 
        gl::ClearColor(1.0, 1.0, 1.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

fn process_main_loop(window: &glutin::Window, obj_list: &mut Vec<&mut game::Object>) {
    let mut cs = control::State::new(); 
    while !window.is_closed() {
        // process window evets
        for ev in window.poll_events() {
            match ev {
                glutin::Event::KeyboardInput(elem_state, _, key_code) => cs.handle_key_input(elem_state, key_code),
                _ => (),
            }
        }
        // if cs.moving() {
        //     println!("control state: {}", cs);
        // }
        // free CPU.
        // We should be checking the elapsed time to see how long we can wait here.
        std::io::timer::sleep(std::time::duration::Duration::milliseconds(8));

        // update all
        for o in obj_list.iter_mut() {
            o.update(&cs)
                .unwrap_or_else(|e| panic!("Error when updating: {}", e));
        }

        // draw all
        clear_screen();
        for o in obj_list.iter() {
            o.draw()
                .unwrap_or_else(|e| panic!("Error when drawing: {}", e));
        }
        unsafe { gl::Flush(); }
        window.swap_buffers();
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

    let p = christmas_star::Parameter {
        vertex_shader_path: "src\\christmas_star\\vertex.glsl",
        fragment_shader_path: "src\\christmas_star\\fragment.glsl",
    };
    let mut obj = christmas_star::ChristmasStar::new();
    obj.init(p)
        .unwrap_or_else(|e| panic!("ChristmasStar init failed: {}", e));
    // need an indent here because draw_list will own the obj
    {
        let mut obj_list : Vec<&mut game::Object> = Vec::new();
        obj_list.push(&mut obj as &mut game::Object);
        process_main_loop(&window, &mut obj_list);
    }
    obj.close();
}