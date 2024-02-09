use glium::{implement_vertex, Surface, uniform};
use winit::dpi::{LogicalSize, PhysicalSize};
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
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_inner_size(1000, 1000)
        .build(&event_loop);
    event_loop.set_control_flow(ControlFlow::Poll);

    let circle = [
        Vertex { position: [1.0, 0.0] },
        Vertex { position: [0.9239, 0.3827] },
        Vertex { position: [0.7071, 0.7071] },
        Vertex { position: [0.3827, 0.9239] },
        Vertex { position: [0.0, 1.0] },
        Vertex { position: [-0.3827, 0.9239] },
        Vertex { position: [-0.7071, 0.7071] },
        Vertex { position: [-0.9239, 0.3827] },
        Vertex { position: [-1.0, 0.0] },
        Vertex { position: [-0.9239, -0.3827] },
        Vertex { position: [-0.7071, -0.7071] },
        Vertex { position: [-0.3827, -0.9239] },
        Vertex { position: [0.0, -1.0] },
        Vertex { position: [0.3827, -0.9239] },
        Vertex { position: [0.7071, -0.7071] },
        Vertex { position: [0.9239, -0.3827] },

    ];
    let vertex_buffer = glium::VertexBuffer::new(&display, &circle).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);

    let vertex_shader_src = r#"
        in vec2 position;

        uniform mat4 matrix;

        void main() {
            gl_Position = matrix * vec4(position, 0.0, 1.0);
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
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                // Simulation Logic
                t += 0.0002;
                let offset = t.sin() * 0.5;

                let uniforms = uniform! {
                    matrix: [
                        [0.1, 0.0, 0.0, 0.0],
                        [0.0, 0.1, 0.0, 0.0],
                        [0.0, 0.0, 0.1, 0.0],
                        [offset , 0.0, 0.0, 1.0f32],
                    ]
                };

                let mut frame = display.draw();
                // fills screen black
                frame.clear_color(0.0, 0.0, 0.0, 1.0);
                // draw triangle
                frame.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
                // process frame
                frame.finish().unwrap();
            },
            _ => {}
        }
    }).expect("TODO: panic message");
}
