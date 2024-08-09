// ---------------------------------------- //
// Project: chip8-rust                      //
//  Author: Kai NeSmith                     //
//    Date: August 2024                     //
// ---------------------------------------- //
// File: instruction.rs                     //
// Description: CHIP-8 instruction handling //
// ---------------------------------------- //

/// Represents a CHIP-8 instruction.
pub struct Instruction {
    pub raw: u16,
}

impl Instruction {
    /// Gets the first nibble of the instruction.
    pub fn nibble1(&self) -> u8 {
        (self.raw >> 12) as u8
    }

    /// Gets the second nibble of the instruction.
    pub fn nibble2(&self) -> u8 {
        ((self.raw & 0x0F00) >> 8) as u8
    }

    /// Gets the third nibble of the instruction.
    pub fn nibble3(&self) -> u8 {
        ((self.raw & 0x00F0) >> 4) as u8
    }

    /// Gets the fourth nibble of the instruction.
    pub fn nibble4(&self) -> u8 {
        (self.raw & 0x000F) as u8
    }

    /// Gets the X register of the instruction.
    pub fn x(&self) -> usize {
        self.nibble2() as usize
    }

    /// Gets the Y register of the instruction.
    pub fn y(&self) -> usize {
        self.nibble3() as usize
    }

    /// Gets the N number of the instruction.
    pub fn n(&self) -> u8 {
        self.nibble4()
    }

    /// Gets the NN immediate of the instruction.
    pub fn nn(&self) -> u8 {
        (self.raw & 0x00FF) as u8
    }

    /// Gets the NNN immediate address of the instruction.
    pub fn nnn(&self) -> u16 {
        (self.raw & 0x0FFF) as u16
    }
}
