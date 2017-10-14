#[macro_use]
extern crate glium;
extern crate image;
extern crate nalgebra;

const NEAR_PLANE: f32 = 0.001;
const FAR_PLANE : f32 = 1000.0;

fn main() {
    use glium::{glutin, Surface};

    let mut translation: nalgebra::Vector3<f32> = nalgebra::Vector3::new(0.0, 0.0, 0.0);
    let mut rotation_z: f32 = 0.0;
    let mut scale: nalgebra::Vector3<f32> = nalgebra::Vector3::new(1.0, 1.0, 1.0);

    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new();
    let context = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let screen_size = display.get_framebuffer_dimensions();

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

	implement_vertex!(Vertex, position, uv);
	let vertex1 = Vertex { position: [-1.0, -1.0, -2.0], uv: [ 0.0, 1.0 ] };
	let vertex2 = Vertex { position: [ 1.0, -1.0, -2.0], uv: [ 1.0, 1.0 ] };
	let vertex3 = Vertex { position: [ -1.0, 1.0, -2.0], uv: [ 0.0, 0.0 ] };

	let vertex4 = Vertex { position: [1.0, 1.0, -2.0], uv: [ 1.0, 0.0] };
	let vertex5 = Vertex { position: [ -1.0, 1.0, -2.0], uv: [ 0.0, 0.0] };
	let vertex6 = Vertex { position: [ 1.0, -1.0, -2.0], uv: [ 1.0, 1.0 ] };
	let shape = vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6];

	let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
	let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

	let mut vertex_shader_src = String::new();
	let mut fragment_shader_src = String::new();

	let mut file = File::open("shaders/vertex.glsl").expect("file not found");
    file.read_to_string(&mut vertex_shader_src).expect("something went wrong reading the file");

    let mut file = File::open("shaders/fragment.glsl").expect("file not found");
    file.read_to_string(&mut fragment_shader_src).expect("something went wrong reading the file");

	let program = glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None).unwrap();

    let projection_matrix: nalgebra::Matrix4<f32> = create_projection_matrix(60.0, screen_size);
    while !closed {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        let mut translation_matrix: nalgebra::Matrix4<f32> = nalgebra::Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let mut rotation_matrix: nalgebra::Matrix4<f32> = nalgebra::Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let mut scale_matrix: nalgebra::Matrix4<f32> = nalgebra::Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0);

        translation_matrix[(0, 3)] = translation[0];
        translation_matrix[(1, 3)] = translation[1];
        translation_matrix[(2, 3)] = translation[2];
        rotation_matrix[(0, 0)] = f32::cos(rotation_z);
        rotation_matrix[(2, 0)] = f32::sin(rotation_z);
        rotation_matrix[(0, 2)] = -f32::sin(rotation_z);
        rotation_matrix[(2, 2)] = f32::cos(rotation_z);
        scale_matrix[(0, 0)] = scale[0];
        scale_matrix[(1, 1)] = scale[1];
        scale_matrix[(2, 2)] = scale[2];

        let transform = translation_matrix * rotation_matrix * scale_matrix;
        let transform: [[f32; 4]; 4] = transform.into();
        let projection_matrix: [[f32; 4]; 4] = projection_matrix.into();
        target.draw(&vertex_buffer, &indices, &program, &uniform! { sampler: &texture, transform: transform, projection_matrix: projection_matrix },
            &Default::default()).unwrap();
        target.finish().unwrap();

        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                	glutin::WindowEvent::Closed => closed = true,
                	glutin::WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                		Some(glutin::VirtualKeyCode::Escape) => closed = true,
                		Some(glutin::VirtualKeyCode::Left) => translation[0] -= 0.1,
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

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    uv: [f32; 2],
}