extern crate sdl2;
extern crate chiprs;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use std::time::{Duration, Instant};

use chiprs::Chip;
use chiprs::display::{Display, DISPLAY_W, DISPLAY_H};

const PIXEL_W: usize = 10;
const PIXEL_H: usize = 10;

pub fn run_sdl_interface(chip: &mut Chip) {
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
    let mut events = sdl_context.event_pump().unwrap();
    let mut next_tick = Instant::now();
    let mut next_step = Instant::now();
    'running: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running;
                }
                Event::KeyDown { keycode: Some(key), .. } => {
                    if key == Keycode::Escape {
                        break 'running;
                    }
                    if let Some(k) = get_chip8_key(key) {
                        chip.keys[k as usize] = true;
                    }
                }
                Event::KeyUp { keycode: Some(key), .. } => {
                    if let Some(k) = get_chip8_key(key) {
                        chip.keys[k as usize] = false;
                    }
                }
                _ => {}
            }
        }

        draw_display(&mut canvas, &chip.display);
        canvas.present();

        let now = Instant::now();
        while next_tick < now {
            chip.tick();
            next_tick += Duration::new(0, 1_000_000_000u32 / 60);
        }
        while next_step < now {
            chip.step().unwrap();
            next_step += Duration::new(0, 1_000_000_000u32 / 500);
        }
    }
}


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

fn get_chip8_key(key: Keycode) -> Option<u8> {
    use sdl2::keyboard::Keycode::*;
    match key {
        Num0 | Kp0 =>
            Some(0x0),
        Num1 | Kp1 =>
            Some(0x1),
        Num2 | Kp2 | Up =>
            Some(0x2),
        Num3 | Kp3 =>
            Some(0x3),
        Num4 | Kp4 | Left =>
            Some(0x4),
        Num5 | Kp5 =>
            Some(0x5),
        Num6 | Kp6 | Right =>
            Some(0x6),
        Num7 | Kp7 =>
            Some(0x7),
        Num8 | Kp8 | Down =>
            Some(0x8),
        Num9 | Kp9 =>
            Some(0x9),
        A =>
            Some(0xA),
        B =>
            Some(0xB),
        C =>
            Some(0xC),
        D =>
            Some(0xD),
        E =>
            Some(0xE),
        F =>
            Some(0xF),
        _ =>
            None,
    }
}
