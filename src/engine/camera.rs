pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub zoom_x: f32,
    pub zoom_y: f32,
    pub rotation: f32,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            x: 0.0,
            y: 0.0,
            zoom_x: 1.0,
            zoom_y: 1.0,
            rotation: 0.0,
        }
    }
}
