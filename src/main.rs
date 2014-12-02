extern crate glutin;
extern crate libc;
extern crate gl;
extern crate time;
extern crate cgmath;

use std::fmt;

mod drawable;
mod glutil;
mod christmas_star;
mod light;


struct ControlState {
    move_up : bool,
    move_down : bool,
    move_left : bool,
    move_right : bool,

}
impl ControlState {
    fn new() -> ControlState {
        ControlState {
            move_up : false,
            move_down : false,
            move_left : false,
            move_right : false,
        }
    }

    fn moving(&self) -> bool {
        self.move_up || self.move_down || self.move_left || self.move_right
    }
}

impl fmt::Show for ControlState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "move_up: {}, move_down: {}, move_left: {}, move_right: {}",
            self.move_up, self.move_down, self.move_left, self.move_right)
    }
}

fn clear_screen() {
    unsafe { 
        gl::ClearColor(1.0, 1.0, 1.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

fn handle_key_input(elem_state: glutin::ElementState, key_code : Option<glutin::VirtualKeyCode>, cs : &mut ControlState) {
    let pressed = elem_state == glutin::ElementState::Pressed;
    match key_code {
        Some(k) => match k {
            glutin::VirtualKeyCode::Left => cs.move_left = pressed,
            glutin::VirtualKeyCode::Right => cs.move_right = pressed,
            glutin::VirtualKeyCode::Up => cs.move_up = pressed,
            glutin::VirtualKeyCode::Down => cs.move_down = pressed,
            _ => (),
        },
        None => (),
    }
}

fn process_main_loop(window: &glutin::Window, obj_list: &Vec<&mut drawable::Drawable>) {
    let mut cs = ControlState::new(); 
    while !window.is_closed() {
        // process window evets
        for ev in window.poll_events() {
            match ev {
                glutin::Event::KeyboardInput(elem_state, _, key_code) => handle_key_input(elem_state, key_code, &mut cs),
                _ => (),
            }
        }
        if cs.moving() {
            println!("control state: {}", cs);
        }
        // free CPU.
        // We should be checking the elapsed time to see how long we can wait here.
        std::io::timer::sleep(std::time::duration::Duration::milliseconds(8));

        // draw objects
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
    // need an indent here because obj_list will own the obj
    {
        let mut obj_list : Vec<&mut drawable::Drawable> = Vec::new();
        obj_list.push(&mut obj as &mut drawable::Drawable);
        process_main_loop(&window, &obj_list);
    }
    obj.close();
}