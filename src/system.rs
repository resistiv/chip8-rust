// ---------------------------------------- //
// Project: chip8-rust                      //
//  Author: Kai NeSmith                     //
//    Date: August 2024                     //
// ---------------------------------------- //
// File: system.rs                          //
// Description: CHIP-8 guts.                //
// ---------------------------------------- //

// TODO: Add configurable constructor using enum with common configurations

use std::fs::{self, Metadata};
use std::io::{Error, ErrorKind};

/// Represents the program counter position at startup.
const PC_START_ADDRESS: u16 = 0x200;
/// Represents the screen width in pixels.
const SCREEN_WIDTH: u16 = 64;
/// Represents the screen height in pixels.
const SCREEN_HEIGHT: u16 = 32;
/// Represents amount of RAM in bytes.
const MEMORY_SIZE: u16 = 4096;

/// Represents the underlying CHIP-8 system.
pub struct Chip8 {
    /// Represents general purpose registers V0-VF.
    registers: [u8; 16],
    /// Represents 4K of RAM.
    memory: [u8; MEMORY_SIZE as usize],
    /// Stores a memory address for later use in an operation.
    index_register: u16,
    /// Points to the current instruction in memory.
    program_counter: u16,
    /// Represents the 16-frame stack.
    stack: [u16; 16],
    /// Points to the current stack frame.
    stack_pointer: u8,
    /// Represents the 60Hz delay timer register.
    delay_timer: u8,
    /// Represents the 60Hz sound timer register.
    sound_timer: u8,
    /// Holds the state of the 16 input keys.
    keypad: [bool; 16],
    /// Holds the state of the graphics buffer.
    graphics_buffer: [u32; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize],
    /// Holds the current opcode being decoded.
    opcode: u16,
}

/// Available configurations for the Chip8 struct.
pub enum Chip8Config {
    Standard,
    // Eti660_64x48,
    // Eti660_64x64,
    // SuperChip48,
}

impl Chip8 {
    /// Initializes a new Chip8 struct.
    pub fn new() -> Chip8 {
        Chip8 {
            registers: [0; 16],
            memory: [0; MEMORY_SIZE as usize],
            index_register: 0,
            program_counter: PC_START_ADDRESS,
            stack: [0; 16],
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [false; 16],
            graphics_buffer: [0; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize],
            opcode: 0,
        }
    }
    
    /// Resets the state of the Chip8 struct.
    pub fn reset(&mut self) {
        self.registers.fill(0);
        self.memory.fill(0);
        self.index_register = 0;
        self.program_counter = PC_START_ADDRESS;
        // Ignore stack, no need to fully clear.
        self.stack_pointer = 0;
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.keypad.fill(false);
        self.graphics_buffer.fill(0);
        self.opcode = 0;

        // TODO: Add font load call.
    }

    // TODO: Add font load function.

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

        // DEBUG: Write memory to dump file, check max size file to ensure all bytes are copied
        // fs::write("load_rom_memdump.bin", self.memory)?;
        // fs::write("testbed.bin", [0xFF; 3584])?;

        Ok(())
    }
}
