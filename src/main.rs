#[macro_use]
extern crate glium;
extern crate time;

mod graphics;

use graphics::{Mygraphics, get_triangle};

fn main() {

    let mut mg = Mygraphics::new("My Game!".to_string());

    let mut t = -8.0f32;

    let t1 = get_triangle(&mg);

    loop {

        if mg.mouse.left != graphics::mouse::ButtonState::Pressed {
            t += 0.02;
        };


        let zoom_x = 1500.0/(mg.window.size_pixels_x as f32);
        let zoom_y = 1500.0/(mg.window.size_pixels_y as f32);
        for i in 1..50 {
            mg.print(&t1, 0.0, 0.0, 3.0*t*(i as f32)/100.0, zoom_x*(i as f32)/50.0,zoom_y*(i as f32)/50.0);
        }
        mg.flush();

        println!("framerate: {}", mg.framerate.get_framerate());
        println!("mouse x,y: {},{} left {:?}, right {:?}", mg.mouse.x, mg.mouse.y,mg.mouse.left,mg.mouse.right);

        if mg.closed {
            return
        }
    }
}
