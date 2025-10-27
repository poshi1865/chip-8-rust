use crate::display::Display;
use crate::display::BUFFER_HEIGHT;
use crate::display::BUFFER_WIDTH;

pub struct Chip8 {
    pub memory: [u8; 4096],
    pub pc: u16,
    pub i: u16,
    pub v: [u8; 16], // General purpose registers
    pub stack: [u16; 17], // Size is 17 because stack pointer starts from 1
    pub stack_pointer: usize,
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
                let x = (instruction & 0x0F00) >> 8;
                let value = instruction & 0x00FF;
                self.set_reg(x as usize, value as u8);
            },
            0x7000 => {
                let x = (instruction & 0x0F00) >> 8;
                let value = instruction & 0x00FF;
                self.add_value_to_reg(x as usize, value as u8);
            },
            0xA000 => {
                let value = instruction & 0x0FFF;
                self.set_index_reg(value);
            },
            0xD000 => {
                let x = ((instruction & 0x0F00) >> 8) as u8;
                let y = ((instruction & 0x00F0) >> 4) as u8;
                let n = (instruction & 0x000F) as u8;
                self.draw_screen(x, y, n);
            }
            _ => panic!("Unimplemented instruction: {:x}", instruction),
        };
    }

    pub fn fetch(&mut self) -> u16 {
        // Each instruction is 2 bytes. This means we have to read 2 words.
        let first_byte: u8 = self.memory[self.pc as usize];
        let second_byte: u8 = self.memory[(self.pc as usize) + 1];

        let instruction: u16 = ((first_byte as u16) << 8) | second_byte as u16;

        //Increment pc
        self.pc += 2;

        return instruction;
    }

    fn clear_screen(&mut self) {
        self.display.clear_screen();
    }

    fn draw_screen(&mut self, x: u8, y: u8, n: u8) {
        // Draw an n pixel long sprite at x and y
        let x_coord = self.v[x as usize] % BUFFER_WIDTH as u8;
        let y_coord = self.v[y as usize] % BUFFER_HEIGHT as u8;

        let mut sprite_address = self.i;

        for i in y_coord..(y_coord + n) {
            // A sprite address contains an 8 bit character
            let sprite: u8 = self.memory[sprite_address as usize];
            // Find the start index of the buffer
            let buffer_start_index = (i as u16 * BUFFER_WIDTH as u16) + x_coord as u16;

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
                
                let mut pixel_value = ((new_value_for_buffer >> j) & 1) as u32;
                self.display.buffer[(buffer_start_index as usize) + 7 - j] = pixel_value;
            }

            // This is chip8 behaviour. If the sprite changes the display buffer, set the
            // vf register to 1.
            if new_value_for_buffer != sprite {
                self.v[0xf] = 1;
            }

            sprite_address += 1;
        }
        self.display.draw_screen();
    }

    fn jump_to_addr(&mut self, address: u16) {
        self.pc = address;
    }

    fn set_reg(&mut self, reg_no: usize, value: u8) {
        self.v[reg_no] = value;
    }

    fn add_value_to_reg(&mut self, reg_no: usize, value: u8) {
        self.v[reg_no] = self.v[reg_no] + value;
    }

    fn set_index_reg(&mut self, value: u16) {
        self.i = value;
    }

    fn call(&mut self, addr: u16) {
        self.stack_pointer += 1;
        self.stack[self.stack_pointer] = self.pc;
        self.pc = addr;
    }

    fn skip_if_eq(&mut self, reg_no: usize, value: u8) {
        if self.v[reg_no] == value {
            self.pc += 2;
        }
    }

    fn skip_if_not_eq(&mut self, reg_no: usize, value: u8) {
        if self.v[reg_no] != value {
            self.pc += 2;
        }
    }

    fn skip5(&mut self, reg_x: usize, reg_y: usize) {
        if self.v[reg_x] == self.v[reg_y] {
            self.pc += 2;
        }
    }

    fn load(&mut self, reg_x: usize, reg_y: usize) {
        self.v[reg_x] = self.v[reg_y]; 
    }

    // Arithmetic ops
    fn OR(&mut self, reg_x: usize, reg_y: usize) {
        self.v[reg_x] = self.v[reg_x] | self.v[reg_y]; 
    }

    fn AND(&mut self, reg_x: usize, reg_y: usize) {
        self.v[reg_x] = self.v[reg_x] & self.v[reg_y]; 
    }

    fn XOR(&mut self, reg_x: usize, reg_y: usize) {
        self.v[reg_x] = self.v[reg_x] ^ self.v[reg_y]; 
    }

    fn ADD(&mut self, reg_x: usize, reg_y: usize) {
        let result = (self.v[reg_x] + self.v[reg_y]) as u16;
        if result > 255 {
            self.v[0xf] = 1;
        }
        else {
            self.v[0xf] = 0;
        }
        // Keep only the lowest 8 bits
        self.v[reg_x] = (result & 0xFF) as u8;
    }

    fn SUB(&mut self, reg_x: usize, reg_y: usize) {
        let mut result = 0;
        if self.v[reg_x] > self.v[reg_y] {
            result = self.v[reg_x] - self.v[reg_y] ;
            self.v[0xf] = 1;
        }
        else {
            result = self.v[reg_y] - self.v[reg_x] ;
            self.v[0xf] = 0;
        }
        self.v[reg_x] = result;
    }

    fn SHR(&mut self, reg_x: usize) {
        let lsb = self.v[reg_x] & 0x01;
        if (lsb == 1) {
            self.v[0xf] = 1;
        }
        else {
            self.v[0xf] = 1;
        }

        self.v[reg_x] = self.v[reg_x] >> 1;
    }

    fn SUBN(&mut self, reg_x: usize, reg_y: usize) {
        let mut result = 0;
        if self.v[reg_x] > self.v[reg_y] {
            result = self.v[reg_x] - self.v[reg_y] ;
            self.v[0xf] = 0;
        }
        else {
            result = self.v[reg_y] - self.v[reg_x] ;
            self.v[0xf] = 1;
        }
        self.v[reg_x] = result;
    }

    fn SHL(&mut self, reg_x: usize) {
        let msb = self.v[reg_x] & 0x80;
        if (msb == 1) {
            self.v[0xf] = 1;
        }
        else {
            self.v[0xf] = 1;
        }

        self.v[reg_x] = self.v[reg_x] << 1;
    }
    // End arithmetic ops

    fn skip_if_reg_not_eq(&mut self, reg_x: usize, reg_y: usize) {
        if self.v[reg_x] != self.v[reg_y] {
            self.pc += 2;
        }
    }

    fn jump_offset(&mut self, addr: u16) {
        self.pc = addr + self.v[0] as u16;
    }

    fn random(&mut self, reg_x: usize, kk: u8) {
        // TODO: Implement random number gen
        self.v[reg_x] = 128 & kk;
    }
}
