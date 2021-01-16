use crate::registers::Register;


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
    pub stack: [u16; 16],
    pub stack_ptr: u16,
    pub ram: [u8; 4096]
}


impl Cpu {
    pub fn new() -> Self {
        Self {
            register: Register::new(),
            stack: [0 as u16; 16],
            stack_ptr: 0,
            ram: [0 as u8; 4096]
            // Timer
            // Delay
        }
    }
}