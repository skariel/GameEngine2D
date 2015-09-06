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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KeyState {
    Pressed,
    Released,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DKeyState {
    Pressed,
    Released,
    Boring,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct KeyId(pub u8);
pub const KEYID_UP        : KeyId = KeyId(111);
pub const KEYID_DOWN      : KeyId = KeyId(116);
pub const KEYID_LEFT      : KeyId = KeyId(113);
pub const KEYID_RIGHT     : KeyId = KeyId(114);
pub const KEYID_SPACE     : KeyId = KeyId(65);
pub const KEYID_BACKSPACE : KeyId = KeyId(22);
pub const KEYID_ENTER     : KeyId = KeyId(36);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    Space,
    Enter,
    Backspace,
    Other{id:KeyId},
}

pub struct Keyboard {
    pub up        : KeyState,
    pub dup       : DKeyState,
    pub down      : KeyState,
    pub ddown     : DKeyState,
    pub left      : KeyState,
    pub dleft     : DKeyState,
    pub right     : KeyState,
    pub dright    : DKeyState,
    pub space     : KeyState,
    pub dspace    : DKeyState,
    pub enter     : KeyState,
    pub denter    : DKeyState,
    pub backspace : KeyState,
    pub dbackspace: DKeyState,
    pub keys      : [KeyState; 256],
    pub dkeys     : [DKeyState; 256],
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            up        : KeyState::Released,
            dup       : DKeyState::Boring,
            down      : KeyState::Released,
            ddown     : DKeyState::Boring,
            left      : KeyState::Released,
            dleft     : DKeyState::Boring,
            right     : KeyState::Released,
            dright    : DKeyState::Boring,
            space     : KeyState::Released,
            dspace    : DKeyState::Boring,
            enter     : KeyState::Released,
            denter    : DKeyState::Boring,
            backspace : KeyState::Released,
            dbackspace: DKeyState::Boring,
            keys      : [KeyState::Released; 256],
            dkeys     : [DKeyState::Boring; 256],
        }
    }

    pub fn cleardiffs(&mut self) {
        for i in 0..127 {
            self.dkeys[i as usize] = DKeyState::Boring;
        }
    }

    pub fn key(&mut self, keyid: KeyId, state: KeyState) {
        let dstate = match self.keys[keyid.0 as usize] {
            KeyState::Pressed => DKeyState::Pressed,
            KeyState::Released => DKeyState::Released,
        };
        self.keys[keyid.0 as usize] = state;
        self.dkeys[keyid.0 as usize] = dstate;
        match keyid {
            KEYID_UP => { self.up=state; self.dup=dstate;},
            KEYID_DOWN => { self.down=state; self.ddown=dstate;},
            KEYID_LEFT => { self.left=state; self.dleft=dstate;},
            KEYID_RIGHT => { self.right=state; self.dright=dstate;},
            KEYID_SPACE => { self.space=state; self.dspace=dstate;},
            KEYID_BACKSPACE => { self.backspace=state; self.dbackspace=dstate;},
            KEYID_ENTER => { self.enter=state; self.denter=dstate;},
            _ => (),
        }
    }
}
