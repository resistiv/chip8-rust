// ---------------------------------------- //
// Project: chip8-rust                      //
//  Author: Kai NeSmith                     //
//    Date: August 2024                     //
// ---------------------------------------- //
// File: system.rs                          //
// Description: CHIP-8 guts.                //
// ---------------------------------------- //

use crate::instruction::Instruction;
use std::fs::{self, Metadata};
use std::io::{Error, ErrorKind};

/// Represents the program counter position at startup.
const PC_START_ADDRESS: u16 = 0x200;
/// Represents the screen width in pixels.
pub const SCREEN_WIDTH: u8 = 64;
/// Represents the screen height in pixels.
pub const SCREEN_HEIGHT: u8 = 32;
/// Represents amount of RAM in bytes.
const MEMORY_SIZE: u16 = 4096;
/// Represents the size of the system font.
const FONT_SIZE: u16 = 80;
/// Represents the system font.
const FONT_DATA: [u8; FONT_SIZE as usize] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];
/// Represents the system font start address.
const FONT_START_ADDRESS: u16 = 0x50;

// Quirks
const VF_RESET_MATH_QUIRK: bool = true;

/// Represents the underlying CHIP-8 system.
pub struct Chip8 {
    /// Represents general purpose registers V0-VF.
    reg_v: [u8; 16],
    /// Represents 4K of RAM.
    memory: [u8; MEMORY_SIZE as usize],
    /// Stores a memory address for later use in an operation.
    reg_i: u16,
    /// Points to the current instruction in memory.
    pc: u16,
    /// Represents the 16-frame stack.
    stack: [u16; 16],
    /// Points to the current stack frame.
    sp: u8,
    /// Represents the 60Hz delay timer register.
    reg_delay: u8,
    /// Represents the 60Hz sound timer register.
    reg_sound: u8,
    /// Holds the state of the 16 input keys.
    pub keypad: [bool; 16],
    /// Holds the state of the graphics buffer.
    pub graphics_buffer: [bool; ((SCREEN_WIDTH as u16) * (SCREEN_HEIGHT as u16)) as usize],
    /// Holds the current instruction being decoded.
    instr: Instruction,
}

/// Core Chip8 function implementations.
impl Chip8 {
    /// Initializes a new Chip8 struct.
    pub fn new() -> Chip8 {
        let mut chip8: Chip8 = Chip8 {
            reg_v: [0; 16],
            memory: [0; MEMORY_SIZE as usize],
            reg_i: 0,
            pc: PC_START_ADDRESS,
            stack: [0; 16],
            sp: 0,
            reg_delay: 0,
            reg_sound: 0,
            keypad: [false; 16],
            graphics_buffer: [false; ((SCREEN_WIDTH as u16) * (SCREEN_HEIGHT as u16)) as usize],
            instr: Instruction { raw: 0 },
        };
        chip8.load_font();
        chip8
    }
    
    /// Resets the state of the Chip8 struct.
    pub fn reset(&mut self) {
        self.reg_v.fill(0);
        self.memory.fill(0);
        self.reg_i = 0;
        self.pc = PC_START_ADDRESS;
        // Ignore stack, no need to fully clear.
        self.sp = 0;
        self.reg_delay = 0;
        self.reg_sound = 0;
        self.keypad.fill(false);
        self.clear_screen();
        self.instr = Instruction { raw: 0 };
        self.load_font();
    }

    /// Attempts to load a ROM file from disk.
    pub fn load_rom(&mut self, rom_path: &String) -> Result<(), Error> {
        // Get file size and max ROM size (RAM size - PC start)
        let file_attributes: Metadata = fs::metadata(rom_path)?;
        let available_memory: u64 = (MEMORY_SIZE - PC_START_ADDRESS) as u64;
        if file_attributes.len() > available_memory {
            return Err(Error::new(ErrorKind::OutOfMemory, "ROM size exceeded available memory space."));
        }

        // Read in file and write to memory
        let rom_bytes: Vec<u8> = fs::read(rom_path)?;
        let rom_memory_region: &mut [u8] = &mut (self.memory)[PC_START_ADDRESS as usize .. MEMORY_SIZE as usize];
        for (dst, src) in rom_memory_region.iter_mut().zip(&rom_bytes) {
            *dst = *src;
        }

        Ok(())
    }

    /// Attempts to cycle the interpreter by one instruction.
    pub fn cycle(&mut self) {
        self.fetch();
        self.execute();
    }

    /// Decrements the special registers.
    pub fn cycle_special_regs(&mut self) {
        if self.reg_delay > 0 {
            self.reg_delay -= 1;
        }
        if self.reg_sound > 0 {
            self.reg_sound -= 1;
        }
    }

    /// Attempts to load the next opcode and increment the PC.
    fn fetch(&mut self) {
        // Ensure PC won't overrun
        if self.pc >= MEMORY_SIZE {
            panic!("Program counter overflowed valid memory space.");
        }

        let opcode_raw: u16 = (self.memory[self.pc as usize] as u16) << 8
                            | (self.memory[(self.pc + 1) as usize] as u16);
        self.instr = Instruction { raw: opcode_raw };
        self.pc += 2;
    }

    /// Attempts to decode and execute the current instruction.
    fn execute(&mut self) {
        match self.instr.nibble1() {
            0x0 => match self.instr.raw {
                0x00E0 => self.clear_screen(),
                0x00EE => self.return_sub(),
                _ => self.unsupported(), // 0NNN: Execute machine lang sub
            },
            0x1 => self.jump(),
            0x2 => self.call_sub(),
            0x3 => self.skip_equal_imm(),
            0x4 => self.skip_not_equal_imm(),
            0x5 => self.skip_equal_reg(),
            0x6 => self.load_imm(),
            0x7 => self.add_imm(),
            0x8 => match self.instr.nibble4() {
                0x0 => self.load_reg(),
                0x1 => self.or(),
                0x2 => self.and(),
                0x3 => self.xor(),
                0x4 => self.add_reg(),
                0x5 => self.sub_reg(),
                0x6 => self.shift_right(),
                0x7 => self.sub_reg_rev(),
                0xE => self.shift_left(),
                _ => self.unknown(),
            },
            0x9 => self.skip_not_equal_reg(),
            0xA => self.load_addr(),
            0xB => self.jump_offset(),
            0xC => self.rand(),
            0xD => self.draw_sprite(),
            0xE => match self.instr.nn() {
                0x9E => self.skip_key_pressed(),
                0xA1 => self.skip_key_not_pressed(),
                _ => self.unknown(),
            }
            0xF => match self.instr.nn() {
                0x07 => self.load_delay(),
                0x0A => self.await_key(),
                0x15 => self.set_delay(),
                0x18 => self.set_sound(),
                0x1E => self.add_addr(),
                0x29 => self.load_digit_addr(),
                0x33 => self.move_bcd(),
                0x55 => self.move_regs(),
                0x65 => self.load_regs(),
                _ => self.unknown(),
            },
            _ => self.unknown(),
        }
    }

    /// Loads the system font into RAM.
    fn load_font(&mut self) {
        let font_memory_region: &mut [u8] = &mut (self.memory)[FONT_START_ADDRESS as usize .. (FONT_START_ADDRESS + FONT_SIZE) as usize];
        for (dst, src) in font_memory_region.iter_mut().zip(&FONT_DATA) {
            *dst = *src;
        }
    }
}

/// Opcode implementations for Chip8.
impl Chip8 {
    /// Panics on an unknown instruction.
    fn unknown(&self) {
        panic!("Unknown instruction: 0x{:04X}", self.instr.raw);
    }

    /// Panics on an unsupported instruction.
    fn unsupported(&self) {
        panic!("Unsupported instruction: 0x{:04X}", self.instr.raw);
    }

    /// 00E0: Clears the video buffer.
    fn clear_screen(&mut self) {
        self.graphics_buffer.fill(false);
    }

    /// 00EE: Return from subroutine
    fn return_sub(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }

    /// 1NNN: PC = #NNN
    fn jump(&mut self) {
        self.pc = self.instr.nnn();
    }

    /// 2NNN: Push PC to stack, PC = #NNN
    fn call_sub(&mut self) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = self.instr.nnn();
    }

    /// 3XNN: Skip next if VX == #NN
    fn skip_equal_imm(&mut self) {
        if self.reg_v[self.instr.x()] == self.instr.nn() {
            self.pc += 2;
        }
    }

    /// 4XNN: Skip next if VX != #NN
    fn skip_not_equal_imm(&mut self) {
        if self.reg_v[self.instr.x()] != self.instr.nn() {
            self.pc += 2;
        }
    }

    /// 5XY0: Skip next if VX == VY
    fn skip_equal_reg(&mut self) {
        if self.reg_v[self.instr.x()] == self.reg_v[self.instr.y()] {
            self.pc += 2;
        }
    }

    /// 6XNN: VX = #NN
    fn load_imm(&mut self) {
        self.reg_v[self.instr.x()] = self.instr.nn();
    }

    /// 7XNN: VX += #NN (Doesn't set VF on overflow)
    fn add_imm(&mut self) {
        self.reg_v[self.instr.x()] = self.reg_v[self.instr.x()].wrapping_add(self.instr.nn());
    }

    /// 8XY0: VX = VY
    fn load_reg(&mut self) {
        self.reg_v[self.instr.x()] = self.reg_v[self.instr.y()];
    }

    /// 8XY1: VX |= VY
    fn or(&mut self) {
        self.reg_v[self.instr.x()] |= self.reg_v[self.instr.y()];

        if VF_RESET_MATH_QUIRK {
            self.reg_v[0xF] = 0;
        }
    }

    /// 8XY2: VX &= VY
    fn and(&mut self) {
        self.reg_v[self.instr.x()] &= self.reg_v[self.instr.y()];

        if VF_RESET_MATH_QUIRK {
            self.reg_v[0xF] = 0;
        }
    }
    
    /// 8XY3: VX ^= VY
    fn xor(&mut self) {
        self.reg_v[self.instr.x()] ^= self.reg_v[self.instr.y()];
        
        if VF_RESET_MATH_QUIRK {
            self.reg_v[0xF] = 0;
        }
    }

    /// 8XY4: VX += VY (Sets VF on overflow)
    fn add_reg(&mut self) {
        let (result, carry) = self.reg_v[self.instr.x()].overflowing_add(self.reg_v[self.instr.y()]);

        self.reg_v[self.instr.x()] = result;
        self.reg_v[0xF] = carry as u8;
    }

    /// 8XY5: VX -= VY (Sets VF on borrow)
    fn sub_reg(&mut self) {
        let (result, overflow) = self.reg_v[self.instr.x()].overflowing_sub(self.reg_v[self.instr.y()]);

        self.reg_v[self.instr.x()] = result;
        self.reg_v[0xF] = !overflow as u8;
    }

    /// 8XY6: VX = VY >> 1 (VF is out bit)
    fn shift_right(&mut self) {
        let out_bit: u8 = self.reg_v[self.instr.y()] & 0x1;
        self.reg_v[self.instr.x()] = self.reg_v[self.instr.y()] >> 1;
        self.reg_v[0xF] = out_bit;
    }

    /// 8XY7: VX = VY - VX (Sets VF on borrow)
    fn sub_reg_rev(&mut self) {
        let (result, overflow) = self.reg_v[self.instr.y()].overflowing_sub(self.reg_v[self.instr.x()]);
        
        self.reg_v[self.instr.x()] = result;
        self.reg_v[0xF] = !overflow as u8;
    }

    /// 8XYE: VX = VY << 1 (VF is out bit)
    fn shift_left(&mut self) {
        let out_bit: u8 = (self.reg_v[self.instr.y()] >> 7) & 0x1;
        self.reg_v[self.instr.x()] = self.reg_v[self.instr.y()] << 1;
        self.reg_v[0xF] = out_bit;
    }

    /// 9XY0: Skip next if VX != VY
    fn skip_not_equal_reg(&mut self) {
        if self.reg_v[self.instr.x()] != self.reg_v[self.instr.y()] {
            self.pc += 2;
        }
    }

    /// ANNN: I = #NNN
    fn load_addr(&mut self) {
        self.reg_i = self.instr.nnn();
    }

    /// BNNN: PC = #NNN + V0
    fn jump_offset(&mut self) {
        self.pc = self.instr.nnn() + self.reg_v[0] as u16;
    }
    
    /// CXNN: VX = rand & #NN
    fn rand(&mut self) {
        self.reg_v[self.instr.x()] = rand::random::<u8>() & self.instr.nn();
    }

    /// DXYN: Draws a sprite at VX, VY, size of N-bytes, sourced from the address in register I. Also sets VF if any ON pixels are set to OFF.
    fn draw_sprite(&mut self) {
        // Extract start coords from registers
        let x: u8 = self.reg_v[self.instr.x() as usize] & (SCREEN_WIDTH - 1) as u8;
        let y: u8 = self.reg_v[self.instr.y() as usize] & (SCREEN_HEIGHT - 1) as u8;

        // Clear VF flag
        self.reg_v[0xF] = 0;

        // Populate pixels
        for row in 0 .. self.instr.n() {
            let pixel_blob = self.memory[(self.reg_i + row as u16) as usize];
            for col in 0 .. 8 {
                if (pixel_blob & (0x80 >> col)) != 0 {
                    let px = (x + col) as usize;
                    let py = (y + row) as usize;

                    if px < SCREEN_WIDTH.into() && py < SCREEN_HEIGHT.into()
                    {
                        let index = (SCREEN_WIDTH as usize) * py + px;
                        if self.graphics_buffer[index]
                        {
                            self.reg_v[0xF] = 1;
                        }
                        self.graphics_buffer[index] ^= true;
                    }
                }
            }
        }
    }

    /// EX9E: Skip next if Key[VX] pressed
    fn skip_key_pressed(&mut self) {
        if self.keypad[self.reg_v[self.instr.x()] as usize] {
            self.pc += 2;
        }
    }

    /// EXA1: Skip next if Key[VX] not pressed
    fn skip_key_not_pressed(&mut self) {
        if !self.keypad[self.reg_v[self.instr.x()] as usize] {
            self.pc += 2;
        }
    }

    /// FX07: VX = DELAY
    fn load_delay(&mut self) {
        self.reg_v[self.instr.x()] = self.reg_delay;
    }

    /// FX0A: Await key, VX = Key pressed.
    fn await_key(&mut self) {
        for key in 0x0 ..= 0xF {
            if self.keypad[key] {
                self.reg_v[self.instr.x()] = key as u8;
                return;
            }
        }
        self.pc -= 2;
    }

    /// FX15: DELAY = VX
    fn set_delay(&mut self) {
        self.reg_delay = self.reg_v[self.instr.x()];
    }

    /// FX18: SOUND = VX
    fn set_sound(&mut self) {
        self.reg_sound = self.reg_v[self.instr.x()];
    }

    /// FX1E: I += VX
    fn add_addr(&mut self) {
        self.reg_i = self.reg_i.wrapping_add(self.reg_v[self.instr.x()] as u16);
    }

    /// FX29: I = Font[VX]
    fn load_digit_addr(&mut self) {
        // Sanity
        if self.reg_v[self.instr.x()] > 0xF {
            panic!("Attempted to fetch font digit greater than 0xF.");
        }

        self.reg_i = FONT_START_ADDRESS + (self.reg_v[self.instr.x()] as u16 * 5);
    }

    /// FX33: [I..I+2] = BCD of VX
    fn move_bcd(&mut self) {
        let mut value: u8 = self.reg_v[self.instr.x()];
        self.memory[(self.reg_i + 2) as usize] = value % 10;
        value /= 10;
        self.memory[(self.reg_i + 1) as usize] = value % 10;
        value /= 10;
        self.memory[self.reg_i as usize] = value;
    }

    /// FX55: [I..I+X] = [V0..VX]; I += X + 1
    fn move_regs(&mut self) {
        for reg in 0 ..= self.instr.x() {
            self.memory[self.reg_i as usize + reg] = self.reg_v[reg];
        }

        self.reg_i += self.instr.x() as u16 + 1;
    }

    /// FX65: [V0..VX] = [I..I+X]; I += X + 1
    fn load_regs(&mut self) {
        for reg in 0 ..= self.instr.x() {
            self.reg_v[reg] = self.memory[self.reg_i as usize + reg];
        }

        self.reg_i += self.instr.x() as u16 + 1;
    }
}
