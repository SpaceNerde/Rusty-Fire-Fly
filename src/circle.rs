use std::f32::consts::PI;
use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

pub fn generate_circle(r: f32, n: i32) -> Vec<Vertex>{
    let mut circle = vec![];

    let angle_increase = (2. * PI) / n as f32;

    for i in 0..n {
        let angle = i as f32 * angle_increase;
        let x = r * angle.cos();
        let y = r * angle.sin();
        circle.push(Vertex { position: [x, y] });
    }

    circle
}

