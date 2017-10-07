#[macro_use]
extern crate glium;
extern crate image;

fn main() {
    use glium::{glutin, Surface};

    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new();
    let context = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut closed = false;

    use std::io::Cursor;
	use std::fs::File;
	use std::io::prelude::*;

    let mut bytes: Vec<u8> = Vec::new();
    let mut file = File::open("rust_logo.jpg").expect("file not found");
    file.read_to_end(&mut bytes).expect("something went wrong reading the file");

	let image = image::load(Cursor::new(&bytes), image::JPEG).unwrap().to_rgba();
	let image_dimensions = image.dimensions();
	let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw()[..], image_dimensions);
	let texture = glium::texture::Texture2d::new(&display, image).unwrap();

	let screen_size = display.get_framebuffer_dimensions();

	implement_vertex!(Vertex, position, uv);
	let vertex1 = Vertex { position: [-1.0, -1.0], uv: [ 0.0, 1.0 ] };
	let vertex2 = Vertex { position: [ 1.0, -1.0], uv: [ 1.0, 1.0 ] };
	let vertex3 = Vertex { position: [ -1.0, 1.0], uv: [ 0.0, 0.0 ] };

	let vertex4 = Vertex { position: [1.0, 1.0], uv: [ 1.0, 0.0] };
	let vertex5 = Vertex { position: [ -1.0, 1.0], uv: [ 0.0, 0.0] };
	let vertex6 = Vertex { position: [ 1.0, -1.0], uv: [ 1.0, 1.0 ] };
	let shape = vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6];

	let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
	let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

	let mut value: f32 = 1.0;

	let mut vertex_shader_src = String::new();
	let mut fragment_shader_src = String::new();

	let mut file = File::open("shaders/vertex.glsl").expect("file not found");
    file.read_to_string(&mut vertex_shader_src).expect("something went wrong reading the file");

    let mut file = File::open("shaders/fragment.glsl").expect("file not found");
    file.read_to_string(&mut fragment_shader_src).expect("something went wrong reading the file");

	let program = glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None).unwrap();

	let mut movement = (0.0f32, 0.0f32);

    while !closed {
    	value += 0.001;
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &uniform! { sampler: &texture, screen_size: (screen_size.0 as f32, screen_size.1 as f32), value: value, movement: movement },
            &Default::default()).unwrap();
        target.finish().unwrap();

        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                	glutin::WindowEvent::Closed => closed = true,
                	glutin::WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                		Some(glutin::VirtualKeyCode::Escape) => closed = true,
                		Some(glutin::VirtualKeyCode::Left) => movement.0 -= 0.1,
                		Some(glutin::VirtualKeyCode::Right) => movement.0 += 0.1,
                		Some(glutin::VirtualKeyCode::Up) => movement.1 += 0.1,
                		Some(glutin::VirtualKeyCode::Down) => movement.1 -= 0.1,
                		_ => ()
                	},
                	_ => ()
                },
                _ => ()
            }
        });
    }
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    uv: [f32; 2]
}