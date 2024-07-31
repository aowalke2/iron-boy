use palette::Palette;
use std::cmp::Ordering;
use tile::{TileAddressing, TileMap};

use crate::bus::Memory;

mod palette;
mod registers;
mod tile;

const VRAM_SIZE: usize = 0x4000;
const OAM_SIZE: usize = 0xA0;
pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

#[derive(PartialEq, Copy, Clone)]
enum Priority {
    Blank,
    Normal,
}

#[derive(PartialEq, Copy, Clone)]
enum Mode {
    OamScan = 2,
    DrawingPixels = 3,
    HBlank = 0,
    VBlank = 1,
}

pub struct Ppu {
    mode: Mode,
    line_ticks: u32,
    ly: u8,
    lyc: u8,
    lcd_enabled: bool,
    window_tile_map: TileMap,
    window_enabled: bool,
    tile_data: TileAddressing,
    bg_tile_map: TileMap,
    object_size: u8,
    object_enabled: bool,
    bg_window_enabled: bool,
    bg_window_priority: [Priority; SCREEN_WIDTH],
    lyc_interrupt: bool,
    mode0_interrupt: bool,
    mode1_interrupt: bool,
    mode2_interrupt: bool,
    scy: u8,
    scx: u8,
    wy: u8,
    wx: u8,
    wy_trigger: bool,
    wy_position: i32,
    bg_palette: Palette,
    obj0_palette: Palette,
    obj1_palette: Palette,
    pub vram: [u8; VRAM_SIZE],
    oam: [u8; OAM_SIZE],
    vrambank: usize,
    pub screen_buffer: Vec<(u8, u8, u8)>,
    pub screen_updated: bool,
    pub interrupt: u8,
}

impl Memory for Ppu {
    fn mem_read(&mut self, address: u16) -> u8 {
        match address {
            0x8000..=0x9FFF => self.vram[(self.vrambank * 0x2000) | (address as usize & 0x1FFF)],
            0xFE00..=0xFE9F => self.oam[address as usize - 0xFE00],
            0xFF40 => self.lcdc_read(),
            0xFF41 => self.stat_read(),
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF46 => 0, // DMA Write only
            0xFF47 => self.bg_palette.into_byte(),
            0xFF48 => self.obj0_palette.into_byte(),
            0xFF49 => self.obj1_palette.into_byte(),
            0xFF4A => self.wy,
            0xFF4B => self.wx,
            0xFF4C => 0xFF,
            0xFF4E => 0xFF,
            _ => 0xFF,
        }
    }

    fn mem_write(&mut self, address: u16, data: u8) {
        match address {
            0x8000..=0x9FFF => self.vram[(self.vrambank * 0x2000) | (address as usize & 0x1FFF)] = data,
            0xFE00..=0xFE9F => self.oam[address as usize - 0xFE00] = data,
            0xFF40 => self.lcdc_write(data),
            0xFF41 => self.stat_write(data),
            0xFF42 => self.scy = data,
            0xFF43 => self.scx = data,
            0xFF44 => {} // Read-only
            0xFF45 => {
                self.lyc = data;
                //self.trigger_lyc_interrupt();
            }
            0xFF46 => panic!("0xFF46 should be handled by Bus"),
            0xFF47 => self.bg_palette = Palette::from_byte(data),
            0xFF48 => self.obj0_palette = Palette::from_byte(data),
            0xFF49 => self.obj1_palette = Palette::from_byte(data),
            0xFF4A => self.wy = data,
            0xFF4B => self.wx = data,
            0xFF4C => {}
            0xFF4E => {}
            _ => panic!("PPU does not handle write {:04X}", address),
        }
    }
}

impl Ppu {
    pub fn new() -> Ppu {
        Ppu {
            mode: Mode::HBlank,
            line_ticks: 0,
            ly: 0,
            lyc: 0,
            lcd_enabled: false,
            window_tile_map: TileMap::High,
            window_enabled: false,
            tile_data: TileAddressing::Unsigned,
            bg_tile_map: TileMap::High,
            object_size: 8,
            object_enabled: false,
            bg_window_enabled: false,
            lyc_interrupt: false,
            mode2_interrupt: false,
            mode1_interrupt: false,
            mode0_interrupt: false,
            scy: 0,
            scx: 0,
            wy: 0,
            wx: 0,
            wy_trigger: false,
            wy_position: -1,
            bg_palette: Palette::from_byte(0),
            obj0_palette: Palette::from_byte(0),
            obj1_palette: Palette::from_byte(1),
            vram: [0; VRAM_SIZE],
            oam: [0; OAM_SIZE],
            screen_buffer: vec![(0, 0, 0); SCREEN_WIDTH * SCREEN_HEIGHT],
            bg_window_priority: [Priority::Normal; SCREEN_WIDTH],
            screen_updated: false,
            interrupt: 0,
            vrambank: 0,
        }
    }

    pub fn cycle(&mut self, ticks: u32) {
        todo!("{}", ticks)
    }
}
