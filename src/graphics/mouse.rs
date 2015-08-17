#[derive(Debug)]
#[derive(PartialEq)]
pub enum ButtonState {
    Pressed,
    Released,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum DButtonState {
    Pressed,
    Released,
    Boring,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Button {
    Left,
    Right,
    Middle,
    Other,
}

pub struct Mouse {
    pub x: i32,
    pub y: i32,
    pub dx: i32,
    pub dy: i32,
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

impl Mouse {
    pub fn new() -> Mouse {
        Mouse {
            x: 0,
            y: 0,
            dx: 0,
            dy: 0,
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
        self.dx = 0;
        self.dy = 0;
        self.dwheelx = 0.0;
        self.dwheely = 0.0;
    }

    pub fn moved(&mut self, x: i32, y:i32) {
        self.dx = x-self.x;
        self.x = x;
        self.dy = y-self.y;
        self.y = y;
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
        };
        match button {
            Button::Left  => {self.left=state; self.dleft=dstate;},
            Button::Right => {self.right=state; self.dright=dstate;},
            Button::Middle  => {self.middle=state; self.dmiddle=dstate;},
            _ => (),
        }
    }
}
