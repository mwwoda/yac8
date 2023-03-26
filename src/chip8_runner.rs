use std::{thread, time};

use crate::chip8::Chip8;
use crate::input::Input;
use crate::sdl_driver::SDLDriver;

pub fn run_with_sdl(mut chip8: Chip8, scale: u32) {
    let clock_speed = 500;
    let refresh_rate = 60;
    let cycles_per_frame = clock_speed / refresh_rate;
    //TODO handle Result
    let mut sdl_driver = SDLDriver::new(scale).unwrap();
    let mut input = Input::new(&sdl_driver.sdl_context);

    let sleep_time = time::Duration::from_millis(((1.0 / refresh_rate as f64) * 1000.0) as u64);

    loop {
        //TODO better way than sleeping, calculate target_instructions and executed_instructions
        // if executed_cycles == cycles_per_frame {
        //     let elapsed_time = now.elapsed();
        // }
        thread::sleep(sleep_time);
        //TODO fix timers
        chip8.decrement_delay_timer();
        //TODO sound support
        chip8.decrement_sound_timer();
        for _ in 0..cycles_per_frame {
            let key = input.poll_keys();
            if chip8.blocked {
                if let Some(k) = key { chip8.set_key(k) }
                continue;
            }

            let instruction = chip8.fetch();
            chip8.handle_op_code(instruction, key);
            if chip8.vram_changed {
                sdl_driver.draw(&chip8);
                chip8.vram_changed = false;
            }
        }
    }
}