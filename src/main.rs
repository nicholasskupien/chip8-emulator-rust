//instead of crate:: use chip8_emulator_rust:: only in main.rs
use chip8_emulator_rust::{CHIP8_SCREEN_HEIGHT,CHIP8_SCREEN_WIDTH, CHIP8_START_OF_PROGRAM, drivers::CartridgeDriver, processor::Processor, Program};
use std::{time, thread};

struct video {
   screen: [[bool; CHIP8_SCREEN_WIDTH]; CHIP8_SCREEN_HEIGHT],
}

fn main() {
   let cartridge_driver = CartridgeDriver::new("./roms/BLITZ");
   let program_size = cartridge_driver.size;
   let program = cartridge_driver.rom;
   let mut processor = Processor::new();
   processor.load(program, program_size, CHIP8_START_OF_PROGRAM);
   processor.print_file(program_size);
   // print_file(program, program_size);

   loop{
      processor.cycle();
      thread::sleep(time::Duration::from_millis(1000));
   }

}


