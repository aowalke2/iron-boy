use bit::BitIndex;

pub const TOTAL_OBJECTS: usize = 40;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ObjectSize {
    Size8x8 = 8,
    Size8x16 = 16,
}

impl From<bool> for ObjectSize {
    fn from(value: bool) -> Self {
        match value {
            true => ObjectSize::Size8x16,
            false => ObjectSize::Size8x8,
        }
    }
}

impl From<ObjectSize> for bool {
    fn from(value: ObjectSize) -> Self {
        match value {
            ObjectSize::Size8x16 => true,
            ObjectSize::Size8x8 => false,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ObjectAttributes {
    x_position: u8,
    y_position: u8,
    tile_index: u8,
    flags: u8,
}

impl ObjectAttributes {
    pub fn new() -> ObjectAttributes {
        ObjectAttributes {
            x_position: 0,
            y_position: 0,
            tile_index: 0,
            flags: 0,
        }
    }

    pub fn x_position(&self) -> u8 {
        self.x_position
    }

    pub fn set_x_position(&mut self, value: u8) {
        self.x_position = value
    }

    pub fn y_position(&self) -> u8 {
        self.y_position
    }

    pub fn set_y_position(&mut self, value: u8) {
        self.y_position = value
    }

    pub fn tile_index(&self) -> u8 {
        self.tile_index
    }

    pub fn set_tile_index(&mut self, value: u8) {
        self.tile_index = value
    }

    pub fn flags(self) -> u8 {
        self.flags
    }

    pub fn set_flags(&mut self, value: u8) {
        self.flags = value
    }

    pub fn priority(&self) -> bool {
        self.flags.bit(7)
    }

    pub fn y_flip(&self) -> bool {
        self.flags.bit(6)
    }

    pub fn x_flip(&self) -> bool {
        self.flags.bit(5)
    }

    pub fn dmg_pallete(&self) -> bool {
        self.flags.bit(4)
    }
}
