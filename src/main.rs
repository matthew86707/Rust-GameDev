#[macro_use]
extern crate glium;

fn main() {
    use glium::{glutin, Surface};

    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new();
    let context = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let mut t : f32 = 0.0;

    let mut closed = false;

	implement_vertex!(Vertex, position, color);
	let vertex1 = Vertex { position: [-1.0, -1.0], color: [ 1.0, 0.0, 0.0, 1.0 ] };
	let vertex2 = Vertex { position: [ 1.0, -1.0], color: [ 0.0, 1.0, 0.0, 1.0 ] };
	let vertex3 = Vertex { position: [ -1.0, 1.0], color: [ 0.0, 0.0, 1.0, 1.0 ] };
	let vertex4 = Vertex { position: [1.0, 1.0], color: [ 1.0, 0.0, 0.0, 1.0 ] };
	let vertex5 = Vertex { position: [ -1.0, 1.0], color: [ 0.0, 0.0, 1.0, 1.0 ] };
	let vertex6 = Vertex { position: [ 1.0, -1.0], color: [ 0.0, 1.0, 0.0, 1.0 ] };
	let shape = vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6];

	let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
	let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

	let vertex_shader_src = r#"
	    #version 140

	    in vec4 color;
	    in vec2 position;

	    out vec4 fragColor;

	    void main() {
	        gl_Position = vec4(position, 0.0, 1.0);
	        fragColor = color;
	    }
	"#;
	let fragment_shader_src = r#"
	    #version 140

	    out vec4 color;
	    in vec4 fragColor;

	    uniform float t;

	    void main() {
	        color = vec4(t, gl_FragCoord.x / 1000, 1.0, 1.0);
	    }
	"#;
	let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    while !closed {
        let mut target = display.draw();
        t += 0.001;
        target.clear_color(0.1, 0.25, 0.2, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &uniform! { t: t },
            &Default::default()).unwrap();
        target.finish().unwrap();

        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                	glutin::WindowEvent::Closed => closed = true,
                	glutin::WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                		Some(glutin::VirtualKeyCode::Escape) => closed = true,
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
    color: [f32; 4]
}