mod cpu;
mod registers;
mod display;
mod globals;

use crate::cpu::Cpu;

use log::{warn, info, error};

fn main() {
    env_logger::init();

    // Get program entry arguments
    let args: Vec<String> = std::env::args().collect();

    let mut display = display::Display::new();


    let mut test = [[0;64]; 32];

    // y, x
    test[16][32] = 1;

    display.blit(&mut test);

    let mut cpu = Cpu::new();
    cpu.init_font();

    info!("{}", cpu.ram[0xFFF]);
    info!("the end")
}
