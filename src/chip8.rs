use minifb::Key;
use minifb::KeyRepeat;
use crate::display::Display;
use crate::display::BUFFER_HEIGHT;
use crate::display::BUFFER_WIDTH;

use crate::keymap::u8_to_key;
use crate::keymap::key_to_u8;

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
                0x00EE => self.return_from_subroutine(),
                _ => panic!("Unimplemented instruction: {:x}", instruction),
            },
            0x1000 => {
                let address = instruction & 0x0FFF;
                self.jump_to_addr(address);
            },
            0x2000 => {
                let address = instruction & 0x0FFF;
                self.call(address);
            },
            0x3000 => {
                let reg_x = ((instruction & 0x0F00) >> 8) as usize;
                let kk = (instruction & 0x00FF) as u8;
                self.skip_if_eq(reg_x as usize, kk);
            },
            0x4000 => {
                let reg_x = ((instruction & 0x0F00) >> 8) as usize;
                let kk = (instruction & 0x00FF) as u8;
                self.skip_if_not_eq(reg_x as usize, kk);
            },
            0x5000 => {
                let reg_x = ((instruction & 0x0F00) >> 8) as usize;
                let reg_y = ((instruction & 0x00F0) >> 4) as usize;
                self.skip5(reg_x as usize, reg_y as usize);
            },
            0x6000 => {
                let reg_x = ((instruction & 0x0F00) >> 8) as usize;
                let value = (instruction & 0x00FF) as u8;
                self.set_reg(reg_x as usize, value);
            },
            0x7000 => {
                let reg_x = ((instruction & 0x0F00) >> 8) as usize;
                let value = (instruction & 0x00FF) as u8;
                self.add_value_to_reg(reg_x, value);
            },
            0x8000 => {
                let reg_x = ((instruction & 0x0F00) >> 8) as usize;
                let reg_y = ((instruction & 0x00F0) >> 4) as usize;
                let typ = (instruction & 0x000F) as u8;
                match typ {
                    0 => self.load(reg_x, reg_y),
                    1 => self.OR(reg_x, reg_y),
                    2 => self.AND(reg_x, reg_y),
                    3 => self.XOR(reg_x, reg_y),
                    4 => self.ADD(reg_x, reg_y),
                    5 => self.SUB(reg_x, reg_y),
                    6 => self.SHR(reg_x),
                    7 => self.SUBN(reg_x, reg_y),
                    0xE => self.SHL(reg_x),
                    _ => panic!("Wrong instruction {:x}", instruction)
                }
            },
            0x9000 => {
                let reg_x = ((instruction & 0x0F00) >> 8) as usize;
                let reg_y = ((instruction & 0x00F0) >> 4) as usize;
                self.skip_if_reg_not_eq(reg_x, reg_y)
            },
            0xA000 => {
                let value = instruction & 0x0FFF;
                self.set_index_reg(value);
            },
            0xB000 => {
                let address = instruction & 0x0FFF;
                self.jump_offset(address);
            },
            0xC000 => {
                let reg_x = ((instruction & 0x0F00) >> 8) as usize;
                let kk = (instruction & 0x00FF) as u8;
                self.random(reg_x, kk);
            },
            0xD000 => {
                let x = ((instruction & 0x0F00) >> 8) as u8;
                let y = ((instruction & 0x00F0) >> 4) as u8;
                let n = (instruction & 0x000F) as u8;
                self.draw_screen(x, y, n);
            },
            0xE000 => {
                let typ = instruction & 0x000F;
                let reg_x = ((instruction & 0x0F00) >> 8) as usize;
                let value = instruction & 0x00FF;

                match typ {
                    0x1 => self.skip_if_key_not_pressed(reg_x),
                    0xE => self.skip_if_key_pressed(reg_x),
                    _ => panic!("Wrong instruction {:x}", instruction)
                }
            },
            0xF000 => {
                let typ = instruction & 0x00FF;
                let reg_x = ((instruction & 0x0F00) >> 8) as usize;
                match typ {
                    0x0007 => self.load_delay_timer(reg_x),
                    0x000A => self.load_key_blocking(reg_x),
                    0x0015 => self.set_delay_timer(reg_x),
                    0x0018 => self.set_sound_timer(reg_x),
                    0x001E => self.add_i(reg_x),
                    0x0029 => self.set_sprite_to_i(reg_x),
                    0x0033 => self.store_reg_to_mem(reg_x),
                    0x0055 => self.store_range(reg_x),
                    0x0065 => self.load_range(reg_x),
                    _ => panic!("Wrong instruction {:x}", instruction)
                }
            },
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

    // instructions
    fn return_from_subroutine(&mut self) {
        self.pc = self.stack[self.stack_pointer];
        self.stack_pointer -= 1;
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
            let buffer_start_index = ((i as u16 * BUFFER_WIDTH as u16) + x_coord as u16) % 2040;

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
        self.v[reg_no] = self.v[reg_no].wrapping_add(value);
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
        let result = (self.v[reg_x].wrapping_add(self.v[reg_y])) as u16;
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
        if lsb == 1 {
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
        if msb == 1 {
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

    fn skip_if_key_pressed(&mut self, reg_x: usize) {
        if self.display.window.is_key_down(u8_to_key(self.v[reg_x])) {
            self.pc += 2;
        }
    }

    fn skip_if_key_not_pressed(&mut self, reg_x: usize) {
        if !self.display.window.is_key_down(u8_to_key(self.v[reg_x])) {
            self.pc += 2;
        }
    }

    fn load_delay_timer(&mut self, reg_x: usize) {
        self.v[reg_x] = self.delay_timer;
    }

    fn load_key_blocking(&mut self, reg_x: usize) {
        // This blocks until a valid key is pressed
        loop {
            let mut complete = false;
            let pressed_keys: Vec<Key> = self.display.window.get_keys_pressed(KeyRepeat::No);
            for key in pressed_keys {
                if key_to_u8(key) != 255 {
                    self.v[reg_x] = key_to_u8(key);
                    complete = true;
                    break;
                }
            }
            if complete {
                break;
            }
        }
    }

    fn set_delay_timer(&mut self, reg_x: usize) {
        self.delay_timer = self.v[reg_x];
    }

    fn set_sound_timer(&mut self, reg_x: usize) {
        self.sound_timer = self.v[reg_x];
    }

    fn add_i(&mut self, reg_x: usize) {
        self.i = self.i + self.v[reg_x] as u16;
    }

    fn set_sprite_to_i(&mut self, reg_x: usize) {
        for i in 0x50..=0x9F {
            if self.memory[i] == self.v[reg_x] {
                self.i = i as u16;
            }
        }
    }

    fn load_vx_in_i(&mut self, reg_x: usize) {
        self.memory[self.i as usize] = self.v[reg_x] / 100;
        self.memory[self.i as usize + 1] = (self.v[reg_x] % 100) / 10;
        self.memory[self.i as usize + 2] = self.v[reg_x] % 10;
    }

    fn store_reg_to_mem(&mut self, reg_x: usize) {
        let mut start_addr = self.i as usize;
        for j in self.v {
            if j == self.v[reg_x] {
                break;
            }
            self.memory[start_addr] = j;
        }
    }

    fn store_range(&mut self, reg_x: usize) {
        let mut start_addr = self.i as usize;
        for j in 0..=reg_x {
            self.memory[start_addr] = self.v[j];
            start_addr += 1;
        }
    }

    fn load_range(&mut self, reg_x: usize) {
        let mut start_addr = self.i as usize;
        for j in 0..=reg_x {
            self.v[j] = self.memory[start_addr];
            start_addr += 1;
        }
    }
}
