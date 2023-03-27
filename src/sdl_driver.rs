use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::Sdl;
use sdl2::video::Window;

use crate::chip8::{CH8_HEIGHT, CH8_WIDTH, Chip8};

const WHITE: Color = Color::RGB(255, 255, 255);
const BLACK: Color = Color::RGB(0, 0, 0);

pub struct SDLDriver {
    pub sdl_context: Sdl,
    canvas: Canvas<Window>,
    pub width: u32,
    pub height: u32,
    pub scale: u32,
}

impl SDLDriver {
    pub fn new(scale: u32) -> Result<SDLDriver, String> {
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

        canvas.set_draw_color(BLACK);
        canvas.clear();
        canvas.present();

        Ok(SDLDriver { sdl_context, canvas, width, height, scale })
    }

    pub fn draw(&mut self, chip8: &Chip8) {
        for (iy, y) in chip8.vram.iter().enumerate() {
            for (ix, x) in y.iter().enumerate() {
                if *x { self.canvas.set_draw_color(WHITE) } else { self.canvas.set_draw_color(BLACK); }
                let rect = Rect::new((ix as u32 * self.scale) as i32, (iy as u32 * self.scale) as i32, self.scale, self.scale);
                //TODO if it's the same we don't need to redraw
                self.canvas.fill_rect(rect).unwrap();
            }
        }

        self.canvas.present();
    }
}


