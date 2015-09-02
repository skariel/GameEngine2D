use engine::window;

#[derive(Debug, PartialEq)]
#[allow(float_cmp)]
pub enum ButtonState {
    Pressed,
    Released,
    Drag{x:f32, y:f32},
}

#[derive(Debug, PartialEq)]
pub enum DButtonState {
    Pressed,
    Released,
    Boring,
    Drag,
    Drop,
}

#[derive(Debug, PartialEq)]
pub enum Button {
    Left,
    Right,
    Middle,
    Other,
}

pub struct Mouse {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub wheelx: f32,
    pub wheely: f32,
    pub dwheelx: f32,
    pub dwheely: f32,
    pub left: ButtonState,
    pub right: ButtonState,
    pub middle: ButtonState,
    pub dleft: DButtonState,
    pub dright: DButtonState,
    pub dmiddle: DButtonState,
}

fn rescale_location(x:f32, y:f32, old_window: &window::Window, new_window: &window::Window) -> (f32,f32) {
    let old_fp_x = (old_window.size_pixels_x as f32)/2.0;
    let old_fp_y = (old_window.size_pixels_y as f32)/2.0;

    let new_fp_x = (new_window.size_pixels_x as f32)/2.0;
    let new_fp_y = (new_window.size_pixels_y as f32)/2.0;

    let pixel_x = (x+1.0)*old_fp_x;
    let pixel_y = (y/old_window.aspect_ratio_y+1.0)*old_fp_y;

    let scaled_x = pixel_x/new_fp_x - 1.0;
    let scaled_y = (pixel_y/new_fp_y - 1.0)*new_window.aspect_ratio_y;
    (scaled_x, scaled_y)
}

impl Mouse {
    pub fn new() -> Mouse {
        Mouse {
            x: 0.0,
            y: 0.0,
            dx: 0.0,
            dy: 0.0,
            wheelx: 0.0,
            wheely: 0.0,
            dwheelx: 0.0,
            dwheely: 0.0,
            left: ButtonState::Released,
            right: ButtonState::Released,
            middle: ButtonState::Released,
            dleft: DButtonState::Boring,
            dright: DButtonState::Boring,
            dmiddle: DButtonState::Boring,
        }
    }

    pub fn cleardiffs(&mut self) {
        self.dleft = DButtonState::Boring;
        self.dright = DButtonState::Boring;
        self.dmiddle = DButtonState::Boring;
        self.dx = 0.0;
        self.dy = 0.0;
        self.dwheelx = 0.0;
        self.dwheely = 0.0;
    }

    pub fn rescale(&mut self, old_window: &window::Window, new_window: &window::Window) {
        let (scaled_x,scaled_y) = rescale_location(self.x, self.y, old_window,new_window);
        self.dx = scaled_x-self.x;
        self.x = scaled_x;
        self.dy = scaled_y-self.y;
        self.y = scaled_y;


        // TODO: iterate over these instead of repeating everthing 3 times...
        if let ButtonState::Drag{x,y} = self.left {
            let (scaled_xi, scaled_yi) = rescale_location(x, y, old_window,new_window);
            self.left = ButtonState::Drag{x:scaled_xi, y:scaled_yi};
        };
        if let ButtonState::Drag{x,y} = self.right {
            let (scaled_xi, scaled_yi) = rescale_location(x, y, old_window,new_window);
            self.right = ButtonState::Drag{x:scaled_xi, y:scaled_yi};
        };
        if let ButtonState::Drag{x,y} = self.middle {
            let (scaled_xi, scaled_yi) = rescale_location(x, y, old_window,new_window);
            self.middle = ButtonState::Drag{x:scaled_xi, y:scaled_yi};
        };
    }

    pub fn moved(&mut self, x: i32, y:i32, window: &window::Window) {
        // updating positions...
        let fp_x = (window.size_pixels_x as f32)/2.0;
        let fp_y = (window.size_pixels_y as f32)/2.0;
        let scaled_x = (x as f32)/fp_x - 1.0;
        let scaled_y = -((y as f32)/fp_y - 1.0)*window.aspect_ratio_y;
        self.dx = scaled_x-self.x;
        self.x = scaled_x;
        self.dy = scaled_y-self.y;
        self.y = scaled_y;

        // testing for new drag
        // TODO: iterate over these?
        if self.left == ButtonState::Pressed {
            self.left = ButtonState::Drag{x:self.x, y:self.y};
        }
        if self.right == ButtonState::Pressed {
            self.right = ButtonState::Drag{x:self.x, y:self.y};
        }
        if self.middle == ButtonState::Pressed {
            self.middle = ButtonState::Drag{x:self.x, y:self.y};
        }
    }

    pub fn wheel(&mut self, dx:f32, dy:f32) {
        self.wheelx += dx;
        self.wheely += dy;
        self.dwheelx = dx;
        self.dwheely = dy;
    }

    pub fn button(&mut self, button: Button, state: ButtonState) {
        let dstate = match state {
            ButtonState::Pressed => DButtonState::Pressed,
            ButtonState::Released => DButtonState::Released,
            ButtonState::Drag{x:_,y:_} => DButtonState::Drop,
        };
        match button {
            Button::Left  => {self.left=state; self.dleft=dstate;},
            Button::Right => {self.right=state; self.dright=dstate;},
            Button::Middle  => {self.middle=state; self.dmiddle=dstate;},
            Button::Other => (),
        }
    }
}
