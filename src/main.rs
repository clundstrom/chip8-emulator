mod cpu;
mod registers;
mod display;
mod globals;

use std::fs::File;
use std::io::Read;
use crate::cpu::Cpu;
use crate::globals::{*};


fn main() {
    env_logger::init();

    // Get program entry arguments
    let args: Vec<String> = std::env::args().collect();

    // Init CPU, RAM, TIMERS, STACK, SP
    let mut cpu: Cpu = Cpu::new();

    // Load base fonts to ram
    cpu.init_font();

    let mut file = File::open("games/INVADERS").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data).expect("File not found!");

    cpu.load_rom(&data);
    // V_RAM, GFX driver
    let mut display = display::Display::new();

    // Prepare clock cycles
    let mut next_step = display.timer.ticks();

    // Emulation loop
    loop {
        let now = display.timer.ticks();

        if next_step <= now {
            while next_step <= now {

                // CPU tick -> reduce delay timer, sound timer
                cpu.tick();
                next_step += TIME_STEP_MS; // 1 Tick
            }
            // Blit graphics
            // Sound
            // Store input
        } else {
            // woah slow down Sonic, you're going too fast.
            display.timer.delay(next_step - now);
        }
    }
}
