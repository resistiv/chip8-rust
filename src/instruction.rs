// ---------------------------------------- //
// Project: chip8-rust                      //
//  Author: Kai NeSmith                     //
//    Date: August 2024                     //
// ---------------------------------------- //
// File: instruction.rs                     //
// Description: CHIP-8 instruction handling //
// ---------------------------------------- //

/// Represents a CHIP-8 opcode.
pub struct Opcode {
    pub instruction: u16,
}

impl Opcode {
    /// Gets the first nibble of the opcode instruction.
    pub fn nibble1(&self) -> u8 {
        ((self.instruction & 0xF000) >> 12) as u8
    }

    /// Gets the second nibble of the opcode instruction.
    pub fn nibble2(&self) -> u8 {
        ((self.instruction & 0x0F00) >> 8) as u8
    }

    /// Gets the third nibble of the opcode instruction.
    pub fn nibble3(&self) -> u8 {
        ((self.instruction & 0x00F0) >> 4) as u8
    }

    /// Gets the fourth nibble of the opcode instruction.
    pub fn nibble4(&self) -> u8 {
        (self.instruction & 0x000F) as u8
    }

    /// Gets the X register of the opcode instruction.
    pub fn x(&self) -> u8 {
        self.nibble2()
    }

    /// Gets the Y register of the opcode instruction.
    pub fn y(&self) -> u8 {
        self.nibble3()
    }

    /// Gets the N number of the opcode instruction.
    pub fn n(&self) -> u8 {
        self.nibble4()
    }

    /// Gets the NN immediate of the opcode instruction.
    pub fn nn(&self) -> u8 {
        (self.instruction & 0x00FF) as u8
    }

    /// Gets the NNN immediate address of the opcode instruction.
    pub fn nnn(&self) -> u16 {
        (self.instruction & 0x0FFF) as u16
    }
}
