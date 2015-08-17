pub struct Window {
    pub size_pixels_x: u32,
    pub size_pixels_y: u32,
    pub aspect_ratio_y: f32,
}

impl Window {
    pub fn new(size_pixels_x: u32, size_pixels_y: u32) -> Window {
        let aspect_ratio_y: f32 = (size_pixels_y as f32)/(size_pixels_x as f32);
        Window{
            size_pixels_x: size_pixels_x,
            size_pixels_y: size_pixels_y,
            aspect_ratio_y: aspect_ratio_y,
        }
    }
}
