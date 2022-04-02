

pub struct Processor {
    ram: [u8; crate::CHIP8_RAM_SIZE_BYTES],
    vram: [[bool; crate::CHIP8_SCREEN_WIDTH]; crate::CHIP8_SCREEN_HEIGHT],
    // pc would have been u12 but to index easier in rust it needs to be usize
    pc: u16,
    reg_index: u16,
    opcode: u16,
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
        self.pc = program_start as u16;
    }

    // Get opcode at current program counter
    pub fn read_opcode(&mut self) -> u16{
        return (self.ram[self.pc as usize] as u16) << 8 | (self.ram[self.pc as usize + 1]) as u16;
    }

    pub fn cycle(&mut self){
        let mut pc_advance = true;
        self.opcode = self.read_opcode();

        // defining some variables to help with processing
        // the naming is based on opcode definition

        println!("FETCH: {:04x}", self.opcode);

        // just put all the nibbles in a tuple
        let nibbles = (
            ((self.opcode & 0xF000) >> 12) as u8,
            ((self.opcode & 0x0F00) >> 8) as u8,
            ((self.opcode & 0x00F0) >> 4) as u8,
            ((self.opcode & (0x000F) as u16)) as u8
            );

        dbg!(nibbles);

        //_*** First nibble of opcode
        //*X**
        //**Y*
        //***N
        let (nibble, x, y, n) = nibbles;

        //*NNN
        let nnn = self.opcode & 0x0FFF;

        //**NN
        let nn = (self.opcode & 0x00FF) as u8;

        match nibbles {

            // http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#0.0
            // 00E0: CLS
            // clear the screen
            (0x0, 0x0, 0xE, 0x0) => {
                self.vram = [[false; crate::CHIP8_SCREEN_WIDTH]; crate::CHIP8_SCREEN_HEIGHT];
            },

            // 00EE: RET
            // return from subroutine (return)
            (0x0, 0x0, 0xE, 0xE) => {
                if self.stack_ptr > 0 {
                    self.stack_ptr = self.stack_ptr - 1;
                    self.pc = self.stack[self.stack_ptr as usize];
                }
                else {
                    println!("EXECUTE: return: {:04x} ERROR no address to return to", self.opcode);
                }
                pc_advance = false;
            },

            // 0NNN: SYS addr
            // 'Call' calling machine code routine
            (0x0, _, _, _) => {
                println!("EXECUTE: SYS addr: {:04x}", self.opcode);
            },

            // 1NNN: JP addr
            // Jump to address NNN
            (0x1, _, _, _) => {
                self.pc = nnn;
                pc_advance = false;
            },

            // 2NNN: CALL addr
            // Call subroutine at NNN (goto NNN;)
            (0x2, _, _, _) => {
                self.stack[self.stack_ptr as usize] = self.pc;
                self.stack_ptr = self.stack_ptr + 1;
                pc_advance = false;

            },

            // 3xNN: SE Vx, byte
            // Skip next instruction if Vx == NN
            (0x3, _, _, _) => {
                if self.reg[x as usize] == nn {
                    self.pc += 2;
                }
            },

            // 4xNN: SNE Vx, byte
            // Skip next instruction if Vx != NN
            (0x4, _, _, _) => {
                if self.reg[x as usize] != nn {
                    self.pc += 2;
                }
            },

            // 5xy0: SE Vx, Vy
            // Skip next instruction if Vx == Vy
            (0x5, _, _, 0x0) => {
                if self.reg[x as usize] == self.reg[y as usize] {
                    self.pc += 2;
                }
            },

            // 6xNN: LD Vx, byte
            // Sets Vx to NN
            (0x6, _, _, _) => {
                self.reg[x as usize] = nn;
            },

            // 7xNN: ADD Vx, byte
            // Add NN to Vx (don't set carry flag)
            (0x7, _, _, _) => {
                self.reg[x as usize] = self.reg[x as usize] + nn;
            },

            // 8xy0: LD Vx, Vy
            // Vx = Vy
            (0x8, _, _, 0x0) => {
                self.reg[x as usize] = self.reg[y as usize];
            },

            // 8xy1: OR Vx, Vy
            // Vx = Vx | Vy (OR)
            (0x8, _, _, 0x1) => {
                self.reg[x as usize] = self.reg[x as usize] | self.reg[y as usize];
            },

            // 8xy2: AND Vx, Vy
            // Vx = Vx & Vy (AND)
            (0x8, _, _, 0x2) => {
                self.reg[x as usize] = self.reg[x as usize] & self.reg[y as usize];
            },

            // 8xy3: XOR Vx, Vy
            // Vx = Vx ^ Vy (XOR)
            (0x8, _, _, 0x3) => {
                self.reg[x as usize] = self.reg[x as usize] ^ self.reg[y as usize];
            },

            // 8xy4: ADD Vx, Vy
            // Vx = Vx + Vy, Set VF (last register) if carry occurs (true false)
            (0x8, _, _, 0x4) => {
                let result = self.reg[x as usize] as u16 + self.reg[y as usize] as u16;
                let mut carry = false;
                if (result > 255){
                    carry = true;
                }
                self.reg[0xF] = carry as u8;
                self.reg[x as usize] = result as u8;
            },

            // 8xy5: SUB Vx, Vy
            // Vx = Vx - Vy, Set VF (last register) if borrow does NOT occur (true false)
            (0x8, _, _, 0x5) => {
                let result = self.reg[x as usize] as i16 - self.reg[y as usize] as i16;
                let mut not_borrow = true;
                if (result < 0){
                    not_borrow = false;
                }
                self.reg[0xF] = not_borrow as u8;
                self.reg[x as usize] = result as u8;
            },

            // 8xy6: SHR Vx {, Vy}
            // Vx = Vx >> 1, Set VF to LSB of Vx (0101 = 1011 >> 1, VF = 1)
            (0x8, _, _, 0x6) => {
                self.reg[0xF] = self.reg[x as usize] & 0x0001;
                self.reg[x as usize] = self.reg[x as usize] >> 1;
            },

            // 8xy7: SUBN Vx, Vy
            // Vx = Vy - Vx, set VF = NOT borrow.
            (0x8, _, _, 0x7) => {
                let result = self.reg[y as usize] as i16 - self.reg[x as usize] as i16;
                let mut not_borrow = true;
                if (result < 0){
                    not_borrow = false;
                }
                self.reg[0xF] = not_borrow as u8;
                self.reg[x as usize] = result as u8;
            },

            // 8xyE: SHR Vx {, Vy}
            // Vx = Vx << 1, Set VF to MSB of Vx (0101 = 1011 >> 1, VF = 1)
            (0x8, _, _, 0xE) => {
                self.reg[0xF] = self.reg[x as usize] >> 7;
                self.reg[x as usize] = self.reg[x as usize] >> 1;
            },

            // 9xy0 - SNE Vx, Vy
            // Skip next instruction if Vx != Vy
            (0x9, _, _, 0x0) => {
                if self.reg[x as usize] != self.reg[y as usize] {
                    self.pc += 2;
                }
            },

            // Annn - LD I, addr
            // Set I = nnn.
            (0xA, _, _, _) => {
                self.reg_index = nnn;
            },

            // Bnnn - JP V0, addr
            // Jump to location nnn + V0.
            (0xB, _, _, _) => {

            },

            // Cxkk - RND Vx, byte
            // Set Vx = random byte AND kk.
            // The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx
            (0xC, _, _, _) => {
            },

            // Dxyn - DRW Vx, Vy, nibble
            // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
            (0xD, _, _, _) => {
            },

            // Ex9E - SKP Vx
            // Skip next instruction if key with the value of Vx is pressed.
            (0xE, _, 0x9, 0xE) => {
            },

            // ExA1 - SKNP Vx
            // Skip next instruction if key with the value of Vx is not pressed.
            (0xE, _, 0xA, 0x1) => {
            },

            // Fx07 - LD Vx, DT
            // Set Vx = delay timer value.
            (0xF, _, 0x0, 0x7) => {
            },

            // Fx0A - LD Vx, K
            // Stop execution, wait for a key press, store the value of the key in Vx.
            (0xF, _, 0x0, 0xA) => {
            },

            // Fx15 - LD DT, Vx
            // Set delay timer = Vx.
            (0xF, _, 0x1, 0x5) => {
            },

            // Fx18 - LD ST, Vx
            // Set sound timer = Vx.
            (0xF, _, 0x1, 0x8) => {
            },

            // Fx1E - ADD I, Vx
            // Set I = I + Vx.
            (0xF, _, 0x1, 0xE) => {
            },

            // Fx29 - LD F, Vx
            // Set I = location of sprite for digit Vx.
            (0xF, _, 0x2, 0x9) => {
            },

            // Fx33 - LD B, Vx
            // Store BCD representation of Vx in memory locations I, I+1, and I+2.
            (0xF, _, 0x3, 0x3) => {
            },

            // Fx55 - LD [I], Vx
            // Store registers V0 through Vx in memory starting at location I.
            (0xF, _, 0x5, 0x5) => {
            },

            // Fx65 - LD Vx, [I]
            // Read registers V0 through Vx from memory starting at location I.
            (0xF, _, 0x6, 0x5) => {
            },

            _ => {
                println!("UNKNOWN INSTR: {:04x}", self.opcode)
            }
        }

        if(pc_advance){
            self.pc += 2;
        }
    }

    // debug
    pub fn print_file(&self, program_size: usize) {
        let mut pc = self.pc;

        while (pc < (program_size as u16 +self.pc)) {
            let opcode = (self.ram[pc as usize] as u16) << 8 | (self.ram[pc as usize + 1]) as u16;
            print!("{:04x} ", opcode);
            if ((pc / 2 + 1) % 8 == 0) {
                println!("");
            }
            pc += 2;
        }
    }
}

