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

use engine::tasklist;


trait Sprite {

}

struct SpriteList<SpriteType: Sprite> {
    pub sprites: Vec<SpriteType>,
}

impl<SharedDataType, SpriteType> Model<SharedDataType>
for  SpriteList<SpriteType>
where SpriteType: Sprite {
    fn handle(&mut self, shared_data: &engine::Data<SharedDataType>) {

    }
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
    fn get_state(&self) -> TaskState {TaskState::Keep}
}
