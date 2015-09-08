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

/*
Calculate the framerate based on the last `total_frames` frames
*/

pub struct FrameRate {
    frames_ns: Vec<u64>, // this contains the time for each frame in nano seconds
    frame_f: usize, // final frame index
    frame_i: usize, // initial frame index
}

impl FrameRate {
    pub fn new(total_frames: usize) -> FrameRate {
        let mut v: Vec<u64> = Vec::new();
        for _ in 0..total_frames {
            v.push(0);
        };
        v[total_frames-1] = 1;
        FrameRate { frames_ns: v, frame_f: total_frames - 1, frame_i: 0 }
    }
    pub fn flush(&mut self) {
        // insert new frame
        self.frame_f += 1;
        if self.frame_f == self.frames_ns.len() {
            self.frame_f = 0;
        }
        self.frame_i += 1;
        if self.frame_i == self.frames_ns.len() {
            self.frame_i = 0;
        }
        let newest_frame_time = precise_time_ns();
        self.frames_ns[self.frame_f] = newest_frame_time;
    }
    pub fn get_framerate(&self) -> f64 {
        let interval = self.frames_ns[self.frame_f] - self.frames_ns[self.frame_i];
        ((self.frames_ns.len()-1) as f64) / (interval as f64) * 1000000000.0
    }
}
