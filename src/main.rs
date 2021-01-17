mod cpu;

mod registers;
mod display;
mod globals;
use crate::cpu::Cpu;

use log::{warn, info, error};
// MEM FROM 0x000 (0) to 0xFFF (4095
// The first 512 bytes, from 0x000 to 0x1FF, are where the original interpreter was located, and should not be used by programs.

fn main() {
    env_logger::init();

    // Get program entry arguments
    let args: Vec<String> = std::env::args().collect();

    let display = display::Display::new();

    let cpu = Cpu::new();
    info!("{}", cpu.ram[0xFFF]);
    info!("the end")
}
