use crate::registers::Register;
use crate::globals::{*};
use log::{debug, warn, info, error};

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

    /// Load binary data to RAM starting from PC_START (0x200)
    pub fn load_rom(&mut self, data: &Vec<u8>) {
        if (PC_START + data.len() as u16 > 0xE8F) {
            panic!("Access violation: Writing to illegal memory.")
        }
        for i in 0..data.len() {
            self.write_byte(PC_START + i as u16, data[i]);
        }
    }

    /// Write byte to ram address
    pub fn write_byte(&mut self, addr: u16, val: u8) {
        self.ram[addr as usize] = val;
    }

    /// Read byte from 2 byte RAM address
    pub fn read_byte(&mut self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    /// One tick represents one CPU instruction
    /// Moves the program counter one step forward
    pub fn tick(&mut self) {
        self.decr_timer(self.delay_timer);
        self.decr_timer(self.sound_timer);

        let hi_byte = self.read_byte(self.register.pc as u16) as u16;
        let lo_byte = self.read_byte((self.register.pc + 1) as u16) as u16;

        // Shift hi byte to MSB and compose opcode
        let opcode: u16 = (hi_byte << 8 | lo_byte) as u16;

        debug!("OPCODE: {:#X}", opcode);

        // execute opcode
        self.execute(opcode);

        // move pc counter
        self.register.pc += 2;
    }


    pub fn execute(&self, opcode: u16) {
        println!("This is a test");
    }
}