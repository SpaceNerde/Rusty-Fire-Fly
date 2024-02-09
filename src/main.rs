mod circle;

use std::time::{Duration, Instant};
use glium::{implement_vertex, Surface, uniform};
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use crate::circle::generate_circle;

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

    let scale = 0.1;
    let radius = 0.25;
    let circle = generate_circle(radius, 20);

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

    let mut x_offset: f32 = 0.0;
    let mut y_offset: f32 = 0.0;
    let mut speed: Vec<f32> = vec![0.02, 0.01];

    let mut last_frame_time = Instant::now();

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
                // limit the program to 60 fps
                let elapsed_time = last_frame_time.elapsed();
                last_frame_time = Instant::now();

                let target_frame_time = Duration::from_millis(16);
                if elapsed_time < target_frame_time {
                    std::thread::sleep(target_frame_time - elapsed_time);
                }

                // request redraw
                window.request_redraw();
            },
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                // Simulation Logic
                x_offset += speed[0];

                // Check for collisions with the window borders
                if x_offset <= -1.0 + radius * scale {
                    x_offset = -1.0 + radius * scale;
                    // Reverse the x direction
                    speed[0] *= -1.0;
                } else if x_offset >= 1.0 - radius * scale {
                    x_offset = 1.0 - radius * scale;
                    // Reverse the x direction
                    speed[0] *= -1.0;
                }

                // Update the position
                y_offset += speed[1];

                // Check for collisions with the window borders
                if y_offset <= -1.0 + radius * scale {
                    y_offset = -1.0 + radius * scale;
                    // Reverse the y direction
                    speed[1] *= -1.0;
                } else if y_offset >= 1.0 - radius * scale {
                    y_offset = 1.0 - radius * scale;
                    // Reverse the y direction
                    speed[1] *= -1.0;
                }

                let uniforms = uniform! {
                    matrix: [
                        [1.0 * scale, 0.0, 0.0, 0.0],
                        [0.0, 1.0 * scale, 0.0, 0.0],
                        [0.0, 0.0, 1.0 * scale, 0.0],
                        [x_offset , y_offset, 0.0, 1.0f32],
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
