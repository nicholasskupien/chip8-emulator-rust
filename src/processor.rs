

pub struct Processor {
    ram: [u8; crate::CHIP8_RAM_SIZE_BYTES],
    vram: [[bool; crate::CHIP8_SCREEN_WIDTH]; crate::CHIP8_SCREEN_HEIGHT],
    // pc would have been u16 (becuase it is max u12) but to index easier in rust it needs to be usize
    pc: usize,
    reg_index: u16,
    opcode: u32,
    reg: [u8; 16],
    stack: [u16; 16],
    stack_ptr: u8,
}

impl Processor {
    pub fn new() -> Self{
        Self {
            ram: [0u8; crate::CHIP8_RAM_SIZE_BYTES],
            vram: [[false; crate::CHIP8_SCREEN_WIDTH]; crate::CHIP8_SCREEN_HEIGHT],
            pc: 0,
            reg_index: 0,
            opcode: 0,
            reg: [0u8; 16],
            stack: [0u16; 16],
            stack_ptr: 0,
        }
    }

    // Load data into ram
    // Need program data, length of data, program start
    pub fn load(&mut self, program: crate::Program, program_size: usize, program_start: usize){
        self.ram[program_start..(program_start+program_size)].copy_from_slice(&program[0..program_size]);
        self.pc = program_start;
    }

    pub fn step(){

    }

    pub fn print_file(&self, program_size: usize) {
        let mut pc = self.pc;

        while (pc < (program_size+self.pc)) {
            let opcode = (self.ram[pc] as u16) << 8 | (self.ram[pc + 1]) as u16;
            print!("{:04x} ", opcode);
            if ((pc / 2 + 1) % 8 == 0) {
                println!("");
            }
            pc += 2;
        }
    }
}

