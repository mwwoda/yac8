use std::collections::VecDeque;

use rand::Rng;

use crate::bit_ops::{get_bit_at_u16, get_bit_at_u8, to_u16, to_u16_from_three, to_u8};
use crate::graphics::Display;

const PROGRAM_POINTER: u16 = 0x200;
const FONT_POINTER: u16 = 0x000;
const CH8_WIDTH: u8 = 64;
const CH8_HEIGHT: u8 = 32;
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

pub struct Chip8 {
    registers: [u8; 16],
    pub i: u16,
    pub memory: [u8; 4096],
    stack: VecDeque<u16>,
    delay_timer: u8,
    sound_timer: u8,
    pub pc: u16,
    pixels: [[bool; CH8_WIDTH as usize]; CH8_HEIGHT as usize],
    pub blocked: bool,
    pub blocked_key_vx: u8,
    config: Config,
}

pub struct Config {
    print_debug_messages: bool,
}

impl Config {
    pub fn new() -> Self {
        Config {
            print_debug_messages: false
        }
    }
}

impl Chip8 {
    pub fn new(ch8_data: Vec<u8>) -> Self {
        let mut memory = [0; 4096];

        Chip8::load_to_memory(&mut memory, &FONT, FONT_POINTER);
        Chip8::load_to_memory(&mut memory, ch8_data.as_slice(), PROGRAM_POINTER);

        Chip8 {
            registers: [0; 16],
            i: 0,
            memory,
            stack: VecDeque::new(),
            delay_timer: 0,
            sound_timer: 0,
            pc: 0x200,
            pixels: [[false; CH8_WIDTH as usize]; CH8_HEIGHT as usize],
            blocked: false,
            blocked_key_vx: 0,
            config: Config::new(),
        }
    }

    pub fn fetch(&mut self) -> u16 {
        //TODO bound check
        let instruction = to_u16(self.memory[(self.pc) as usize], self.memory[(self.pc + 1) as usize]);
        self.pc += 2;
        instruction
    }

    fn load_to_memory(memory: &mut [u8], data: &[u8], start_point: u16) {
        let mut curr_point = start_point;
        for byte in data {
            memory[curr_point as usize] = *byte;
            curr_point += 1
        }
    }

    pub fn set_pixel(&mut self, y: u8, x: u8, val: bool) {
        self.pixels[y as usize][x as usize] = val;
    }

    pub fn get_pixel(&self, y: u8, x: u8) -> bool {
        self.pixels[y as usize][x as usize]
    }

    pub fn set_vf(&mut self, val: u8) {
        self.registers[0xf] = val
    }

    pub fn handle_op_code(&mut self, hex: u16, display: &mut Display, key: Option<u8>) {
        let nibbles = (
            ((hex & 0xF000) >> 12_u8) as u8,
            ((hex & 0x0F00) >> 8_u8) as u8,
            ((hex & 0x00F0) >> 4_u8) as u8,
            (hex & 0x000F) as u8
        );
        match nibbles {
            (0x0, 0x0, 0xe, 0x0) => self.clear_display(hex, display),
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
            (0x8, x, _, 0x6) => self.shift_right(hex, x),
            (0x8, x, y, 0x7) => self.subtract_x_from_y_and_assign_to_x(hex, x, y),
            (0x8, x, _, 0xe) => self.shift_left(hex, x),
            (0x9, x, y, 0x0) => self.skip_if_registers_not_equal(hex, x, y),
            (0xa, n1, n2, n3) => self.set_i(hex, n1, n2, n3),
            (0xb, n1, n2, n3) => self.jump_plus_v0(hex, n1, n2, n3),
            (0xc, x, n1, n2) => self.set_vx_to_rand_and_nn(hex, x, n1, n2),
            (0xd, x, y, n) => self.draw(hex, display, x, y, n),
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

    fn clear_display(&mut self, hex: u16, display: &mut Display) {
        self.print_debug_message(hex, "Clear Display");
        self.pixels = [[false; CH8_WIDTH as usize]; CH8_HEIGHT as usize];
        display.clear();
    }

    fn draw(&mut self, hex: u16, display: &mut Display, x: u8, y: u8, n: u8) {
        self.print_debug_message(hex, "Draw");
        let vx = self.registers[x as usize] & 63;
        let vy = self.registers[y as usize] & 31;

        display.draw(vx, vy, n, self);
    }

    fn set_i(&mut self, hex: u16, n1: u8, n2: u8, n3: u8) {
        self.print_debug_message(hex, "Set I");
        let addr = to_u16_from_three(n1, n2, n3);
        self.i = addr;
    }

    fn set_register_to(&mut self, hex: u16, x: u8, n1: u8, n2: u8) {
        self.print_debug_message(hex, "Sets Vx = NN");
        let val = to_u8(n1, n2);
        self.registers[x as usize] = val;
    }

    fn add_value_to_register(&mut self, hex: u16, x: u8, n1: u8, n2: u8) {
        self.print_debug_message(hex, "Sets Vx += NN");
        let val = to_u8(n1, n2) as u16;
        let res = self.registers[x as usize] as u16 + val;
        self.registers[x as usize] = res as u8;
    }

    fn jump(&mut self, hex: u16, n1: u8, n2: u8, n3: u8) {
        self.print_debug_message(hex, "Jump to NNN");
        let addr = to_u16_from_three(n1, n2, n3);
        self.pc = addr;
    }

    fn set_x_to_y(&mut self, hex: u16, x: u8, y: u8) {
        self.print_debug_message(hex, "Sets Vx = Vy");
        self.registers[x as usize] = self.registers[y as usize];
    }

    fn set_x_to_y_or(&mut self, hex: u16, x: u8, y: u8) {
        self.print_debug_message(hex, "Sets Vx |= Vy");
        self.registers[x as usize] |= self.registers[y as usize];
    }

    fn set_x_to_y_and(&mut self, hex: u16, x: u8, y: u8) {
        self.print_debug_message(hex, "Sets Vx &= Vy");
        self.registers[x as usize] &= self.registers[y as usize];
    }

    fn set_x_to_y_xor(&mut self, hex: u16, x: u8, y: u8) {
        self.print_debug_message(hex, "Sets Vx ^= Vy");
        self.registers[x as usize] ^= self.registers[y as usize];
    }

    fn add_y_to_x(&mut self, hex: u16, x: u8, y: u8) {
        self.print_debug_message(hex, "Sets Vx += Vy");
        let res = self.registers[x as usize] as u16 + self.registers[y as usize] as u16;
        self.registers[x as usize] = res as u8;
        self.set_vf(get_bit_at_u16(res, 8) as u8);
    }

    fn subtract_y_from_x(&mut self, hex: u16, x: u8, y: u8) {
        self.print_debug_message(hex, "Sets Vx -= Vy");
        let borrow = u8::from(self.registers[y as usize] < self.registers[x as usize]);
        let res = self.registers[x as usize] as i16 - self.registers[y as usize] as i16;
        self.registers[x as usize] = res as u8;
        self.set_vf(borrow);
    }

    fn shift_right(&mut self, hex: u16, x: u8) {
        self.print_debug_message(hex, "Sets Vx >>= 1");
        let lsb = get_bit_at_u8(self.registers[x as usize], 0);
        self.registers[x as usize] >>= 1;
        self.set_vf(lsb as u8);
    }

    fn subtract_x_from_y_and_assign_to_x(&mut self, hex: u16, x: u8, y: u8) {
        self.print_debug_message(hex, "Sets Vx = Vy - Vx");
        let borrow = u8::from(self.registers[x as usize] < self.registers[y as usize]);
        let res = self.registers[y as usize] as i16 - self.registers[x as usize] as i16;
        self.registers[x as usize] = res as u8;
        self.set_vf(borrow);
    }

    fn shift_left(&mut self, hex: u16, x: u8) {
        self.print_debug_message(hex, "Sets Vx <<= 1");
        self.registers[x as usize] <<= 1;
        let msb = get_bit_at_u8(self.registers[x as usize], 7);
        self.set_vf(msb as u8);
    }

    fn skip_if_equal(&mut self, hex: u16, x: u8, n1: u8, n2: u8) {
        self.print_debug_message(hex, "Skip if Vx == NN");
        let val = to_u8(n1, n2);
        if self.registers[x as usize] == val { self.skip() }
    }

    fn skip_if_not_equal(&mut self, hex: u16, x: u8, n1: u8, n2: u8) {
        self.print_debug_message(hex, "Skip if Vx != NN");
        let val = to_u8(n1, n2);
        if self.registers[x as usize] != val { self.skip() }
    }

    fn skip_if_registers_equal(&mut self, hex: u16, x: u8, y: u8) {
        self.print_debug_message(hex, "Skip if Vx == Vy");
        if self.registers[x as usize] == self.registers[y as usize] { self.skip() }
    }

    fn skip_if_registers_not_equal(&mut self, hex: u16, x: u8, y: u8) {
        self.print_debug_message(hex, "Skip if Vx != Vy");
        if self.registers[x as usize] != self.registers[y as usize] { self.skip() }
    }

    fn jump_plus_v0(&mut self, hex: u16, n1: u8, n2: u8, n3: u8) {
        self.print_debug_message(hex, "Jump to PC = V0 + NNN");
        let addr = to_u16_from_three(n1, n2, n3);
        self.pc = addr + self.registers[0] as u16;
    }

    fn set_vx_to_delay(&mut self, hex: u16, x: u8) {
        self.print_debug_message(hex, "Sets Vx = delay");
        self.registers[x as usize] = self.delay_timer;
    }

    fn set_delay_timer(&mut self, hex: u16, x: u8) {
        self.print_debug_message(hex, "Sets delay = Vx");
        self.delay_timer = self.registers[x as usize];
    }

    fn set_sound_timer(&mut self, hex: u16, x: u8) {
        self.print_debug_message(hex, "Sets sound = Vx");
        self.sound_timer = self.registers[x as usize];
    }

    fn add_vx_to_i(&mut self, hex: u16, x: u8) {
        self.print_debug_message(hex, "Sets I += Vx");
        self.i += self.registers[x as usize] as u16;
    }

    fn call(&mut self, hex: u16, n1: u8, n2: u8, n3: u8) {
        self.print_debug_message(hex, "Sets sub");
        let addr = to_u16_from_three(n1, n2, n3);
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
            if k == self.registers[x as usize] {
                self.skip();
            }
        }
    }

    fn skip_if_not_pressed(&mut self, hex: u16, x: u8, key: Option<u8>) {
        self.print_debug_message(hex, "Skip if key != Vx");
        if let Some(k) = key {
            if k == self.registers[x as usize] {
                return;
            }
        }
        self.skip();
    }

    fn binary_coded_decimal(&mut self, hex: u16, x: u8) {
        self.print_debug_message(hex, "Binary coded decimal");

        let mut curr_val = self.registers[x as usize];
        let binary_hundred = curr_val / 100;
        self.memory[self.i as usize] = binary_hundred;
        curr_val -= binary_hundred * 100;

        let binary_tens = curr_val / 10;
        self.memory[(self.i + 1) as usize] = binary_tens;
        curr_val -= binary_tens * 10;

        self.memory[(self.i + 2) as usize] = curr_val;
    }

    fn reg_dump(&mut self, hex: u16, x: u8) {
        self.print_debug_message(hex, "Reg dump");
        for n in 0..=x {
            self.memory[(self.i + n as u16) as usize] = self.registers[n as usize];
        }
    }

    fn reg_load(&mut self, hex: u16, x: u8) {
        self.print_debug_message(hex, "Reg load");
        for n in 0..=x {
            self.registers[n as usize] = self.memory[(self.i + n as u16) as usize];
        }
    }

    fn set_vx_to_rand_and_nn(&mut self, hex: u16, x: u8, n1: u8, n2: u8) {
        self.print_debug_message(hex, "Set VX to rand() & NN");
        let val = to_u8(n1, n2) as u16;
        let mut rng = rand::thread_rng();
        self.registers[x as usize] = (rng.gen_range(0..256) & val) as u8;
    }

    fn get_key(&mut self, hex: u16, x: u8) {
        self.print_debug_message(hex, "Waiting for key");
        self.blocked_key_vx = x;
        self.blocked = true;
    }

    fn set_i_to_sprite(&mut self, hex: u16, x: u8) {
        self.print_debug_message(hex, "Set I to value of sprite at Vx");
        let character = self.registers[x as usize];
        self.i = FONT_POINTER + (character * 5) as u16;
    }

    pub fn set_key(&mut self, key: u8) {
        self.registers[self.blocked_key_vx as usize] = key;
        self.blocked = false;
    }

    pub fn decrement_delay_timer(&mut self) {
        self.delay_timer = self.delay_timer.saturating_sub(1)
    }

    pub fn decrement_sound_timer(&mut self) {
        self.sound_timer = self.sound_timer.saturating_sub(1)
    }

    fn print_debug_message(&self, hex: u16, name: &str) {
        if self.config.print_debug_messages { println!("{:#06x} {}", hex, name) }
    }
}