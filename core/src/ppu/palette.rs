#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White = 0,
    LightGrey = 1,
    DarkGrey = 2,
    Black = 3,
}

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        match value {
            0 => Color::White,
            1 => Color::LightGrey,
            2 => Color::DarkGrey,
            _ => Color::Black,
        }
    }
}

impl From<Color> for (u8, u8, u8) {
    fn from(color: Color) -> Self {
        match color {
            Color::White => (255, 255, 255),
            Color::LightGrey => (192, 192, 192),
            Color::DarkGrey => (96, 96, 96),
            Color::Black => (0, 0, 0),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Palette {
    data: [Color; 4],
}

impl Palette {
    pub fn new(value: u8) -> Self {
        let mut pallete = Palette { data: [Color::White; 4] };
        for i in 0..pallete.data.len() {
            pallete.data[i] = Color::from((value >> (i * 2)) & 0b11)
        }
        pallete
    }

    pub fn pixel_color(&self, color: u8) -> Color {
        self.data[color as usize]
    }

    pub fn write(&mut self, value: u8) {
        for i in 0..self.data.len() {
            self.data[i] = Color::from((value >> (i * 2)) & 0b11)
        }
    }

    pub fn read(&self) -> u8 {
        let mut pallete = 0;
        for i in 0..self.data.len() {
            pallete |= (self.data[i] as u8) << (i * 2);
        }
        pallete
    }
}

pub fn color_index(byte1: u8, byte2: u8, pixel_index: u8) -> u8 {
    let lsb = (byte1 >> pixel_index) & 0b1;
    let msb = ((byte2 >> pixel_index) & 0b1) << 1;
    msb | lsb
}
