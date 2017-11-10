extern crate glium;
extern crate image;
extern crate nalgebra;

use Engine::Shape;
use Engine;
use ::Engine::{Vertex};

pub struct GameObject{
    translation_matrix: nalgebra::Matrix4<f32>,
    rotation_matrix: nalgebra::Matrix4<f32>,
    scale_matrix: nalgebra::Matrix4<f32>,
    pub transform : [[f32; 4]; 4],
    pub vertex_buffer : glium::VertexBuffer<Engine::Vertex>,
    pub program : glium::Program,
    pub texture : glium::texture::Texture2d
}

impl GameObject{
    pub fn new(model : Shape, shaderProgram : glium::Program, tex : glium::texture::Texture2d,  display : &glium::Display) -> GameObject{
        use Engine::Vertex;
        let vertex1 = Vertex { position: [-1.0, -1.0, -2.0], uv: [ 0.0, 1.0 ] };
        let vertex2 = Vertex { position: [ 1.0, -1.0, -2.0], uv: [ 1.0, 1.0 ] };
        let vertex3 = Vertex { position: [ -1.0, 1.0, -2.0], uv: [ 0.0, 0.0 ] };

        let vertex4 = Vertex { position: [1.0, 1.0, -2.0], uv: [ 1.0, 0.0] };
        let vertex5 = Vertex { position: [ -1.0, 1.0, -2.0], uv: [ 0.0, 0.0] };
        let vertex6 = Vertex { position: [ 1.0, -1.0, -2.0], uv: [ 1.0, 1.0 ] };
        let shape = vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6];

        let vertex_b = glium::VertexBuffer::new(display, &shape).unwrap();
        GameObject{
            translation_matrix : nalgebra::Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0),
            rotation_matrix : nalgebra::Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0),
            scale_matrix : nalgebra::Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0),
            transform : [[0.0, 0.0, 0.0, 0.0],[0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]],
            vertex_buffer : vertex_b,
            program : shaderProgram,
            texture : tex
        }
    }
    pub fn recalculateMatrix(&mut self){
        let transform = self.translation_matrix * self.rotation_matrix * self.scale_matrix;
        self.transform = transform.into();
    }
    pub fn translate(&mut self, dx : f32, dy : f32, dz : f32){
        self.translation_matrix[(0, 3)] += dx;
        self.translation_matrix[(1, 3)] += dy;
        self.translation_matrix[(2, 3)] += dz;
    }
}