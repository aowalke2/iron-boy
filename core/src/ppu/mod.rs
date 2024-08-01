use palette::Palette;
use registers::{LcdControl, LcdStatus};
use std::cmp::Ordering;
use tile::{TileData, TileMap};

use crate::bus::Memory;

mod oam;
mod palette;
mod registers;
mod tile;

const VRAM_SIZE: usize = 0x4000;
const OAM_SIZE: usize = 0xA0;
pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;
const SCANLINE_LENGTH: u32 = 456;
const SCANLINE_COUNT: u8 = 154;

#[derive(PartialEq, Copy, Clone)]
enum Priority {
    Blank,
    Normal,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum PpuMode {
    OamScan = 2,
    DrawingPixels = 3,
    HBlank = 0,
    VBlank = 1,
}

impl From<u8> for PpuMode {
    fn from(value: u8) -> Self {
        match value {
            0 => PpuMode::HBlank,
            1 => PpuMode::VBlank,
            2 => PpuMode::OamScan,
            _ => PpuMode::DrawingPixels,
        }
    }
}

impl From<PpuMode> for u8 {
    fn from(value: PpuMode) -> Self {
        match value {
            PpuMode::HBlank => 0,
            PpuMode::VBlank => 1,
            PpuMode::OamScan => 2,
            PpuMode::DrawingPixels => 3,
        }
    }
}

pub struct Ppu {
    ticks: u32,
    lcd_control: LcdControl,
    lcd_status: LcdStatus,
    scy: u8,
    scx: u8,
    ly: u8,
    lyc: u8,
    bg_palette: Palette,
    obj0_palette: Palette,
    obj1_palette: Palette,
    wy: u8,
    wx: u8,
    bg_window_priority: [Priority; SCREEN_WIDTH],
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
            0xFF40 => self.lcd_control.read(),
            0xFF41 => self.lcd_status.read(),
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF46 => 0, // DMA Write only
            0xFF47 => self.bg_palette.into(),
            0xFF48 => self.obj0_palette.into(),
            0xFF49 => self.obj1_palette.into(),
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
            0xFF40 => {
                // may need to do stuff when the lcd_enable changes?
                self.lcd_control.write(data);
            }
            0xFF41 => self.lcd_status.write(data),
            0xFF42 => self.scy = data,
            0xFF43 => self.scx = data,
            0xFF44 => {} // Read-only
            0xFF45 => {
                self.lyc = data;
                self.trigger_lyc_interrupt();
            }
            0xFF46 => panic!("0xFF46 should be handled by Bus"),
            0xFF47 => self.bg_palette = Palette::from(data),
            0xFF48 => self.obj0_palette = Palette::from(data),
            0xFF49 => self.obj1_palette = Palette::from(data),
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
            ticks: 0,
            lcd_control: LcdControl::new(0),
            lcd_status: LcdStatus::new(0),
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            bg_palette: Palette::from(0),
            obj0_palette: Palette::from(0),
            obj1_palette: Palette::from(1),
            wy: 0,
            wx: 0,
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
        if !self.lcd_control.lcd_enabled() {
            return;
        }

        self.ticks += ticks;
        if self.ticks >= SCANLINE_LENGTH {
            self.ticks -= SCANLINE_LENGTH;
            self.ly = (self.ly + 1) % SCANLINE_COUNT;
        }

        self.trigger_lyc_interrupt();
        if self.ly >= 144 && self.lcd_status.ppu_mode() != PpuMode::VBlank {
            self.change_mode(PpuMode::VBlank);
            return;
        }

        if self.ly < 144 {
            match self.ticks {
                0..=80 => {
                    if self.lcd_status.ppu_mode() != PpuMode::OamScan {
                        self.change_mode(PpuMode::OamScan);
                    }
                }
                81..=252 => {
                    if self.lcd_status.ppu_mode() != PpuMode::DrawingPixels {
                        self.change_mode(PpuMode::DrawingPixels);
                    }
                }
                _ => {
                    if self.lcd_status.ppu_mode() != PpuMode::HBlank {
                        self.change_mode(PpuMode::HBlank);
                    }
                }
            }
        }
    }

    fn trigger_lyc_interrupt(&mut self) {
        if self.lyc == self.ly && self.lcd_status.lyc_interrupt() {
            self.interrupt |= 0b10
        }
    }

    fn change_mode(&mut self, ppu_mode: PpuMode) {
        self.lcd_status.set_ppu_mode(ppu_mode);
        let interrupt_triggered;
        match ppu_mode {
            PpuMode::HBlank => {
                // more to add?
                interrupt_triggered = self.lcd_status.mode0_interrupt();
            }
            PpuMode::VBlank => {
                // more to add?
                self.interrupt |= 0b01; //Vblank Interrupt
                interrupt_triggered = self.lcd_status.mode1_interrupt();
            }
            PpuMode::OamScan => {
                interrupt_triggered = self.lcd_status.mode2_interrupt();
            }
            PpuMode::DrawingPixels => {
                // more to add?
                interrupt_triggered = false
            }
        }

        if interrupt_triggered {
            self.interrupt |= 0b10
        }
    }
}
