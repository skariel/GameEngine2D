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

use engine;
use engine::{camera, scoped_threadpool};

#[derive(PartialEq)]
pub enum TaskState {
    Remove,
    Continue,
}

pub trait Model<SharedDataType> : Send {
    fn handle(&mut self, shared_data: &engine::Data<SharedDataType>);
}

pub trait Drawable {
    fn draw(&self, camera: &camera::Camera, graphics: &mut engine::graphics::Graphics);
}

pub trait Task<SharedDataType> {
    fn get_model(&self) -> &Model<SharedDataType>;
    fn get_model_mut(&mut self) -> &mut Model<SharedDataType>;
    fn get_new_tasks(&mut self) -> Option<Vec<Box<Task<SharedDataType>>>> {None}
    fn get_drawable(&self) -> Option<Box<Drawable>> {None}
    #[allow(unused_variables)]
    fn share(&self, shared_data: &mut SharedDataType, camera: &mut camera::Camera) {}
    fn get_state(&self) -> TaskState {TaskState::Continue}
}

pub struct TaskList<SharedDataType> {
    tasks: Vec<Box<Task<SharedDataType>>>,
}

impl<SharedDataType: Sync> TaskList<SharedDataType> {
    pub fn new() -> TaskList<SharedDataType> {
        TaskList{
            tasks: Vec::new(),
        }
    }

    pub fn add(&mut self, task: Box<Task<SharedDataType>>) {
        self.tasks.push(task);
    }

    pub fn flush_share(&self, shared_data: &mut SharedDataType, camera: &mut camera::Camera) {
        for task in self.tasks.iter() {
            task.share(shared_data, camera);
        }
    }

    pub fn flush_handle_and_draw(&mut self, shared_data: &engine::Data<SharedDataType>, graphics: &mut engine::graphics::Graphics, pool: &mut scoped_threadpool::Pool, drawables: &mut Vec<Box<Drawable>>) {
        // handle and draw...
        drawables.clear();
        for task in self.tasks.iter() {
            if let Some(drawable) = task.get_drawable() {
                drawables.push(drawable);
            }
        }
        pool.scoped(|scope| {
            for task in self.tasks.iter_mut() {
                let mut model = task.get_model_mut();
                scope.execute(move || {
                    model.handle(shared_data);
                });
            }
            for drawable in drawables {
                drawable.draw(&shared_data.camera, graphics);
            }
        });
        // remove tasks
        self.tasks.retain(|task| task.get_state() != TaskState::Remove);
        // add new tasks
        let mut newtasks: Vec<Box<Task<SharedDataType>>> = Vec::new();
        for task in self.tasks.iter_mut() {
            if let Some(tasks) = task.get_new_tasks() {
                newtasks.extend(tasks.into_iter());
            }
        }
        self.tasks.extend(newtasks.into_iter());
    }
}
