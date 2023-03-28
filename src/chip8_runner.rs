use std::{thread, time};

use crate::chip8::Chip8;
use crate::input::Input;
use crate::sdl_driver::SDLDriver;

const CLOCK_SPEED: u32 = 500;
const REFRESH_RATE: u32 = 60;
const CYCLES_PER_FRAME: u32 = CLOCK_SPEED / REFRESH_RATE;

pub fn run_with_sdl(chip8: &mut Chip8, scale: u32) {
    let mut sdl_driver = SDLDriver::new(scale).unwrap();
    let mut input = Input::new(&sdl_driver.sdl_context);

    let sleep_time = time::Duration::from_millis(((1.0 / REFRESH_RATE as f64) * 1000.0) as u64);

    loop {
        for _ in 0..CYCLES_PER_FRAME {
            let key = input.poll_keys();
            if chip8.blocked {
                if let Some(k) = key { chip8.set_key(k) }
                continue;
            }

            chip8.execute_next_opcode(key);
        }


        chip8.handle_vblank();

        if chip8.vram_changed {
            sdl_driver.draw(chip8);
            chip8.vram_changed = false;
        }

        //TODO better way than sleeping, calculate target_instructions and executed_instructions
        // if executed_cycles == cycles_per_frame {
        //     let elapsed_time = now.elapsed();
        // }
        thread::sleep(sleep_time);

        //TODO fix timers
        //TODO sound support
        chip8.decrement_timers();
    }
}

pub fn run_stop_on_blocked(chip8: &mut Chip8) {
    loop {
        for _ in 0..CYCLES_PER_FRAME {
            chip8.execute_next_opcode(None);

            if chip8.blocked {
                return;
            }
        }

        chip8.handle_vblank();
        chip8.decrement_timers();
    }
}

pub fn run_for_cycles(chip8: &mut Chip8, cycles: u16) {
    let mut elapsed_cycles = 0;
    loop {
        for _ in 0..CYCLES_PER_FRAME {
            chip8.execute_next_opcode(None);

            elapsed_cycles += 1;
            if elapsed_cycles == cycles {
                return;
            }
        }

        chip8.handle_vblank();
        chip8.decrement_timers();
    }
}