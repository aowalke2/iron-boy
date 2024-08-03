use super::tile::{TileAccess, TILE_HEIGHT, TILE_WIDTH};

#[derive(Debug, Copy, Clone)]
pub struct Background {
    scy: u8,
    scx: u8,
}

impl Background {
    pub fn new() -> Background {
        Background { scy: 0, scx: 0 }
    }

    pub fn scx(&self) -> u8 {
        self.scx
    }

    pub fn set_scx(&mut self, value: u8) {
        self.scx = value
    }

    pub fn scy(&self) -> u8 {
        self.scy
    }

    pub fn set_scy(&mut self, value: u8) {
        self.scy = value
    }
}

impl TileAccess for Background {
    fn tile_map_coordinates(&self, lx: u8, ly: u8) -> (u8, u8) {
        let x = lx.wrapping_add(self.scx);
        let y = ly.wrapping_add(self.scy);
        (x, y)
    }

    fn tile_pixel_coordinates(&self, x: u8, y: u8) -> (u8, u8) {
        let x = 7 - (x % TILE_WIDTH as u8);
        let y = (y % TILE_HEIGHT as u8) * 2;
        (x, y)
    }
}
