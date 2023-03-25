use sdl2::event::Event;
use sdl2::keyboard::Scancode;

pub struct Input {
    event_pump: sdl2::EventPump,
}

impl Input {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        //TODO handle Result
        Input { event_pump: sdl_context.event_pump().unwrap() }
    }

    pub fn poll_keys(&mut self) -> Option<u8> {
        for ev in self.event_pump.poll_iter() {
            if let Event::Quit { .. } = ev {
                panic!("Quit event")
            }
        }

        for (scancode, res) in self.event_pump.keyboard_state().scancodes() {
            //TODO handle multiple key presses at once
            match self.to_chip8_key(scancode) {
                Some(key) if res => return Some(key),
                _ => {}
            }
        }

        None
    }

    fn to_chip8_key(&self, scancode: Scancode) -> Option<u8> {
        match scancode {
            Scancode::Num1 => Some(0x1),
            Scancode::Num2 => Some(0x2),
            Scancode::Num3 => Some(0x3),
            Scancode::Num4 => Some(0xc),
            Scancode::Q => Some(0x4),
            Scancode::W => Some(0x5),
            Scancode::E => Some(0x6),
            Scancode::R => Some(0xd),
            Scancode::A => Some(0x7),
            Scancode::S => Some(0x8),
            Scancode::D => Some(0x9),
            Scancode::F => Some(0xe),
            Scancode::Z => Some(0xa),
            Scancode::X => Some(0x0),
            Scancode::C => Some(0xb),
            Scancode::V => Some(0xf),
            _ => { None }
        }
    }
}
