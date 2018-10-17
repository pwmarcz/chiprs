extern crate sdl2;
extern crate rand;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use std::time::Duration;

pub mod instr;
pub mod memory;
pub mod display;
pub mod chip;

use display::{Display, DISPLAY_W, DISPLAY_H};
use chip::Chip;

const PIXEL_W: usize = 10;
const PIXEL_H: usize = 10;

fn draw_display(canvas: &mut WindowCanvas,
                display: &Display) {
    canvas.set_draw_color(Color::RGB(200, 200, 200));
    canvas.clear();
    canvas.set_draw_color(Color::RGB(100, 100, 100));
    for i in 0..DISPLAY_W {
        for j in 0..DISPLAY_H {
            if display.at(i, j) {
                canvas.fill_rect(Rect::new(
                    (i * PIXEL_W) as i32, (j * PIXEL_H) as i32,
                    PIXEL_W as u32, PIXEL_H as u32)).unwrap();
            }
        }
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("chiprs",
                                        (DISPLAY_W*PIXEL_W) as u32,
                                        (DISPLAY_H*PIXEL_H) as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut chip = Chip::new();
    chip.memory.load_font();

    use instr::Instr::*;

    chip.memory.load_program(0x200, &[
        LD_R_B(0, 3),
        LD_R_B(1, 10),
        LD_R_B(2, 0xA),
        LD_F_R(2),
        DRW(0, 1, 5),

        LD_R_B(0, 8),
        LD_R_B(1, 10),
        LD_R_B(2, 0x7),
        LD_F_R(2),
        DRW(0, 1, 5),

        JP(0xFFF),
    ]);
    chip.pc = 0x200;

    let mut events = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        draw_display(&mut canvas, &chip.display);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        if chip.pc != 0xFFF {
            chip.step().unwrap();
        }
    }
}
