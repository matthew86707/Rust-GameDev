#[macro_use]
extern crate glium;
extern crate image;
extern crate nalgebra;
extern crate rand;
extern crate noise;
extern crate ncollide;
extern crate glium_text;

mod GameObject;
mod Camera;
mod PrimitiveShapes;
mod UIElement;
mod Quaternion;

use glium::{glutin, Surface};
use glium::backend::Facade;
use glium::Display;

use std::io::Cursor;
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpStream;
use std::io::prelude::*;
use std::io::BufWriter;
use std::io::BufReader;
use glium::texture::cubemap::{Cubemap};
use glium::framebuffer::{SimpleFrameBuffer};
use PrimitiveShapes::Vertex;
use ncollide::shape::Triangle;
use ncollide::shape::Triangle3;
use ncollide::query::Ray;

fn main() {

    use UIElement::UIElement;
    
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
    let texture_skybox = load_texture("nebula.jpg", &display);
    let snow_texture = load_texture("Snow.jpg", &display);
    let texture_rock = load_texture("rock.jpg", &display);
    let water_texture = load_texture("water.jpg", &display);

    let mut cubemap = texture_to_cubemap(&texture_skybox, &display);

	implement_vertex!(Vertex, position, uv, normal);

    let mut world_seed : i32 = 8;

    let mut collisionTriangles : Vec<Triangle3<f32>> = Vec::new();

   // let mut stream = TcpStream::connect("localhost:4242").unwrap();
   //  {
   //      let mut reader = BufReader::new(&stream);
   //      let mut line = String::new();
   //      reader.read_line(&mut line);

   //      println!("World Seed From Server : {}", line);
   //      line.pop();
   //      world_seed = line.parse::<i32>().unwrap();
   //  }

	//let shape_terrain = PrimitiveShapes::get_plane(512, 512, world_seed);
    let shape_terrain = PrimitiveShapes::get_sphere(64, 64, true, true, &mut collisionTriangles);
    let shape_water = PrimitiveShapes::get_sphere(64, 64, false, false, &mut collisionTriangles);
	let vertex_buffer_terrain = glium::VertexBuffer::new(&display, &shape_terrain).unwrap();
    let vertex_buffer_water = glium::VertexBuffer::new(&display, &shape_water).unwrap();
	let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_buffer_skybox = get_cube_vertex_buffer(&display);
    let indices_skybox = get_index_buffer(&display);

    let vertex1 = Vertex { position: [-10.0, -10.0, -2.0], uv: [ 0.0, 1.0 ], normal: [0.0, 0.0, 0.0] };
    let vertex2 = Vertex { position: [ 1.0, -10.0, -2.0], uv: [ 1.0, 1.0 ], normal: [0.0, 0.0, 0.0] };
    let vertex3 = Vertex { position: [ -10.0, 1.0, -2.0], uv: [ 0.0, 0.0 ], normal: [0.0, 0.0, 0.0] };

    let vertex4 = Vertex { position: [1.0, 1.0, -2.0], uv: [ 1.0, 0.0], normal: [0.0, 0.0, 0.0] };
    let vertex5 = Vertex { position: [ -10.0, 1.0, -2.0], uv: [ 0.0, 0.0], normal: [0.0, 0.0, 0.0] };
    let vertex6 = Vertex { position: [ 1.0, -10.0, -2.0], uv: [ 1.0, 1.0 ], normal: [0.0, 0.0, 0.0] };

    let vertex_buffer_player = glium::VertexBuffer::new(&display, &vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6]).unwrap();

    let aspect_ratio : f32 = display.get_framebuffer_dimensions().0 as f32 / display.get_framebuffer_dimensions().1 as f32;

    let vertex1 = Vertex { position: [-1.0, -1.0, -0.5], uv: [ -aspect_ratio, 1.0 ], normal: [0.0, 0.0, 0.0] };
    let vertex2 = Vertex { position: [ 1.0, -1.0, -0.5], uv: [ aspect_ratio, 1.0 ], normal: [0.0, 0.0, 0.0] };
    let vertex3 = Vertex { position: [ -1.0, 1.0, -0.5], uv: [ -aspect_ratio, -1.0 ], normal: [0.0, 0.0, 0.0] };

    let vertex4 = Vertex { position: [1.0, 1.0, -0.5], uv: [ aspect_ratio, -1.0], normal: [0.0, 0.0, 0.0] };
    let vertex5 = Vertex { position: [ -1.0, 1.0, -0.5], uv: [ -aspect_ratio, -1.0], normal: [0.0, 0.0, 0.0] };
    let vertex6 = Vertex { position: [ 1.0, -1.0, -0.5], uv: [ aspect_ratio, 1.0 ], normal: [0.0, 0.0, 0.0] };

    let vertex_buffer_profiler = glium::VertexBuffer::new(&display, &vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6]).unwrap();

    let mut scale_a : f32 = 500.0;
    let mut scale_b : f32 = 1.0;

    let vertex1 = Vertex { position: [1.0 * scale_a, 2.0 * scale_b, -1.0 * scale_a], uv: [ 0.0, 1.0 ], normal: [0.0, 0.0, 0.0] };
    let vertex2 = Vertex { position: [ 1.0 * scale_a, 2.0 * scale_b, 1.0 * scale_a], uv: [ 1.0, 1.0 ], normal: [0.0, 0.0, 0.0] };
    let vertex3 = Vertex { position: [ -1.0 * scale_a, 2.0 * scale_b, -1.0 * scale_a], uv: [ 0.0, 0.0 ], normal: [0.0, 0.0, 0.0] };

    let vertex4 = Vertex { position: [-1.0 * scale_a, 2.0 * scale_b, 1.0 * scale_a], uv: [ 1.0, 0.0], normal: [0.0, 0.0, 0.0] };
    let vertex5 = Vertex { position: [ -1.0 * scale_a, 2.0 * scale_b, -1.0 * scale_a], uv: [ 0.0, 0.0], normal: [0.0, 0.0, 0.0] };
    let vertex6 = Vertex { position: [ 1.0 * scale_a, 2.0 * scale_b, 1.0 * scale_a], uv: [ 1.0, 1.0 ], normal: [0.0, 0.0, 0.0] };

    //let vertex_buffer_water = glium::VertexBuffer::new(&display, &vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6]).unwrap();

	let program = create_shader_program("shaders/vertex.glsl", "shaders/fragment.glsl", &display);
    let program_water = create_shader_program("shaders/vertex_water.glsl", "shaders/fragment_water.glsl", &display);
    let program_player = create_shader_program("shaders/vertex_player.glsl", "shaders/fragment_player.glsl", &display);
    let program_UI = create_shader_program("shaders/vertex_ui.glsl", "shaders/fragment_ui.glsl", &display);
    let program_profiler = create_shader_program("shaders/vertex_profiler.glsl", "shaders/fragment_profiler.glsl", &display);
    let program_skybox = create_shader_program("shaders/vertex_skybox.glsl", "shaders/fragment_skybox.glsl", &display);


    let mut mainCam : Camera::Camera = Camera::Camera::new();
    let projection_matrix: nalgebra::Matrix4<f32> = mainCam.create_projection_matrix(95.0, screen_size);

    {

    let mut game_objects : Vec<GameObject> = Vec::new();

    let mut test_object : GameObject = GameObject::new(Shape::Plane, &texture, &program, &vertex_buffer_terrain);

    let mut water : GameObject = GameObject::new(Shape::Plane, &water_texture, &program_water, &vertex_buffer_water);

    let mut profiler : GameObject = GameObject::new(Shape::Plane, &water_texture, &program_profiler, &vertex_buffer_profiler);

    let mut player_objects : Vec<GameObject> = Vec::new();

    let mut debug_ping_object : GameObject = GameObject::new(Shape::Plane, &texture, &program_player, &vertex_buffer_player);

    use glium::uniforms::SamplerWrapFunction::Clamp;

    let skybox_sampled = cubemap.sampled().wrap_function(glium::uniforms::SamplerWrapFunction::Clamp);

    let mut mx : f64 = 0.0;
    let mut my : f64 = 0.0;
    let mut dx : f64 = 0.0;
    let mut dy : f64 = 0.0;
    let mut mouseScroll : f32 = 0.0;

    let mut draw_params : glium::draw_parameters::DrawParameters = Default::default();
    draw_params.polygon_mode = glium::draw_parameters::PolygonMode::Fill;
    draw_params.blend =  glium::Blend::alpha_blending();
    draw_params.depth = glium::Depth {
         test: glium::draw_parameters::DepthTest::IfLess,
               write: true,
                .. Default::default()
   };

   let mut should_spawn : bool = false;

   let mut light_y : f32 = 0.0;
   
  
   use ncollide::query::RayCast;
   use nalgebra::geometry::Rotation;
   use ncollide::math::Isometry;
   use nalgebra::geometry::Rotation3;
   use nalgebra::core::Vector3;
   use nalgebra::geometry::Point3;
   use ncollide::query::RayIntersection;
   use ncollide::query::RayIntersection3;
   
   let mut intersection_counter : i32 = 0;
   let mut counter : i32 = 0;

   let mut glow_position : [f32; 3] = [0.0, 0.0, 0.0];

   mainCam.set_rotation_scale(5.0);

   let mut light_pos : [f32; 3] = [0.0, 135.0, -5.0];
   let light_radius : f32 = 100.0;
    glow_effect_multiplier = 0.0;

    use std::time::{Instant, Duration};

    let mut delta_time : f64;

    let mut just_past = Instant::now();

    // The `TextSystem` contains the shaders and elements used for text display.
    let system = glium_text::TextSystem::new(&display);

    // Creating a `FontTexture`, which a regular `Texture` which contains the font.
    // Note that loading the systems fonts is not covered by this library.
    let font = glium_text::FontTexture::new(&display, std::fs::File::open(&std::path::Path::new("NotoSans-Regular.ttf")).unwrap(), 64).unwrap();

    // Creating a `TextDisplay` which contains the elements required to draw a specific sentence.
    let text = glium_text::TextDisplay::new(&system, &font, "Hello world!");

    // Finally, drawing the text is done like this:
    let matrix = [[0.2, 0.0, 0.0, 0.0],
              [0.0, 0.2, 0.0, 0.0],
              [0.0, 0.0, 1.0, 0.0],
              [-0.9, 0.0, 0.0, 1.0]];

    let mut frames_past : i32 = 0;
    let mut time_elapsed : f32 = 0.0;
    
    while !closed {
        //Time control code
        //let mut 
        let mut new_now = Instant::now();
        let delta_duration = new_now.duration_since(just_past);
        just_past = new_now;

        delta_time = delta_duration.as_secs() as f64 + (delta_duration.subsec_nanos() as f64 * 10e-9);
        //println!("Delta Time Nano {}", delta_duration.subsec_nanos());

        let fps = 1.0 / delta_time;

        let text = glium_text::TextDisplay::new(&system, &font, &fps.to_string());

        counter = counter + 1;
        let mut vision_ray = Ray::<Point3<f32>> {
            origin : Point3::from_coordinates(mainCam.position),
            dir : mainCam.forward()
        };

        light_pos = [f32::sin(program_counter) * light_radius, f32::cos(program_counter) * light_radius, -5.0];


        //Handle networking

        {
        //println!("Getting light pos");
        //Format player position
        let position_x_string = glow_position[0].to_string();
        let position_y_string = glow_position[1].to_string();
        let position_z_string = glow_position[2].to_string();
        let to_send_string = format!("{}:{}:{}\n", position_x_string, position_y_string, position_z_string);

        //Send light location as TCP packet
        if(glow_effect_multiplier > 0.0){
        //println!("{}", to_send_string);
       
        // let _ = stream.write(to_send_string.as_bytes());
        // stream.flush();
        // }else{
        //     let failed_string = "NA\n".to_string();
        //     println!("{}", failed_string);
        //     let _ = stream.write(failed_string.as_bytes());
        //     stream.flush();
        // }

        }

        //Read player data
        // {
        // let mut reader = BufReader::new(&stream);
        // let mut line = String::new();
        // reader.read_line(&mut line);

        // line.pop();

        // let mut data : Vec<&str> = Vec::new(); 
        // data = line.split(":").collect::<Vec<&str>>();;

        // if(player_objects.len() < (data.len() / 4)){
        //     println!("Adding Player...");
        //     for i in  0..(((data.len() / 4) - player_objects.len()) + 1){
        //     player_objects.push(GameObject::new(Shape::Plane, &texture, &program_player, &vertex_buffer_player));
        // }
        // }
        
        // if(data.len() > 3){
        //     if(data.get(3).unwrap().parse::<f32>().unwrap() == 1.0){
        //     glow_effect_multiplier = 1.0;
        //     glow_position[0] = data.get(0).unwrap().parse::<f32>().unwrap();
        //     glow_position[1] = data.get(1).unwrap().parse::<f32>().unwrap();
        //     glow_position[2] = data.get(2).unwrap().parse::<f32>().unwrap();
        //     }
        // }

        // for i in 0..(data.len() / 4)  {
        //     let mut player = &mut player_objects.get_mut(i).unwrap();
        //     player.set_position(data.get((i * 4) + 1).unwrap().parse::<f32>().unwrap(), data.get((i * 4) + 2).unwrap().parse::<f32>().unwrap(), data.get((i * 4) + 3).unwrap().parse::<f32>().unwrap());
        // }

        }

        program_counter += 0.0005;

        if glow_effect_multiplier > 0.0 {
        glow_effect_multiplier = glow_effect_multiplier - 0.0005;
        }

        //debug_ping_object.translate(0.0005, 0.0, 0.0);

        let mut target = display.draw();
        target.clear_color_and_depth((0.25, 0.45, 1.0, 1.0), 1.0);

        let projection_matrix: [[f32; 4]; 4] = projection_matrix.into();

        target.draw(&vertex_buffer_skybox, &indices_skybox, &program_skybox, &uniform! {skybox : skybox_sampled, projection_matrix: projection_matrix, view_matrix :  mainCam.get_view_matrix(false)},
            &draw_params).unwrap();

      //  debug_ping_object.recalculateMatrix();
     //   target.draw(debug_ping_object.vertex_buffer, &indices, debug_ping_object.program, &uniform! {time : program_counter, transform: debug_ping_object.transform, projection_matrix: projection_matrix, view_matrix : mainCam.get_view_matrix(true)},
      //  &draw_params).unwrap();

        if should_spawn {
            game_objects.push(GameObject::new(Shape::Plane, &texture, &program, &vertex_buffer_terrain));
            should_spawn = false;
        }

        for player in &mut player_objects{
            player.recalculateMatrix();
            target.draw(player.vertex_buffer, &indices, player.program, &uniform! { time : program_counter, transform: player.transform, projection_matrix: projection_matrix, view_matrix : mainCam.get_view_matrix(true)},
            &draw_params).unwrap();
        }

        for gameObject in &mut game_objects{
            gameObject.recalculateMatrix();
            target.draw(gameObject.vertex_buffer, &indices, gameObject.program, &uniform! {light_position : light_pos, glowPosition : glow_position, shading_intensity : shading_intensity, time : program_counter, sampler: gameObject.texture, snowSampler : &snow_texture,rockSampler : &texture_rock, transform: gameObject.transform, projection_matrix: projection_matrix, view_matrix : mainCam.get_view_matrix(true), glowEffect : 1.0 as f32, light_location : light_y},
            &draw_params).unwrap();

        }

        water.recalculateMatrix();
        target.draw(water.vertex_buffer, &indices, water.program, &uniform! {light_position : light_pos, sampler: water.texture, transform: water.transform, projection_matrix: projection_matrix, view_matrix : mainCam.get_view_matrix(true)},
            &draw_params).unwrap();

        target.draw(profiler.vertex_buffer, &indices, profiler.program, &uniform! {time_passed : program_counter, mouseX : mx as f32 / screen_size.0 as f32, mouseY : my as f32 / screen_size.1 as f32, zoom_uniform : (1.0 + mouseScroll) * 0.01},
            &draw_params).unwrap();

       //glium_text::draw(&text, &system, &mut target, matrix, (1.0, 1.0, 0.0, 1.0));
       
        target.finish().unwrap();

        let mut very_new_now = Instant::now();
        //println!("Time since top of game loop : {}", very_new_now.duration_since(new_now).subsec_nanos());
        use glium::glutin::MouseScrollDelta;
        events_loop.poll_events(|ev| {
            
            match ev {

                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CursorMoved{position, ..} => {
                        dx = mx - position.0;
                        dy = my - position.1;
                        mx = position.0;
                        my = position.1;
                        mainCam.rotate(nalgebra::Vector3::new(0.0, (dy as f32 / 30.0), -(dx as f32 / 30.0)));
                        //mainCam.rotate(nalgebra::Vector3::new(0.0, (dx as f32 / 30.0), 0.0));
                    },
                    glutin::WindowEvent::MouseWheel { delta: MouseScrollDelta::LineDelta(x, y), .. } => {
                        mouseScroll = mouseScroll / y as f32;
                        println!("Scrolled!");
                    },
                glutin::WindowEvent::MouseInput {button, ..} => {
            let mut toi : f32 = std::f32::MAX;
            for i in 0..8000 as usize {
            
                let triangle = &collisionTriangles[i];
                match triangle.toi_and_normal_with_ray(&Rotation3::identity(), &vision_ray, true) {
                Some(n) => {
                    if(n.toi < toi){
                    println!("Collision!  {}", i); 
                    println!("{}, {}, {}", (vision_ray.origin + vision_ray.dir * n.toi).x, (vision_ray.origin + vision_ray.dir * n.toi).y, (vision_ray.origin + vision_ray.dir * n.toi).z);
                    glow_position = [(vision_ray.origin + vision_ray.dir * n.toi).x, (vision_ray.origin + vision_ray.dir * n.toi).y, (vision_ray.origin + vision_ray.dir * n.toi).z]; glow_effect_multiplier = 1.0; 
                    toi = n.toi;
                    }
                },
                None => {}
            }
        }
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
                        Some(glutin::VirtualKeyCode::Q) => {debug_ping_object.set_position(mainCam.position.x, mainCam.position.y, mainCam.position.z)},
                        Some(glutin::VirtualKeyCode::E) => mainCam.translate(nalgebra::Vector3::new(0.75, -0.75, 0.0)),
                        Some(glutin::VirtualKeyCode::X) => {},
                        Some(glutin::VirtualKeyCode::C) => {light_y = -100.0},
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

fn texture_to_cubemap(texture : &glium::Texture2d, display : &glium::Display) -> glium::texture::cubemap::Cubemap {
    
    use glium::texture::CubeLayer;


    let mut cubemap = Cubemap::empty(display, 1024).unwrap();
    {
    let mut fb = texture.as_surface();

    let mut neg_x = SimpleFrameBuffer::new(display, cubemap.main_level().image(CubeLayer::NegativeX)).unwrap();
    let mut pos_x = SimpleFrameBuffer::new(display, cubemap.main_level().image(CubeLayer::PositiveX)).unwrap();
    let mut neg_y = SimpleFrameBuffer::new(display, cubemap.main_level().image(CubeLayer::NegativeY)).unwrap();
    let mut pos_y = SimpleFrameBuffer::new(display, cubemap.main_level().image(CubeLayer::PositiveY)).unwrap();
    let mut neg_z = SimpleFrameBuffer::new(display, cubemap.main_level().image(CubeLayer::NegativeZ)).unwrap();
    let mut pos_z = SimpleFrameBuffer::new(display, cubemap.main_level().image(CubeLayer::PositiveZ)).unwrap();


    add_skybox_texture(&mut pos_z, &fb, 0, 0);
    add_skybox_texture(&mut neg_z, &fb, 0, 0);
    add_skybox_texture(&mut pos_y, &fb, 0, 0);
    add_skybox_texture(&mut neg_y, &fb, 0, 0);
    add_skybox_texture(&mut pos_x, &fb, 0, 0);
    add_skybox_texture(&mut neg_x, &fb, 0, 0);
    }

    cubemap

}

fn add_skybox_texture<'a>(save_into : &mut SimpleFrameBuffer, src : &SimpleFrameBuffer, x_start : u32, y_start : u32) {

    use glium::{Rect, BlitTarget};

    src.blit_color(
        &Rect {
            left : x_start,
            bottom : y_start,
            width : 1024,
            height : 1024
        },
        save_into,
        &BlitTarget{
            left : 0,
            bottom : 0,
            width : 1024,
            height : 1024
        },
        glium::uniforms::MagnifySamplerFilter::Linear
    );
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

pub fn get_cube_vertex_buffer(display: &glium::Display) -> glium::VertexBuffer<Vertex> {
        let mut vertices = vec![Vertex { position: [-0.5, -0.5, 0.5], uv: [0.0, 1.0], normal : [0.0, 0.0, 0.0] }, //0 back
                                Vertex { position: [0.5, -0.5, 0.5], uv: [1.0, 1.0], normal : [0.0, 0.0, 0.0]}, //1
                                Vertex { position: [-0.5, 0.5, 0.5], uv: [0.0, 0.0], normal : [0.0, 0.0, 0.0]}, //2
                                Vertex { position: [0.5, 0.5, 0.5], uv: [1.0, 0.0], normal : [0.0, 0.0, 0.0]}, //3

                                Vertex { position: [-0.5, 0.5, 0.5], uv: [0.0, 0.0], normal : [0.0, 0.0, 0.0]}, //4 top
                                Vertex { position: [0.5, 0.5, 0.5], uv: [1.0, 0.0], normal : [0.0, 0.0, 0.0]}, //5
                                Vertex { position: [-0.5, 0.5, -0.5], uv: [0.0, 1.0], normal : [0.0, 0.0, 0.0]}, //6
                                Vertex { position: [0.5, 0.5, -0.5], uv: [1.0, 1.0] , normal : [0.0, 0.0, 0.0]}, //7

                                Vertex { position: [-0.5, 0.5, -0.5], uv: [0.0, 0.0], normal : [0.0, 0.0, 0.0]}, //8 front
                                Vertex { position: [0.5, 0.5, -0.5], uv: [1.0, 0.0], normal : [0.0, 0.0, 0.0]}, //9
                                Vertex { position: [-0.5, -0.5, -0.5], uv: [0.0, 1.0], normal : [0.0, 0.0, 0.0]}, //10
                                Vertex { position: [0.5, -0.5, -0.5], uv: [1.0, 1.0], normal : [0.0, 0.0, 0.0]}, //11

                                Vertex { position: [-0.5, -0.5, -0.5], uv: [0.0, 1.0], normal : [0.0, 0.0, 0.0]}, //12 bottom
                                Vertex { position: [0.5, -0.5, -0.5], uv: [1.0, 1.0], normal : [0.0, 0.0, 0.0]}, //13
                                Vertex { position: [-0.5, -0.5, 0.5], uv: [0.0, 0.0], normal : [0.0, 0.0, 0.0] }, //14
                                Vertex { position: [0.5, -0.5, 0.5], uv: [0.0, 1.0], normal : [0.0, 0.0, 0.0]}, //15

                                Vertex { position: [0.5, -0.5, 0.5], uv: [0.0, 1.0], normal : [0.0, 0.0, 0.0]}, //16 right
                                Vertex { position: [0.5, -0.5, -0.5], uv: [1.0, 1.0], normal : [0.0, 0.0, 0.0]}, //17
                                Vertex { position: [0.5, 0.5, 0.5], uv: [0.0, 0.0], normal : [0.0, 0.0, 0.0]}, //18
                                Vertex { position: [0.5, 0.5, -0.5], uv: [1.0, 0.0], normal : [0.0, 0.0, 0.0]}, //19

                                Vertex { position: [-0.5, -0.5, -0.5], uv: [0.0, 1.0], normal : [0.0, 0.0, 0.0]}, //20 left
                                Vertex { position: [-0.5, -0.5, 0.5], uv: [1.0, 1.0], normal : [0.0, 0.0, 0.0]}, //21
                                Vertex { position: [-0.5, 0.5, -0.5], uv: [0.0, 0.0], normal : [0.0, 0.0, 0.0]}, //22       
                                Vertex { position: [-0.5, 0.5, 0.5], uv: [1.0, 0.0], normal : [0.0, 0.0, 0.0] } //23
                                ];
        glium::VertexBuffer::new(display, &vertices).unwrap()
    }
pub fn get_index_buffer(display: &glium::Display) -> glium::IndexBuffer<u16> {
        let indices = vec![0, 1, 2, 2, 1, 3, 4, 5, 6, 6, 5, 7, 8, 9, 10, 10, 9, 11, 12, 13, 14, 14, 13, 15, 16, 17, 18, 18, 17, 19, 20, 21, 22, 22, 21, 23];
        glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap()
    }



