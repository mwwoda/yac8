use std::collections::VecDeque;

use rand::Rng;

use crate::bit_ops::{get_bit_at, to_u8};
use crate::quirks::{CH8_QUIRKS, Quirks};
use crate::registers::Registers;
use crate::to_u16;

const FONT: [u8; 80] = [0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80];  // F

const FONT_POINTER: u16 = 0x000;
const PROGRAM_POINTER: u16 = 0x200;

pub const CH8_WIDTH: u8 = 64;
pub const CH8_HEIGHT: u8 = 32;

pub type Chip8Vram = [[bool; CH8_WIDTH as usize]; CH8_HEIGHT as usize];

pub struct Chip8 {
    pub registers: Registers,
    memory: [u8; 4096],
    stack: VecDeque<u16>,
    delay_timer: u8,
    sound_timer: u8,
    pc: u16,
    pub vram: Chip8Vram,
    pub vram_changed: bool,
    pub blocked: bool,
    blocked_key_vx: u8,
    config: Config,
    pub vblank: VBLank,
    //pub v_blank: VBLank,
}

pub enum VBLank {
    WaitForDraw,
    WaitForInterrupt,
    Free,
}

pub struct Config {
    print_debug_messages: bool,
    quirks: Quirks,
}

impl Config {
    fn default() -> Self {
        Config {
            print_debug_messages: false,
            quirks: CH8_QUIRKS,
        }
    }
}

impl Chip8 {
    pub fn new(rom: Vec<u8>) -> Self {
        let mut chip8 = Chip8 {
            registers: Registers::default(),
            memory: [0; 4096],
            stack: VecDeque::new(),
            delay_timer: 0,
            sound_timer: 0,
            pc: 0x200,
            vram: [[false; CH8_WIDTH as usize]; CH8_HEIGHT as usize],
            vram_changed: false,
            blocked: false,
            blocked_key_vx: 0,
            config: Config::default(),
            vblank: VBLank::Free,
        };

        chip8.load_to_memory(&FONT, FONT_POINTER);
        chip8.load_to_memory(rom.as_slice(), PROGRAM_POINTER);

        chip8
    }

    pub fn fetch(&mut self) -> u16 {
        let instruction = to_u16!(self.memory[(self.pc) as usize], self.memory[(self.pc + 1) as usize]);
        self.pc += 2;
        instruction
    }

    pub fn load_to_memory(&mut self, data: &[u8], start_point: u16) {
        let mut curr_point = start_point;
        for byte in data {
            self.memory[curr_point as usize] = *byte;
            curr_point += 1
        }
    }

    pub fn set_pixel(&mut self, y: u8, x: u8, val: bool) {
        self.vram[y as usize][x as usize] = val;
    }

    pub fn get_pixel(&self, y: u8, x: u8) -> bool {
        self.vram[y as usize][x as usize]
    }

    pub fn handle_op_code(&mut self, hex: u16, key: Option<u8>) {
        let nibbles = (
            ((hex & 0xF000) >> 12_u8) as u8,
            ((hex & 0x0F00) >> 8_u8) as u8,
            ((hex & 0x00F0) >> 4_u8) as u8,
            (hex & 0x000F) as u8
        );
        match nibbles {
            (0x0, 0x0, 0xe, 0x0) => self.clear_display(hex),
            (0x0, 0x0, 0xe, 0xe) => self.return_sub(hex),
            (0x0, _, _, _) => panic!("UNHANDLED COMMAND - MACHINED CODE ROUTINE"),
            (0x1, n1, n2, n3) => self.jump(hex, n1, n2, n3),
            (0x2, n1, n2, n3) => self.call(hex, n1, n2, n3),
            (0x3, x, n1, n2) => self.skip_if_equal(hex, x, n1, n2),
            (0x4, x, n1, n2) => self.skip_if_not_equal(hex, x, n1, n2),
            (0x5, x, y, 0x0) => self.skip_if_registers_equal(hex, x, y),
            (0x6, x, n1, n2) => self.set_register_to(hex, x, n1, n2),
            (0x7, x, n1, n2) => self.add_value_to_register(hex, x, n1, n2),
            (0x8, x, y, 0x0) => self.set_x_to_y(hex, x, y),
            (0x8, x, y, 0x1) => self.set_x_to_y_or(hex, x, y),
            (0x8, x, y, 0x2) => self.set_x_to_y_and(hex, x, y),
            (0x8, x, y, 0x3) => self.set_x_to_y_xor(hex, x, y),
            (0x8, x, y, 0x4) => self.add_y_to_x(hex, x, y),
            (0x8, x, y, 0x5) => self.subtract_y_from_x(hex, x, y),
            (0x8, x, y, 0x6) => self.shift_right(hex, x, y),
            (0x8, x, y, 0x7) => self.subtract_x_from_y_and_assign_to_x(hex, x, y),
            (0x8, x, y, 0xe) => self.shift_left(hex, x, y),
            (0x9, x, y, 0x0) => self.skip_if_registers_not_equal(hex, x, y),
            (0xa, n1, n2, n3) => self.set_i(hex, n1, n2, n3),
            (0xb, n1, n2, n3) => self.jump_plus_v0(hex, n1, n2, n3),
            (0xc, x, n1, n2) => self.set_vx_to_rand_and_nn(hex, x, n1, n2),
            (0xd, x, y, n) => self.draw(hex, x, y, n),
            (0xe, x, 0x9, 0xe) => self.skip_if_pressed(hex, x, key),
            (0xe, x, 0xa, 0x1) => self.skip_if_not_pressed(hex, x, key),
            (0xf, x, 0x0, 0x7) => self.set_vx_to_delay(hex, x),
            (0xf, x, 0x0, 0xa) => self.get_key(hex, x),
            (0xf, x, 0x1, 0x5) => self.set_delay_timer(hex, x),
            (0xf, x, 0x1, 0x8) => self.set_sound_timer(hex, x),
            (0xf, x, 0x1, 0xe) => self.add_vx_to_i(hex, x),
            (0xf, x, 0x2, 0x9) => self.set_i_to_sprite(hex, x),
            (0xf, x, 0x3, 0x3) => self.binary_coded_decimal(hex, x),
            (0xf, x, 0x5, 0x5) => self.reg_dump(hex, x),
            (0xf, x, 0x6, 0x5) => self.reg_load(hex, x),
            _ => panic!("{:#06x} {hex} not recognized command", hex)
        }
    }

    fn skip(&mut self) {
        self.pc += 2;
    }

    fn clear_display(&mut self, hex: u16) {
        self.print_debug_message(hex, "Clear Display");
        self.vram = [[false; CH8_WIDTH as usize]; CH8_HEIGHT as usize];
        self.vram_changed = true;
    }

    pub fn draw(&mut self, hex: u16, x: u8, y: u8, n: u8) {
        if self.config.quirks.display_wait(&mut self.vblank) {
            self.pc -= 2;
            return;
        }

        self.print_debug_message(hex, "Draw");
        let vx = self.registers.get(x) & 63;
        let vy = self.registers.get(y) & 31;
        let mut flipped = false;

        for row in 0..n {
            for pix in 0..8 {
                let i_val = self.memory[self.registers.i as usize + row as usize];
                let i_bit = get_bit_at(i_val, 7 - pix);
                let curr_x = vx + pix;
                let curr_y = vy + row;

                if i_bit && curr_x < CH8_WIDTH && curr_y < CH8_HEIGHT {
                    let current_pixel = self.get_pixel(curr_y, curr_x);
                    let screen_val = i_bit ^ current_pixel;
                    if !screen_val && current_pixel { flipped = true };

                    self.set_pixel(curr_y, curr_x, screen_val);
                    self.vram_changed = true;
                }
            }
        }

        self.registers.set_vf(flipped as u8);
    }

    fn set_i(&mut self, hex: u16, n1: u8, n2: u8, n3: u8) {
        self.print_debug_message(hex, "Set I");
        let addr = to_u16!(n1, n2, n3);
        self.registers.i = addr;
    }

    fn set_register_to(&mut self, hex: u16, x: u8, n1: u8, n2: u8) {
        self.print_debug_message(hex, "Sets Vx = NN");
        let val = to_u8(n1, n2);
        self.registers.set(x, val);
    }

    fn add_value_to_register(&mut self, hex: u16, x: u8, n1: u8, n2: u8) {
        self.print_debug_message(hex, "Sets Vx += NN");
        let val = to_u8(n1, n2) as u16;
        let res = self.registers.get(x) as u16 + val;
        self.registers.set(x, res as u8);
    }

    fn jump(&mut self, hex: u16, n1: u8, n2: u8, n3: u8) {
        self.print_debug_message(hex, "Jump to NNN");
        let addr = to_u16!(n1, n2, n3);
        self.pc = addr;
    }

    fn set_x_to_y(&mut self, hex: u16, x: u8, y: u8) {
        self.print_debug_message(hex, "Sets Vx = Vy");
        self.registers.set(x, self.registers.get(y));
    }

    fn set_x_to_y_or(&mut self, hex: u16, x: u8, y: u8) {
        self.print_debug_message(hex, "Sets Vx |= Vy");
        self.registers.or(x, y);
        self.config.quirks.vf_reset(&mut self.registers);
    }

    fn set_x_to_y_and(&mut self, hex: u16, x: u8, y: u8) {
        self.print_debug_message(hex, "Sets Vx &= Vy");
        self.registers.and(x, y);
        self.config.quirks.vf_reset(&mut self.registers);
    }

    fn set_x_to_y_xor(&mut self, hex: u16, x: u8, y: u8) {
        self.print_debug_message(hex, "Sets Vx ^= Vy");
        self.registers.xor(x, y);
        self.config.quirks.vf_reset(&mut self.registers);
    }

    fn add_y_to_x(&mut self, hex: u16, x: u8, y: u8) {
        self.print_debug_message(hex, "Sets Vx += Vy");
        let res = self.registers.get(x) as u16 + self.registers.get(y) as u16;
        self.registers.set(x, res as u8);
        self.registers.set_vf(get_bit_at(res, 8) as u8);
    }

    fn subtract_y_from_x(&mut self, hex: u16, x: u8, y: u8) {
        self.print_debug_message(hex, "Sets Vx -= Vy");
        let borrow = u8::from(self.registers.get(y) < self.registers.get(x));
        let res = self.registers.get(x) as i16 - self.registers.get(y) as i16;
        self.registers.set(x, res as u8);
        self.registers.set_vf(borrow);
    }

    fn shift_right(&mut self, hex: u16, x: u8, y: u8) {
        self.print_debug_message(hex, "Sets Vx >>= 1");
        self.config.quirks.shifting(&mut self.registers, x, y);
        let lsb = get_bit_at(self.registers.get(x), 0);
        self.registers.shift_right(x, 1);
        self.registers.set_vf(lsb as u8);
    }

    fn subtract_x_from_y_and_assign_to_x(&mut self, hex: u16, x: u8, y: u8) {
        self.print_debug_message(hex, "Sets Vx = Vy - Vx");
        let borrow = u8::from(self.registers.get(x) < self.registers.get(y));
        let res = self.registers.get(y) as i16 - self.registers.get(x) as i16;
        self.registers.set(x, res as u8);
        self.registers.set_vf(borrow);
    }

    fn shift_left(&mut self, hex: u16, x: u8, y: u8) {
        self.print_debug_message(hex, "Sets Vx <<= 1");
        self.config.quirks.shifting(&mut self.registers, x, y);
        self.registers.shift_left(x, 1);
        let msb = get_bit_at(self.registers.get(x), 7);
        self.registers.set_vf(msb as u8);
    }

    fn skip_if_equal(&mut self, hex: u16, x: u8, n1: u8, n2: u8) {
        self.print_debug_message(hex, "Skip if Vx == NN");
        let val = to_u8(n1, n2);
        if self.registers.get(x) == val { self.skip() }
    }

    fn skip_if_not_equal(&mut self, hex: u16, x: u8, n1: u8, n2: u8) {
        self.print_debug_message(hex, "Skip if Vx != NN");
        let val = to_u8(n1, n2);
        if self.registers.get(x) != val { self.skip() }
    }

    fn skip_if_registers_equal(&mut self, hex: u16, x: u8, y: u8) {
        self.print_debug_message(hex, "Skip if Vx == Vy");
        if self.registers.get(x) == self.registers.get(y) { self.skip() }
    }

    fn skip_if_registers_not_equal(&mut self, hex: u16, x: u8, y: u8) {
        self.print_debug_message(hex, "Skip if Vx != Vy");
        if self.registers.get(x) != self.registers.get(y) { self.skip() }
    }

    fn jump_plus_v0(&mut self, hex: u16, n1: u8, n2: u8, n3: u8) {
        self.print_debug_message(hex, "Jump to PC = V0 + NNN");
        let addr = to_u16!(n1, n2, n3);
        self.pc = addr + self.config.quirks.jumping(&mut self.registers, n1) as u16;
    }

    fn set_vx_to_delay(&mut self, hex: u16, x: u8) {
        self.print_debug_message(hex, "Sets Vx = delay");
        self.registers.set(x, self.delay_timer);
    }

    fn set_delay_timer(&mut self, hex: u16, x: u8) {
        self.print_debug_message(hex, "Sets delay = Vx");
        self.delay_timer = self.registers.get(x);
    }

    fn set_sound_timer(&mut self, hex: u16, x: u8) {
        self.print_debug_message(hex, "Sets sound = Vx");
        self.sound_timer = self.registers.get(x);
    }

    fn add_vx_to_i(&mut self, hex: u16, x: u8) {
        self.print_debug_message(hex, "Sets I += Vx");
        self.registers.i += self.registers.get(x) as u16;
    }

    fn call(&mut self, hex: u16, n1: u8, n2: u8, n3: u8) {
        self.print_debug_message(hex, "Sets sub");
        let addr = to_u16!(n1, n2, n3);
        self.stack.push_front(self.pc);
        self.pc = addr;
    }

    fn return_sub(&mut self, hex: u16) {
        self.print_debug_message(hex, "Return sub");
        let addr = self.stack.pop_front().unwrap();
        self.pc = addr;
    }

    fn skip_if_pressed(&mut self, hex: u16, x: u8, key: Option<u8>) {
        self.print_debug_message(hex, "Skip if key == Vx");
        if let Some(k) = key {
            if k == self.registers.get(x) {
                self.skip();
            }
        }
    }

    fn skip_if_not_pressed(&mut self, hex: u16, x: u8, key: Option<u8>) {
        self.print_debug_message(hex, "Skip if key != Vx");
        if let Some(k) = key {
            if k == self.registers.get(x) {
                return;
            }
        }
        self.skip();
    }

    fn binary_coded_decimal(&mut self, hex: u16, x: u8) {
        self.print_debug_message(hex, "Binary coded decimal");

        let mut curr_val = self.registers.get(x);
        let binary_hundred = curr_val / 100;
        self.memory[self.registers.i as usize] = binary_hundred;
        curr_val -= binary_hundred * 100;

        let binary_tens = curr_val / 10;
        self.memory[(self.registers.i + 1) as usize] = binary_tens;
        curr_val -= binary_tens * 10;

        self.memory[(self.registers.i + 2) as usize] = curr_val;
    }

    fn reg_dump(&mut self, hex: u16, x: u8) {
        self.print_debug_message(hex, "Reg dump");
        for n in 0..=x {
            self.memory[(self.registers.i + n as u16) as usize] = self.registers.get(n);
        }
        self.config.quirks.memory(&mut self.registers, x as u16);
    }

    fn reg_load(&mut self, hex: u16, x: u8) {
        self.print_debug_message(hex, "Reg load");
        for n in 0..=x {
            self.registers.set(n, self.memory[(self.registers.i + n as u16) as usize]);
        }
        self.config.quirks.memory(&mut self.registers, x as u16);
    }

    fn set_vx_to_rand_and_nn(&mut self, hex: u16, x: u8, n1: u8, n2: u8) {
        self.print_debug_message(hex, "Set VX to rand() & NN");
        let val = to_u8(n1, n2) as u16;
        let mut rng = rand::thread_rng();
        self.registers.set(x, (rng.gen_range(0..256) & val) as u8);
    }

    fn get_key(&mut self, hex: u16, x: u8) {
        self.print_debug_message(hex, "Waiting for key");
        self.blocked_key_vx = x;
        self.blocked = true;
    }

    fn set_i_to_sprite(&mut self, hex: u16, x: u8) {
        self.print_debug_message(hex, "Set I to value of sprite at Vx");
        let character = self.registers.get(x);
        self.registers.i = FONT_POINTER + (character * 5) as u16;
    }

    pub fn set_key(&mut self, key: u8) {
        self.registers.set(self.blocked_key_vx, key);
        self.blocked = false;
    }

    fn decrement_delay_timer(&mut self) {
        self.delay_timer = self.delay_timer.saturating_sub(1)
    }

    fn decrement_sound_timer(&mut self) {
        self.sound_timer = self.sound_timer.saturating_sub(1)
    }

    pub fn decrement_timers(&mut self) {
        self.decrement_delay_timer();
        self.decrement_sound_timer();
    }

    fn print_debug_message(&self, hex: u16, name: &str) {
        if self.config.print_debug_messages { println!("{:#06x} {}", hex, name) }
    }
}