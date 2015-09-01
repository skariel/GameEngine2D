pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub zoom_x: f32,
    pub zoom_y: f32,
    rotation: f32,
    cos: f32,
    sin: f32,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            x: 0.0,
            y: 0.0,
            zoom_x: 1.0,
            zoom_y: 1.0,
            rotation: 0.0,
            cos: 1.0,
            sin: 0.0,
        }
    }

    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }

    pub fn increase_rotation(&mut self, inc: f32) {
        let old_rotation = self.rotation;
        self.set_rotation(old_rotation+inc);
    }

    pub fn get_cos(&self) -> f32 {
        self.cos
    }

    pub fn get_sin(&self) -> f32 {
        self.sin
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
        self.cos = rotation.cos();
        self.sin = rotation.sin();
    }

}
