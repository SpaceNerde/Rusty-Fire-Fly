mod circle;
pub mod rusty_vertex;

use std::time::{Duration, Instant};
use glium::{Surface, uniform};
use glium::uniforms::UniformType;
use rand::Rng;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use crate::circle::generate_circle;
use crate::rusty_vertex::Vertex;

pub struct Fly {
    pub body: Vec<Vertex>,
    pub speed: [f32; 2],
    pub pos: [f32; 2],
    pub accumulation: f32,
    pub glowing: bool,
}

impl Fly {
    fn new(radius: f32, complexity: i32) -> Self {
        let mut rng = rand::thread_rng();
        let body: Vec<Vertex> = generate_circle(radius, complexity);
        let speed: [f32; 2] = [
            rng.gen_range(0.0000001..0.00023) * (if rng.gen_bool(0.5) { 1. } else { -1. }),
            rng.gen_range(0.0000001..0.00023) * (if rng.gen_bool(0.5) { 1. } else { -1. }),
        ];
        let pos: [f32; 2] = [
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        ];
        let accumulation = rng.gen_range(0.0..100.0);

        Fly {
            body,
            speed,
            pos,
            accumulation,
            glowing: false,
        }
    }
}

fn main() {
    // ##################
    // Simulation Setting
    // ##################

    // flies settings
    let scale = 0.05;
    let radius = 0.1;
    let complexity = 20;
    let amount_of_flies = 2500;
    let energy_charge: f32 = 1.;
    let energy_discharge: f32 = 10.;
    let max_range_of_neighbour = 1;

    // ################
    // Simulation Setup
    // ################

    // init event loop
    let event_loop = EventLoop::new().unwrap();
    // init window
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_inner_size(1000, 1000)
        .build(&event_loop);
    event_loop.set_control_flow(ControlFlow::Poll);

    let vertex_shader_src = r#"
        uniform mat4 matrix;
        uniform vec3 color;

        in vec2 position;
        out vec3 vertex_color;

        void main() {
            vertex_color = color;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        in vec3 vertex_color;
        out vec4 color;

        void main() {
            color = vec4(vertex_color, 1.0);
        }
    "#;

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    // create a fly
    let mut flies: Vec<Fly> = vec![];

    for _ in 0..amount_of_flies {
        let fly = Fly::new(radius, complexity);
        flies.push(fly);
    }

    let gray = (128.0/255.0, 128.0/255.0, 128.0/255.0f32);
    let yellow = (255.0, 255.0, 0.0f32);

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

                let target_frame_time = Duration::from_millis(8);
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

                let mut frame = display.draw();
                // fills screen black
                frame.clear_color(0.0, 0.0, 0.0, 1.0);

                for i in 0..flies.len() {
                    let vertex_buffer = glium::VertexBuffer::new(&display, &flies[i].body).unwrap();

                    flies[i].pos[0] += flies[i].speed[0];

                    // Check for collisions with the window borders
                    if flies[i].pos[0] <= -1.0 + radius * scale {
                        flies[i].pos[0] = -1.0 + radius * scale;
                        // Reverse the x direction
                        flies[i].speed[0] *= -1.0;
                    } else if flies[i].pos[0] >= 1.0 - radius * scale {
                        flies[i].pos[0] = 1.0 - radius * scale;
                        // Reverse the x direction
                        flies[i].speed[0] *= -1.0;
                    }

                    // Update the position
                    flies[i].pos[1] += flies[i].speed[1];

                    // Check for collisions with the window borders
                    if flies[i].pos[1] <= -1.0 + radius * scale {
                        flies[i].pos[1] = -1.0 + radius * scale;
                        // Reverse the y direction
                        flies[i].speed[1] *= -1.0;
                    } else if flies[i].pos[1] >= 1.0 - radius * scale {
                        flies[i].pos[1] = 1.0 - radius * scale;
                        // Reverse the y direction
                        flies[i].speed[1] *= -1.0;
                    }

                    // flies glowing cycle based on their accumulation
                    if flies[i].accumulation <= 500. && !flies[i].glowing {
                        flies[i].accumulation += energy_charge;
                        if flies[i].accumulation > 100. {
                            flies[i].accumulation = 100.;
                            flies[i].glowing = true;
                            for j in 0..flies.len() {
                                if i != j {
                                    let distance = ((flies[i].pos[0] - flies[j].pos[0]).powi(2)
                                        + (flies[i].pos[1] - flies[j].pos[1]).powi(2))
                                        .sqrt();
                                    // If the distance is within the specified radius, add 5 to accumulation
                                    if distance <= 0.09 {
                                        flies[j].accumulation += 5.0;
                                    }
                                }
                            }

                        }
                    } else if flies[i].accumulation >= 0. && flies[i].glowing {
                        flies[i].accumulation -= energy_discharge;
                        if flies[i].accumulation < 0. {
                            flies[i].accumulation = 0.;
                            flies[i].glowing = false;
                        }
                    }

                    let uniforms = uniform! {
                        matrix: [
                            [1.0 * scale, 0.0, 0.0, 0.0],
                            [0.0, 1.0 * scale, 0.0, 0.0],
                            [0.0, 0.0, 1.0 * scale, 0.0],
                            [flies[i].pos[0] , flies[i].pos[1], 0.0, 1.0f32],
                        ],
                        color: (if !flies[i].glowing {gray} else {yellow}),
                    };

                    // draw flies
                    frame.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
                }

                // process frame
                frame.finish().unwrap();
            },
            _ => {}
        }
    }).expect("TODO: panic message");
}
