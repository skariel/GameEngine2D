use glium;
use engine::graphics;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 4]
}
implement_vertex!(Vertex, position, color);

pub fn get_triangle(g: &graphics::Graphics) -> glium::VertexBuffer<Vertex> {
    let triangle =
        [
            Vertex { position: [-0.2, -0.2], color: [0.0,1.0,1.0,0.5]},
            Vertex { position: [ 0.0,  0.2], color: [1.0,0.0,1.0,0.5]},
            Vertex { position: [ 0.2, -0.07], color: [1.0,1.0,0.0,0.5]}
        ];
    g.compile(&triangle)
}
