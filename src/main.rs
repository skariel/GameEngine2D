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


        mg.print(&t1, 0.0, 0.0, t, 0.5,1.5);
        mg.print(&t1, 0.0, 0.0, t*0.95, 0.5,1.5);
        mg.print(&t1, 0.0, 0.0, t*0.9, 0.5,1.5);
        mg.print(&t1, 0.0, 0.0, t*0.85, 0.5,1.5);
        mg.print(&t1, 0.0, 0.0, t*0.8, 0.5,1.5);
        mg.flush();

        println!("framerate: {}", mg.framerate.get_framerate());
        println!("mouse x,y: {},{} left {:?}, right {:?}", mg.mouse.x, mg.mouse.y,mg.mouse.left,mg.mouse.right);

        if mg.closed {
            return
        }
    }
}
