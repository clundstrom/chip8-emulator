const PIXEL_WIDTH: i32 = 64;
const PIXEL_HEIGHT: i32 = 32;

extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub struct Display {}

impl Display {
    pub fn main(&self) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();
    }
}