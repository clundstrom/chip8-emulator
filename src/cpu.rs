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
    }


    ///
    /// Matches correct OPCODE and calls for execution.
    ///
    /// Reference: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.1
    ///
    /// In these listings, the following variables are used:
    /// nnn or addr - A 12-bit value, the lowest 12 bits of the instruction
    /// n or nibble - A 4-bit value, the lowest 4 bits of the instruction
    /// x - A 4-bit value, the lower 4 bits of the high byte of the instruction
    /// y - A 4-bit value, the upper 4 bits of the low byte of the instruction
    /// kk or byte - An 8-bit value, the lowest 8 bits of the instruction
    pub fn execute(&self, opcode: u16) {
        let nnn = opcode & 0x0FFF;
        let n = opcode & 0x000F;
        let x = opcode & 0x0F00;
        let y = opcode & 0x00F0;
        let kk = opcode & 0x00FF;

        // split opcode into 4 bit nibbles for pattern matching
        let nb = (
            (opcode & 0xF000) >> 12,
            (opcode & 0x0F00) >> 8,
            (opcode & 0x00F0) >> 4,
            (opcode & 0x000F) >> 0);


        let pc_res = match nb {
            (0x00, 0x00, 0x0e, 0x00) => debug!("CLS: 00e0"),
            (0x00, 0x00, 0x0e, 0x0e) => debug!("RET: 00ee"),
            (0x00, _, _, _) => debug!("SYS ADDR: 0nnn"),
            (0x01, _, _, _) => debug!("JP addr: 1nnn"),
            (0x02, _, _, _) => debug!("call addr: 2nnn"),
            (0x03, _, _, _) => debug!("3xkk - SE Vx, byte"),
            (0x04, _, _, _) => debug!("4xkk - SNE Vx, byte"),
            (0x05, _, _, 0x00) => debug!("5xy0 - SE Vx, Vy"),
            (0x06, _, _, _) => debug!("6xkk - LD Vx, byte"),
            (0x07, _, _, _) => debug!("7xkk - ADD Vx, byte"),
            (0x08, _, _, 0x00) => debug!("8xy0 - LD Vx, Vy"),
            (0x08, _, _, 0x01) => debug!("OR Vx, Vy"),
            (0x08, _, _, 0x02) => debug!("AND Vx, Vy"),
            (0x08, _, _, 0x03) => debug!("XOR Vx, Vy"),
            (0x08, _, _, 0x04) => debug!("ADD Vx, Vy"),
            (0x08, _, _, 0x05) => debug!("SUB Vx, Vy"),
            (0x08, _, _, 0x06) => debug!("SHR Vx Vy"),
            (0x08, _, _, 0x07) => debug!("SUBN Vx, Vy"),
            (0x08, _, _, 0x0e) => debug!("SHL Vx Vy"),
            (0x09, _, _, 0x00) => debug!("SNE Vx, Vy"),
            (0x0A, _, _, _) => debug!("LD I, addr"),
            (0x0B, _, _, _) => debug!("JP V0, addr"),
            (0x0C, _, _, _) => debug!("Cxkk - RND Vx, byte"),
            (0x0D, _, _, _) => debug!("DRW Vx, Vy, nibble"),
            (0x0E, _, 0x09, 0x0E) => debug!("SKP Vx"),
            (0x0E, _, 0x0A, 0x01) => debug!("SKNP Vx"),
            (0x0F, _, 0x00, 0x07) => debug!("LD Vx, DT"),
            (0x0F, _, 0x00, 0x0A) => debug!("LD Vx, K"),
            (0x0F, _, 0x01, 0x05) => debug!("LD DT, Vx"),
            (0x0F, _, 0x01, 0x08) => debug!("LD ST, Vx"),
            (0x0F, _, 0x01, 0x0E) => debug!("ADD I, Vx"),
            (0x0F, _, 0x02, 0x09) => debug!("LD F, Vx"),
            (0x0F, _, 0x03, 0x03) => debug!("LD B, Vx"),
            (0x0F, _, 0x05, 0x05) => debug!("LD [I], Vx"),
            (0x0F, _, 0x06, 0x05) => debug!("LD Vx, [I]"),
            _ => {}
        };
        println!("This is a test");
    }
}