use glium;
use engine::shapes::Vertex;

pub struct DrawParams<'s> {
    pub shape: &'s glium::VertexBuffer<Vertex>,
    pub tx: f32,
    pub ty: f32,
    pub rotation: f32,
    pub zoom_x: f32,
    pub zoom_y: f32,
}

pub struct Surface<'s>{
    pub drawparams: Vec<DrawParams<'s>>,
}

impl<'s> Surface<'s> {
    pub fn new() -> Surface<'s> {
        Surface {
            drawparams: Vec::new(),
        }
    }

    pub fn draw(&mut self, shape: &'s glium::VertexBuffer<Vertex>, tx: f32, ty: f32, rotation: f32, zoom_x: f32, zoom_y: f32) {
        self.drawparams.push(
            DrawParams{
                    shape: shape,
                    tx: tx,
                    ty: ty,
                    rotation: rotation,
                    zoom_x: zoom_x,
                    zoom_y: zoom_y,
            }
        );
    }
}
