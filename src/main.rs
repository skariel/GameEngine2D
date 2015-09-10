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
use engine::{Engine, shapes, spritelist, camera};

struct MySharedData {
    num: i64,
}

struct MySpriteModel {
    x: f32,
    y: f32,
}

impl spritelist::SpriteModel<MySharedData> for MySpriteModel {
    fn handle(&mut self, shared_data: &engine::Data<MySharedData>) {
        self.x += 0.01;
        if shared_data.user_shared_data.num.wrapping_rem(10) == 0 {
            println!("global data: {}", shared_data.user_shared_data.num);
        }
    }
    fn share(&self, shared_data: &mut MySharedData, camera: &mut camera::Camera) {
        camera.x += 0.0005;
        camera.rotation += 0.003;
        camera.zoom_x *= 0.999;
        camera.zoom_y *= 0.999;
        shared_data.num += 1;
    }
}

struct MySpriteDrawable {
    x: f32,
    y: f32,
}

struct MyDrawableData {
    fig: rc::Rc<glium::VertexBuffer<shapes::Vertex>>,
}
impl spritelist::SpriteDrawable<MyDrawableData>
for MySpriteDrawable {
    fn draw(&self, user_data: &MyDrawableData, camera: &camera::Camera, graphics: &mut engine::graphics::Graphics) {
        graphics.print(&camera, &user_data.fig, self.x, self.y, 0.0, 1.0, 1.0);
    }
}

struct MySprite {
    fig: rc::Rc<glium::VertexBuffer<shapes::Vertex>>,
}

impl spritelist::Sprite<MySharedData, MySpriteModel, MySpriteDrawable, MyDrawableData>
for MySprite {
    fn get_drawable(&self, sprite_model: &MySpriteModel) -> MySpriteDrawable {
        MySpriteDrawable {
            x: sprite_model.x,
            y: sprite_model.y,
        }
    }
    fn get_user_drawable_data(&self) -> MyDrawableData {
        MyDrawableData {
            fig: self.fig.clone(),
        }
    }
}

fn main() {
    let mut mg = Engine::new("My Game!".to_owned(), MySharedData { num: 1 });
    let mut my_sprite_list: spritelist::SpriteList<MySpriteModel, MySprite, MySharedData, MySpriteDrawable, MyDrawableData> =
        spritelist::SpriteList::new(MySprite {
            fig: rc::Rc::new(shapes::get_triangle(&mg.graphics))
        });

    my_sprite_list.push(MySpriteModel {
            x: -0.9,
            y: 0.1,
    });

    mg.tasklist.add(Box::new(my_sprite_list));

    let mut t = -8.0f32;

    let t1 = shapes::get_triangle(&mg.graphics);

    let mut tx = 0.0;
    let mut ty = 0.0;

    loop {

        if mg.mouse.left != engine::mouse::ButtonState::Pressed {
            t += 0.02;
        };

        let zoom = 1500.0 / (mg.graphics.window.size_pixels_x as f32);
        for i in 1..40 {
            mg.graphics.print(&mg.camera, &t1, tx, ty, 3.0*t*(i as f32)/100.0,zoom,zoom);
        }
        mg.flush();

        println!("framerate: {}", mg.framerate.get_framerate());
        println!("mouse x,y: {},{} left {:?}, right {:?}",
            mg.mouse.x, mg.mouse.y,mg.mouse.left,mg.mouse.right);
        println!("up: {:?}", mg.keyboard.up);

        if let engine::mouse::ButtonState::Drag{x:_,y:_} = mg.mouse.left {
            tx = mg.mouse.x;
            ty = mg.mouse.y;
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
