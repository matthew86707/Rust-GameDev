extern crate glium;
extern crate image;
extern crate nalgebra;

pub struct UIElement<'a>{
	pub texture :  &'a glium::Texture2d,
	pub x_pos : f32,
	pub y_pos : f32,
	pub x_size : f32,
	pub y_size : f32,
	translation_matrix: nalgebra::Matrix4<f32>,
    pub transform : [[f32; 4]; 4]
}

impl<'a> UIElement<'a>{
	pub fn new(tex : &'a glium::Texture2d, x_pos : f32, y_pos : f32, x_size : f32, y_size : f32) -> UIElement<'a>{
		UIElement{
			texture : tex,
			x_pos : x_pos,
			y_pos : y_pos,
			x_size : x_size,
			y_size : y_size,
			translation_matrix :nalgebra::Matrix4::new(1.0, 0.0, 0.0, x_pos, 0.0, 1.0, 0.0, y_pos, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0),
			transform : nalgebra::Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0).into() 
		}
	}
	pub fn recalculateMatrix(&mut self){
        self.transform = self.translation_matrix.into();
    }
    pub fn translate(&mut self, dx : f32, dy : f32, dz : f32){
        self.translation_matrix[(0, 3)] += dx;
        self.translation_matrix[(1, 3)] += dy;
        self.translation_matrix[(2, 3)] += dz;
        self.recalculateMatrix();
    }
}