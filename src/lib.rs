// Used to store custom types etc.

pub const CHIP8_SCREEN_WIDTH: usize = 64;
pub const CHIP8_SCREEN_HEIGHT: usize = 32;
pub const CHIP8_RAM_SIZE_BYTES: usize = 4096;
pub const CHIP8_START_OF_PROGRAM: usize = 512;
pub const CHIP8_PROGRAM_SIZE: usize = 3584;

pub mod drivers;
pub mod processor;

use drivers::CartridgeDriver;


pub type Program = [u8; crate::CHIP8_PROGRAM_SIZE];
