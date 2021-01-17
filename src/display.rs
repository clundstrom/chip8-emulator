extern crate sdl2;

use crate::globals::{*};
use sdl2::pixels::Color;
use self::sdl2::video::Window;
use self::sdl2::render::WindowCanvas;
use self::sdl2::rect::Rect;
use self::sdl2::TimerSubsystem;

pub struct Display {
    pub canvas: WindowCanvas,
    pub v_ram: [[i32; PIXEL_WIDTH]; PIXEL_HEIGHT],
    pub timer: TimerSubsystem,
}

impl Display {
    pub fn new() -> Self {
        let v_ram = [[0; PIXEL_WIDTH]; PIXEL_HEIGHT];
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let timer: TimerSubsystem = sdl_context.timer().unwrap();

        let window: Window = video_subsystem.window("CHIP8 Emulator", 800, 600)
            .position_centered()
            .build()
            .unwrap();


        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();

        Display {
            canvas,
            v_ram,
            timer,
        }
    }

    /// Print 2D array of pixels to canvas
    pub fn blit(&mut self, pixels: &mut [[i32; PIXEL_WIDTH]; PIXEL_HEIGHT]) {
        for (y, row_arr) in pixels.iter().enumerate() {
            for (x, &col_arr) in row_arr.iter().enumerate() {
                if col_arr != 0 {
                    self.canvas.set_draw_color(Color::RGB(0, 0, 0));
                } else {
                    self.canvas.set_draw_color(Color::RGB(0, 255, 255));
                }
                self.canvas
                    .fill_rect(Rect::new(x as i32, y as i32, DISPLAY_SCALE, DISPLAY_SCALE));
            }
        }
        self.canvas.present();
    }
}