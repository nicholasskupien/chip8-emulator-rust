//instead of crate:: use chip8_emulator_rust:: only in main.rs
use chip8_emulator_rust::{CHIP8_SCREEN_HEIGHT,CHIP8_SCREEN_WIDTH, drivers::CartridgeDriver, processor::Processor, Program};

struct video {
   screen: [[bool; CHIP8_SCREEN_WIDTH]; CHIP8_SCREEN_HEIGHT],
}

fn main() {
   let cartridge_driver = CartridgeDriver::new("C:/Users/Nick Skupien/Documents/GitHub/chip8-emulator-rust/src/roms/BLITZ");
   let program_size = cartridge_driver.size;
   let program = cartridge_driver.rom;
   let processor = Processor::new();
   print_file(program, program_size);

}

fn print_file(program: chip8_emulator_rust::Program, program_size: usize){
   let mut pc = 0;

   while(pc < program_size){
      let opcode = (program[pc] as u16) << 8 | (program[pc + 1]) as u16;
      print!("{:04x} ", opcode);
      if((pc/2 + 1) % 8 == 0) {
         println!("");
      }
      pc += 2;
   }
}
