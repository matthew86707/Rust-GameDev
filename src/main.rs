#[macro_use]
extern crate glium;
extern crate image;
extern crate nalgebra;
extern crate rand;

mod GameObject;
mod Camera;
mod PrimitiveShapes;
mod UIElement;

use glium::{glutin, Surface};
use std::io::Cursor;
use std::fs::File;
use std::io::prelude::*;

fn main() {

    use UIElement::UIElement;
    use PrimitiveShapes::Vertex;
    use GameObject::Shape;
    use GameObject::GameObject;
  
    let mut program_counter : f32 = 0.0;
    let mut glow_effect_multiplier : f32 = 0.0;

    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new();
    let context = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    {

    let screen_size = display.get_framebuffer_dimensions();
    let mut closed = false;

	let texture = load_texture("grass.jpg", &display);
    let snow_texture = load_texture("Snow.jpg", &display);
    let texture_rock = load_texture("rock.jpg", &display);

	implement_vertex!(Vertex, position, uv, normal);

	let shape_terrain = PrimitiveShapes::get_plane(128, 128);
	let vertex_buffer_terrain = glium::VertexBuffer::new(&display, &shape_terrain).unwrap();
	let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);


	let program = create_shader_program("shaders/vertex.glsl", "shaders/fragment.glsl", &display);
    let program_UI = create_shader_program("shaders/vertex_ui.glsl", "shaders/fragment_ui.glsl", &display);


    let mut mainCam : Camera::Camera = Camera::Camera::new();
    let projection_matrix: nalgebra::Matrix4<f32> = mainCam.create_projection_matrix(95.0, screen_size);

    {

    let mut Selectedgame_objects : Vec<GameObject> = Vec::new();
    let mut game_objects : Vec<GameObject> = Vec::new();

    let mut test_object : GameObject = GameObject::new(Shape::Plane, &texture, &program, &vertex_buffer_terrain);

    let mut mx : f64 = 0.0;
    let mut my : f64 = 0.0;
    let mut dx : f64 = 0.0;
    let mut dy : f64 = 0.0;

    let mut draw_params : glium::draw_parameters::DrawParameters = Default::default();
    draw_params.polygon_mode = glium::draw_parameters::PolygonMode::Fill;
    draw_params.blend =  glium::Blend::alpha_blending();
    draw_params.depth = glium::Depth {
         test: glium::draw_parameters::DepthTest::IfLess,
               write: true,
                .. Default::default()
   };

   let mut should_spawn : bool = false;

    while !closed {
        program_counter += 0.00005;
        glow_effect_multiplier = (1.57 + f32::sin(program_counter) / 2.0);

        let mut target = display.draw();
        target.clear_color_and_depth((0.25, 0.45, 1.0, 1.0), 1.0);

        let projection_matrix: [[f32; 4]; 4] = projection_matrix.into();

        if should_spawn {
            Selectedgame_objects.push(GameObject::new(Shape::Plane, &texture, &program, &vertex_buffer_terrain));
            should_spawn = false;
        }

        for gameObject in &mut game_objects{
            gameObject.recalculateMatrix();
            target.draw(gameObject.vertex_buffer, &indices, gameObject.program, &uniform! { time : program_counter, sampler: gameObject.texture, snowSampler : &snow_texture,rockSampler : &texture_rock, transform: gameObject.transform, projection_matrix: projection_matrix, view_matrix : mainCam.get_view_matrix(), glowEffect : 1.0 as f32},
            &draw_params).unwrap();

        }

        for gameObject in &mut Selectedgame_objects{
            gameObject.recalculateMatrix();
            target.draw(gameObject.vertex_buffer, &indices, gameObject.program, &uniform! { time : program_counter, sampler: gameObject.texture , snowSampler : &snow_texture, rockSampler : &texture_rock, transform: gameObject.transform, projection_matrix: projection_matrix, view_matrix : mainCam.get_view_matrix(), glowEffect : glow_effect_multiplier},
            &draw_params).unwrap();
        }

        
        target.finish().unwrap();

        events_loop.poll_events(|ev| {
            
            match ev {

                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::MouseMoved{position, ..} => {
                    dx = mx - position.0;
                    dy = my - position.1;
                    mx = position.0;
                    my = position.1;
                    mainCam.rotate(nalgebra::Vector3::new(0.0, 0.0, dx as f32 / 3.0));
                    mainCam.rotate(nalgebra::Vector3::new(0.0, -(dy as f32 / 3.0), 0.0));
                },
                	glutin::WindowEvent::Closed => closed = true,
                	glutin::WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                		Some(glutin::VirtualKeyCode::Escape) => closed = true,
                        Some(glutin::VirtualKeyCode::Right) => {match Selectedgame_objects.get_mut(0) { 
                            Some(mut obj) => obj.translate(-0.1, 0.0, 0.0),
                            _ => ()
                        }},
                        Some(glutin::VirtualKeyCode::Left) => {match Selectedgame_objects.get_mut(0) { 
                            Some(mut obj) => obj.translate(0.1, 0.0, 0.0),
                            _ => ()
                        }},
                        Some(glutin::VirtualKeyCode::Up) => {match Selectedgame_objects.get_mut(0) { 
                            Some(mut obj) => obj.translate(0.0, 0.0, -0.1),
                            _ => ()
                        }},
                        Some(glutin::VirtualKeyCode::Down) => {match Selectedgame_objects.get_mut(0) { 
                            Some(mut obj) => obj.translate(0.0, 0.0, 0.1),
                            _ => ()
                        }},
                        Some(glutin::VirtualKeyCode::P) => {should_spawn = true;},
                        Some(glutin::VirtualKeyCode::O) => {should_spawn = true;},
                        Some(glutin::VirtualKeyCode::Return) => {
                            let mut left_over : Vec<GameObject> = Selectedgame_objects.drain(0..).collect();
                            game_objects.extend(left_over);
                        },
                        Some(glutin::VirtualKeyCode::Z) => {
                            if draw_params.polygon_mode == glium::draw_parameters::PolygonMode::Line{
                                draw_params.polygon_mode = glium::draw_parameters::PolygonMode::Fill;
                            }else{
                                 draw_params.polygon_mode = glium::draw_parameters::PolygonMode::Line;
                            }
                        },
                        Some(glutin::VirtualKeyCode::W) => mainCam.translate(nalgebra::Vector3::new(0.0, 0.0, 0.25)),
                        Some(glutin::VirtualKeyCode::S) => mainCam.translate(nalgebra::Vector3::new(0.0, 0.0, -0.25)),
                        Some(glutin::VirtualKeyCode::A) => mainCam.translate(nalgebra::Vector3::new(-0.25, 0.0, 0.0)),
                        Some(glutin::VirtualKeyCode::D) => mainCam.translate(nalgebra::Vector3::new(0.25, 0.0, 0.0)),
                        Some(glutin::VirtualKeyCode::Q) => mainCam.translate(nalgebra::Vector3::new(0.25, 0.25, 0.0)),
                        Some(glutin::VirtualKeyCode::E) => mainCam.translate(nalgebra::Vector3::new(0.25, -0.25, 0.0)),
                		_ => ()
                	},
                	_ => ()
                },
                _ => ()
            
        }
        });
    }
}
}
}

pub fn load_texture(location : &str, display : &glium::Display) -> glium::Texture2d{
    use std::io::Cursor;
    use std::fs::File;
    use std::io::prelude::*;

    let mut bytes_rock: Vec<u8> = Vec::new();
    let mut file_rock = File::open(location).expect("file not found");
    file_rock.read_to_end(&mut bytes_rock).expect("something went wrong reading the file");

    
    let image_rock = image::load(Cursor::new(&bytes_rock), image::JPEG).unwrap().to_rgba();
    let image_dimensions_rock = image_rock.dimensions();
    let image_rock = glium::texture::RawImage2d::from_raw_rgba_reversed(&image_rock.into_raw()[..], image_dimensions_rock);
    let texture_rock = glium::texture::Texture2d::new(display, image_rock).unwrap();
    return texture_rock;
}

fn create_shader_program(vertex_shader_path : &str, fragment_shader_path : &str, display : &glium::Display) -> glium::Program{
    let mut vertex_shader_src = String::new();
    let mut fragment_shader_src = String::new();

    let mut file = File::open(vertex_shader_path).expect("file not found");
    file.read_to_string(&mut vertex_shader_src).expect("something went wrong reading the file");

    let mut file = File::open(fragment_shader_path).expect("file not found");
    file.read_to_string(&mut fragment_shader_src).expect("something went wrong reading the file");

    let program = glium::Program::from_source(display, &vertex_shader_src, &fragment_shader_src, None).unwrap();
    return program;
}


