extern crate glium;
extern crate image;
extern crate nalgebra;
use PrimitiveShapes::Vertex;

pub enum Shape{
    Plane,
    Cube,
    Sphere(i32, i32),
    Model,
    Terrain
}

pub struct GameObject<'a>{
    translation_matrix: nalgebra::Matrix4<f32>,
    rotation_matrix: nalgebra::Matrix4<f32>,
    scale_matrix: nalgebra::Matrix4<f32>,
    pub transform : [[f32; 4]; 4],
    pub texture : &'a glium::Texture2d,
    pub program : &'a glium::Program,
    pub vertex_buffer : &'a glium::VertexBuffer<Vertex>

}

impl<'a> GameObject<'a>{
    pub fn new(model : Shape, tex : &'a glium::Texture2d, program : &'a glium::Program, vertex_buffer : &'a glium::VertexBuffer<Vertex>) -> GameObject<'a>{
        GameObject{
            translation_matrix : nalgebra::Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0),
            rotation_matrix : nalgebra::Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0),
            scale_matrix : nalgebra::Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0),
            transform : [[0.0, 0.0, 0.0, 0.0],[0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0]],
            texture : tex,
            program : program,
            vertex_buffer : vertex_buffer
        }
    }
    pub fn regenTerrain(&mut self,  vertex_buffer : &'a glium::VertexBuffer<Vertex>){
        self.vertex_buffer = vertex_buffer;
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