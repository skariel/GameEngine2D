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

use glium;
use engine::shapes::Vertex;

pub struct DrawParams<'s> {
    pub shape: &'s glium::VertexBuffer<Vertex>,
    pub tx: f32,
    pub ty: f32,
    pub rotation: f32,
    pub zoom_x: f32,
    pub zoom_y: f32,
}

pub struct Surface<'s>{
    pub drawparams: Vec<DrawParams<'s>>,
}

impl<'s> Surface<'s> {
    pub fn new() -> Surface<'s> {
        Surface {
            drawparams: Vec::new(),
        }
    }

    pub fn draw(&mut self, shape: &'s glium::VertexBuffer<Vertex>, tx: f32, ty: f32, rotation: f32, zoom_x: f32, zoom_y: f32) {
        self.drawparams.push(
            DrawParams{
                    shape: shape,
                    tx: tx,
                    ty: ty,
                    rotation: rotation,
                    zoom_x: zoom_x,
                    zoom_y: zoom_y,
            }
        );
    }
}
