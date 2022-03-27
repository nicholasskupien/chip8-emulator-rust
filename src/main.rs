mod drivers;

use drivers::CartridgeDriver;

pub const CHIP8_SCREEN_WIDTH: usize = 64;
pub const CHIP8_SCREEN_HEIGHT: usize = 32;
pub const CHIP8_RAM_SIZE_BYTES: usize = 4096;
pub const CHIP8_START_OF_PROGRAM: usize = 512;
pub const CHIP8_PROGRAM_SIZE: usize = 3584;

fn main() {
   let cartridge_driver = CartridgeDriver::new("C:/Users/Nick Skupien/Documents/GitHub/chip8-emulator-rust/src/roms/BLITZ");
   let mut program_size = cartridge_driver.size;

   while(program_size > 0){
      
   }
}
