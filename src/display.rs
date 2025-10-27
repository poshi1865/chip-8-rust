use minifb::{Window, WindowOptions};

pub const BUFFER_WIDTH: usize = 64;
pub const BUFFER_HEIGHT: usize = 32;
pub const BACKGROUND_COLOR: u32 = 0x0FAFAFA;
pub const FOREGROUND_COLOR: u32 = 0x0;

pub struct Display {
    pub window: Window,
    pub screen_width: usize,
    pub screen_height: usize,
    pub buffer: [u32; (BUFFER_HEIGHT * BUFFER_WIDTH) as usize]
}

impl Display {
    pub fn new(
        title: &str,
        screen_width: usize,
        screen_height: usize,
        buffer: [u32; (BUFFER_HEIGHT * BUFFER_WIDTH) as usize]
    ) -> Display {
        let mut window = Window::new(
            title,
            screen_width,
            screen_height,
            WindowOptions::default(),
        ).unwrap();

        return Display {
            window: window,
            screen_width: screen_width,
            screen_height: screen_height,
            buffer: buffer
        };
    }
    pub fn create_window(&mut self) {
        self.window = Window::new(
            "Chip8",
            self.screen_width,
            self.screen_height,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        self.window.set_target_fps(60);
    }

    // We don't modify self.buffer in clearing or drawing the screen 
    // because self.buffer is used for bitwise ops.
    // Those bitwise ops expect a 1 or 0 value.
    pub fn clear_screen(&mut self) {
        let mut buffer = [0; BUFFER_WIDTH * BUFFER_HEIGHT];
        for i in 0..(BUFFER_WIDTH * BUFFER_HEIGHT) {
            buffer[i] = BACKGROUND_COLOR;
        }
        self.window.update_with_buffer(&buffer, BUFFER_WIDTH, BUFFER_HEIGHT).unwrap();
    }

    pub fn draw_screen(&mut self) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        let mut buffer = [0; BUFFER_WIDTH * BUFFER_HEIGHT];
        for i in 0..(BUFFER_WIDTH * BUFFER_HEIGHT) {
            if self.buffer[i] == 1 {
                buffer[i] = FOREGROUND_COLOR;
            }
            else {
                buffer[i] = BACKGROUND_COLOR;
            }
        }
        self.window.update_with_buffer(&buffer, BUFFER_WIDTH, BUFFER_HEIGHT).unwrap();
    }
}
