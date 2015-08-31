extern crate time;

use time::precise_time_ns;

pub struct Time {
    pub curr_time_ms: f64,
    pub dt_ms: f64,
}

impl Time {
    pub fn new() -> Time {
        let mut t = Time{
            curr_time_ms: 0f64,
            dt_ms: 0f64,
        };
        t.flush();
        t
    }

    pub fn flush(&mut self) {
        let new_time = precise_time_ns() as f64 / 1000.0;
        self.dt_ms = new_time - self.curr_time_ms;
        self.curr_time_ms = new_time;
    }
}
