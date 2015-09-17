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

extern crate time;

use time::precise_time_ns;

pub struct Time {
    pub curr_time_ms: f64,
    pub dt_ms: f64,
}

impl Time {
    pub fn new() -> Time {
        let mut t = Time { curr_time_ms: 0f64, dt_ms: 0f64 };
        t.flush();
        t
    }

    pub fn flush(&mut self) {
        let new_time = precise_time_ns() as f64 / 1000.0;
        self.dt_ms = new_time - self.curr_time_ms;
        self.curr_time_ms = new_time;
    }
}
