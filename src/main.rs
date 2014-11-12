extern crate glutin;
extern crate libc;
extern crate gl;

fn main() {
    let window = glutin::Window::new().unwrap();
    unsafe { window.make_current() };
    gl::load_with(|symbol| window.get_proc_address(symbol));
    unsafe { gl::ClearColor(0.0, 0.0, 1.0, 1.0); }

    while !window.is_closed() {
        window.wait_events();
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }
        unsafe { gl::Flush(); }
        window.swap_buffers();
    }
}