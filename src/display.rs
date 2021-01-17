extern crate sdl2;

use crate::globals::{*};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use self::sdl2::video::Window;
use self::sdl2::render::WindowCanvas;

pub struct Display {
    canvas: WindowCanvas,
    v_ram: [[i32; PIXEL_WIDTH]; PIXEL_HEIGHT],
}

impl Display {
    pub fn new() -> Self {
        let v_ram = [[0; PIXEL_WIDTH]; PIXEL_HEIGHT];
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window: Window = video_subsystem.window("CHIP-8 Emulator", 800, 600)
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
        }
    }
}