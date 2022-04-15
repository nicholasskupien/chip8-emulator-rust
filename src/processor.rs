use std::sync::Arc;
use rand::{Rng};
use crate::{CHIP8_SCREEN_WIDTH, CHIP8_SCREEN_HEIGHT};
use std::sync::atomic::{AtomicU8, Ordering};
use std::thread;
use std::thread::JoinHandle;

pub struct Processor {
    ram: [u8; crate::CHIP8_RAM_SIZE_BYTES],
    program_size: usize,
    vram: [[bool; crate::CHIP8_SCREEN_WIDTH]; crate::CHIP8_SCREEN_HEIGHT],
    // pc would have been u12 but to index easier in rust it needs to be usize
    pc: u16,
    reg_index: u16,
    opcode: u16,
    reg: [u8; 16],
    stack: [u16; 16],
    stack_ptr: u8,
    keypad_irq: bool,
    keypad_irq_dest: u8,
    delay_timer: AtomicU8,
    sound_timer: AtomicU8,
    debug: usize,
    breakpoint: bool,
    thread: JoinHandle<()>,
}

impl Processor {
    pub fn new() -> Self{
        Self {
            ram: [0u8; crate::CHIP8_RAM_SIZE_BYTES],
            program_size: 0,
            vram: [[false; crate::CHIP8_SCREEN_WIDTH]; crate::CHIP8_SCREEN_HEIGHT],
            pc: 0,
            reg_index: 0,
            opcode: 0,
            reg: [0u8; 16],
            stack: [0u16; 16],
            stack_ptr: 0,
            keypad_irq: false,
            keypad_irq_dest: 0,
            delay_timer: AtomicU8::new(0),
            sound_timer: AtomicU8::new(0),
            debug: 0,
            breakpoint: true,
            thread: thread::spawn(||{}),
        }
    }

    // Load data into ram
    // Need program data, length of data, program start
    pub fn load(&mut self, program: crate::Program, program_size: usize, program_start: usize){
        self.ram[program_start..(program_start+program_size)].copy_from_slice(&program[0..program_size]);
        self.program_size = program_size;

        for i in 0..crate::FONT_SET.len() {
            self.ram[i] = crate::FONT_SET[i];
        }

        self.pc = program_start as u16;
    }

    // Get opcode at current program counter
    pub fn read_opcode(&self, pc: u16) -> u16{
        return (self.ram[pc as usize] as u16) << 8 | (self.ram[pc as usize + 1]) as u16;
    }

    pub fn set_debug(&mut self, debug: usize){
        self.debug = debug;
    }

    // Spawn thread for timers
    // pub fn start_processor(&mut self) {
    //     delay_timer = AtomicU8::clone(&self.delay_timer);
    //     sound_timer = AtomicU8::clone(&self.sound_timer);
    //     self.thread = thread::spawn(move ||{
    //         loop{
    //             delay_timer.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x|{
    //                 if x > 0 {
    //                     Some(x - 1)
    //                 }
    //                 else{
    //                     Some(x)
    //                 }
    //             });
    //             sound_timer.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x|{
    //                 if x > 0 {
    //                     Some(x - 1)
    //                 }
    //                 else{
    //                     Some(x)
    //                 }
    //             });
    //         }
    //     })
    // }

    pub fn cycle_timers(&mut self) {
        delay_timer.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x|{
            if x > 0 {
                Some(x - 1)
            }
            else{
                Some(x)
            }
        });
        sound_timer.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |x|{
            if x > 0 {
                Some(x - 1)
            }
            else{
                Some(x)
            }
        });
    }

    pub fn cycle(&mut self, keypad: [bool; 16]) -> [[bool; crate::CHIP8_SCREEN_WIDTH]; crate::CHIP8_SCREEN_HEIGHT]{

        // Keypad Interrupt
        if self.keypad_irq == true {
            for k in 0..keypad.len(){
                if keypad[k] == true {
                    self.reg[self.keypad_irq_dest as usize] = k as u8;
                    self.keypad_irq = false;
                }
            }
            return self.vram;
        }

        if self.debug == 3{
            //break at every instruction
            if self.breakpoint == true {
                for k in 0..keypad.len() {
                    if keypad[k] == true {
                        self.breakpoint = false;
                    }
                }
                return self.vram;
            }

        }

        // Delay & Sound Timers (TODO Move this to its own thread)
        // if self.delay_timer > 0{
        //     self.delay_timer -= 1;
        // }
        //
        // if self.sound_timer > 0{
        //     self.sound_timer -= 1;
        // }


        if self.debug > 1 {

            clearscreen::clear().expect("failed to clear screen");

            println!("OP: {:04x}", self.opcode);

            //print registers
            for r in 0..self.reg.len(){
                println!("V{:X} = {:2X}", r, self.reg[r]);
            }

            println!("I = {:2X}", self.reg_index);

            println!("PC = {:2X}", self.pc);

            println!("SP = {:2X}", self.stack_ptr);

            for s in 0..self.stack.len(){
                println!("S{:X} = {:2X}", s, self.stack[s]);
            }

            self.display();

        }
        else if self.debug == 1 {
            clearscreen::clear().expect("failed to clear screen");
            // self.print_file(self.program_size);
        }

        let mut pc_advance = true;
        self.opcode = self.read_opcode(self.pc);

        // defining some variables to help with processing
        // the naming is based on opcode definition

        // just put all the nibbles in a tuple
        let nibbles = (
            ((self.opcode & 0xF000) >> 12) as u8,
            ((self.opcode & 0x0F00) >> 8) as u8,
            ((self.opcode & 0x00F0) >> 4) as u8,
            (self.opcode & (0x000F) as u16) as u8
            );

        // dbg!(nibbles);

        //_*** First nibble of opcode
        //*X**
        //**Y*
        //***N
        let (_nibble, x, y, n) = nibbles;

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
                    self.pc = self.stack[self.stack_ptr as usize - 1];
                    //unnecessary but good for debugging
                    self.stack[self.stack_ptr as usize - 1] = 0;
                    self.stack_ptr = self.stack_ptr - 1;
                }
                else {
                    println!("EXECUTE: return: {:04x} ERROR no address to return to", self.opcode);
                }
                // pc_advance = false;
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
                // println!("Setting program counter to: {}", nnn);
                pc_advance = false;
            },

            // 2NNN: CALL addr
            // Call subroutine at NNN (goto NNN;)
            (0x2, _, _, _) => {
                self.stack_ptr = self.stack_ptr + 1;
                self.stack[self.stack_ptr as usize - 1] = self.pc;
                self.pc = nnn;
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
                self.reg[x as usize] = (self.reg[x as usize] as u16 + nn as u16) as u8;
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
                if result > 255{
                    carry = true;
                }
                self.reg[0xF] = carry as u8;
                self.reg[x as usize] = result as u8;
            },

            // 8xy5: SUB Vx, Vy
            // Vx = Vx - Vy, Set VF (last register) if borrow does NOT occur (true false)
            (0x8, _, _, 0x5) => {
                let result = self.reg[x as usize].wrapping_sub(self.reg[y as usize]);

                let mut not_borrow = self.reg[x as usize] >= self.reg[y as usize];

                self.reg[0xF] = not_borrow as u8;
                self.reg[x as usize] = result;
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
                let result = self.reg[y as usize].wrapping_sub(self.reg[x as usize]);

                let mut not_borrow = self.reg[y as usize] >= self.reg[x as usize];

                self.reg[0xF] = not_borrow as u8;
                self.reg[x as usize] = result;
            },

            // 8xyE: SHR Vx {, Vy}
            // Vx = Vx << 1, Set VF to MSB of Vx (0101 = 1011 >> 1, VF = 1)
            (0x8, _, _, 0xE) => {
                self.reg[0xF] = self.reg[x as usize] >> 7;
                self.reg[x as usize] = self.reg[x as usize] << 1;
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
                self.pc = self.reg[0] as u16 + nnn;
                pc_advance = false;
            },

            // Cxkk - RND Vx, byte
            // Set Vx = random byte AND kk.
            // The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx
            (0xC, _, _, _) => {
                let random: u8 = rand::thread_rng().gen::<u8>();
                self.reg[x as usize] = random & nn;
            },

            // Dxyn - DRW Vx, Vy, nibble
            // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
            // Sprites shouldn't wrap they should clip! Only the position wraps https://www.reddit.com/r/EmuDev/comments/gft7r9/another_beginner_post_for_clarification_for_my/
            (0xD, _, _, _) => {
                let reg_x = self.reg[x as usize] as usize % CHIP8_SCREEN_WIDTH;
                let reg_y = self.reg[y as usize] as usize % CHIP8_SCREEN_HEIGHT;

                self.reg[0xF] = 0x0;

                for row in 0..n as usize {
                    let y_index = (reg_y + row);

                    if y_index >= CHIP8_SCREEN_HEIGHT{
                        break;
                    }

                    let mut vram_row = &mut self.vram[y_index];
                    let sprite_row = self.ram[self.reg_index as usize + row];

                    for col in 0..8 as usize {
                        let x_index = (col + reg_x);

                        if x_index >= CHIP8_SCREEN_WIDTH{
                            break;
                        }

                        let previous_pixel = vram_row[x_index];

                        vram_row[x_index] = vram_row[x_index] ^ ((sprite_row & (0x80 >> col)) != 0);

                        // if the previous pixel was 1 and was replaced with a 0 there is collision
                        if previous_pixel == true && vram_row[x_index] == false{
                            self.reg[0xF]  = 0x1;
                        }
                    }
                }

                //need flag when display bit flip
            },

            // Ex9E - SKP Vx
            // Skip next instruction if key with the value of Vx is pressed.
            (0xE, _, 0x9, 0xE) => {
                if keypad[self.reg[x as usize] as usize] == true {
                    self.pc += 2;
                }
            },

            // ExA1 - SKNP Vx
            // Skip next instruction if key with the value of Vx is not pressed.
            (0xE, _, 0xA, 0x1) => {
                if keypad[self.reg[x as usize] as usize] == false {
                    self.pc += 2;
                }
            },

            // Fx07 - LD Vx, DT
            // Set Vx = delay timer value.
            (0xF, _, 0x0, 0x7) => {
                self.reg[x as usize] = self.delay_timer.load(Ordering::Relaxed);
            },

            // Fx0A - LD Vx, K
            // Stop execution, wait for a key press, store the value of the key in Vx.
            (0xF, _, 0x0, 0xA) => {
                self.keypad_irq = true;
                self.keypad_irq_dest = x;
            },

            // Fx15 - LD DT, Vx
            // Set delay timer = Vx.
            (0xF, _, 0x1, 0x5) => {
                self.delay_timer.store(self.reg[x as usize], Ordering::Relaxed);
            },

            // Fx18 - LD ST, Vx
            // Set sound timer = Vx.
            (0xF, _, 0x1, 0x8) => {
                self.sound_timer.store(self.reg[x as usize], Ordering::Relaxed);
            },

            // Fx1E - ADD I, Vx
            // Set I = I + Vx.
            (0xF, _, 0x1, 0xE) => {
                self.reg_index = (self.reg_index as u32 + self.reg[x as usize] as u32) as u16;
            },

            // Fx29 - LD F, Vx
            // Set I = location of sprite for digit Vx.
            (0xF, _, 0x2, 0x9) => {
                // fonts stored starting at ram[0] and each font takes 5 bytes of memory
                self.reg_index = (self.reg[x as usize] * 5) as u16;

            },

            // Fx33 - LD B, Vx
            // Store BCD representation of Vx in memory locations I, I+1, and I+2.
            (0xF, _, 0x3, 0x3) => {
                self.ram[self.reg_index as usize] = self.reg[x as usize] / 100;
                self.ram[self.reg_index as usize + 1] = (self.reg[x as usize] % 100) / 10;
                self.ram[self.reg_index as usize + 2] = self.reg[x as usize] % 10;
            },

            // Fx55 - LD [I], Vx
            // Store registers V0 through Vx in memory starting at location I.
            (0xF, _, 0x5, 0x5) => {
                let range = x as usize + 1;
                for r in 0..range{
                    self.ram[self.reg_index as usize + r] = self.reg[r];
                }
            },

            // Fx65 - LD Vx, [I]
            // Read registers V0 through Vx from memory starting at location I.
            (0xF, _, 0x6, 0x5) => {
                let range = x as usize + 1;
                for r in 0..range{
                    self.reg[r] = self.ram[self.reg_index as usize + r];
                }
            },

            _ => {
                println!("UNKNOWN INSTR: {:04x}", self.opcode)
            }
        }

        if pc_advance{
            self.pc += 2;
        }

        self.breakpoint = true;
        return self.vram;
    }

    // debug
    pub fn print_file(&self, program_size: usize) {
        let mut pc = self.pc;

        while pc < (program_size as u16 +self.pc) {
            let opcode = (self.ram[pc as usize] as u16) << 8 | (self.ram[pc as usize + 1]) as u16;
            if opcode != self.opcode {
                print!("{:04x} ", opcode);
            }
            else{
                print!("<{:04x}> ", opcode);
            }
            if (pc / 2 + 1) % 8 == 0 {
                println!();
            }
            pc += 2;
        }
    }

    pub fn display(&self){
        // println!("{:#?}", self.vram);
        for row in 0..crate::CHIP8_SCREEN_HEIGHT{
            for col in 0..crate::CHIP8_SCREEN_WIDTH{
                if self.vram[row][col] == true{
                    print!("*");
                }
                else{
                    print!("_");
                }
            }
            print!("\n");

        }
    }
}

