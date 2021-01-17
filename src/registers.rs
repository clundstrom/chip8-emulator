/// Details of a CHIP-8 registry
///
/// * Vx 16 General purpose 8-bit registers denoted Vx where x =  ```[0-F]```
/// * i  Word register
/// * pc Program counter: Programs in Chip-8 start at 0x200 (512).
/// The first 512 bits are reserved for interpreter.
pub struct Register {
    pub v0: u8,
    pub v1: u8,
    pub v2: u8,
    pub v3: u8,
    pub v4: u8,
    pub v5: u8,
    pub v6: u8,
    pub v7: u8,
    pub v8: u8,
    pub v9: u8,
    pub va: u8,
    pub vb: u8,
    pub vc: u8,
    pub vd: u8,
    pub ve: u8,

    // Carry flag. Set to 1 if any screen pixels are flipped from set to unset when a sprite is drawn and set to 0 otherwise
    pub vf: u8,
    // Long register for mem addr
    pub i: u16,
    // Program counter
    pub pc: u16,
}


impl Register {
    pub fn new() -> Self {
        Self {
            v0: 0 as u8,
            v1: 0 as u8,
            v2: 0 as u8,
            v3: 0 as u8,
            v4: 0 as u8,
            v5: 0 as u8,
            v6: 0 as u8,
            v7: 0 as u8,
            v8: 0 as u8,
            v9: 0 as u8,
            va: 0 as u8,
            vb: 0 as u8,
            vc: 0 as u8,
            vd: 0 as u8,
            ve: 0 as u8,
            vf: 0 as u8,
            i: 0 as u16,
            pc: 0x200,
        }
    }
}
