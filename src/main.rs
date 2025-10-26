mod chip8;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use chip8::Chip8;

mod display;
use display::BUFFER_WIDTH;
use display::BUFFER_HEIGHT;
use display::Display;

fn init_machine(rom_path: String) -> Chip8 {

    let mut display: Display = Display::new(
        "Chip 8",
        1280,
        720,
        [0; BUFFER_HEIGHT * BUFFER_WIDTH]
    );

    // Create chip8 instance
    let mut chip8: Chip8 = Chip8 {
        memory: [0; 4096],
        pc: 0,
        i: 0,
        v: [0; 16],
        stack: [0; 16],
        delay_timer: 0,
        sound_timer: 0,
        display: display,
        keypad: [false; 16],
    };

    // Load fonts
    let fonts: [u8; 80] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80  // F
    ];

    let mut font_counter = 0;
    // Write the fonts to memory into addresses 050->09f
    for i in 0x50..=0x9F {
        chip8.memory[i] = fonts[font_counter];
        font_counter += 1;
    }

    // Load ROM
    let file_buffer = BufReader::new(File::open(rom_path).unwrap());

    let mut address_counter = 0x200;
    for byte_or_error in file_buffer.bytes() {
        let byte = byte_or_error.unwrap();
        chip8.memory[address_counter] = byte;
        address_counter += 1;
    }

    // reset to start to begin execution
    chip8.pc = 0x200;

    chip8.display.create_window();
    chip8
}

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("You need to pass a rom!!.\nUsage: chip8 path_to_rom");
        std::process::exit(-1);
    }

    let rom_path: String = args[1].clone();
    let mut chip8 = init_machine(rom_path);

    loop {
        let instruction: u16 = chip8.fetch();
        chip8.decode_and_execute(instruction);

        // chip8.draw_screen();
        // chip8.clear_screen();
    }
}
