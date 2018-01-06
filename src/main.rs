#[macro_use]
extern crate glium;
extern crate image;
extern crate nalgebra;
extern crate rand;
extern crate noise;

mod GameObject;
mod Camera;
mod PrimitiveShapes;
mod UIElement;

use glium::{glutin, Surface};
use std::io::Cursor;
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpStream;
 use std::io::prelude::*;
    use std::io::BufWriter;
    use std::io::BufReader;

fn main() {

    use UIElement::UIElement;
    use PrimitiveShapes::Vertex;
    use GameObject::Shape;
    use GameObject::GameObject;
  
    let mut program_counter : f32 = 0.0;
    let mut glow_effect_multiplier : f32 = 0.0;
    let mut shading_intensity : f32 = 1.0;

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
    let water_texture = load_texture("water.jpg", &display);

	implement_vertex!(Vertex, position, uv, normal);


    let mut stream = TcpStream::connect("localhost:4242").unwrap();
    let mut world_seed : i32;
    {
        let mut reader = BufReader::new(&stream);
        let mut line = String::new();
        reader.read_line(&mut line);

        println!("World Seed From Server : {}", line);
        line.pop();
        world_seed = line.parse::<i32>().unwrap();
    }

	let shape_terrain = PrimitiveShapes::get_plane(512, 512, world_seed);
	let vertex_buffer_terrain = glium::VertexBuffer::new(&display, &shape_terrain).unwrap();
	let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex1 = Vertex { position: [-1.0, -1.0, -2.0], uv: [ 0.0, 1.0 ], normal: [0.0, 0.0, 0.0] };
    let vertex2 = Vertex { position: [ 1.0, -1.0, -2.0], uv: [ 1.0, 1.0 ], normal: [0.0, 0.0, 0.0] };
    let vertex3 = Vertex { position: [ -1.0, 1.0, -2.0], uv: [ 0.0, 0.0 ], normal: [0.0, 0.0, 0.0] };

    let vertex4 = Vertex { position: [1.0, 1.0, -2.0], uv: [ 1.0, 0.0], normal: [0.0, 0.0, 0.0] };
    let vertex5 = Vertex { position: [ -1.0, 1.0, -2.0], uv: [ 0.0, 0.0], normal: [0.0, 0.0, 0.0] };
    let vertex6 = Vertex { position: [ 1.0, -1.0, -2.0], uv: [ 1.0, 1.0 ], normal: [0.0, 0.0, 0.0] };

    let vertex_buffer_player = glium::VertexBuffer::new(&display, &vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6]).unwrap();

    let mut scale_a : f32 = 500.0;
    let mut scale_b : f32 = 1.0;

    let vertex1 = Vertex { position: [1.0 * scale_a, 2.0 * scale_b, -1.0 * scale_a], uv: [ 0.0, 1.0 ], normal: [0.0, 0.0, 0.0] };
    let vertex2 = Vertex { position: [ 1.0 * scale_a, 2.0 * scale_b, 1.0 * scale_a], uv: [ 1.0, 1.0 ], normal: [0.0, 0.0, 0.0] };
    let vertex3 = Vertex { position: [ -1.0 * scale_a, 2.0 * scale_b, -1.0 * scale_a], uv: [ 0.0, 0.0 ], normal: [0.0, 0.0, 0.0] };

    let vertex4 = Vertex { position: [-1.0 * scale_a, 2.0 * scale_b, 1.0 * scale_a], uv: [ 1.0, 0.0], normal: [0.0, 0.0, 0.0] };
    let vertex5 = Vertex { position: [ -1.0 * scale_a, 2.0 * scale_b, -1.0 * scale_a], uv: [ 0.0, 0.0], normal: [0.0, 0.0, 0.0] };
    let vertex6 = Vertex { position: [ 1.0 * scale_a, 2.0 * scale_b, 1.0 * scale_a], uv: [ 1.0, 1.0 ], normal: [0.0, 0.0, 0.0] };

    let vertex_buffer_water = glium::VertexBuffer::new(&display, &vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6]).unwrap();


	let program = create_shader_program("shaders/vertex.glsl", "shaders/fragment.glsl", &display);
    let program_water = create_shader_program("shaders/vertex_water.glsl", "shaders/fragment_water.glsl", &display);
    let program_player = create_shader_program("shaders/vertex_player.glsl", "shaders/fragment_player.glsl", &display);
    let program_UI = create_shader_program("shaders/vertex_ui.glsl", "shaders/fragment_ui.glsl", &display);


    let mut mainCam : Camera::Camera = Camera::Camera::new();
    let projection_matrix: nalgebra::Matrix4<f32> = mainCam.create_projection_matrix(95.0, screen_size);

    {

    let mut game_objects : Vec<GameObject> = Vec::new();

    let mut test_object : GameObject = GameObject::new(Shape::Plane, &texture, &program, &vertex_buffer_terrain);

    let mut water : GameObject = GameObject::new(Shape::Plane, &water_texture, &program_water, &vertex_buffer_water);

    let mut player_objects : Vec<GameObject> = Vec::new();

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

   water.set_position(500.0, 0.0, 500.0);

    while !closed {

        //Handle networking

        {
        //Format player position
        let position_x_string = mainCam.position[0].to_string();
        let position_y_string = mainCam.position[1].to_string();
        let position_z_string = mainCam.position[2].to_string();
        let to_send_string = format!("{}:{}:{}\n", position_x_string, position_y_string, position_z_string);

        //Send player location as TCP packet
        let _ = stream.write(to_send_string.as_bytes());
        stream.flush();
        }

        //Read player data
        {
        let mut reader = BufReader::new(&stream);
        let mut line = String::new();
        reader.read_line(&mut line);

        line.pop();

        let mut data : Vec<&str> = Vec::new(); 
        data = line.split(":").collect::<Vec<&str>>();;

        if(player_objects.len() < (data.len() / 4)){
            println!("Adding Player...");
            for i in  0..(((data.len() / 4) - player_objects.len()) + 1){
            player_objects.push(GameObject::new(Shape::Plane, &texture, &program_player, &vertex_buffer_player));
        }
        }

        for i in 0..(data.len() / 4)  {
            let mut player = &mut player_objects.get_mut(i).unwrap();
            player.set_position(data.get((i * 4) + 1).unwrap().parse::<f32>().unwrap(), data.get((i * 4) + 2).unwrap().parse::<f32>().unwrap(), data.get((i * 4) + 3).unwrap().parse::<f32>().unwrap());
        }

        }
        program_counter += 0.00005;
        glow_effect_multiplier = (1.57 + f32::sin(program_counter) / 2.0);

        let mut target = display.draw();
        target.clear_color_and_depth((0.25, 0.45, 1.0, 1.0), 1.0);

        let projection_matrix: [[f32; 4]; 4] = projection_matrix.into();

        if should_spawn {
            game_objects.push(GameObject::new(Shape::Plane, &texture, &program, &vertex_buffer_terrain));
            should_spawn = false;
        }

        for player in &mut player_objects{
            player.recalculateMatrix();
            target.draw(player.vertex_buffer, &indices, player.program, &uniform! {time : program_counter, transform: player.transform, projection_matrix: projection_matrix, view_matrix : mainCam.get_view_matrix()},
            &draw_params).unwrap();
        }

        for gameObject in &mut game_objects{
            gameObject.recalculateMatrix();
            target.draw(gameObject.vertex_buffer, &indices, gameObject.program, &uniform! {shading_intensity : shading_intensity, time : program_counter, sampler: gameObject.texture, snowSampler : &snow_texture,rockSampler : &texture_rock, transform: gameObject.transform, projection_matrix: projection_matrix, view_matrix : mainCam.get_view_matrix(), glowEffect : 1.0 as f32},
            &draw_params).unwrap();

        }

        water.recalculateMatrix();
        target.draw(water.vertex_buffer, &indices, water.program, &uniform! {sampler: water.texture, transform: water.transform, projection_matrix: projection_matrix, view_matrix : mainCam.get_view_matrix()},
            &draw_params).unwrap();
       
        target.finish().unwrap();

        events_loop.poll_events(|ev| {
            
            match ev {

                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CursorMoved{position, ..} => {
                    dx = mx - position.0;
                    dy = my - position.1;
                    mx = position.0;
                    my = position.1;
                    mainCam.rotate(nalgebra::Vector3::new(0.0, 0.0, (dx as f32 / 3.0)));
                    mainCam.rotate(nalgebra::Vector3::new(0.0, (dy as f32 / 3.0), 0.0));
                },
                	glutin::WindowEvent::Closed => closed = true,
                	glutin::WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                		Some(glutin::VirtualKeyCode::Escape) => closed = true,

                        Some(glutin::VirtualKeyCode::P) => {should_spawn = true;},
                        Some(glutin::VirtualKeyCode::O) => {should_spawn = true;},
                        Some(glutin::VirtualKeyCode::Z) => {
                            if draw_params.polygon_mode == glium::draw_parameters::PolygonMode::Line{
                                draw_params.polygon_mode = glium::draw_parameters::PolygonMode::Fill;
                            }else{
                                 draw_params.polygon_mode = glium::draw_parameters::PolygonMode::Line;
                            }
                        },
                        Some(glutin::VirtualKeyCode::W) => {
                                                                let forward_vec = mainCam.forward();
                                                                mainCam.translate(forward_vec * 1.5)
                                                            },
                        Some(glutin::VirtualKeyCode::S) => {
                                                                let forward_vec = -mainCam.forward();
                                                                mainCam.translate(forward_vec * 1.5)
                                                            },
                        Some(glutin::VirtualKeyCode::D) => {
                                                                let right_vec = mainCam.right();
                                                                mainCam.translate(right_vec * 1.5)
                                                            },
                        Some(glutin::VirtualKeyCode::A) => {
                                                                let right_vec = -mainCam.right();
                                                                mainCam.translate(right_vec * 1.5)
                                                            },
                        Some(glutin::VirtualKeyCode::Q) => mainCam.translate(nalgebra::Vector3::new(0.75, 0.75, 0.0)),
                        Some(glutin::VirtualKeyCode::E) => mainCam.translate(nalgebra::Vector3::new(0.75, -0.75, 0.0)),
                        Some(glutin::VirtualKeyCode::X) => {shading_intensity = 0.0},
                        Some(glutin::VirtualKeyCode::C) => {shading_intensity = 1.0},
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


