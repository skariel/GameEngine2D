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

extern crate glium;
extern crate scoped_threadpool;
extern crate num_cpus;

pub mod mouse;
pub mod keyboard;
pub mod window;
pub mod framerate;
pub mod tasklist;
pub mod spritelist;
pub mod frame_time;
pub mod graphics;
pub mod shapes;
pub mod camera;

const FRAMERATE_FRAMES: usize = 128;

pub struct Data<'s, SharedDataType: 's> {
    pub framerate: &'s framerate::FrameRate,
    pub mouse: &'s mouse::Mouse,
    pub keyboard: &'s keyboard::Keyboard,
    pub time: &'s frame_time::Time,
    pub camera: &'s camera::Camera,
    pub user_shared_data: &'s SharedDataType,
}

pub struct Engine<'a, SharedDataType> {
    pub graphics: graphics::Graphics<'a>,
    pub camera: camera::Camera,
    pub framerate: framerate::FrameRate,
    pub mouse: mouse::Mouse,
    pub keyboard: keyboard::Keyboard,
    pub time: frame_time::Time,
    pub tasklist: tasklist::TaskList<SharedDataType>,
    pub user_shared_data: SharedDataType,
    pool: scoped_threadpool::Pool,
    drawables: Vec<Box<tasklist::Drawable>>,
}

impl<'a, SharedDataType: Sync> Engine<'a, SharedDataType> {
    pub fn new(title: String, shared_data: SharedDataType) -> Engine<'a, SharedDataType> {
        Engine {
            graphics: graphics::Graphics::new(title),
            camera: camera::Camera::new(),
            framerate: framerate::FrameRate::new(FRAMERATE_FRAMES),
            mouse: mouse::Mouse::new(),
            keyboard: keyboard::Keyboard::new(),
            time: frame_time::Time::new(),
            tasklist: tasklist::TaskList::new(),
            user_shared_data: shared_data,
            pool: scoped_threadpool::Pool::new(num_cpus::get() as u32),
            drawables: Vec::new(),
        }
    }

    pub fn flush(&mut self) {
        self.framerate.flush();
        self.time.flush();
        self.graphics.flush();
        self.graphics.poll_events(&mut self.mouse, &mut self.keyboard);
        self.tasklist.flush_share(&mut self.user_shared_data, &mut self.camera);
        self.tasklist.flush_handle_and_draw(
            &Data {
                keyboard: &self.keyboard,
                mouse: &self.mouse,
                time: &self.time,
                framerate: &self.framerate,
                camera: &self.camera,
                user_shared_data: &self.user_shared_data,
            },
            &mut self.graphics,
            &mut self.pool,
            &mut self.drawables,
        );
    }
}
