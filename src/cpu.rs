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
    pub ram: [u8; RAM_SIZE as usize],
    pub delay_timer: u8,
    pub sound_timer: u8,
}


impl Cpu {
    pub fn new() -> Self {
        Self {
            register: Register::new(),
            stack: [0 as u16; STACK_SIZE as usize],
            stack_ptr: 0,
            ram: [0 as u8; RAM_SIZE as usize],
            delay_timer: 0,
            sound_timer: 0,
        }
    }
    /// Loads base fonts 0-10, A-F into interpreter section 0x000-0x1FF RAM
    pub fn init_font(&mut self) {
        for i in 0..FONT_DEFAULTS.len() {
            self.ram[i] = FONT_DEFAULTS[i];
        }
    }

    fn decr_timer(&self, mut timer: u8) {
        if timer > 0 {
            timer -= 1;
        }
    }

    pub fn tick(&mut self) {
        self.decr_timer(self.delay_timer);
        self.decr_timer(self.sound_timer);

        // get opcode
        let test = self.register.pc +1;
        let opcode = (self.ram[self.register.pc + 1] as u16);

        // run opcode
        let a = 2;

    }
}