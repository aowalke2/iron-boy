#![crate_name = "core"]
#![crate_type = "lib"]

pub use crate::io::joypad::JoypadButton;
pub use crate::ppu::{SCREEN_HEIGHT, SCREEN_WIDTH};

mod boot_rom;
mod bus;
mod cartridge;
mod cpu;
pub mod game_boy;
mod io;
mod ppu;
