extern crate cgmath;
use drawable;

use cgmath::{Vector3};

pub struct Directional {
    pub position: cgmath::Vector3<f32>,
    pub name: String,
}

impl Directional {
    pub fn new(name: String, position: cgmath::Vector3<f32>) -> Directional {
        Directional {
            name: name,
            position: position,
        }
    }
}

impl drawable::Drawable for Directional {
    fn draw(&self) -> Result<(),String> {
        Ok(())
    }
}