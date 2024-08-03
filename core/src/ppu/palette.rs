#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White = 0,
    LightGray = 1,
    DarkGray = 2,
    Black = 3,
}

impl From<u8> for Color {
    fn from(value: u8) -> Color {
        match value {
            0 => Color::White,
            1 => Color::LightGray,
            2 => Color::DarkGray,
            _ => Color::Black,
        }
    }
}

impl From<Color> for (u8, u8, u8) {
    fn from(color: Color) -> (u8, u8, u8) {
        match color {
            Color::White => (255, 255, 255),
            Color::LightGray => (192, 192, 192),
            Color::DarkGray => (96, 96, 96),
            Color::Black => (0, 0, 0),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Palette {
    colors: [Color; 4],
}

impl From<u8> for Palette {
    fn from(value: u8) -> Self {
        let mut colors = [Color::White; 4];
        for i in 0..4 {
            colors[i] = Color::from((value >> (i * 2)) & 0b11)
        }
        Palette { colors }
    }
}

impl From<Palette> for u8 {
    fn from(palette: Palette) -> Self {
        let mut value = 0;
        for i in 0..palette.colors.len() {
            value |= (palette.colors[i] as u8) << (i * 2);
        }
        value
    }
}

impl Palette {
    pub fn color(&self, c: u8) -> Color {
        self.colors[c as usize]
    }
}
