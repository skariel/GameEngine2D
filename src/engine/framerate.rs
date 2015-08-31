extern crate time;

use time::precise_time_ns;

/*
Calculate the framerate based on the last `total_frames` frames
*/

pub struct FrameRate {
    frames_ns: Vec<u64>, // this contains the time for each frame in nano seconds
    frame_f: usize,      // final frame index
    frame_i: usize,      // initial frame index
}

impl FrameRate {
    pub fn new(total_frames: usize) -> FrameRate {
        let mut v: Vec<u64> = Vec::new();
        for _ in 0..total_frames {
            v.push(0);
        };
        v[total_frames-1] = 1;
        FrameRate {
            frames_ns: v,
            frame_f: total_frames-1,
            frame_i: 0,
        }
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
