//instead of crate:: use chip8_emulator_rust:: only in main.rs
use chip8_emulator_rust::{CHIP8_SCREEN_HEIGHT,CHIP8_SCREEN_WIDTH, CHIP8_START_OF_PROGRAM, drivers::CartridgeDriver, drivers::InputDriver, drivers::DisplayDriver, processor::Processor, Program};
use std::{time, thread};

struct video {
   screen: [[bool; CHIP8_SCREEN_WIDTH]; CHIP8_SCREEN_HEIGHT],
}

fn main() {
   let sdl_context = sdl2::init().unwrap();

   let cartridge_driver = CartridgeDriver::new("./roms/TANK");
   let mut input_driver = InputDriver::new(&sdl_context);
   let mut display_driver = DisplayDriver::new(&sdl_context);


   let program_size = cartridge_driver.size;
   let program = cartridge_driver.rom;
   let mut processor = Processor::new();
   processor.set_debug(0);
   processor.load(program, program_size, CHIP8_START_OF_PROGRAM);
   processor.print_file(program_size);
   // pri

   loop{
      let keypad = input_driver.poll().expect("Error retrieving input");
      let vram = processor.cycle(keypad);

      display_driver.draw(&vram);

      // thread::sleep(time::Duration::from_millis(2));
   }

}


