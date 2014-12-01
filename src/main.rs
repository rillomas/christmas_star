extern crate glutin;
extern crate libc;
extern crate gl;
extern crate time;
extern crate cgmath;

mod drawable;
mod glutil;
mod christmas_star;
mod light;

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

    let p = christmas_star::Parameter {
        vertex_shader_path: "src\\christmas_star\\vertex.glsl",
        fragment_shader_path: "src\\christmas_star\\fragment.glsl",
    };
    let mut obj = christmas_star::ChristmasStar::new();
    obj.init(p)
        .unwrap_or_else(|e| panic!("ChristmasStar init failed: {}", e));
    {
        let mut obj_list : Vec<&mut drawable::Drawable> = Vec::new();
        obj_list.push(&mut obj as &mut drawable::Drawable);
        while !window.is_closed() {
            // process window evets
            for ev in window.poll_events() {
                match ev {
                    glutin::Event::KeyboardInput(_, _, key_code) => {
                        match key_code {
                            Some(k) => match k {
                                glutin::VirtualKeyCode::Left => println!("Left pressed"),
                                glutin::VirtualKeyCode::Right => println!("Right pressed"),
                                glutin::VirtualKeyCode::Up => println!("Up pressed"),
                                glutin::VirtualKeyCode::Down => println!("Down pressed"),
                                _ => (),
                            },
                            None => (),
                        }
                    },
                    _ => (),
                }
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
    obj.close();
}