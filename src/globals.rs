pub(crate) const PIXEL_WIDTH: usize = 64;
pub(crate) const PIXEL_HEIGHT: usize = 32;
pub(crate) const DISPLAY_SCALE: u32 = 10;
pub(crate) const DISPLAY_FREQ: u32 = 60;
pub(crate) const TIME_STEP_MS: u32 = 1000 / DISPLAY_FREQ;
pub(crate) const RAM_SIZE: u16 = 4096;
pub(crate) const STACK_SIZE: u16 = 16;
pub(crate) const PC_START: u16 = 0x200;
pub(crate) const FONT_SIZE: u8 = 5;
pub(crate) const FONT_DEFAULTS: [u8; 80] = [
    0xF0,
    0x90,
    0x90,
    0x90,
    0xF0,
    0x20,
    0x60,
    0x20,
    0x20,
    0x70,
    0xF0,
    0x10,
    0xF0,
    0x80,
    0xF0,
    0xF0,
    0x10,
    0xF0,
    0x10,
    0xF0,
    0x90,
    0x90,
    0xF0,
    0x10,
    0x10,
    0xF0,
    0x80,
    0xF0,
    0x10,
    0xF0,
    0xF0,
    0x80,
    0xF0,
    0x90,
    0xF0,
    0xF0,
    0x10,
    0x20,
    0x40,
    0x40,
    0xF0,
    0x90,
    0xF0,
    0x90,
    0xF0,
    0xF0,
    0x90,
    0xF0,
    0x10,
    0xF0,
    0xF0,
    0x90,
    0xF0,
    0x90,
    0x90,
    0xE0,
    0x90,
    0xE0,
    0x90,
    0xE0,
    0xF0,
    0x80,
    0x80,
    0x80,
    0xF0,
    0xE0,
    0x90,
    0x90,
    0x90,
    0xE0,
    0xF0,
    0x80,
    0xF0,
    0x80,
    0xF0,
    0xF0,
    0x80,
    0xF0,
    0x80,
    0x80
];