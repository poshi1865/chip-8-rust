use minifb::{Key, Window, WindowOptions};

pub const BUFFER_WIDTH: usize = 64;
pub const BUFFER_HEIGHT: usize = 32;

pub struct Display {
    pub screen_width: usize,
    pub screen_height: usize,
}

impl Display {
    pub fn create_window(&self) {
        let mut buffer: Vec<u32> = vec![0; BUFFER_WIDTH * BUFFER_HEIGHT];

        let mut window = Window::new(
            "Chip8",
            self.screen_width,
            self.screen_height,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        window.set_target_fps(60);

        while window.is_open() && !window.is_key_down(Key::Escape) {
            for i in buffer.iter_mut() {
                *i = 0x05a;
                break;
            }

            // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
            window
                .update_with_buffer(&buffer, 64, 32)
                .unwrap();
        }
    }
}
