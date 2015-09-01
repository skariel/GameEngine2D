#[macro_use]
extern crate glium;
extern crate time;

pub mod engine;

use engine::{Engine, surface, shapes, tasklist};

use std::rc;

struct MySprite {
    spawned:bool,
    x:f32,
    y:f32,
    fig: rc::Rc<glium::VertexBuffer<shapes::Vertex>>,
}

struct MySharedData {
    num: i64,
}

impl tasklist::Task<MySharedData> for MySprite {
    fn handle(&mut self, tasklist: &mut tasklist::TaskList<MySharedData>, data: &engine::Data<MySharedData>) -> tasklist::TaskState {
        self.x += 0.01;
        if self.x>-0.3 && !self.spawned {
            self.spawned = true;
            tasklist.add(Box::new(MySprite {
                    spawned: false,
                    x: -0.9,
                    y: self.y+0.2,
                    fig: self.fig.clone(),
                    }));
        }
        if self.x>0.7 {
            return tasklist::TaskState::Remove;
        }
        println!("global data: {}", data.shared.num);
        tasklist::TaskState::OK
    }
    fn draw<'k>(&'k self, surface: &mut surface::Surface<'k>) {
        surface.draw(&self.fig, self.x, self.y, 0.0, 1.0, 1.0);
    }
    fn share(&self, data: &mut MySharedData) {
        data.num += 1;
    }
}

fn main() {

    let mut mg = Engine::new("My Game!".to_string(), MySharedData{num:1});
    let fig = shapes::get_triangle(&mg.graphics);
    mg.tasklist.add(Box::new(MySprite {
        spawned:false,
        x: -0.9,
        y: -0.4,
        fig: rc::Rc::new(fig),
        }));

    let mut t = -8.0f32;

    let t1 = shapes::get_triangle(&mg.graphics);

    let mut tx = 0.0;
    let mut ty = 0.0;

    loop {

        if mg.mouse.left != engine::mouse::ButtonState::Pressed {
            t += 0.02;
        };

        let zoom = 1500.0/(mg.graphics.window.size_pixels_x as f32);
        for i in 1..50 {
            mg.graphics.print(&t1, tx, ty, 3.0*t*(i as f32)/100.0,zoom,zoom);
        }
        mg.flush();

        println!("framerate: {}", mg.framerate.get_framerate());
        println!("mouse x,y: {},{} left {:?}, right {:?}", mg.mouse.x, mg.mouse.y,mg.mouse.left,mg.mouse.right);
        println!("up: {:?}", mg.keyboard.up);

        match mg.mouse.left {
            engine::mouse::ButtonState::Drag{x:_,y:_} => {
                tx=mg.mouse.x;
                ty=mg.mouse.y;
                println!("DRAGGING!!!!!!!!!!!!!!!!");
            },
            _ => (),
        };

        if mg.mouse.dleft == engine::mouse::DButtonState::Pressed {
            tx = mg.mouse.x;
            ty = mg.mouse.y;
        }

        if mg.graphics.window.closed {
            return
        }
    }
}
