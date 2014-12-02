extern crate glutin;

use std::fmt;

pub struct State {
    pub move_up : bool,
    pub move_down : bool,
    pub move_left : bool,
    pub move_right : bool,
}

impl State {
    pub fn new() -> State {
        State {
            move_up : false,
            move_down : false,
            move_left : false,
            move_right : false,
        }
    }

    pub fn moving(&self) -> bool {
        self.move_up || self.move_down || self.move_left || self.move_right
    }


    pub fn handle_key_input(&mut self, elem_state: glutin::ElementState, key_code : Option<glutin::VirtualKeyCode>) {
        let pressed = elem_state == glutin::ElementState::Pressed;
        match key_code {
            Some(k) => match k {
                glutin::VirtualKeyCode::Left => self.move_left = pressed,
                glutin::VirtualKeyCode::Right => self.move_right = pressed,
                glutin::VirtualKeyCode::Up => self.move_up = pressed,
                glutin::VirtualKeyCode::Down => self.move_down = pressed,
                _ => (),
            },
            None => (),
        }
    }
}

impl fmt::Show for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "move_up: {}, move_down: {}, move_left: {}, move_right: {}",
            self.move_up, self.move_down, self.move_left, self.move_right)
    }
}

