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

pub struct Window {
    pub size_pixels_x: u32,
    pub size_pixels_y: u32,
    pub aspect_ratio_y: f32,
    pub closed: bool,
}

impl Window {
    pub fn new(size_pixels_x: u32, size_pixels_y: u32) -> Window {
        let aspect_ratio_y: f32 = (size_pixels_y as f32)/(size_pixels_x as f32);
        Window{
            size_pixels_x: size_pixels_x,
            size_pixels_y: size_pixels_y,
            aspect_ratio_y: aspect_ratio_y,
            closed: false,
        }
    }
}
