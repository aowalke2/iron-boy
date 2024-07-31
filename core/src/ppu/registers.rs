use bit::BitIndex;

use super::{
    oam::OamSize,
    tile::{TileData, TileMap},
    Mode, Ppu,
};

#[derive(Debug, Clone, Copy)]
pub struct LcdControl {
    data: u8,
}

impl LcdControl {
    pub fn new(data: u8) -> Self {
        LcdControl { data }
    }

    pub fn read(&self) -> u8 {
        self.data
    }

    pub fn write(&mut self, data: u8) {
        self.data = data
    }

    // not sure if i ever need to set bits individually
    pub fn lcd_enabled(&self) -> bool {
        self.data.bit(7)
    }

    pub fn set_lcd_enabled(&mut self, status: bool) {
        self.data.set_bit(7, status);
    }

    pub fn window_tile_map(&self) -> TileMap {
        self.data.bit(6).into()
    }

    pub fn set_window_tile_map(&mut self, tile_map: TileMap) {
        self.data.set_bit(6, tile_map.into());
    }

    pub fn window_enabled(&self) -> bool {
        self.data.bit(5)
    }

    pub fn set_window_enabled(&mut self, status: bool) {
        self.data.set_bit(5, status);
    }

    pub fn tile_data(&self) -> TileData {
        self.data.bit(4).into()
    }

    pub fn set_tile_data(&mut self, tile_data: TileData) {
        self.data.set_bit(4, tile_data.into());
    }

    pub fn bg_tile_map(&self) -> TileMap {
        self.data.bit(3).into()
    }

    pub fn set_bg_tile_map(&mut self, tile_map: TileMap) {
        self.data.set_bit(3, tile_map.into());
    }

    pub fn object_size(&self) -> OamSize {
        self.data.bit(2).into()
    }

    pub fn set_object_size(&mut self, object_size: OamSize) {
        self.data.set_bit(2, object_size.into());
    }

    pub fn object_enabled(&self) -> bool {
        self.data.bit(1)
    }

    pub fn set_object_enabled(&mut self, status: bool) {
        self.data.set_bit(1, status);
    }

    pub fn bg_window_enabled(&self) -> bool {
        self.data.bit(0)
    }

    pub fn set_bg_window_enabled(&mut self, status: bool) {
        self.data.set_bit(0, status);
    }
}

// TODO: make registers structs ??
impl Ppu {
    pub fn stat_read(&self) -> u8 {
        let mut data = 0x80;

        data |= (self.lyc_interrupt as u8) << 6;
        data |= (self.mode2_interrupt as u8) << 5;
        data |= (self.mode1_interrupt as u8) << 4;
        data |= (self.mode0_interrupt as u8) << 3;
        data |= ((self.ly == self.lyc) as u8) << 2;
        data |= self.mode as u8;

        data
    }

    pub fn stat_write(&mut self, data: u8) {
        self.lyc_interrupt = data & 0x40 == 0x40;
        self.mode2_interrupt = data & 0x20 == 0x20;
        self.mode1_interrupt = data & 0x10 == 0x10;
        self.mode0_interrupt = data & 0x08 == 0x08;
    }
}

#[cfg(test)]
mod tests {}
