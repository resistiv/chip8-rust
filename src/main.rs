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
mod instruction;

use crate::system::*;

use std::env;
use std::io::Error;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

/// Factor by which to scale the window up.
const SCALE_FACTOR: u32 = 8;
const WINDOW_WIDTH: u32 = SCREEN_WIDTH as u32 * SCALE_FACTOR;
const WINDOW_HEIGHT: u32 = SCREEN_HEIGHT as u32 * SCALE_FACTOR;
const TICKS_PER_FRAME: usize = 10;

/// Main entry point.
fn main() -> Result<(), Error> {
    // Load arguments
    let rom_path: String = env::args().nth(1).expect("No ROM file provided.");

    // Initialize SDL window
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(
            "chip8-rust",
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        )
        .position_centered()
        .build()
        .unwrap();

    // Initialize drawing canvas
    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .accelerated()
        .build()
        .unwrap();
    canvas.clear();
    canvas.present();

    // Initialize event pump
    let mut event_pump = sdl_context
        .event_pump()
        .unwrap();

    // Initialize Chip8 system
    let mut chip8: Chip8 = Chip8::new();
    chip8.load_rom(&rom_path)?;

    // Execution loop
    'execute: loop {
        for event in event_pump.poll_iter() {

        }

        // Cycle the interpreter
        for _ in 0 .. TICKS_PER_FRAME {
            chip8.cycle();
        }
        chip8.cycle_special_regs();

        // Draw results
        draw_screen(&chip8, &mut canvas);
    }

    // Execution loop
    // let interval = Duration::from_micros(2000);
    // let mut next_time = Instant::now() + interval;
    // loop {
    //     chip8.cycle();
    //     sleep(next_time - Instant::now());
    //     next_time += interval;
    // }

    Ok(())
}

/// Updates the screen
fn draw_screen(chip8: &Chip8, canvas: &mut Canvas<Window>) {
    // Clear canvas (black)
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    // Draw in rects as pixels
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for (i, pixel) in chip8.graphics_buffer.iter().enumerate() {
        if *pixel {
            let x = (i % (SCREEN_WIDTH as usize)) as u32;
            let y = (i / (SCREEN_HEIGHT as usize)) as u32;
            let rect = Rect::new(x as i32, y as i32, 1, 1);
            canvas.fill_rect(rect).unwrap();
        }
    }
    canvas.present();
}
