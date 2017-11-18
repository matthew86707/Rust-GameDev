#[macro_use]
extern crate glium;
extern crate image;
extern crate nalgebra;
extern crate rand;

mod GameObject;
mod Camera;
mod PrimitiveShapes;
mod UIElement;

fn main() {

use glium::{glutin, Surface};
use UIElement::UIElement;
use PrimitiveShapes::Vertex;
use GameObject::Shape;
use GameObject::GameObject;

  
    let mut programCounter : f32 = 0.0;
    let mut glowEffectMultiplier : f32 = 0.0;

    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new();
    let context = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    //let mut terrain_vb : glium::VertexBuffer<Vertex> = glium::VertexBuffer::new(&display, &PrimitiveShapes::get_plane(256, 256)).unwrap();

    {

    let mut translation: nalgebra::Vector3<f32> = nalgebra::Vector3::new(0.0, 0.0, 0.0);
    let mut rotation_z: f32 = 0.0;
    let mut rotation_y: f32 = 0.0;
    let mut scale: nalgebra::Vector3<f32> = nalgebra::Vector3::new(1.0, 1.0, 1.0);

    

    let screen_size = display.get_framebuffer_dimensions();

    let mut closed = false;

    use std::io::Cursor;
	use std::fs::File;
	use std::io::prelude::*;

    let vertex1 = Vertex { position: [-100.0, -100.0, -2.0], uv: [ 0.0, 1.0 ] };
    let vertex2 = Vertex { position: [ 100.0, -100.0, -2.0], uv: [ 1.0, 1.0 ] };
    let vertex3 = Vertex { position: [ -100.0, 100.0, -2.0], uv: [ 0.0, 0.0 ] };

    let vertex4 = Vertex { position: [100.0, 100.0, -2.0], uv: [ 1.0, 0.0] };
    let vertex5 = Vertex { position: [ -100.0, 100.0, -2.0], uv: [ 0.0, 0.0] };
    let vertex6 = Vertex { position: [ 100.0, -100.0, -2.0], uv: [ 1.0, 1.0 ] };
    let shape = vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6];

    let vertex_buffer_back = glium::VertexBuffer::new(&display, &shape).unwrap();

    let vertex1 = Vertex { position: [100.0, -100.0, -2.0], uv: [ 0.0, 1.0 ] };
    let vertex2 = Vertex { position: [ -100.0, -100.0, -2.0], uv: [ 1.0, 1.0 ] };
    let vertex3 = Vertex { position: [ 100.0, 100.0, -2.0], uv: [ 0.0, 0.0 ] };

    let vertex4 = Vertex { position: [-100.0, 100.0, -2.0], uv: [ 1.0, 0.0] };
    let vertex5 = Vertex { position: [ 100.0, 100.0, -2.0], uv: [ 0.0, 0.0] };
    let vertex6 = Vertex { position: [ -100.0, -100.0, -2.0], uv: [ 1.0, 1.0 ] };
    let shape = vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6];

    let vertex_buffer_front = glium::VertexBuffer::new(&display, &shape).unwrap();
    

	let texture = load_texture("grass.jpg", &display);

    let snow_texture = load_texture("Snow.jpg", &display);

    //let texture_ui = load_texture("loading_screen.jpg", &display);

    let texture_rock = load_texture("rock.jpg", &display);

	implement_vertex!(Vertex, position, uv);

	let shape_terrain = PrimitiveShapes::get_plane(512, 512);

	let vertex_buffer_terrain = glium::VertexBuffer::new(&display, &shape_terrain).unwrap();
	let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);



	let mut vertex_shader_src = String::new();
	let mut fragment_shader_src = String::new();

	let mut file = File::open("shaders/vertex.glsl").expect("file not found");
    file.read_to_string(&mut vertex_shader_src).expect("something went wrong reading the file");

    let mut file = File::open("shaders/fragment.glsl").expect("file not found");
    file.read_to_string(&mut fragment_shader_src).expect("something went wrong reading the file");

	let program = glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None).unwrap();





    let mut vertex_shader_src_UI = String::new();
    let mut fragment_shader_src_UI = String::new();

    let mut file = File::open("shaders/vertex_ui.glsl").expect("file not found");
    file.read_to_string(&mut vertex_shader_src_UI).expect("something went wrong reading the file");

    let mut file = File::open("shaders/fragment_ui.glsl").expect("file not found");
    file.read_to_string(&mut fragment_shader_src_UI).expect("something went wrong reading the file");

    let program_UI = glium::Program::from_source(&display, &vertex_shader_src_UI, &fragment_shader_src_UI, None).unwrap();





    let mut mainCam : Camera::Camera = Camera::Camera::new();

    let projection_matrix: nalgebra::Matrix4<f32> = mainCam.create_projection_matrix(95.0, screen_size);



    //let tex_posx = load_texture("skybox/TropicalSunnyDayBack2048.jpg", &display);
    //let tex_negx = load_texture("skybox/TropicalSunnyDayFront2048.jpg", &display);
    //let tex_posy = load_texture("skybox/TropicalSunnyDayUp2048.jpg", &display);
    //let tex_negy = load_texture("skybox/TropicalSunnyDayDown2048.jpg", &display);
    //let tex_posz = load_texture("skybox/TropicalSunnyDayLeft2048.jpg", &display);
    //let tex_negz = load_texture("skybox/TropicalSunnyDayRight2048.jpg", &display);

    //let cubemap = glium::texture::Cubemap::empty(&display, 2048).unwrap();

    let mut vertex_shader_src_sky = String::new();
    let mut fragment_shader_src_sky = String::new();

    let mut file = File::open("shaders/vertex_skybox.glsl").expect("file not found");
    file.read_to_string(&mut vertex_shader_src_sky).expect("something went wrong reading the file");

    let mut file = File::open("shaders/fragment_skybox.glsl").expect("file not found");
    file.read_to_string(&mut fragment_shader_src_sky).expect("something went wrong reading the file");

    let program_skybox = glium::Program::from_source(&display, &vertex_shader_src_sky, &fragment_shader_src_sky, None).unwrap();

    //  let skybox_vertex_buffer = {
    //     #[derive(Copy, Clone)]
    //     struct Vertex {
    //         position: [f32; 3],
    //     }

    //     implement_vertex!(Vertex, position);

    //     let side2: f32 = 50.0 / 2.0;

    //     glium::VertexBuffer::new(&display,
    //         &[
    //             // Front
    //         Vertex { position: [-side2, -side2,  side2] },
    //         Vertex { position: [ side2, -side2,  side2] },
    //         Vertex { position: [ side2,  side2,  side2] },
    //             Vertex { position: [-side2,  side2,  side2] },
    //         // Right
    //         Vertex { position: [ side2, -side2,  side2] },
    //         Vertex { position: [ side2, -side2, -side2] },
    //         Vertex { position: [ side2,  side2, -side2] },
    //             Vertex { position: [ side2,  side2,  side2] },
    //         // Back
    //         Vertex { position: [-side2, -side2, -side2] },
    //         Vertex { position: [-side2,  side2, -side2] },
    //         Vertex { position: [ side2,  side2, -side2] },
    //             Vertex { position: [ side2, -side2, -side2] },
    //         // Left
    //         Vertex { position: [-side2, -side2,  side2] },
    //         Vertex { position: [-side2,  side2,  side2] },
    //             Vertex { position: [-side2,  side2, -side2] },
    //             Vertex { position: [-side2, -side2, -side2] },
    //             // Bottom
    //         Vertex { position: [-side2, -side2,  side2] },
    //         Vertex { position: [-side2, -side2, -side2] },
    //         Vertex { position: [ side2, -side2, -side2] },
    //             Vertex { position: [ side2, -side2,  side2] },
    //         // Top
    //             Vertex { position: [-side2,  side2,  side2] },
    //         Vertex { position: [ side2,  side2,  side2] },
    //         Vertex { position: [ side2,  side2, -side2] },
    //             Vertex { position: [-side2,  side2, -side2] },
    //         ]
    //     ).unwrap()
    // };

     // let skybox_index_buffer = glium::IndexBuffer::new(&display,
     //        glium::index::PrimitiveType::TrianglesList,
     //        &[
     //            // Front
     //            0u16, 2, 1, 0, 3, 2,
     //            // Right
     //            4, 6, 5, 4, 7, 6,
     //            // Back
     //            8, 10, 9, 8, 11, 10,
     //            // Left
     //            12, 14, 13, 12, 15, 14,
     //            // Bottom
     //            16, 18, 17, 16, 19, 18,
     //            // Top
     //            20, 22, 21, 20, 23, 22,
     // ]).unwrap();


    let dest_rect1 = glium::BlitTarget {
        left: 0,
        bottom: 0,
        width: 2048,
        height: 2048,
    };

    let scale: f32 = 1.0;
    let scale2: f32 = 1.0;
    let mut t: f32 = 0.0;



   

    {
    let mut SelectedGameObjects : Vec<GameObject> = Vec::new();
    let mut GameObjects : Vec<GameObject> = Vec::new();

    let mut testObject : GameObject = GameObject::new(Shape::Plane, &texture, &program, &vertex_buffer_terrain);

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

   let mut shouldSpawn : bool = false;

    while !closed {
        programCounter += 0.00050;
        glowEffectMultiplier = (1.57 + f32::sin(programCounter) / 2.0);



        //Draw skybox to framebuffers

        // let  framebuffer1 = glium::framebuffer::SimpleFrameBuffer::new(&display,
        //                 cubemap.main_level().image(glium::texture::CubeLayer::PositiveX)).unwrap();
        // let  framebuffer2 = glium::framebuffer::SimpleFrameBuffer::new(&display,
        //                 cubemap.main_level().image(glium::texture::CubeLayer::NegativeX)).unwrap();
        // let  framebuffer3 = glium::framebuffer::SimpleFrameBuffer::new(&display,
        //                 cubemap.main_level().image(glium::texture::CubeLayer::PositiveY)).unwrap();
        // let  framebuffer4 = glium::framebuffer::SimpleFrameBuffer::new(&display,
        //                 cubemap.main_level().image(glium::texture::CubeLayer::NegativeY)).unwrap();
        // let  framebuffer5 = glium::framebuffer::SimpleFrameBuffer::new(&display,
        //                 cubemap.main_level().image(glium::texture::CubeLayer::PositiveZ)).unwrap();
        // let  framebuffer6 = glium::framebuffer::SimpleFrameBuffer::new(&display,
        //                 cubemap.main_level().image(glium::texture::CubeLayer::NegativeZ)).unwrap();

        // tex_posx.as_surface().blit_whole_color_to(&framebuffer1, &dest_rect1,
        //                 glium::uniforms::MagnifySamplerFilter::Linear);
        // tex_negx.as_surface().blit_whole_color_to(&framebuffer2, &dest_rect1,
        //                 glium::uniforms::MagnifySamplerFilter::Linear);
        // tex_negy.as_surface().blit_whole_color_to(&framebuffer3, &dest_rect1,
        //                 glium::uniforms::MagnifySamplerFilter::Linear);
        // tex_posy.as_surface().blit_whole_color_to(&framebuffer4, &dest_rect1,
        //                 glium::uniforms::MagnifySamplerFilter::Linear);
        // tex_posz.as_surface().blit_whole_color_to(&framebuffer5, &dest_rect1,
        //                 glium::uniforms::MagnifySamplerFilter::Linear);
        // tex_negz.as_surface().blit_whole_color_to(&framebuffer6, &dest_rect1,
        //                 glium::uniforms::MagnifySamplerFilter::Linear);

         let mut target = display.draw();
        target.clear_color_and_depth((0.25, 0.45, 1.0, 1.0), 1.0);

        let projection_matrix: [[f32; 4]; 4] = projection_matrix.into();

        // let skybox_uniforms = uniform! {
        //      projection: projection_matrix,
        //      view: mainCam.get_view_matrix(),
        //      cubetex: cubemap.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Linear),
        // };

       

        // target.draw(&skybox_vertex_buffer, &skybox_index_buffer, &program_skybox,
        //     &skybox_uniforms, &draw_params).unwrap();

        if shouldSpawn {
            SelectedGameObjects.push(GameObject::new(Shape::Plane, &texture, &program, &vertex_buffer_terrain));
            shouldSpawn = false;
        }

        
        for gameObject in &mut GameObjects{
            gameObject.recalculateMatrix();
            target.draw(gameObject.vertex_buffer, &indices, gameObject.program, &uniform! { time : programCounter, sampler: gameObject.texture, snowSampler : &snow_texture,rockSampler : &texture_rock, transform: gameObject.transform, projection_matrix: projection_matrix, view_matrix : mainCam.get_view_matrix(), glowEffect : 1.0 as f32},
            &draw_params).unwrap();

        }

        for gameObject in &mut SelectedGameObjects{
            gameObject.recalculateMatrix();
            target.draw(gameObject.vertex_buffer, &indices, gameObject.program, &uniform! { time : programCounter, sampler: gameObject.texture , snowSampler : &snow_texture, rockSampler : &texture_rock, transform: gameObject.transform, projection_matrix: projection_matrix, view_matrix : mainCam.get_view_matrix(), glowEffect : glowEffectMultiplier},
            &draw_params).unwrap();
        }

       //  for ui_element in &mut UIElements{
       //     ui_element.recalculateMatrix();
       //     target.draw(&vertex_buffer, &indices, &program_UI, &uniform! { sampler: ui_element.texture, transform: ui_element.transform},
       //     &draw_params).unwrap();
       //
       // }
        
        
        target.finish().unwrap();

        events_loop.poll_events(|ev| {
            
                
            match ev {

                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::MouseMoved{position, ..} => {
                    dx = mx - position.0;
                    dy = my - position.1;
                    mx = position.0;
                    my = position.1;
                    rotation_z += (dx as f32) * (0.05);
                    mainCam.rotate(nalgebra::Vector3::new(0.0, 0.0, dx as f32 / 3.0));
                    rotation_y += (dy as f32) * (0.05);
                    mainCam.rotate(nalgebra::Vector3::new(0.0, -(dy as f32 / 3.0), 0.0));
                },
                	glutin::WindowEvent::Closed => closed = true,
                	glutin::WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                		Some(glutin::VirtualKeyCode::Escape) => closed = true,
                        Some(glutin::VirtualKeyCode::Right) => {match SelectedGameObjects.get_mut(0) { 
                            Some(mut obj) => obj.translate(-0.1, 0.0, 0.0),
                            _ => ()
                        }},
                        Some(glutin::VirtualKeyCode::Left) => {match SelectedGameObjects.get_mut(0) { 
                            Some(mut obj) => obj.translate(0.1, 0.0, 0.0),
                            _ => ()
                        }},
                        Some(glutin::VirtualKeyCode::Up) => {match SelectedGameObjects.get_mut(0) { 
                            Some(mut obj) => obj.translate(0.0, 0.0, -0.1),
                            _ => ()
                        }},
                        Some(glutin::VirtualKeyCode::Down) => {match SelectedGameObjects.get_mut(0) { 
                            Some(mut obj) => obj.translate(0.0, 0.0, 0.1),
                            _ => ()
                        }},
                        Some(glutin::VirtualKeyCode::P) => {shouldSpawn = true;},
                        Some(glutin::VirtualKeyCode::O) => {shouldSpawn = true;},
                        Some(glutin::VirtualKeyCode::R) => {

                                   //terrain_vb = glium::VertexBuffer::new(&display, &PrimitiveShapes::get_plane(16, 16)).unwrap();
                                   //SelectedGameObjects.get_mut(0).unwrap().regenTerrain(&terrain_vb);
                                
                        },
                        Some(glutin::VirtualKeyCode::Return) => {
                            let mut left_over : Vec<GameObject> = SelectedGameObjects.drain(0..).collect();
                            GameObjects.extend(left_over);
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
   // UIElements.clear();
    //drop(texture_ui);
   // drop(UIElements.get_mut(0));
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


