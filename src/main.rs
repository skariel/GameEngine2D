// Copyright 2015 Jose Ariel Keselman
//
// This file is part of GameEngine2D.
//
// GameEngine2D is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// GameEngine2D is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with GameEngine2D.  If not, see <http://www.gnu.org/licenses/>.

#![feature(plugin)]
#![plugin(clippy)]

#[macro_use]
extern crate glium;
extern crate time;

pub mod engine;

use std::rc;
use engine::{Engine, shapes, tasklist, camera};

struct MySpriteModel {
    spawned:bool,
    x:f32,
    y:f32,
}

struct MySpriteTask {
    model: MySpriteModel,
    fig: rc::Rc<glium::VertexBuffer<shapes::Vertex>>,
}

struct MySpriteDrawable {
    fig: rc::Rc<glium::VertexBuffer<shapes::Vertex>>,
    x: f32,
    y: f32,
}

struct MySharedData {
    num: i64,
}

impl tasklist::Drawable for MySpriteDrawable {
    fn draw(&self, camera: &camera::Camera, graphics: &mut engine::graphics::Graphics) {
        graphics.print(&camera, &self.fig, self.x, self.y, 0.0, 1.0, 1.0);
    }
}

impl tasklist::Model<MySharedData> for MySpriteModel {
    fn handle(&mut self, shared_data: &engine::Data<MySharedData>) {
        self.x += 0.01;
        println!("global data: {}", shared_data.user_shared_data.num);
    }
}

impl tasklist::Task<MySharedData> for MySpriteTask {
    fn get_drawable(&self) -> Option<Box<tasklist::Drawable>> {
        Some(Box::new(MySpriteDrawable {
            x: self.model.x,
            y: self.model.y,
            fig: self.fig.clone(),
        }))
    }

    fn get_new_tasks(&mut self) -> Option<Vec<Box<tasklist::Task<MySharedData>>>> {
        if self.model.x>-0.3 && !self.model.spawned {
            self.model.spawned = true;
            let mut tasklist: Vec<Box<tasklist::Task<MySharedData>>> = Vec::new();
            tasklist.push(Box::new(MySpriteTask {
                    model: MySpriteModel {
                        spawned: false,
                        x: -0.9,
                        y: self.model.y,
                    },
                    fig: self.fig.clone(),
                    }));
            return Some(tasklist);
        }
        None
    }
    fn get_model(&self) -> & tasklist::Model<MySharedData> {
        &self.model
    }
    fn get_model_mut(&mut self) -> &mut tasklist::Model<MySharedData> {
        &mut self.model
    }
    fn share(&self, shared_data: &mut MySharedData, camera: &mut camera::Camera) {
        camera.x += 0.0005;
        camera.rotation += 0.003;
        camera.zoom_x *= 0.999;
        camera.zoom_y *= 0.999;
        shared_data.num += 1;
    }
    fn get_state(&self) -> tasklist::TaskState {
        if self.model.x > 1.1 {
            return tasklist::TaskState::Remove;
        }
        tasklist::TaskState::Keep
    }
}

fn main() {
    let mut mg = Engine::new("My Game!".to_owned(), MySharedData{num:1});
    let fig = shapes::get_triangle(&mg.graphics);
    mg.tasklist.add(Box::new(MySpriteTask {
        model: MySpriteModel {
            spawned:false,
            x: -0.9,
            y: 0.3,
        },
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
        for i in 1..40 {
            mg.graphics.print(&mg.camera, &t1, tx, ty, 3.0*t*(i as f32)/100.0,zoom,zoom);
        }
        mg.flush();

        println!("framerate: {}", mg.framerate.get_framerate());
        println!("mouse x,y: {},{} left {:?}, right {:?}", mg.mouse.x, mg.mouse.y,mg.mouse.left,mg.mouse.right);
        println!("up: {:?}", mg.keyboard.up);

        if let engine::mouse::ButtonState::Drag{x:_,y:_} = mg.mouse.left {
            tx=mg.mouse.x;
            ty=mg.mouse.y;
            println!("DRAGGING!!!!!!!!!!!!!!!!");
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
