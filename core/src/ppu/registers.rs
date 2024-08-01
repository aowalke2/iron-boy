use bit::BitIndex;

use super::{
    oam::OamSize,
    tile::{TileData, TileMap},
    PpuMode,
};

#[derive(Debug, Clone, Copy)]
pub struct LcdControl {
    value: u8,
}

impl LcdControl {
    pub fn new(value: u8) -> Self {
        LcdControl { value }
    }

    pub fn read(&self) -> u8 {
        self.value
    }

    pub fn write(&mut self, data: u8) {
        self.value = data
    }

    // not sure if i ever need to set bits individually
    pub fn lcd_enabled(&self) -> bool {
        self.value.bit(7)
    }

    pub fn set_lcd_enabled(&mut self, status: bool) {
        self.value.set_bit(7, status);
    }

    pub fn window_tile_map(&self) -> TileMap {
        self.value.bit(6).into()
    }

    pub fn set_window_tile_map(&mut self, tile_map: TileMap) {
        self.value.set_bit(6, tile_map.into());
    }

    pub fn window_enabled(&self) -> bool {
        self.value.bit(5)
    }

    pub fn set_window_enabled(&mut self, status: bool) {
        self.value.set_bit(5, status);
    }

    pub fn tile_data(&self) -> TileData {
        self.value.bit(4).into()
    }

    pub fn set_tile_data(&mut self, tile_data: TileData) {
        self.value.set_bit(4, tile_data.into());
    }

    pub fn bg_tile_map(&self) -> TileMap {
        self.value.bit(3).into()
    }

    pub fn set_bg_tile_map(&mut self, tile_map: TileMap) {
        self.value.set_bit(3, tile_map.into());
    }

    pub fn object_size(&self) -> OamSize {
        self.value.bit(2).into()
    }

    pub fn set_object_size(&mut self, object_size: OamSize) {
        self.value.set_bit(2, object_size.into());
    }

    pub fn object_enabled(&self) -> bool {
        self.value.bit(1)
    }

    pub fn set_object_enabled(&mut self, status: bool) {
        self.value.set_bit(1, status);
    }

    pub fn bg_window_enabled(&self) -> bool {
        self.value.bit(0)
    }

    pub fn set_bg_window_enabled(&mut self, status: bool) {
        self.value.set_bit(0, status);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LcdStatus {
    value: u8,
}

impl LcdStatus {
    pub fn new(value: u8) -> Self {
        LcdStatus { value }
    }

    pub fn read(&self) -> u8 {
        self.value
    }

    pub fn write(&mut self, data: u8) {
        self.value = data & !0b0111_1000
    }

    pub fn lyc_interrupt(&self) -> bool {
        self.value.bit(6)
    }

    pub fn set_lyc_interrupt(&mut self, status: bool) {
        self.value.set_bit(6, status);
    }

    pub fn mode2_interrupt(&self) -> bool {
        self.value.bit(5)
    }

    pub fn set_mode2_interrupt(&mut self, status: bool) {
        self.value.set_bit(5, status);
    }

    pub fn mode1_interrupt(&self) -> bool {
        self.value.bit(4)
    }

    pub fn set_mode1_interrupt(&mut self, status: bool) {
        self.value.set_bit(4, status);
    }

    pub fn mode0_interrupt(&self) -> bool {
        self.value.bit(3)
    }

    pub fn set_mode0_interrupt(&mut self, status: bool) {
        self.value.set_bit(3, status);
    }

    pub fn lyc_equals_ly(&self) -> bool {
        self.value.bit(2)
    }

    pub fn set_lyc_equals_ly(&mut self, status: bool) {
        self.value.set_bit(2, status);
    }

    pub fn ppu_mode(&self) -> PpuMode {
        self.value.bit_range(0..2).into()
    }

    pub fn set_ppu_mode(&mut self, ppu_mode: PpuMode) {
        self.value.set_bit_range(0..2, ppu_mode.into());
    }
}

#[cfg(test)]
mod tests {}
