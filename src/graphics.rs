use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::Sdl;
use sdl2::video::Window;

use crate::bit_ops::get_bit_at_u8;
use crate::Chip8;

const CH8_WIDTH: u8 = 64;
const CH8_HEIGHT: u8 = 32;

pub struct Display {
    pub sdl_context: Sdl,
    canvas: Canvas<Window>,
    pub width: u32,
    pub height: u32,
    pub scale: u32,
}

impl Display {
    pub fn new(scale: u32) -> Result<Display, String> {
        let sdl_context = sdl2::init()?;
        let video = sdl_context.video()?;

        let width = CH8_WIDTH as u32 * scale;
        let height = CH8_HEIGHT as u32 * scale;

        let window = video
            .window("yac8", width, height)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Ok(Display { sdl_context, canvas, width, height, scale })
    }

    pub fn draw(&mut self, x: u8, y: u8, n: u8, chip8: &mut Chip8) {
        let white = Color::RGB(255, 255, 255);
        let black = Color::RGB(0, 0, 0);
        let mut flipped = false;

        for row in 0..n {
            for pix in 0..8 {
                let i_val = chip8.memory[chip8.i as usize + row as usize];
                let i_bit = get_bit_at_u8(i_val, 7 - pix);
                let curr_x = x + pix;
                let curr_y = y + row;

                if i_bit && curr_x < CH8_WIDTH && curr_y < CH8_HEIGHT {
                    let current_pixel = chip8.get_pixel(curr_y, curr_x);
                    let screen_val = i_bit ^ current_pixel;
                    if !screen_val && current_pixel { flipped = true };

                    //TODO if it's the same we don't need to redraw
                    if screen_val { self.canvas.set_draw_color(white) } else { self.canvas.set_draw_color(black); }
                    let rect = Rect::new((curr_x as u32 * self.scale) as i32, (curr_y as u32 * self.scale) as i32, self.scale, self.scale);

                    //TODO handle Result
                    self.canvas.fill_rect(rect).unwrap();
                    chip8.set_pixel(curr_y, curr_x, screen_val);
                }
            }
        }

        chip8.set_vf(flipped as u8);

        self.canvas.present();
    }

    pub fn clear(&mut self) {
        let black = Color::RGB(0, 0, 0);
        self.canvas.set_draw_color(black);
        self.canvas.clear()
    }
}


