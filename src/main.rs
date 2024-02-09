use glium::{implement_vertex, Surface};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn main() {
    // init event loop
    let event_loop = EventLoop::new().unwrap();
    // init window
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);
    event_loop.set_control_flow(ControlFlow::Poll);

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut t: f32 = 0.0;

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("Closing");
                elwt.exit();
            },
            Event::AboutToWait => {
                window.request_redraw();
            },
            Event::WindowEvent {
                event: WindowEvent::Resized(window_size),
                ..
            } => {
                display.resize(window_size.into());
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                // Simulation Logic
                t += 0.0002;
                let offset = t.sin() * 0.5;

                // define shape (triangle)
                let shape = vec![
                    Vertex { position: [-0.5 + offset, -0.5] },
                    Vertex { position: [ 0.0 + offset,  0.5] },
                    Vertex { position: [ 0.5 + offset, -0.25] },
                ];
                let vertex_buffer = glium::vertex::VertexBuffer::new(&display, &shape).unwrap();

                let mut frame = display.draw();
                // fills screen black
                frame.clear_color(0.0, 0.0, 0.0, 1.0);
                // draw triangle
                frame.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
                // process frame
                frame.finish().unwrap();
            },
            _ => {}
        }
    }).expect("TODO: panic message");
}
