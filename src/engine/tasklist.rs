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
    Draw,
    DontDraw,
}

pub trait Model<T> : Send {
    fn handle(&mut self, data: &engine::Data<T>);
    #[allow(unused_variables)]
    fn share(&self, data: &mut T, camera: &mut camera::Camera) {}
    fn get_state(&self) -> TaskState {TaskState::DontDraw}
}

pub trait Task<T> {
    fn get_model(&self) -> &Model<T>;
    fn get_mut_model(&mut self) -> &mut Model<T>;
    fn get_new_tasks(&mut self) -> Option<Vec<Box<Task<T>>>> {None}
    #[allow(unused_variables)]
    fn draw<'k>(&self, camera: &camera::Camera, graphics: &mut engine::graphics::Graphics) {}
}

pub struct TaskList<T> {
    tasks: Vec<Box<Task<T>>>,
}

impl<T: Sync> TaskList<T> {
    pub fn new() -> TaskList<T> {
        TaskList{
            tasks: Vec::new(),
        }
    }

    pub fn add(&mut self, task: Box<Task<T>>) {
        self.tasks.push(task);
    }

    pub fn flush_share(&self, shared_data: &mut T, camera: &mut camera::Camera) {
        for task in self.tasks.iter() {
            task.get_model().share(shared_data, camera);
        }
    }

    fn handle_everybody(&mut self, data: &engine::Data<T>, pool: &mut scoped_threadpool::Pool) {
        pool.scoped(|scope| {
            for task in self.tasks.iter_mut() {
                let mut model = task.get_mut_model();
                scope.execute(move || {
                    model.handle(data);
                });
            }
        });
    }

    fn draw_everybody(&self, camera: &engine::camera::Camera, graphics: &mut engine::graphics::Graphics) {
        for task in self.tasks.iter() {
            if task.get_model().get_state() != TaskState::Draw {
                continue
            }
            task.draw(camera, graphics);
        }
    }

    pub fn flush_handle_and_draw(&mut self, data: &engine::Data<T>, graphics: &mut engine::graphics::Graphics, pool: &mut scoped_threadpool::Pool) {
        // handle everybody
        self.handle_everybody(data, pool);
        // draw!
        self.draw_everybody(&data.camera, graphics);
        // remove tasks
        self.tasks.retain(|task| task.get_model().get_state() != TaskState::Remove);
        // add new tasks
        let mut newtasks: Vec<Box<Task<T>>> = Vec::new();
        for task in self.tasks.iter_mut() {
            if let Some(tasks) = task.get_new_tasks() {
                newtasks.extend(tasks.into_iter());
            }
        }
        self.tasks.extend(newtasks.into_iter());
    }
}
