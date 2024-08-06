// ---------------------------------------- //
// Project: chip8-rust                      //
//  Author: Kai NeSmith                     //
//    Date: August 2024                     //
// ---------------------------------------- //
// File: main.rs                            //
// Description: Main entry point.           //
// ---------------------------------------- //
// Notes:                                   //
// - This is my first attempt at creating   //
//   ANYTHING in Rust, so I figured I'd     //
//   start with something interesting :)    //
// ---------------------------------------- //

mod system;

use crate::system::Chip8;
use std::env;
use std::io::Error;

/// Main entry point.
fn main() -> Result<(), Error> {
    // Load arguments
    let rom_path: String = env::args().nth(1).expect("No ROM file provided.");

    // Initialize Chip8 system
    let mut chip8: Chip8 = Chip8::new();
    chip8.load_rom(&rom_path)?;

    Ok(())
}
