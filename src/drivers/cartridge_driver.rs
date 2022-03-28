//Taken from https://github.com/starrhorne/chip8-rust/blob/master/src/drivers/cartridge_driver.rs
use std::fs::File;
use std::io::prelude::*;
use crate::Program;

pub struct CartridgeDriver {
    pub rom: Program,
    pub size: usize,
}

impl CartridgeDriver {
    pub fn new(filename: &str) -> Self {
        let mut f = File::open(filename).expect("file not found");

        // creates buffer initialized to 0
        let mut buffer = [0u8; crate::CHIP8_PROGRAM_SIZE];

        let bytes_read = if let Ok(bytes_read) = f.read(&mut buffer) {
            bytes_read
        } else {
            0
        };

        CartridgeDriver {
            rom: buffer,
            size: bytes_read,
        }
    }
}