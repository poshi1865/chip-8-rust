use crate::display::Display;

use crate::display::BUFFER_HEIGHT;
use crate::display::BUFFER_WIDTH;

pub struct Chip8 {
    pub memory: [u8; 4096],
    pub pc: u16,
    pub i: u16,
    pub v: [u8; 16], // General purpose registers
    pub stack: [u16; 16],
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub display: Display,
    pub keypad: [bool; 16]
}

impl Chip8 {
    pub fn decode_and_execute(&mut self, instruction: u16) {
    }

    pub fn fetch(&mut self) -> u16 {
        // Each instruction is 2 bytes. This means we have to read 2 words.
        let first_byte: u8 = self.memory[self.pc as usize];
        let second_byte: u8 = self.memory[(self.pc as usize) + 1];

        let instruction: u16 = ((first_byte as u16) << 8) | second_byte as u16;

        //Increment pc
        self.pc += 1;

        return instruction;
    }

    pub fn clear_screen(&mut self) {
        self.display.clear_screen();
    }

    pub fn draw_screen(&mut self, x: u8, y: u8, n: u16) {
        // Draw an n pixel long sprite at x and y
        let x_coord = self.v[x] % BUFFER_WIDTH;
        let y_coord = self.v[y] % BUFFER_HEIGHT;

        let sprite_address = self.i;

        for i in 0..n {
            for i in buffer.iter_mut() {
                *i = 0x05a;
                break;
            }
        }


        self.display.draw_screen(buffer);
    }

    pub fn jump_to_addr(&mut self, address: u16) {
        self.pc = address;
    }

    pub fn set_reg(&mut self, reg_no: u8, value: u8) {
        self.v[reg_no as usize] = value;
    }

    pub fn add_value_to_reg(&mut self, reg_no: u8, value: u8) {
        self.v[reg_no as usize] = self.v[reg_no as usize] + value;
    }

    pub fn set_index_reg(&mut self, value: u16) {
        self.i = value;
    }
}
