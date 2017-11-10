
extern crate glium;
extern crate image;
extern crate nalgebra;

const NEAR_PLANE: f32 = 0.001;
const FAR_PLANE : f32 = 1000.0;

use GameObject::GameObject;
 use glium::{glutin, Surface};

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
}

pub enum Shape{
    Plane,
    Cube,
    Sphere(i32, i32),
    Model
}

pub struct Engine{
	game_objects : Vec<GameObject>,
  display : glium::Display, 
  projection_matrix : [[f32; 4]; 4],
  events_loop : glium::glutin::EventsLoop
}

impl Engine{
	pub fn new() -> Engine{

    implement_vertex!(Vertex, position, uv);

    use glium::{glutin, Surface};

    let mut events_l = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new();
    let context = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_l).unwrap();

    let screen_size = display.get_framebuffer_dimensions();

    let projection_m: nalgebra::Matrix4<f32> = create_projection_matrix(85.0, screen_size);
    let projection_m: [[f32; 4]; 4] = projection_m.into();

		Engine{
			game_objects : Vec::new(),
      display,
      projection_matrix : projection_m,
      events_loop : events_l
		}
	}
  pub fn init(&mut self){

    

  }

  pub fn startMainLoop(&mut self){
    //self.init();

    use std::io::Cursor;
    use std::fs::File;
    use std::io::prelude::*;

    let mut bytes: Vec<u8> = Vec::new();
    let mut file = File::open("rust_logo.jpg").expect("file not found");
    file.read_to_end(&mut bytes).expect("something went wrong reading the file");

    let image = image::load(Cursor::new(&bytes), image::JPEG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw()[..], image_dimensions);
    let texture = glium::texture::Texture2d::new(&self.display, image).unwrap();

    let mut vertex_shader_src = String::new();
    let mut fragment_shader_src = String::new();

     let mut file = File::open("shaders/vertex.glsl").expect("file not found");
    file.read_to_string(&mut vertex_shader_src).expect("something went wrong reading the file");

    let mut file = File::open("shaders/fragment.glsl").expect("file not found");
    file.read_to_string(&mut fragment_shader_src).expect("something went wrong reading the file");

    let program = glium::Program::from_source(&self.display, &vertex_shader_src, &fragment_shader_src, None).unwrap();


    let mut translation: nalgebra::Vector3<f32> = nalgebra::Vector3::new(0.0, 0.0, 0.0);
    let mut rotation_z: f32 = 0.0;
    let mut rotation_y: f32 = 0.0;
    let mut scale: nalgebra::Vector3<f32> = nalgebra::Vector3::new(1.0, 1.0, 1.0);

    let mut mx : f64 = 0.0;
    let mut my : f64 = 0.0;
    let mut dx : f64 = 0.0;
    let mut dy : f64 = 0.0;

    let mut closed = false;

    use std::clone::{Clone};

    let display : &glium::Display = &self.display;

    self.registerGameObject(GameObject::new(Shape::Plane, 
                program, texture, &display));

    while !closed {

        self.renderGameObjects();      

        //let mut myObj : GameObject = GameObject::new(Shape::Plane, program, texture, &self.display); 

        let mut shouldSpawn : bool = false;

        self.events_loop.poll_events(|ev| {
            
                
            match ev {

                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::MouseMoved{position, ..} => {
                    dx = mx - position.0;
                    dy = my - position.1;
                    mx = position.0;
                    my = position.1;
                    rotation_z += (dx as f32) * (0.05);
                    rotation_y += (dy as f32) * (0.05);
                },
                  glutin::WindowEvent::Closed => closed = true,
                  glutin::WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                    Some(glutin::VirtualKeyCode::Escape) => closed = true,
                    Some(glutin::VirtualKeyCode::N) => {shouldSpawn = true;}
                    Some(glutin::VirtualKeyCode::Left) => {},
                    Some(glutin::VirtualKeyCode::Left) => {},
                    Some(glutin::VirtualKeyCode::Right) => translation[0] += 0.1,
                    Some(glutin::VirtualKeyCode::Up) => translation[1] += 0.1,
                    Some(glutin::VirtualKeyCode::Down) => translation[1] -= 0.1,
                    Some(glutin::VirtualKeyCode::Space) => { scale[0] += 0.1; scale[1] += 0.1; },
                    Some(glutin::VirtualKeyCode::C) => { scale[0] -= 0.1; scale[1] -= 0.1; },
                    Some(glutin::VirtualKeyCode::A) => rotation_z += 0.1,
                    Some(glutin::VirtualKeyCode::D) => rotation_z -= 0.1,
                    _ => ()
                  },
                  _ => ()
                },
                _ => ()
            }
           
        });
         if(shouldSpawn){
             // self.registerGameObject(GameObject::new(Shape::Plane, 
             //   program, texture, &self.display));
            }
    }
  }
	pub fn registerGameObject(&mut self, object : GameObject){
		self.game_objects.push(object);
	}
	pub fn renderGameObjects(&mut self){

    let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

	    let mut translation_matrix: nalgebra::Matrix4<f32> = nalgebra::Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
      let mut rotation_matrix: nalgebra::Matrix4<f32> = nalgebra::Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
      let mut scale_matrix: nalgebra::Matrix4<f32> = nalgebra::Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);

        let transform = translation_matrix * rotation_matrix * scale_matrix;
        let transform: [[f32; 4]; 4] = transform.into();

		for g_object in &mut self.game_objects{
			g_object.recalculateMatrix();
        	target.draw(&g_object.vertex_buffer, &indices, &g_object.program, &uniform! { sampler: &g_object.texture, transform: g_object.transform, projection_matrix: self.projection_matrix, view_matrix : transform },
       		 &Default::default()).unwrap();
		}
    target.finish().unwrap();
	}
}
fn create_projection_matrix(fov: f32, screen_size: (u32, u32)) -> nalgebra::Matrix4<f32> {
    let aspect_ratio: f32 = screen_size.0 as f32 / screen_size.1 as f32;
    let y_scale = (1.0 / f32::tan(f32::to_radians(fov / 2.0))) * aspect_ratio;
    let x_scale = y_scale / aspect_ratio;
    let frustum_length = FAR_PLANE - NEAR_PLANE;

    let mut matrix: nalgebra::Matrix4<f32> = nalgebra::Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    matrix[(0, 0)] = x_scale;
    matrix[(1, 1)] = y_scale;
    matrix[(2, 2)] = -((FAR_PLANE + NEAR_PLANE) / frustum_length);
    matrix[(3, 2)] = -1.0;
    matrix[(2, 3)] = -((2.0 * NEAR_PLANE * FAR_PLANE) / frustum_length);
    matrix[(3, 3)] = 0.0;

    matrix
}




