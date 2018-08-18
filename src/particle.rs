extern crate glium;
extern crate image;
extern crate nalgebra;

pub struct Particle{
    pub x : f32,
    pub y : f32,
    pub mass : f32
}

impl Particle{
    pub fn new(x : f32, y : f32, mass : f32) -> Particle{
        Particle{
            x : x,
            y : y,
            mass : mass
        }
    }
    pub fn translate(&mut self, dx : f32, dy : f32){
        self.x += dx;
        self.y += dy;
    }
}