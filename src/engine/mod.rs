extern crate glium;

pub mod mouse;
pub mod keyboard;
pub mod window;
pub mod framerate;
pub mod tasklist;
pub mod time;
pub mod draw;
pub mod graphics;
pub mod shapes;

const FRAMERATE_FRAMES: usize = 128;

pub struct Engine<'a> {
    pub graphics: graphics::Graphics<'a>,
    pub framerate: framerate::FrameRate,
    pub mouse: mouse::Mouse,
    pub keyboard: keyboard::Keyboard,
    pub time: time::Time,
    pub tasklist: tasklist::TaskList,
}

impl<'a> Engine<'a> {
    pub fn new(title: String) -> Engine<'a> {
        Engine {
            graphics: graphics::Graphics::new(title),
            framerate: framerate::FrameRate::new(FRAMERATE_FRAMES),
            mouse: mouse::Mouse::new(),
            keyboard: keyboard::Keyboard::new(),
            time: time::Time::new(),
            tasklist: tasklist::TaskList::new(),
        }
    }

    pub fn flush(&mut self) {
        self.framerate.flush();
        self.time.flush();
        self.graphics.flush();
        self.graphics.poll_events(&mut self.mouse, &mut self.keyboard);
        let drawlist = self.tasklist.flush();
        for params in drawlist.params.iter() {
            self.graphics.print_params(params);
        }
    }
}
