use super::{
    tile::{TileAccess, TILE_HEIGHT, TILE_WIDTH},
    VIEWPORT_HEIGHT, VIEWPORT_WIDTH,
};

pub struct Window {
    wy: u8,
    wx: u8,
    line_counter: u8,
}

impl Window {
    pub fn new() -> Window {
        Window {
            wy: 0,
            wx: 0,
            line_counter: 0,
        }
    }

    pub fn wx(&self) -> u8 {
        self.wx
    }

    pub fn set_wx(&mut self, value: u8) {
        if value < 7 {
            return;
        }

        self.wx = value
    }

    pub fn wy(&self) -> u8 {
        self.wy
    }

    pub fn set_wy(&mut self, value: u8) {
        self.wy = value
    }

    pub fn reset_line_counter(&mut self) {
        self.line_counter = 0;
    }

    pub fn inside_window(&self, window_enabled: bool, lx: u8, ly: u8) -> bool {
        if !window_enabled {
            return false;
        }
        lx >= self.wx.wrapping_sub(7) && ly >= self.wy
    }

    pub fn increment_line_counter(&mut self, window_enabled: bool, ly: u8) {
        let window_visible =
            window_enabled && self.wx - 7 < VIEWPORT_WIDTH as u8 && self.wy < VIEWPORT_HEIGHT as u8 && ly >= self.wy;

        if window_visible {
            self.line_counter = self.line_counter.saturating_add(1);
        }
    }
}

impl TileAccess for Window {
    fn tile_map_coordinates(&self, lx: u8, _: u8) -> (u8, u8) {
        let x = lx.wrapping_sub(self.wx.wrapping_sub(7));
        let y = self.line_counter;
        (x, y)
    }

    fn tile_pixel_coordinates(&self, x: u8, y: u8) -> (u8, u8) {
        let x = self.wx.wrapping_sub(x) % TILE_WIDTH;
        let y = ((y - self.wy) % TILE_HEIGHT) * 2;
        (x, y)
    }
}
