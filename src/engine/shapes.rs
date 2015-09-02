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
use engine::graphics;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 4]
}
implement_vertex!(Vertex, position, color);

pub fn get_triangle(g: &graphics::Graphics) -> glium::VertexBuffer<Vertex> {
    let triangle =
        [
            Vertex { position: [-0.2, -0.2], color: [0.0,1.0,1.0,0.5]},
            Vertex { position: [ 0.0,  0.2], color: [1.0,0.0,1.0,0.5]},
            Vertex { position: [ 0.2, -0.2], color: [1.0,1.0,0.0,0.5]}
        ];
    g.compile(&triangle)
}
