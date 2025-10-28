use minifb::Key;

pub fn u8_to_key(num: u8) -> Key {
    match num {
        0 => return Key::Key1,
        1 => return Key::Key2,
        2 => return Key::Key3,
        3 => return Key::Key4,
        4 => return Key::Q,
        5 => return Key::W,
        6 => return Key::E,
        7 => return Key::R,
        8 => return Key::A,
        9 => return Key::S,
        10 => return Key::D,
        11 => return Key::F,
        12 => return Key::Z,
        13 => return Key::X,
        14 => return Key::C,
        15 => return Key::V,
        _ => return Key::Unknown
    }
}

pub fn key_to_u8(key: Key) -> u8 {
    match key {
        Key::Key1 => return 0,
        Key::Key2 => return 1,
        Key::Key3 => return 2,
        Key::Key4 => return 3,
        Key::Q => return 4,
        Key::W => return 5,
        Key::E => return 6,
        Key::R => return 7,
        Key::A => return 8,
        Key::S => return 9,
        Key::D => return 10,
        Key::F => return 11,
        Key::Z => return 12,
        Key::X => return 13,
        Key::C => return 14,
        Key::V => return 15,
        _ => return 255,
    }
}
