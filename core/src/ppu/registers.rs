use bit::BitIndex;

use super::{
    oam::ObjectSize,
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

    pub fn write(&mut self, value: u8) {
        self.value = value
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

    pub fn object_size(&self) -> ObjectSize {
        self.value.bit(2).into()
    }

    pub fn set_object_size(&mut self, object_size: ObjectSize) {
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
        LcdStatus { value: value & 0b0111_1000 }
    }

    pub fn read(&self) -> u8 {
        self.value
    }

    pub fn write(&mut self, value: u8) {
        self.value = value & 0b0111_1000
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
mod tests {
    use crate::ppu::{
        oam::ObjectSize,
        registers::LcdStatus,
        tile::{TileData, TileMap},
        PpuMode,
    };

    use super::LcdControl;

    #[test]
    fn test_lcd_control() {
        let mut lcd_control = LcdControl::new(0b1110_0111);
        assert_eq!(lcd_control.lcd_enabled(), true);
        assert_eq!(lcd_control.window_tile_map(), TileMap::High);
        assert_eq!(lcd_control.window_enabled(), true);
        assert_eq!(lcd_control.tile_data(), TileData::SignedAddress);
        assert_eq!(lcd_control.bg_tile_map(), TileMap::Low);
        assert_eq!(lcd_control.object_size(), ObjectSize::Size8x16);
        assert_eq!(lcd_control.object_enabled(), true);
        assert_eq!(lcd_control.bg_window_enabled(), true);

        lcd_control.set_lcd_enabled(false);
        lcd_control.set_window_tile_map(TileMap::Low);
        lcd_control.set_window_enabled(false);
        lcd_control.set_tile_data(TileData::UnsignedAddress);
        lcd_control.set_bg_tile_map(TileMap::High);
        lcd_control.set_object_size(ObjectSize::Size8x8);
        lcd_control.set_object_enabled(false);
        lcd_control.set_bg_window_enabled(false);

        assert_eq!(lcd_control.read(), 0b0001_1000);
        lcd_control.write(0b1110_0111);
        assert_eq!(lcd_control.read(), 0b1110_0111);
    }

    #[test]
    fn test_lcd_status() {
        let mut lcd_status = LcdStatus::new(0b1111_1111);
        assert_eq!(lcd_status.lyc_interrupt(), true);
        assert_eq!(lcd_status.mode2_interrupt(), true);
        assert_eq!(lcd_status.mode1_interrupt(), true);
        assert_eq!(lcd_status.mode0_interrupt(), true);
        assert_eq!(lcd_status.lyc_equals_ly(), false);
        assert_eq!(lcd_status.ppu_mode(), PpuMode::HBlank);

        lcd_status.set_lyc_interrupt(false);
        lcd_status.set_mode2_interrupt(false);
        lcd_status.set_mode1_interrupt(false);
        lcd_status.set_mode0_interrupt(false);
        lcd_status.set_lyc_equals_ly(true);
        lcd_status.set_ppu_mode(PpuMode::OamScan);

        assert_eq!(lcd_status.read(), 0b0000_0110);
        lcd_status.write(0b1110_0111);
        assert_eq!(lcd_status.read(), 0b0110_0000);
    }
}
