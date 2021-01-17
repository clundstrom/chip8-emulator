use crate::registers::Register;
use crate::globals::{*};

/// Implementation of CPU
///
/// * 16x8 bit register.
/// * 16x16 bit stack to store return addresses
/// * 4096 byte RAM. 8 bit array.
/// * Timer todo
/// * Delay todo
///
pub struct Cpu {
    pub register: Register,
    pub stack: [u16; STACK_SIZE as usize],
    pub stack_ptr: u16,
    pub ram: [u8; RAM_SIZE as usize]
}


impl Cpu {
    pub fn new() -> Self {
        Self {
            register: Register::new(),
            stack: [0 as u16; STACK_SIZE],
            stack_ptr: 0,
            ram: [0 as u8; RAM_SIZE]
            // Timer
            // Delay

        }
    }
    /// Loads base fonts into RAM to interpreter section 0x000-0x1FF
    pub fn init_font(&mut self){
        for i in 0..FONT_DEFAULTS.len(){
            self.ram[i] = FONT_DEFAULTS[i];
        }
    }
}