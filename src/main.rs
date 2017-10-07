#[macro_use]
extern crate glium;
extern crate rand;

fn main() {
    use glium::{glutin, Surface};
    use rand::Rng;

	let mut rng = rand::thread_rng();
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new();
    let context = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let mut t : f32 = 0.0;
    let screen_size = display.get_framebuffer_dimensions();

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
	    out float random;

	    void main() {
	        gl_Position = vec4(position, 0.0, 1.0);
	        fragColor = color;
	    }
	"#;
	let fragment_shader_src = r#"
	    #version 140

	    out vec4 color;
	    in vec4 fragColor;
	    uniform float random;

	    uniform float t;
	    uniform vec2 screenSize;

	    void main() {
	    	vec2 center = screenSize / 2;
	    	center.x += sin(t * 25) * (screenSize.x / 2);
	    	center.y += sin(t * 100) * (screenSize.y / 2);

	    	float dist = distance(gl_FragCoord.xy, center);

	        color = vec4(random, dist / (screenSize.x / 2.0), dist / (screenSize.y / 2.0), 1.0);
	    }
	"#;
	let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    while !closed {
        let mut target = display.draw();
        t += 0.0001;
        target.clear_color(0.1, 0.25, 0.2, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &uniform! { t: t, screenSize: (screen_size.0 as f32, screen_size.1 as f32), random: rng.gen::<f32>() },
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
    color: [f32; 4],
}