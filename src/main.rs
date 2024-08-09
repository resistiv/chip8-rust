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

mod chip8;
mod instruction;

use crate::chip8::*;

use std::env;
use std::io::Error;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

/// Factor by which to scale the window up.
const SCALE_FACTOR: u32 = 8;
const WINDOW_WIDTH: u32 = SCREEN_WIDTH as u32 * SCALE_FACTOR;
const WINDOW_HEIGHT: u32 = SCREEN_HEIGHT as u32 * SCALE_FACTOR;
const TICKS_PER_FRAME: usize = 10;
const COLOR_BACKGROUND: Color = Color::RGB(0x66, 0x10, 0x4B);
const COLOR_FOREGROUND: Color = Color::RGB(0xDB, 0x22, 0xA1);

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
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'execute;
                },
                Event::KeyDown { keycode: Some(key), .. } => {
                    if let Some(key_val) = process_key(key) {
                        chip8.keypad[key_val] = true;
                    }
                },
                Event::KeyUp { keycode: Some(key), .. } => {
                    if let Some(key_val) = process_key(key) {
                        chip8.keypad[key_val] = false;
                    }
                },
                _ => (),
            }
        }

        // Cycle the interpreter
        for _ in 0 .. TICKS_PER_FRAME {
            chip8.cycle();
        }
        chip8.cycle_special_regs();

        // Draw results
        draw_screen(&chip8, &mut canvas);
    }

    Ok(())
}

/// Updates the screen
fn draw_screen(chip8: &Chip8, canvas: &mut Canvas<Window>) {
    // Clear canvas
    canvas.set_draw_color(COLOR_BACKGROUND);
    canvas.clear();

    // Draw in rects as pixels
    canvas.set_draw_color(COLOR_FOREGROUND);
    for (i, pixel) in chip8.graphics_buffer.iter().enumerate() {
        if *pixel {
            let x = (i % (SCREEN_WIDTH as usize)) as u32;
            let y = (i / (SCREEN_WIDTH as usize)) as u32;
            let rect = Rect::new((x * SCALE_FACTOR) as i32, (y * SCALE_FACTOR) as i32, SCALE_FACTOR, SCALE_FACTOR);
            canvas.fill_rect(rect).unwrap();
        }
    }
    canvas.present();
}

/// Converts a keycode into a keypad index.
fn process_key(key: Keycode) -> Option<usize> {
    match key {
        Keycode::Num1 =>    Some(0x1),
        Keycode::Num2 =>    Some(0x2),
        Keycode::Num3 =>    Some(0x3),
        Keycode::Num4 =>    Some(0xC),
        Keycode::Q =>       Some(0x4),
        Keycode::W =>       Some(0x5),
        Keycode::E =>       Some(0x6),
        Keycode::R =>       Some(0xD),
        Keycode::A =>       Some(0x7),
        Keycode::S =>       Some(0x8),
        Keycode::D =>       Some(0x9),
        Keycode::F =>       Some(0xE),
        Keycode::Z =>       Some(0xA),
        Keycode::X =>       Some(0x0),
        Keycode::C =>       Some(0xB),
        Keycode::V =>       Some(0xF),
        _ =>                None,
    }
}
