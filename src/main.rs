#[macro_use]
extern crate glium;
extern crate time;

pub mod graphics;

use graphics::{Mygraphics, get_triangle};

fn main() {

    let mut mg = Mygraphics::new("My Game!".to_string());

    let mut t = -8.0f32;

    let t1 = get_triangle(&mg);

    loop {

        if mg.mouse.left != graphics::mouse::ButtonState::Pressed {
            t += 0.02;
        };


        let zoom = 1500.0/(mg.window.size_pixels_x as f32);
        for i in 1..50 {
            mg.print(&t1, 0.0, 0.0, 3.0*t*(i as f32)/100.0,zoom,zoom);
        }
        mg.flush();

        println!("framerate: {}", mg.framerate.get_framerate());
        println!("mouse x,y: {},{} left {:?}, right {:?}", mg.mouse.x, mg.mouse.y,mg.mouse.left,mg.mouse.right);

        if mg.closed {
            return
        }
    }
}
