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
        match instruction & 0xF000 {
            0x0000 => match instruction {
                0x00E0 => self.clear_screen(),
                0x00EE => {},
                _ => panic!("Unimplemented instruction: {:x}", instruction),
            },
            0x1000 => {
                let address = instruction & 0x0FFF;
                self.jump_to_addr(address);
            },
            0x6000 => {
                let x = instruction & 0x0F00;
                let value = instruction & 0x00FF;
                self.set_reg(x as u8, value as u8);
            },
            0x7000 => {
                let x = instruction & 0x0F00;
                let value = instruction & 0x00FF;
                self.add_value_to_reg(x as u8, value as u8);
            },
            0xA000 => {
                let value = instruction & 0x0FFF;
                self.set_index_reg(value);
            },
            0xD000 => {
                let x = (instruction & 0x0F00) as u8;
                let y = (instruction & 0x00F0) as u8;
                let n = (instruction & 0x000F) as u8;
                self.draw_screen(x, y, n);
            }
            _ => panic!("Unimplemented instruction: {:x}", instruction),
        };
    }

    pub fn fetch(&mut self) -> u16 {
        // Each instruction is 2 bytes. This means we have to read 2 words.
        let first_byte: u8 = self.memory[self.pc as usize];
        println!("First byte {:x}", first_byte);
        let second_byte: u8 = self.memory[(self.pc as usize) + 1];
        println!("Second byte {:x}", second_byte);

        let instruction: u16 = ((first_byte as u16) << 8) | second_byte as u16;

        //Increment pc
        self.pc += 2;

        return instruction;
    }

    pub fn clear_screen(&mut self) {
        self.display.clear_screen();
    }

    pub fn draw_screen(&mut self, x: u8, y: u8, n: u8) {
        // Draw an n pixel long sprite at x and y
        let x_coord = self.v[x as usize] % BUFFER_WIDTH as u8;
        let y_coord = self.v[y as usize] % BUFFER_HEIGHT as u8;

        let sprite_address = self.i;

        for i in y_coord..(y_coord + n) {
            // A sprite address contains an 8 bit character
            let sprite: u8 = self.memory[sprite_address as usize];
            // Find the start index of the buffer
            let buffer_start_index = (i * BUFFER_WIDTH as u8) + x_coord;

            let mut buffer_value: u8 = 0;
            for j in 0..8 {
                buffer_value = (buffer_value << 1) | (self.display.buffer[(j + buffer_start_index) as usize] as u8);
            }

            // Xor the sprite and the display buffer to find out the new state of the
            // buffer
            let new_value_for_buffer = sprite ^ buffer_value;

            // Write this to the display buffer
            for j in 0..8 {

                // Right shift the new value by j and AND by 1. For example when j is 1:
                // Original: 10110101
                //
                // After RS: 01011010
                // 1 in BIN: 00000001
                // AND both: 00000000 ( 0 in decimal. This is the second last bit of the original value)
                
                self.display.buffer[(buffer_start_index as usize) + 7 - j] = ((new_value_for_buffer >> j) & 1) as u32;
            }

            // This is chip8 behaviour. If the sprite changes the display buffer, set the
            // vf register to 1.
            if new_value_for_buffer != sprite {
                self.v[0xf] = 1;
            }
        }

        self.display.draw_screen();
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
