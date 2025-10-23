pub struct Chip8 {
    pub memory: [u8; 4096],
    pub pc: u16,
    pub i: u16,
    pub v: [u8; 16], // General purpose registers
    pub stack: [u16; 16],
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub display: [bool; 64 * 32],
    pub keypad: [bool; 16]
}

impl Chip8 {
    pub fn decode_and_execute(&mut self, instruction: String) {
    }

    pub fn fetch(&self) -> u16 {
        // Each instruction is 2 bytes. This means we have to read 2 words.
        let first_byte: u8 = self.memory[pc];
        let second_byte: u8 = self.memory[pc + 1];

        let instruction: u16 = ((first_byte as u16) << 8) | second_byte as u16;

        //Increment pc
        self.pc += 1;

        return instruction;
    }

    pub fn clear_screen() {
    }

    pub fn draw_screen() {
    }

    pub fn jump_to_addr(address: u16) {
    }

    pub fn set_reg(reg_no: u8, value: u8) {
    }

    pub fn add_value_to_reg(reg_no: u8, value: u8) {
    }

    pub fn set_index_reg(value: u16) {
    }

}
