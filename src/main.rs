mod cpu;
mod registers;
mod display;
mod globals;

use crate::cpu::Cpu;

use log::{warn, info, error};
use crate::globals::DISPLAY_FREQ;

const TIME_STEP_MS: u32 = 1000 / DISPLAY_FREQ;

fn main() {
    env_logger::init();

    // Get program entry arguments
    let args: Vec<String> = std::env::args().collect();

    // Init CPU, RAM, TIMERS, STACK, SP
    let mut cpu = Cpu::new();

    // Load base fonts to ram
    cpu.init_font();

    // V_RAM, GFX driver
    let mut display = display::Display::new();

    // Prepare clock cycles
    let mut next_step = display.timer.ticks();

    // Emulation loop
    loop {
        let now = display.timer.ticks();

        if next_step <= now {

            while next_step <= now {

                // CPU stuff??? verify

                next_step += TIME_STEP_MS; // 1 Tick
            }

            // Blit graphics
            // Sound
            // Store input
        } else {
            // Hold up there Sonic, you're going too fast.
            display.timer.delay(next_step - now);
        }
    }


    // Test
    let mut test = [[0; 64]; 32];
    // y, x
    test[16][32] = 1;


    display.blit(&mut test);

    info!("{}", cpu.ram[0xFFF]);
}
