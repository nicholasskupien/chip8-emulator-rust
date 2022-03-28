

pub struct Processor {
    ram: [u8; crate::CHIP8_RAM_SIZE_BYTES],
    vram: [[bool; crate::CHIP8_SCREEN_WIDTH]; crate::CHIP8_SCREEN_HEIGHT],
    pc: u16,
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
    pub fn load(){

    }

    pub fn step(){

    }
}

