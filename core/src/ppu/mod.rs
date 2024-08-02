use background::Background;
use palette::Palette;
use registers::{LcdControl, LcdStatus};
use std::cmp::Ordering;
use tile::TileAccess;
use window::Window;

use crate::bus::Memory;

mod background;
mod oam;
mod palette;
mod registers;
mod tile;
mod window;

const VRAM_SIZE: usize = 0x4000;
const OAM_SIZE: usize = 0xA0;

pub const VIEWPORT_WIDTH: usize = 160;
pub const VIEWPORT_HEIGHT: usize = 144;
const BACKGROUND_WIDTH: usize = 256;
const BACKGROUND_HEIGHT: usize = BACKGROUND_WIDTH;

const MAX_SCANLINE_Y: u8 = 153;
const MAX_VIEWPORT_SCANLINE_Y: u8 = VIEWPORT_HEIGHT as u8 - 1;

const OAM_SCAN_CYCLES: u32 = 80;
const DRAW_PIXELS_CYCLES: u32 = 172;
const HBLANK_CYCLES: u32 = 204;
const VBLANK_CYCLES: u32 = 456;

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
    cycles: u32,
    lcd_control: LcdControl,
    lcd_status: LcdStatus,
    background: Background,
    ly: u8,
    lyc: u8,
    bg_palette: Palette,
    obj0_palette: Palette,
    obj1_palette: Palette,
    window: Window,
    priority_map: [bool; BACKGROUND_WIDTH * BACKGROUND_HEIGHT],
    oam: [u8; OAM_SIZE],
    oam_buffer: Vec<(usize, u8)>,
    vram: [u8; VRAM_SIZE],
    pub screen_buffer: Vec<(u8, u8, u8)>,
    pub frame_completed: bool,
    pub interrupt: u8,
}

impl Memory for Ppu {
    fn mem_read(&mut self, address: u16) -> u8 {
        match address {
            0x8000..=0x9FFF => self.vram[address as usize - 0x8000],
            0xFE00..=0xFE9F => self.oam[address as usize - 0xFE00],
            0xFF40 => self.lcd_control.read(),
            0xFF41 => self.lcd_status.read(),
            0xFF42 => self.background.scy(),
            0xFF43 => self.background.scx(),
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF47 => self.bg_palette.into(),
            0xFF48 => self.obj0_palette.into(),
            0xFF49 => self.obj1_palette.into(),
            0xFF4A => self.window.wy(),
            0xFF4B => self.window.wx(),
            0xFF4C => 0xFF,
            0xFF4E => 0xFF,
            _ => 0xFF,
        }
    }

    fn mem_write(&mut self, address: u16, data: u8) {
        match address {
            0x8000..=0x9FFF => self.vram[address as usize - 0x8000] = data,
            0xFE00..=0xFE9F => self.oam[address as usize - 0xFE00] = data,
            0xFF40 => self.set_lcd_control(data),
            0xFF41 => self.lcd_status.write(data),
            0xFF42 => self.background.set_scy(data),
            0xFF43 => self.background.set_scx(data),
            0xFF44 => {} // Read-only
            0xFF45 => self.set_lyc(data),
            0xFF47 => self.bg_palette = Palette::from(data),
            0xFF48 => self.obj0_palette = Palette::from(data),
            0xFF49 => self.obj1_palette = Palette::from(data),
            0xFF4A => self.window.set_wx(data),
            0xFF4B => self.window.set_wx(data),
            0xFF4C => {}
            0xFF4E => {}
            _ => panic!("PPU does not handle write {:04X}", address),
        }
    }
}

impl Ppu {
    pub fn new() -> Ppu {
        Ppu {
            cycles: 0,
            lcd_control: LcdControl::new(0),
            lcd_status: LcdStatus::new(0),
            background: Background::new(),
            ly: 0,
            lyc: 0,
            bg_palette: Palette::from(0),
            obj0_palette: Palette::from(0),
            obj1_palette: Palette::from(1),
            window: Window::new(),
            oam: [0; OAM_SIZE],
            oam_buffer: Vec::new(),
            vram: [0; VRAM_SIZE],
            screen_buffer: vec![(0, 0, 0); VIEWPORT_WIDTH * VIEWPORT_HEIGHT],
            priority_map: [false; BACKGROUND_WIDTH * BACKGROUND_HEIGHT],
            frame_completed: false,
            interrupt: 0,
        }
    }

    pub fn cycle(&mut self, cycles: u32) {
        if !self.lcd_control.lcd_enabled() {
            return;
        }

        self.cycles += cycles;
        match self.lcd_status.ppu_mode() {
            PpuMode::OamScan => {
                if self.cycles < OAM_SCAN_CYCLES {
                    return;
                }
                self.build_oam_buffer();
                self.lcd_status.set_ppu_mode(PpuMode::DrawingPixels);
                self.cycles -= OAM_SCAN_CYCLES
            }
            PpuMode::DrawingPixels => {
                if self.cycles < DRAW_PIXELS_CYCLES {
                    return;
                }
                self.render_scanline();
                self.lcd_status.set_ppu_mode(PpuMode::HBlank);
                if self.lcd_status.mode0_interrupt() {
                    self.interrupt |= 0x02
                }
                self.cycles -= DRAW_PIXELS_CYCLES
            }
            PpuMode::HBlank => {
                if self.cycles < HBLANK_CYCLES {
                    return;
                }
                if self.ly >= MAX_VIEWPORT_SCANLINE_Y {
                    self.lcd_status.set_ppu_mode(PpuMode::VBlank);
                    if self.lcd_status.mode1_interrupt() {
                        self.interrupt |= 0x02
                    }
                    self.frame_completed = true;
                    self.interrupt |= 0x01
                } else {
                    self.window.increment_line_counter(self.lcd_control.window_enabled(), self.ly);
                    self.set_ly(self.ly + 1);
                    self.lcd_status.set_ppu_mode(PpuMode::OamScan);
                    if self.lcd_status.mode2_interrupt() {
                        self.interrupt |= 0x02
                    }
                }
                self.cycles -= HBLANK_CYCLES;
            }
            PpuMode::VBlank => {
                if self.cycles < VBLANK_CYCLES {
                    return;
                }
                self.set_ly(self.ly + 1);
                if self.ly > MAX_SCANLINE_Y {
                    self.lcd_status.set_ppu_mode(PpuMode::OamScan);
                    self.window.reset_line_counter();
                    self.set_ly(0)
                }
                self.cycles -= VBLANK_CYCLES;
            }
        }
    }

    fn set_lcd_control(&mut self, value: u8) {
        self.lcd_control.write(value);
        if !self.lcd_control.lcd_enabled() {
            self.clear_screen();
            self.window.reset_line_counter();
            self.set_ly(0);
            self.lcd_status.set_ppu_mode(PpuMode::HBlank);
            self.cycles = 0;
        }
    }

    fn set_ly(&mut self, value: u8) {
        self.ly = value;
        self.check_lyc_equals_ly();
    }

    fn set_lyc(&mut self, value: u8) {
        self.lyc = value;
        self.check_lyc_equals_ly();
    }

    fn check_lyc_equals_ly(&mut self) {
        self.lcd_status.set_lyc_equals_ly(false);
        if self.lyc != self.ly {
            return;
        }
        self.lcd_status.set_lyc_equals_ly(true);
        if self.lcd_status.lyc_interrupt() {
            self.interrupt |= 0x02
        }
    }

    fn build_oam_buffer(&self) {
        //todo!()
    }

    fn clear_screen(&mut self) {
        for i in 0..self.priority_map.len() {
            if i < self.screen_buffer.len() {
                self.screen_buffer[i] = (255, 255, 255);
            }
            self.priority_map[i] = false;
        }
        self.frame_completed = true;
    }

    fn render_scanline(&mut self) {
        if self.lcd_control.bg_window_enabled() {
            self.render_background_window_line();
        }

        if self.lcd_control.object_enabled() {
            //self.render_object_line();
        }
    }

    fn render_background_window_line(&mut self) {
        for x in 0..VIEWPORT_WIDTH as u8 {
            let (tile_index_address, tile_pixel_x, tile_pixel_y) = self.background_window_tile_info(x);
            let tile_index = self.mem_read(tile_index_address);
            let tile_address = self.lcd_control.tile_data().address(tile_index);

            let pixel_address = tile_address + tile_pixel_y as u16;
            let byte1 = self.mem_read(pixel_address);
            let byte2 = self.mem_read(pixel_address + 1);
            let color_index = ((byte1 >> tile_pixel_x) & 1) | ((byte2 >> tile_pixel_x) & 1) << 1;

            let priority_index = x as usize + self.ly as usize * BACKGROUND_WIDTH;
            if color_index == 0 {
                self.priority_map[priority_index] = true;
            }

            let color = self.bg_palette.color(color_index);
            let pixel_index = x as usize + self.ly as usize * VIEWPORT_WIDTH;
            self.screen_buffer[pixel_index] = color.into();
        }
    }

    fn background_window_tile_info(&mut self, lx: u8) -> (u16, u8, u8) {
        let inside_window = self.window.inside_window(self.lcd_control.window_enabled(), lx, self.ly);

        if inside_window {
            let base_address = self.lcd_control.window_tile_map().base_address();
            let (tile_map_x, tile_map_y) = self.window.tile_map_coordinates(lx, self.ly);

            let tile_index_address = self.window.tile_index_address(base_address, tile_map_x, tile_map_y);
            let (tile_pixel_x, tile_pixel_y) = self.window.tile_pixel_coordinates(lx, self.ly);
            (tile_index_address, tile_pixel_x, tile_pixel_y)
        } else {
            let base_address = self.lcd_control.bg_tile_map().base_address();
            let (tile_map_x, tile_map_y) = self.background.tile_map_coordinates(lx, self.ly);

            let tile_index_address = self.background.tile_index_address(base_address, tile_map_x, tile_map_y);
            let (tile_pixel_x, tile_pixel_y) = self.background.tile_pixel_coordinates(tile_map_x, tile_map_y);
            (tile_index_address, tile_pixel_x, tile_pixel_y)
        }
    }
}
