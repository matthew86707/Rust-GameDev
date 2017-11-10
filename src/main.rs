#[macro_use]

extern crate glium;
extern crate image;
extern crate nalgebra;

mod Camera;
mod GameObject;
mod Engine;
mod Input;

fn main() {

    let mut mainEngine : Engine::Engine = Engine::Engine::new();
    mainEngine.startMainLoop();
}

