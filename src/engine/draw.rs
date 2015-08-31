use glium;
use engine::shapes::Vertex;

pub struct DrawParams<'s> {
    pub shape: &'s glium::VertexBuffer<Vertex>,
    pub tx: f32,
    pub ty: f32,
    pub r: f32,
    pub zoom_x: f32,
    pub zoom_y: f32,
}

pub struct DrawList<'s>{
    pub params: Vec<DrawParams<'s>>,
}

impl<'s> DrawList<'s> {
    pub fn new() -> DrawList<'s> {
        DrawList {
            params: Vec::new(),
        }
    }

    pub fn draw(&mut self, shape: &'s glium::VertexBuffer<Vertex>, tx: f32, ty: f32, r: f32, zoom_x: f32, zoom_y: f32) {
        self.params.push(
            DrawParams{
                    shape: shape,
                    tx: tx,
                    ty: ty,
                    r: r,
                    zoom_x: zoom_x,
                    zoom_y: zoom_y,
            }
        );
    }
}
