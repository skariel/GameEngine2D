extern crate glium;

pub mod mouse;
pub mod keyboard;
pub mod window;
pub mod framerate;
pub mod tasklist;
pub mod time;
pub mod surface;
pub mod graphics;
pub mod shapes;
pub mod camera;

const FRAMERATE_FRAMES: usize = 128;

pub struct Data<'s, T: 's>  {
    pub framerate: &'s framerate::FrameRate,
    pub mouse: &'s mouse::Mouse,
    pub keyboard: &'s keyboard::Keyboard,
    pub time: &'s time::Time,
    pub camera: &'s camera::Camera,
    pub shared: &'s T,
}

pub struct Engine<'a, T> {
    pub graphics: graphics::Graphics<'a>,
    pub framerate: framerate::FrameRate,
    pub mouse: mouse::Mouse,
    pub keyboard: keyboard::Keyboard,
    pub time: time::Time,
    pub tasklist: tasklist::TaskList<T>,
    pub shared_data: T,
}

impl<'a, T> Engine<'a, T> {
    pub fn new(title: String, shared_data: T) -> Engine<'a, T> {
        Engine {
            graphics: graphics::Graphics::new(title),
            framerate: framerate::FrameRate::new(FRAMERATE_FRAMES),
            mouse: mouse::Mouse::new(),
            keyboard: keyboard::Keyboard::new(),
            time: time::Time::new(),
            tasklist: tasklist::TaskList::new(),
            shared_data: shared_data,
        }
    }

    pub fn flush(&mut self) {
        self.framerate.flush();
        self.time.flush();
        self.graphics.flush();
        self.graphics.poll_events(&mut self.mouse, &mut self.keyboard);
        self.tasklist.flush_share(&mut self.shared_data, &mut self.graphics.camera);
        let surface = self.tasklist.flush_handle_and_draw(
            &Data {
                keyboard: &self.keyboard,
                mouse: &self.mouse,
                time: &self.time,
                framerate: &self.framerate,
                camera: &self.graphics.camera,
                shared: &self.shared_data,
            }
        );
        for params in surface.drawparams.iter() {
            self.graphics.print_params(params);
        }
    }
}
