use sdl2;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::CHIP8_SCREEN_WIDTH;
use crate::CHIP8_SCREEN_HEIGHT;

const SCALE_FACTOR: u32 = 7;
const SCREEN_WIDTH: u32 = (CHIP8_SCREEN_WIDTH as u32) * SCALE_FACTOR;
const SCREEN_HEIGHT: u32 = (CHIP8_SCREEN_HEIGHT as u32) * SCALE_FACTOR;

pub struct DisplayDriver {
    canvas: Canvas<Window>,
}

impl DisplayDriver {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video_subsys = sdl_context.video().unwrap();
        let window = video_subsys
            .window(
                "rust-sdl2_gfx: draw line & FPSManager",
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
            )
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(pixels::Color::RGB(100, 100, 100));
        canvas.clear();
        canvas.present();

        DisplayDriver { canvas: canvas }
    }

    pub fn draw(&mut self, pixels: &[[bool; CHIP8_SCREEN_WIDTH]; CHIP8_SCREEN_HEIGHT]) {
        for (y, row) in pixels.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let x = (x as u32) * SCALE_FACTOR;
                let y = (y as u32) * SCALE_FACTOR;

                self.canvas.set_draw_color(color(u8::from(col)));
                let _ = self.canvas
                    .fill_rect(Rect::new(x as i32, y as i32, SCALE_FACTOR, SCALE_FACTOR));
            }
        }
        self.canvas.present();
    }
}

fn color(value: u8) -> pixels::Color {
    if value == 0 {
        pixels::Color::RGB(150, 210, 150)
    } else {
        pixels::Color::RGB(30, 40, 30)
    }
}