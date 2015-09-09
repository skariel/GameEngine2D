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

use std::marker;
use engine;
use engine::{tasklist, camera};

/**************************************************************
 *    MODEL IMPLEMENTATION!
 **************************************************************/

pub trait InnerModel<SharedDataType> {
    fn handle(&mut self, shared_data: &engine::Data<SharedDataType>);
    fn share(&self, shared_data: &mut SharedDataType, camera: &mut camera::Camera);
}

struct MySpriteModel<SharedDataType, InnerModelType>
where InnerModelType: InnerModel<SharedDataType> {
    inner_models:  Vec<InnerModelType>,
    _phantom: marker::PhantomData<SharedDataType>,
}

impl<SharedDataType, InnerModelType> tasklist::Model<SharedDataType>
for MySpriteModel<SharedDataType, InnerModelType>
where InnerModelType: Send + InnerModel<SharedDataType>, SharedDataType: Send {
    fn handle(&mut self, shared_data: &engine::Data<SharedDataType>) {
        for inner_model in &mut self.inner_models {
            inner_model.handle(shared_data);
        }
    }
}

/**************************************************************
 *    DRAWABLE IMPLEMENTATION!
 **************************************************************/

pub trait InnerDrawable<DrawableUserDataType> {
    fn draw(&self, user_data: &DrawableUserDataType, camera: &camera::Camera, graphics: &mut engine::graphics::Graphics);
}

struct MySpriteDrawable<InnerDrawableType, DrawableUserDataType>
where InnerDrawableType: InnerDrawable<DrawableUserDataType> {
    inner_drawables: Vec<InnerDrawableType>,
    drawable_user_data: DrawableUserDataType,
}

impl<InnerDrawableType, DrawableUserDataType> tasklist::Drawable
for MySpriteDrawable<InnerDrawableType, DrawableUserDataType>
where InnerDrawableType: InnerDrawable<DrawableUserDataType> {
    fn draw(&self, camera: &camera::Camera, graphics: &mut engine::graphics::Graphics) {
        for drawable in &self.inner_drawables {
            drawable.draw(&self.drawable_user_data, camera, graphics);
        }
    }
}

/**************************************************************
 *    TASK IMPLEMENTATION!
 **************************************************************/

pub trait InnerTask<SharedDataType, InnerModelType, InnerDrawableType, DrawableUserDataType>
where InnerModelType: InnerModel<SharedDataType>, InnerDrawableType: InnerDrawable<DrawableUserDataType> {
    fn get_drawable(&self, inner_model: &InnerModelType) -> InnerDrawableType;
    fn get_user_drawable_data(&self) -> DrawableUserDataType;
}

struct MySpriteTask<InnerModelType, InnerTaskType, SharedDataType, InnerDrawableType, DrawableUserDataType>
where
    InnerModelType: InnerModel<SharedDataType>,
    InnerDrawableType: InnerDrawable<DrawableUserDataType>,
    InnerTaskType: InnerTask<SharedDataType, InnerModelType, InnerDrawableType, DrawableUserDataType> {

    model: MySpriteModel<SharedDataType, InnerModelType>,
    inner_task: InnerTaskType,
    _phantom1: marker::PhantomData<SharedDataType>,
    _phantom2: marker::PhantomData<InnerDrawableType>,
    _phantom3: marker::PhantomData<DrawableUserDataType>,
}

impl<InnerModelType, InnerTaskType, InnerDrawableType, SharedDataType, DrawableUserDataType> tasklist::Task<SharedDataType>
for MySpriteTask<InnerModelType, InnerTaskType, SharedDataType, InnerDrawableType, DrawableUserDataType>
where
    SharedDataType: Send,
    DrawableUserDataType: 'static,
    InnerModelType: Send + InnerModel<SharedDataType>,
    InnerDrawableType: 'static + InnerDrawable<DrawableUserDataType>,
    InnerTaskType: InnerTask<SharedDataType, InnerModelType, InnerDrawableType, DrawableUserDataType> {

    fn get_drawable(&self) -> Option<Box<tasklist::Drawable>> {
        let mut inner_drawables = Vec::new();
        for model in &self.model.inner_models {
            inner_drawables.push(self.inner_task.get_drawable(model));
        }
        Some(Box::new(MySpriteDrawable {
            inner_drawables: inner_drawables,
            drawable_user_data: self.inner_task.get_user_drawable_data(),
        }))
    }

    fn get_new_tasks(&mut self) -> Option<Vec<Box<tasklist::Task<SharedDataType>>>> {
        // TODO: elaborate here :)
        None
    }
    fn get_model(&self) -> &tasklist::Model<SharedDataType> {
        &self.model
    }
    fn get_model_mut(&mut self) -> &mut tasklist::Model<SharedDataType> {
        &mut self.model
    }
    fn share(&self, shared_data: &mut SharedDataType, camera: &mut camera::Camera) {
        for model in &self.model.inner_models {
            model.share(shared_data, camera);
        }
    }
    fn get_state(&self) -> tasklist::TaskState {
        // TODO: elaborate here :)
        tasklist::TaskState::Keep
    }
}
