pub struct Chip8 {
    pub memory: [u8; 4096],
    pc: u16,
    i: u16,
    v: [u8, 16], // General purpose registers
    stack: [u16, 16],
    delay_timer: u8,
    sound_timer: u8,
    display: [bool; 64 x 32],
    keyboard: [bool; 16]
}
